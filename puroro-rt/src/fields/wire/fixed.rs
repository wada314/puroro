//! Semantic protobuf types that share fixed-width wire types (I32 / I64).
//!
//! Same layering as [`super::varint`]: one marker type per protobuf fixed-width
//! family member; singular/repeated wrappers parametrise over these traits.
//!
//! Float [`ProtoEmpty`] uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES};

use ::puroro::DecodeError;

use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DefaultIn, ProtoEmpty};

// ---------------------------------------------------------------------------
// Core traits (wire helpers)
// ---------------------------------------------------------------------------

/// Wire semantics for protobuf types encoded as 4 little-endian bytes (I32).
pub trait Fixed32ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

/// Wire semantics for protobuf types encoded as 8 little-endian bytes (I64).
pub trait Fixed64ProtoType {
    type Value: Copy + PartialEq;

    fn proto_zero() -> Self::Value;

    fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError>;

    fn encode_wire(value: Self::Value, buf: &mut impl BufMut);
}

// ---------------------------------------------------------------------------
// Markers
// ---------------------------------------------------------------------------

macro_rules! proto_fixed32_marker {
    ($(#[$meta:meta])* $name:ident($inner:ty)) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl Fixed32ProtoType for $name {
            type Value = $inner;

            #[inline]
            fn proto_zero() -> Self::Value {
                <$inner>::default()
            }

            #[inline]
            fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError> {
                Ok(<$inner>::from_le_bytes(read_fixed32(buf)?))
            }

            #[inline]
            fn encode_wire(value: Self::Value, buf: &mut impl BufMut) {
                buf.put_slice(&value.to_le_bytes());
            }
        }
    };
}

macro_rules! proto_fixed64_marker {
    ($(#[$meta:meta])* $name:ident($inner:ty)) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl Fixed64ProtoType for $name {
            type Value = $inner;

            #[inline]
            fn proto_zero() -> Self::Value {
                <$inner>::default()
            }

            #[inline]
            fn decode_wire(buf: &mut impl Buf) -> Result<Self::Value, DecodeError> {
                Ok(<$inner>::from_le_bytes(read_fixed64(buf)?))
            }

            #[inline]
            fn encode_wire(value: Self::Value, buf: &mut impl BufMut) {
                buf.put_slice(&value.to_le_bytes());
            }
        }
    };
}

#[inline]
fn read_fixed32(buf: &mut impl Buf) -> Result<[u8; FIXED32_BYTES], DecodeError> {
    if buf.remaining() < FIXED32_BYTES {
        return Err(DecodeError::TruncatedMessage);
    }
    let mut bytes = [0u8; FIXED32_BYTES];
    buf.copy_to_slice(&mut bytes);
    Ok(bytes)
}

#[inline]
fn read_fixed64(buf: &mut impl Buf) -> Result<[u8; FIXED64_BYTES], DecodeError> {
    if buf.remaining() < FIXED64_BYTES {
        return Err(DecodeError::TruncatedMessage);
    }
    let mut bytes = [0u8; FIXED64_BYTES];
    buf.copy_to_slice(&mut bytes);
    Ok(bytes)
}

proto_fixed32_marker! {
    /// Protobuf `fixed32`.
    ProtoFixed32(u32)
}

proto_fixed32_marker! {
    /// Protobuf `sfixed32`.
    ProtoSFixed32(i32)
}

proto_fixed32_marker! {
    /// Protobuf `float`.
    ProtoFloat(f32)
}

proto_fixed64_marker! {
    /// Protobuf `fixed64`.
    ProtoFixed64(u64)
}

proto_fixed64_marker! {
    /// Protobuf `sfixed64`.
    ProtoSFixed64(i64)
}

proto_fixed64_marker! {
    /// Protobuf `double`.
    ProtoDouble(f64)
}

// ---------------------------------------------------------------------------
// Slot infrastructure for f32 / f64
// ---------------------------------------------------------------------------

impl AddressableSlot for f32 {}
impl AddressableSlot for f64 {}

impl<A: Allocator + Clone> DefaultIn<A> for f32 {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        0.0
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for f64 {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        0.0
    }
}

/// Float empty check uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).
impl ProtoEmpty for f32 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0.0
    }
}

/// Float empty check uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).
impl ProtoEmpty for f64 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0.0
    }
}
