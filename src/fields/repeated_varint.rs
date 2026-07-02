//! Repeated varint field wrapper — generic over wire type and encode policy.
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]. Growth and release borrow the message allocator.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedVec;
use ::unmanaged::vec::VecGuard;

use crate::decode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

use super::repeated_encoding::RepeatedVarintEncoding;
use super::varint::{self, VarintProtoType};

/// Repeated field whose elements share a varint wire representation.
///
/// Param `E` is [`Packed`](super::repeated_encoding::Packed) or
/// [`Expanded`](super::repeated_encoding::Expanded) — affects **encode only**.
/// [`merge`](Self::merge) accepts both packed and expanded wire forms.
pub struct RepeatedVarintField<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator> {
    values: ManuallyDrop<UnmanagedVec<T::Value>>,
    _marker: PhantomData<(E, A)>,
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator> RepeatedVarintField<T, E, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Value] {
        &self.values
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns a growable handle over the elements, borrowing `alloc`.
    pub fn values_mut<'a>(&'a mut self, alloc: &'a A) -> VecGuard<'a, T::Value, &'a A> {
        // SAFETY: `alloc` owns this vector's buffer for its whole lifetime.
        unsafe { self.values.with_alloc(alloc) }
    }

    /// Empties the vector (keeps the buffer capacity).
    pub fn clear(&mut self, alloc: &A) {
        // SAFETY: `alloc` owns this vector's buffer.
        let mut g = unsafe { self.values.with_alloc(alloc) };
        g.clear();
    }

    pub fn encoded_len(&self, field: u32) -> usize {
        if self.values.is_empty() {
            0
        } else {
            E::encoded_len::<T>(field, self.as_slice())
        }
    }

    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B) {
        if !self.values.is_empty() {
            E::encode::<B, T>(field, self.as_slice(), buf);
        }
    }

    /// Merges one packed (LEN) or expanded (VARINT) occurrence — appends element(s).
    pub fn merge<B: Buf>(
        &mut self,
        alloc: &A,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        // SAFETY: `alloc` owns this vector's buffer.
        let mut g = unsafe { self.values.with_alloc(alloc) };
        match wire_type {
            WireType::Len => {
                let len = decode::decode_varint(buf)? as usize;
                if buf.remaining() < len {
                    return Err(DecodeError::TruncatedMessage);
                }
                let mut sub = buf.take(len);
                while sub.has_remaining() {
                    let raw = decode::decode_varint(&mut sub)?;
                    g.push(T::decode_wire(raw)?);
                }
            }
            WireType::Varint => {
                let raw = decode::decode_varint(buf)?;
                g.push(T::decode_wire(raw)?);
            }
            _ => return Err(DecodeError::InvalidTag),
        }
        Ok(())
    }

    /// Releases the backing buffer through `alloc`. Terminal; call once from the
    /// owning message's `Drop`.
    pub fn deallocate(&mut self, alloc: &A) {
        // SAFETY: called once; `alloc` owns the buffer. Elements are `Copy`
        // scalars with no per-element cleanup.
        let v = unsafe { ManuallyDrop::take(&mut self.values) };
        unsafe { v.deallocate(alloc) };
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedVarint<T, E, A> = RepeatedVarintField<T, E, A>;

pub type RepeatedPackedVarintField<T, A> =
    RepeatedVarintField<T, super::repeated_encoding::Packed, A>;
pub type RepeatedExpandedVarintField<T, A> =
    RepeatedVarintField<T, super::repeated_encoding::Expanded, A>;

pub type RepeatedPackedInt32<A> = RepeatedPackedVarintField<varint::ProtoInt32, A>;
pub type RepeatedExpandedInt32<A> = RepeatedExpandedVarintField<varint::ProtoInt32, A>;
