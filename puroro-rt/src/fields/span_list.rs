//! Lazy span list shared by repeated LEN fields and maps.
//!
//! Both store [`WireSpan`] offsets during the parent scan and build the real
//! container on first `bind`. The apply closure is the only difference: append
//! one element, or merge one map entry. A failed `bind` releases the partial
//! container here, so that path is not copied at each field.

use ::core::cell::UnsafeCell;

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::puroro::DecodeError;

use crate::decode::WireSpan;
use crate::fields::shared::{FieldDeallocate, MessageCommonAlloc};

/// Offset list plus an optional materialised `Ready` value.
///
/// Public only because it is [`MapLayout::Storage`](crate::MapLayout::Storage)
/// and [`RepeatedLayout::Storage`](crate::RepeatedLayout::Storage). Callers use
/// `store_span` / `bind` on the field, not this type.
#[doc(hidden)]
pub struct SpanList<Ready, A: Allocator> {
    spans: AllocVec<WireSpan, A>,
    ready: UnsafeCell<Option<Ready>>,
}

impl<Ready, A: Allocator> SpanList<Ready, A> {
    /// Empty list. `alloc` owns the span vector.
    #[inline]
    pub(crate) fn new(alloc: A) -> Self {
        Self {
            spans: AllocVec::new_in(alloc),
            ready: UnsafeCell::new(None),
        }
    }

    /// Rebuild storage after cloning the span vector and the materialised value.
    #[inline]
    pub(crate) fn from_parts(spans: AllocVec<WireSpan, A>, ready: Option<Ready>) -> Self {
        Self {
            spans,
            ready: UnsafeCell::new(ready),
        }
    }

    #[inline]
    pub(crate) fn spans(&self) -> &AllocVec<WireSpan, A> {
        &self.spans
    }

    /// Shared view of the materialised value, if one has been built.
    ///
    /// # Safety
    /// `ready` is written only through `&mut self` ([`Self::store_span`]) or
    /// through [`Self::materialize`] after the parent stream is finished. The
    /// caller must not overlap this shared borrow with those writes.
    #[inline]
    pub(crate) unsafe fn ready_ref(&self) -> Option<&Ready> {
        unsafe { (*self.ready.get()).as_ref() }
    }

    /// Take the materialised value so the caller can release it explicitly.
    ///
    /// The span vector itself drops with `self` (it stores its allocator).
    #[inline]
    pub(crate) fn take_ready(&mut self) -> Option<Ready> {
        self.ready.get_mut().take()
    }

    /// Record `span`. If already materialised, apply it immediately.
    ///
    /// A failed apply leaves the span in the list and leaves `ready` in place.
    pub(crate) fn store_span<F>(&mut self, span: WireSpan, apply: F) -> Result<(), DecodeError>
    where
        F: FnOnce(&mut Ready, WireSpan) -> Result<(), DecodeError>,
    {
        self.spans.push(span);
        if let Some(ready) = self.ready.get_mut() {
            apply(ready, span)?;
        }
        Ok(())
    }

    /// Build `Ready` from every stored span on the first call.
    ///
    /// `new_empty` runs only when nothing is materialised yet, so a second
    /// `bind` does not construct (and then drop) an unused container.
    /// On apply failure the partial value is released through `common` and
    /// not stored.
    ///
    /// # Safety
    /// Callers only materialise after the parent stream is finished, and never
    /// overlap this with [`Self::store_span`] (`&mut self`). `ready` is not
    /// borrowed elsewhere for the duration of this call.
    pub(crate) unsafe fn materialize<'a, C, N, F>(
        &'a self,
        common: &C,
        new_empty: N,
        mut apply: F,
    ) -> Result<&'a Ready, DecodeError>
    where
        C: MessageCommonAlloc<Alloc = A>,
        N: FnOnce() -> Ready,
        F: FnMut(&mut Ready, WireSpan) -> Result<(), DecodeError>,
        Ready: FieldDeallocate<C>,
    {
        // SAFETY: caller guarantees `ready` is not borrowed for this call.
        let ready = unsafe { &mut *self.ready.get() };
        if ready.is_none() {
            let mut value = new_empty();
            for &span in &self.spans {
                if let Err(e) = apply(&mut value, span) {
                    FieldDeallocate::deallocate(&mut value, common);
                    return Err(e);
                }
            }
            *ready = Some(value);
        }
        Ok(ready.as_ref().expect("span list materialised"))
    }
}
