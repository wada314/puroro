//! Varint family markers (`ProtoInt32`, enums, [`ProtoBool`], …).
//!
//! Numeric / enum wire codecs live on [`NumericalType`](super::numerical::NumericalType).
//! [`ProtoBool`] keeps a small inherent wire API for bit-packed singular storage.

use ::core::convert::TryFrom;
use ::core::marker::PhantomData;

use ::allocator_api2::alloc::Allocator;
use ::protobuf_core::Varint;

use ::puroro::DecodeError;

use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DefaultIn, ProtoEmpty};

// ---------------------------------------------------------------------------
// Enum markers
// ---------------------------------------------------------------------------

pub struct Open;
pub struct Closed;

pub trait ProtoEnumStorage: Copy + PartialEq + 'static {
    fn proto_zero() -> Self;
    fn to_wire(self) -> i32;
}

pub trait OpenEnum: ProtoEnumStorage + From<i32> {}
pub trait ClosedEnum: ProtoEnumStorage + TryFrom<i32, Error = i32> {}

/// Allocator-free enum type marker (`Open` / `Closed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoEnum<E, K>(PhantomData<(E, K)>);

impl<E: ProtoEnumStorage> AddressableSlot for E {}

impl<E: ProtoEnumStorage, A: Allocator + Clone> DefaultIn<A> for E {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        E::proto_zero()
    }
}

impl<E: ProtoEnumStorage> ProtoEmpty for E {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == E::proto_zero()
    }
}

// ---------------------------------------------------------------------------
// Numeric markers (wire via NumericalType)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt64;

/// Protobuf `int32`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt64;

/// Protobuf `sint32` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSInt32;

/// Protobuf `sint64` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSInt64;

/// Protobuf `bool` type marker — varint 0 or 1.
///
/// Singular / oneof: slot is `()`; logical value via [`BitPacked`](crate::BitPacked).
/// Repeated: plain `bool` elements in the vec (no MessageCommon bit index).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool;

impl ProtoBool {
    #[inline]
    pub fn decode_wire(raw: u64) -> Result<bool, DecodeError> {
        Ok(Varint::from_uint64(raw).to_bool())
    }

    #[inline]
    pub fn encode_wire(value: bool) -> u64 {
        Varint::from_bool(value).to_uint64()
    }
}
