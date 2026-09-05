//! Lazy `map<K, V>`: store entry-LEN spans, materialise a [`MapField`] on first get.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::cell::UnsafeCell;
use ::core::hash::Hash;
use ::puroro::DecodeError;

use super::MapKey;
use super::field::{MapField, MapFieldRef};
use crate::decode::WireSpan;
use crate::fields::shared::{FieldDeallocate, MessageCommon};
use crate::fields::wire::repeated_element::{RepeatedElement, RepeatedElementMerge};

/// Offset list of map-entry LENs, optionally promoted to a [`MapField`].
///
/// The parent scan only appends [`WireSpan`]s. The first [`bind`](Self::bind)
/// decodes them (last-wins per key). A later store into an already-ready map
/// merges that one entry immediately.
pub struct LazyMapField<K, V, const FIELD: u32, A: Allocator = Global>
where
    K: MapKey,
    V: RepeatedElement,
{
    spans: AllocVec<WireSpan, A>,
    ready: UnsafeCell<Option<MapField<K, V, FIELD, A>>>,
}

impl<K, V, const FIELD: u32, A> LazyMapField<K, V, FIELD, A>
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            spans: AllocVec::new_in(alloc),
            ready: UnsafeCell::new(None),
        }
    }

    /// Record one map-entry LEN. If already materialised, merge it now.
    pub fn store_span<P>(
        &mut self,
        span: WireSpan,
        wire: &[u8],
        common: &MessageCommon<P, A>,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
    {
        self.spans.push(span);
        if let Some(map) = self.ready.get_mut() {
            let payload = span.slice(wire)?;
            map.merge_entry_body(&mut &payload[..], common.alloc.clone(), 0)?;
        }
        Ok(())
    }

    /// Materialise on first call, then return the catalog map view.
    pub fn bind<'a, P>(
        &'a self,
        wire: &'a [u8],
        common: &'a MessageCommon<P, A>,
    ) -> Result<MapFieldRef<'a, K, V, FIELD, A, P>, DecodeError>
    where
        A: Clone,
        K: RepeatedElementMerge<A>,
        V: RepeatedElementMerge<A>,
        K::Element<A>: Eq + Hash,
    {
        // SAFETY: callers only `bind` after the parent stream is finished, and
        // never overlap it with `store_span` (`&mut self`).
        let ready = unsafe { &mut *self.ready.get() };
        if ready.is_none() {
            let mut map = MapField::new_in(common.alloc.clone());
            for &span in &self.spans {
                let payload = match span.slice(wire) {
                    Ok(p) => p,
                    Err(e) => {
                        FieldDeallocate::deallocate(&mut map, common);
                        return Err(e);
                    }
                };
                if let Err(e) = map.merge_entry_body(&mut &payload[..], common.alloc.clone(), 0) {
                    FieldDeallocate::deallocate(&mut map, common);
                    return Err(e);
                }
            }
            *ready = Some(map);
        }
        Ok(ready.as_ref().expect("map materialised").bind(common))
    }

    /// Release a materialised [`MapField`]. Spans need no teardown.
    pub fn deallocate<P>(&mut self, common: &MessageCommon<P, A>) {
        if let Some(mut map) = self.ready.get_mut().take() {
            FieldDeallocate::deallocate(&mut map, common);
        }
    }
}
