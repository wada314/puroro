//! Semantic protobuf types that share wire type [`WireType::Varint`].
//!
//! Wire encode/decode uses [`protobuf_core::Varint`] — puroro does not duplicate
//! zigzag or varint byte logic here.

use ::core::convert::TryFrom;
use ::protobuf_core::Varint;

use crate::error::DecodeError;
use crate::wire_type::WireType;

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// Wire semantics for a protobuf type encoded as a base-128 varint.
///
/// One implementation per protobuf *type* (not per message field). Uses
/// [`Varint`] from `protobuf-core` for all encode/decode conversions.
pub trait VarintProtoType {
    /// Rust storage (`i32`, `u64`, `bool`, …).
    type Value: Copy + PartialEq;

    /// Value used for IMPLICIT omit-on-encode and unset slot storage.
    fn proto_zero() -> Self::Value;

    /// Converts a decoded raw varint (numeric wire value) into the semantic value.
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;

    /// Converts a semantic value into the raw varint numeric value for the wire.
    fn encode_wire(value: Self::Value) -> u64;
}

// ---------------------------------------------------------------------------
// Marker types — delegate to protobuf-core Varint API
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt32;

impl VarintProtoType for ProtoUInt32 {
    type Value = u32;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Varint::from_uint64(raw).try_to_uint32().map_err(Into::into)
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_uint32(value).to_uint64()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt64;

impl VarintProtoType for ProtoUInt64 {
    type Value = u64;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(Varint::from_uint64(raw).to_uint64())
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_uint64(value).to_uint64()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt32;

impl VarintProtoType for ProtoInt32 {
    type Value = i32;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Varint::from_uint64(raw).try_to_int32().map_err(Into::into)
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_int32(value).to_uint64()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt64;

impl VarintProtoType for ProtoInt64 {
    type Value = i64;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(Varint::from_uint64(raw).to_int64())
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_int64(value).to_uint64()
    }
}

/// Protobuf `sint32` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSint32;

impl VarintProtoType for ProtoSint32 {
    type Value = i32;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Varint::from_uint64(raw).try_to_sint32().map_err(Into::into)
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_sint32(value).to_uint64()
    }
}

/// Protobuf `sint64` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSint64;

impl VarintProtoType for ProtoSint64 {
    type Value = i64;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(Varint::from_uint64(raw).to_sint64())
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_sint64(value).to_uint64()
    }
}

/// Protobuf `bool` — varint 0 or 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool;

impl VarintProtoType for ProtoBool {
    type Value = bool;

    fn proto_zero() -> Self::Value {
        false
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(Varint::from_uint64(raw).to_bool())
    }

    fn encode_wire(value: Self::Value) -> u64 {
        Varint::from_bool(value).to_uint64()
    }
}

/// Protobuf `enum` on the wire — identical to `int32` varint encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoEnum;

impl VarintProtoType for ProtoEnum {
    type Value = i32;

    fn proto_zero() -> Self::Value {
        0
    }

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        ProtoInt32::decode_wire(raw)
    }

    fn encode_wire(value: Self::Value) -> u64 {
        ProtoInt32::encode_wire(value)
    }
}

/// Returns `true` when `raw` is a known variant of `E`.
pub fn enum_value_is_known<E>(raw: i32) -> bool
where
    E: TryFrom<i32, Error = i32>,
{
    E::try_from(raw).is_ok()
}

/// Always [`WireType::Varint`] for singular field merge/encode checks.
pub const WIRE_TYPE: WireType = WireType::Varint;
