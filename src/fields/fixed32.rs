//! Semantic protobuf types that share wire type [`WireType::Int32`].
//!
//! Same layering as [`super::varint`]: one marker type per protobuf fixed32
//! family member; singular/repeated wrappers parametrise over `T: Fixed32ProtoType`.

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
pub const WIRE_TYPE: WireType = WireType::Int32;

// Fixed32ProtoType implementations (ProtoFixed32, ProtoFloat, …) — planned.
