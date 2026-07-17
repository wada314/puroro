//! Semantic protobuf types that share wire type [`WireType::Varint`].
//!
//! Markers (`ProtoInt32`, …) are allocator-free. Singular slots store bare
//! [`VarintProtoType::Value`] (`i32`, `()`, …); repeated elements use the same.

use ::core::convert::TryFrom;
use ::core::marker::PhantomData;

use ::allocator_api2::alloc::Allocator;
use ::protobuf_core::Varint;

use ::puroro::DecodeError;

use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DeallocateIn, DefaultIn, ProtoEmpty};

// ---------------------------------------------------------------------------
// Core trait (wire helpers)
// ---------------------------------------------------------------------------

/// Wire semantics for a protobuf type encoded as a base-128 varint.
pub trait VarintProtoType {
    /// Inner / repeated-element / singular-slot payload (`i32`, `u64`, `bool`, enum, …).
    type Value: Copy;

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}

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

impl<E: OpenEnum> VarintProtoType for ProtoEnum<E, Open> {
    type Value = E;

    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(E::from(ProtoInt32::decode_wire(raw)?))
    }

    #[inline]
    fn encode_wire(value: Self::Value) -> u64 {
        ProtoInt32::encode_wire(value.to_wire())
    }
}

impl<E: ClosedEnum> VarintProtoType for ProtoEnum<E, Closed> {
    type Value = E;

    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        let wire = ProtoInt32::decode_wire(raw)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum { raw })
    }

    #[inline]
    fn encode_wire(value: Self::Value) -> u64 {
        ProtoInt32::encode_wire(value.to_wire())
    }
}

impl<E: ProtoEnumStorage> AddressableSlot for E {}

impl<E: ProtoEnumStorage, A: Allocator + Clone> DefaultIn<A> for E {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        E::proto_zero()
    }
}

impl<E: ProtoEnumStorage, A: Allocator + Clone> DeallocateIn<A> for E {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {}
}

impl<E: ProtoEnumStorage> ProtoEmpty for E {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == E::proto_zero()
    }
}

// ---------------------------------------------------------------------------
// Numeric / bool markers
// ---------------------------------------------------------------------------

macro_rules! proto_varint_marker {
    (
        $(#[$meta:meta])*
        $name:ident($inner:ty),
        decode = $decode:expr,
        encode = $encode:expr $(,)?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl VarintProtoType for $name {
            type Value = $inner;

            #[inline]
            fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
                ($decode)(raw)
            }

            #[inline]
            fn encode_wire(value: Self::Value) -> u64 {
                ($encode)(value)
            }
        }
    };
}

proto_varint_marker! {
    ProtoUInt32(u32),
    decode = |raw| Varint::from_uint64(raw).try_to_uint32().map_err(DecodeError::from),
    encode = |value| Varint::from_uint32(value).to_uint64(),
}

proto_varint_marker! {
    ProtoUInt64(u64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_uint64()),
    encode = |value| Varint::from_uint64(value).to_uint64(),
}

proto_varint_marker! {
    /// Protobuf `int32`.
    ProtoInt32(i32),
    decode = |raw| Varint::from_uint64(raw).try_to_int32().map_err(DecodeError::from),
    encode = |value| Varint::from_int32(value).to_uint64(),
}

proto_varint_marker! {
    ProtoInt64(i64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_int64()),
    encode = |value| Varint::from_int64(value).to_uint64(),
}

proto_varint_marker! {
    /// Protobuf `sint32` — varint with ZigZag encoding.
    ProtoSint32(i32),
    decode = |raw| Varint::from_uint64(raw).try_to_sint32().map_err(DecodeError::from),
    encode = |value| Varint::from_sint32(value).to_uint64(),
}

proto_varint_marker! {
    /// Protobuf `sint64` — varint with ZigZag encoding.
    ProtoSint64(i64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_sint64()),
    encode = |value| Varint::from_sint64(value).to_uint64(),
}

/// Protobuf `bool` type marker — varint 0 or 1.
///
/// Singular / oneof: slot is `()`; logical value via [`BitPacked`](crate::BitPacked).
/// Repeated (future): plain `bool` elements, no bit index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool;

impl VarintProtoType for ProtoBool {
    type Value = bool;

    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(Varint::from_uint64(raw).to_bool())
    }

    #[inline]
    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_bool(value).to_uint64()
    }
}
