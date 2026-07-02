//! Repeated LEN field wrapper (`repeated string`, `repeated bytes`, …).
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]; each element is itself allocator-less
//! ([`LenProtoType::Storage`]). Releasing therefore drains and frees every
//! element first, then frees the buffer.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedVec;

use crate::encode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::len::{self, LenProtoType};
use super::presence::PresenceBits;

/// Repeated field whose elements are length-delimited records (one tag per element).
pub struct RepeatedLenField<T: LenProtoType, A: Allocator> {
    values: ManuallyDrop<UnmanagedVec<T::Storage>>,
    _marker: PhantomData<A>,
}

impl<T: LenProtoType, A: Allocator> RepeatedLenField<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Storage] {
        &self.values
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Binds this field to its message `common` state (for the allocator),
    /// producing a short-lived [`RepeatedLenFieldMut`] view.
    ///
    /// This is the entry point for every mutation (`push_in` / `merge` /
    /// `clear`): generated code calls `field.bind(&mut common).…()` instead of
    /// threading the allocator through each method. Repeated fields carry no
    /// presence bit, so the view needs only `common`.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> RepeatedLenFieldMut<'f, 'c, T, Pb, A> {
        RepeatedLenFieldMut::new(self, common)
    }

    pub fn encoded_len(&self, field: u32) -> usize {
        self.values
            .iter()
            .map(|v| encode::encoded_len_len_field(field, T::as_bytes(v).len()))
            .sum()
    }

    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B) {
        for v in self.values.iter() {
            encode::encode_len_field(field, T::as_bytes(v), buf);
        }
    }

    /// Releases every element and the backing buffer through owned `alloc`
    /// clones. Terminal; call once from the owning message's `Drop`.
    pub fn deallocate(&mut self, alloc: A)
    where
        A: Clone,
    {
        // SAFETY: called once; owned clones of the message allocator own the
        // buffer and every element.
        let mut v = unsafe { ManuallyDrop::take(&mut self.values) };
        {
            let mut g = unsafe { v.with_alloc(alloc.clone()) };
            while let Some(elem) = g.pop() {
                unsafe { T::deallocate(elem, alloc.clone()) };
            }
        }
        unsafe { v.deallocate(alloc) };
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a repeated LEN field to its message common state,
/// produced by [`RepeatedLenField::bind`].
///
/// Bundles the element buffer with the allocator context so that generated code
/// can mutate through a single call. Repeated fields have no presence bit, so
/// the view carries only `common` (for the allocator). Every method consumes
/// the view, so a fresh `bind` precedes each mutation.
pub struct RepeatedLenFieldMut<'f, 'c, T: LenProtoType, Pb: PresenceBits, A: Allocator> {
    field: &'f mut RepeatedLenField<T, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T: LenProtoType, Pb: PresenceBits, A: Allocator>
    RepeatedLenFieldMut<'f, 'c, T, Pb, A>
{
    #[inline]
    fn new(field: &'f mut RepeatedLenField<T, A>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    /// Appends an element built from a payload slice, allocating through owned
    /// clones of the message allocator.
    pub fn push_in(self, v: impl AsRef<[u8]>) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        let stored = T::store_from_slice(v.as_ref(), alloc.clone())?;
        // SAFETY: owned clones of the message allocator are interchangeable and
        // own this vector's buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.push(stored);
        Ok(())
    }

    /// Merges one LEN element: decodes and appends it.
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let alloc = self.common.alloc.clone();
        let stored = T::decode(buf, alloc.clone())?;
        // SAFETY: owned clones of the message allocator own this vector's buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.push(stored);
        Ok(())
    }

    /// Drops every element and empties the vector (keeps the buffer capacity).
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: owned clones of the message allocator own this vector's buffer
        // and every element.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        while let Some(elem) = g.pop() {
            unsafe { T::deallocate(elem, alloc.clone()) };
        }
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedLen<T, A> = RepeatedLenField<T, A>;
pub type RepeatedString<A> = RepeatedLenField<len::ProtoString, A>;
pub type RepeatedBytes<A> = RepeatedLenField<len::ProtoBytes, A>;
