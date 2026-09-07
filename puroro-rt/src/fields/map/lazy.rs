//! Lazy `map<K, V>`: store entry-LEN spans, materialise a [`MapField`] on first get.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::cell::UnsafeCell;
use ::core::hash::Hash;
use ::puroro::DecodeError;

use super::MapKey;
use super::field::{MapField, MapFieldRef};
use crate::decode::WireSpan;
use crate::fields::shared::{FieldDeallocate, MessageCommonAlloc};
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
    pub fn store_span<Cx: MessageCommonAlloc<Alloc = A>>(
        &mut self,
        span: WireSpan,
        wire: &[u8],
        common: &Cx,
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
            map.merge_entry_body(&mut &payload[..], common.clone_alloc(), 0)?;
        }
        Ok(())
    }

    /// Materialise on first call, then return the catalog map view.
    pub fn bind<'a, Cx: MessageCommonAlloc<Alloc = A>>(
        &'a self,
        wire: &'a [u8],
        common: &'a Cx,
    ) -> Result<MapFieldRef<'a, K, V, FIELD, A, Cx>, DecodeError>
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
            let mut map = MapField::new_in(common.clone_alloc());
            for &span in &self.spans {
                let payload = match span.slice(wire) {
                    Ok(p) => p,
                    Err(e) => {
                        FieldDeallocate::deallocate(&mut map, common);
                        return Err(e);
                    }
                };
                if let Err(e) = map.merge_entry_body(&mut &payload[..], common.clone_alloc(), 0) {
                    FieldDeallocate::deallocate(&mut map, common);
                    return Err(e);
                }
            }
            *ready = Some(map);
        }
        Ok(ready.as_ref().expect("map materialised").bind(common))
    }

    /// Release a materialised [`MapField`]. Spans need no teardown.
    pub fn deallocate<Cx: MessageCommonAlloc<Alloc = A>>(&mut self, common: &Cx) {
        if let Some(mut map) = self.ready.get_mut().take() {
            FieldDeallocate::deallocate(&mut map, common);
        }
    }
}
