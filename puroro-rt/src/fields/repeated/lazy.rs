//! Lazy `repeated` LEN: store payload spans, materialise a [`RepeatedField`]
//! on first get.
//!
//! Invalid UTF-8 currently fails that getter (same as eager `merge`). Parking a
//! bad occurrence in unknown fields, like a closed-enum unknown, is deferred.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::cell::UnsafeCell;
use ::puroro::DecodeError;
use ::unmanaged::{DeallocateIn, UnmanagedString, UnmanagedVec};

use super::encoding::RepeatedEncoding;
use super::field::{RepeatedField, RepeatedFieldRef};
use crate::decode::WireSpan;
use crate::fields::shared::{FieldDeallocate, MessageCommon};
use crate::fields::wire::len::{ProtoBytes, ProtoString, ProtoStringUnchecked};
use crate::fields::wire::repeated_element::RepeatedElement;

/// Decode a scanned LEN **body** (length prefix already consumed).
pub trait DecodeLenBody<A: Allocator>: RepeatedElement {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<Self::Element<A>, DecodeError>;
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoString {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedString<A>, DecodeError> {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc.clone());
        vec.extend_from_slice(bytes);
        match UnmanagedString::from_utf8(UnmanagedVec::from_vec(vec)) {
            Ok(s) => Ok(s),
            Err(v) => {
                // SAFETY: `alloc` owns the buffer produced above.
                unsafe { v.deallocate(&alloc) };
                Err(DecodeError::InvalidUtf8)
            }
        }
    }
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoBytes {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc);
        vec.extend_from_slice(bytes);
        Ok(UnmanagedVec::from_vec(vec))
    }
}

impl<A: Allocator + Clone> DecodeLenBody<A> for ProtoStringUnchecked {
    fn decode_len_body(bytes: &[u8], alloc: A) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        ProtoBytes::decode_len_body(bytes, alloc)
    }
}

/// Offset list of element LENs, optionally promoted to a [`RepeatedField`].
///
/// The parent scan only appends [`WireSpan`]s. The first [`bind`](Self::bind)
/// decodes them in order. A later store into an already-ready field appends
/// that one occurrence immediately.
pub struct LazyRepeatedField<T, E, const FIELD: u32, A: Allocator = Global>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
{
    spans: AllocVec<WireSpan, A>,
    ready: UnsafeCell<Option<RepeatedField<T, E, FIELD, A>>>,
}

impl<T, E, const FIELD: u32, A> LazyRepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            spans: AllocVec::new_in(alloc),
            ready: UnsafeCell::new(None),
        }
    }

    /// Record one element LEN. If already materialised, append it now.
    pub fn store_span<P>(
        &mut self,
        span: WireSpan,
        wire: &[u8],
        common: &MessageCommon<P, A>,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
        T: DecodeLenBody<A>,
    {
        self.spans.push(span);
        if let Some(field) = self.ready.get_mut() {
            append_span(field, span, wire, common.alloc.clone())?;
        }
        Ok(())
    }

    /// Materialise on first call, then return the catalog repeated view.
    pub fn bind<'a, P>(
        &'a self,
        wire: &'a [u8],
        common: &'a MessageCommon<P, A>,
    ) -> Result<RepeatedFieldRef<'a, T, E, FIELD, A, P>, DecodeError>
    where
        A: Clone,
        T: DecodeLenBody<A>,
        T::Element<A>: DeallocateIn<A>,
    {
        // SAFETY: callers only `bind` after the parent stream is finished, and
        // never overlap it with `store_span` (`&mut self`).
        let ready = unsafe { &mut *self.ready.get() };
        if ready.is_none() {
            let mut field = RepeatedField::new_in(common.alloc.clone());
            for &span in &self.spans {
                if let Err(e) = append_span(&mut field, span, wire, common.alloc.clone()) {
                    FieldDeallocate::deallocate(&mut field, common);
                    return Err(e);
                }
            }
            *ready = Some(field);
        }
        Ok(ready
            .as_ref()
            .expect("repeated field materialised")
            .bind(common))
    }

    /// Release a materialised [`RepeatedField`]. Spans need no teardown.
    pub fn deallocate<P>(&mut self, common: &MessageCommon<P, A>)
    where
        T::Element<A>: DeallocateIn<A>,
    {
        if let Some(mut field) = self.ready.get_mut().take() {
            FieldDeallocate::deallocate(&mut field, common);
        }
    }
}

fn append_span<T, E, const FIELD: u32, A>(
    field: &mut RepeatedField<T, E, FIELD, A>,
    span: WireSpan,
    wire: &[u8],
    alloc: A,
) -> Result<(), DecodeError>
where
    T: DecodeLenBody<A>,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
{
    let payload = span.slice(wire)?;
    let elem = T::decode_len_body(payload, alloc.clone())?;
    field.push_in(elem, alloc);
    Ok(())
}
