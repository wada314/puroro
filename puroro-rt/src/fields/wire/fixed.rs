//! Semantic protobuf types that share fixed-width wire types (I32 / I64).
//!
//! Same layering as [`super::varint`]: one marker type per protobuf fixed-width
//! family member; singular/repeated wrappers parametrise over these traits.
//!
//! Implementations are planned; this module is scaffolding only for now.

#![allow(dead_code)]

use ::bytes::{Buf, BufMut};

use ::puroro::DecodeError;

/// Wire semantics for protobuf types encoded as 4 little-endian bytes (I32).
pub(crate) trait Fixed32ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

/// Wire semantics for protobuf types encoded as 8 little-endian bytes (I64).
pub(crate) trait Fixed64ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

// Fixed32ProtoType / Fixed64ProtoType implementations — planned.
