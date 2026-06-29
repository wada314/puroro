//! Singular scalar field wrappers generic over wire-encoding traits.
//!
//! Varint-backed fields use [`VarintProtoType`](super::varint::VarintProtoType)
//! as the type parameter. Fixed-width scalars use
//! [`Fixed32ProtoType`](super::fixed32::Fixed32ProtoType) /
//! [`Fixed64ProtoType`](super::fixed64::Fixed64ProtoType) (same pattern).

use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::{MessageParts, MessagePartsMut};
use super::presence::PresenceBits;
use super::varint::{self, VarintProtoType};

// ---------------------------------------------------------------------------
// IMPLICIT presence
// ---------------------------------------------------------------------------

/// Singular scalar with IMPLICIT presence, generic over a varint protobuf type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImplicitVarintField<T: VarintProtoType> {
    value: T::Value,
}

impl<T: VarintProtoType> ImplicitVarintField<T> {
    /// Creates a field holding the protobuf type-zero.
    pub fn new() -> Self {
        Self {
            value: T::proto_zero(),
        }
    }

    /// Returns the stored value.
    #[inline]
    pub fn get(&self) -> T::Value {
        self.value
    }

    /// Sets the value.
    #[inline]
    pub fn set(&mut self, v: T::Value) {
        self.value = v;
    }

    /// Wire byte length, or `0` when omitted (value equals type-zero).
    pub fn encoded_len<const FIELD: u32>(&self) -> usize {
        if self.value == T::proto_zero() {
            0
        } else {
            encode::encoded_len_varint_field(FIELD, T::encode_wire(self.value))
        }
    }

    /// Encodes when present on wire (non-type-zero).
    pub fn encode_raw<const FIELD: u32, B: BufMut>(&self, buf: &mut B) {
        if self.value != T::proto_zero() {
            encode::encode_varint_field(FIELD, T::encode_wire(self.value), buf);
        }
    }

    /// Merges one wire occurrence (last value wins).
    pub fn merge<B: Buf>(
        &mut self,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        self.value = T::decode_wire(raw)?;
        Ok(())
    }
}

impl<T: VarintProtoType> Default for ImplicitVarintField<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// EXPLICIT presence
// ---------------------------------------------------------------------------

/// Singular scalar with EXPLICIT presence (presence bit + value slot).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExplicitVarintField<T: VarintProtoType> {
    value: T::Value,
}

impl<T: VarintProtoType> ExplicitVarintField<T> {
    /// Creates an unset field (bit clear, value slot at type-zero).
    pub fn new() -> Self {
        Self {
            value: T::proto_zero(),
        }
    }

    /// Returns whether the presence bit is set.
    #[inline]
    pub fn has<P, A, const BIT: usize>(&self, parts: MessageParts<P, A>) -> bool
    where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        parts.presence.is_set(BIT)
    }

    /// Returns an [`Optional`] wrapping the semantic value.
    pub fn get<P, A, D, const BIT: usize>(
        &self,
        parts: MessageParts<P, A>,
        default: D,
    ) -> Optional<T::Value, D>
    where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
        D: HasDefault<T::Value>,
        T::Value: Copy,
    {
        let v = if parts.presence.is_set(BIT) {
            Some(self.value)
        } else {
            None
        };
        Optional::new(v, default)
    }

    /// Marks the field set and stores `v`.
    #[inline]
    pub fn set<P, A, const BIT: usize>(
        &mut self,
        parts: &mut MessagePartsMut<P, A>,
        v: T::Value,
    ) where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        parts.set_presence(BIT, true);
        self.value = v;
    }

    /// Clears presence and resets the value slot to type-zero.
    #[inline]
    pub fn clear<P, A, const BIT: usize>(&mut self, parts: &mut MessagePartsMut<P, A>)
    where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        parts.set_presence(BIT, false);
        self.value = T::proto_zero();
    }

    /// Wire byte length when the presence bit is set.
    pub fn encoded_len<P, A, const FIELD: u32, const BIT: usize>(
        &self,
        parts: MessageParts<P, A>,
    ) -> usize
    where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if parts.presence.is_set(BIT) {
            encode::encoded_len_varint_field(FIELD, T::encode_wire(self.value))
        } else {
            0
        }
    }

    /// Encodes when the presence bit is set.
    pub fn encode_raw<P, A, B: BufMut, const FIELD: u32, const BIT: usize>(
        &self,
        parts: MessageParts<P, A>,
        buf: &mut B,
    ) where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if parts.presence.is_set(BIT) {
            encode::encode_varint_field(FIELD, T::encode_wire(self.value), buf);
        }
    }

    /// Merges one wire occurrence (sets presence bit; last value wins).
    pub fn merge<P, A, B: Buf, const BIT: usize>(
        &mut self,
        parts: &mut MessagePartsMut<P, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        parts.set_presence(BIT, true);
        self.value = T::decode_wire(raw)?;
        Ok(())
    }
}

impl<T: VarintProtoType> Default for ExplicitVarintField<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Type aliases for generated code ergonomics
// ---------------------------------------------------------------------------

/// IMPLICIT varint field; protobuf type is the type parameter `T`.
pub type ImplicitVarint<T> = ImplicitVarintField<T>;

/// EXPLICIT varint field; default provider `D` is passed to [`ExplicitVarintField::get`].
pub type ExplicitVarint<T> = ExplicitVarintField<T>;

/// IMPLICIT `int32`.
pub type ImplicitInt32 = ImplicitVarintField<varint::ProtoInt32>;
/// IMPLICIT open enum (raw `i32` on wire).
pub type ImplicitEnum = ImplicitVarintField<varint::ProtoEnum>;
