//! Semantic protobuf types that share wire type [`WireType::Int64`].
//!
//! Same layering as [`super::varint`]: one marker type per protobuf fixed64
//! family member; singular/repeated wrappers parametrise over `T: Fixed64ProtoType`.

use ::bytes::{Buf, BufMut};

use crate::error::DecodeError;
use crate::wire_type::WireType;

/// Wire semantics for protobuf types encoded as 8 little-endian bytes (I64).
pub trait Fixed64ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

/// Always [`WireType::Int64`] for singular field merge/encode checks.
pub const WIRE_TYPE: WireType = WireType::Int64;

// Fixed64ProtoType implementations (ProtoFixed64, ProtoDouble, …) — planned.
