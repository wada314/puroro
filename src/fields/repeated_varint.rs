//! Repeated varint field wrapper — generic over wire type and encode policy.

use ::core::marker::PhantomData;

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AVec;

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
    values: AVec<T::Value, A>,
    _encoding: PhantomData<E>,
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator> RepeatedVarintField<T, E, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: AVec::new_in(alloc),
            _encoding: PhantomData,
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

    pub fn push(&mut self, v: T::Value) {
        self.values.push(v);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn encoded_len(&self, field: u32) -> usize {
        if self.values.is_empty() {
            0
        } else {
            E::encoded_len::<T>(field, &self.values)
        }
    }

    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B) {
        if !self.values.is_empty() {
            E::encode::<B, T>(field, &self.values, buf);
        }
    }

    /// Merges one packed (LEN) or expanded (VARINT) occurrence — appends element(s).
    pub fn merge<B: Buf>(&mut self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        match wire_type {
            WireType::Len => {
                let len = decode::decode_varint(buf)? as usize;
                if buf.remaining() < len {
                    return Err(DecodeError::TruncatedMessage);
                }
                let mut sub = buf.take(len);
                while sub.has_remaining() {
                    let raw = decode::decode_varint(&mut sub)?;
                    self.values.push(T::decode_wire(raw)?);
                }
            }
            WireType::Varint => {
                let raw = decode::decode_varint(buf)?;
                self.values.push(T::decode_wire(raw)?);
            }
            _ => return Err(DecodeError::InvalidTag),
        }
        Ok(())
    }
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator + Clone> Clone
    for RepeatedVarintField<T, E, A>
where
    T::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            _encoding: PhantomData,
        }
    }
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator + Clone> PartialEq
    for RepeatedVarintField<T, E, A>
where
    T::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator + Clone> Eq
    for RepeatedVarintField<T, E, A>
where
    T::Value: Eq,
{
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, A: Allocator + Clone> ::core::fmt::Debug
    for RepeatedVarintField<T, E, A>
where
    T::Value: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatedVarintField")
            .field(&self.values)
            .finish()
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
