//! Semantic protobuf types that share fixed-width wire types (I32 / I64).
//!
//! Same layering as [`super::varint`]: one marker type per protobuf fixed-width
//! family member; singular/repeated wrappers parametrise over these traits.

use ::bytes::{Buf, BufMut};

use crate::error::DecodeError;
use crate::wire_type::WireType;

/// Wire semantics for protobuf types encoded as 4 little-endian bytes (I32).
pub trait Fixed32ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

/// Always [`WireType::Int32`] for singular field merge/encode checks.
pub const WIRE_TYPE_I32: WireType = WireType::Int32;

/// Wire semantics for protobuf types encoded as 8 little-endian bytes (I64).
pub trait Fixed64ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

/// Always [`WireType::Int64`] for singular field merge/encode checks.
pub const WIRE_TYPE_I64: WireType = WireType::Int64;

// Fixed32ProtoType / Fixed64ProtoType implementations — planned.
