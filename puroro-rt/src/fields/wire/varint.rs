//! Semantic protobuf types that share wire type [`WireType::Varint`].
//!
//! Wire encode/decode uses [`protobuf_core::Varint`] — puroro does not duplicate
//! zigzag or varint byte logic here.
//!
//! # Singular vs repeated
//!
//! Each `Proto*` type is a **thin wrapper** over its payload (`ProtoInt32(i32)`,
//! …). Singular fields ([`ScalarProtoType`](super::scalar::ScalarProtoType))
//! store the wrapper itself. Repeated fields use [`VarintProtoType::Value`]
//! (the inner primitive / enum) in the element buffer so `as_slice()` stays
//! `&[i32]` / `&[E]`.

use ::derive_more::{Deref, DerefMut, From, Into};
use ::protobuf_core::Varint;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::{DeallocateIn, DefaultIn, ProtoEmpty};

// ---------------------------------------------------------------------------
// Core trait (repeated + shared wire helpers)
// ---------------------------------------------------------------------------

/// Wire semantics for a protobuf type encoded as a base-128 varint.
///
/// [`Value`](Self::Value) is the **element type for repeated fields** and the
/// inner payload of the thin wrapper. Singular fields store the wrapper type
/// itself (see [`ScalarProtoType`](super::scalar::ScalarProtoType)).
pub trait VarintProtoType {
    /// Inner / repeated-element type (`i32`, `u64`, `bool`, enum newtype, …).
    type Value: Copy;

    /// Converts a decoded raw varint (numeric wire value) into the semantic value.
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;

    /// Converts a semantic value into the raw varint numeric value for the wire.
    fn encode_wire(value: Self::Value) -> u64;
}

// ---------------------------------------------------------------------------
// Generated protobuf enum newtypes
// ---------------------------------------------------------------------------

/// Wire/storage behaviour for a generated protobuf enum newtype.
///
/// Implemented once per protobuf enum by the code generator. Open enums accept
/// any wire value in [`decode_from_wire`](Self::decode_from_wire); closed enums
/// reject unknown values there (and use [`merge_closed`](crate::fields::singular::field::SingularFieldMut::merge_closed)
/// on decode to divert them to unknown fields when appropriate).
pub trait ProtoEnumStorage: Copy + PartialEq + 'static {
    fn proto_zero() -> Self;
    fn to_wire(self) -> i32;
    fn decode_from_wire(wire: i32) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

/// Thin wrapper around a generated enum newtype `E` for singular fields.
///
/// Repeated enum fields (if any) still store bare `E` via
/// [`VarintProtoType::Value`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Deref, DerefMut, From)]
#[repr(transparent)]
pub struct ProtoEnum<E: ProtoEnumStorage>(pub E);

impl<E: ProtoEnumStorage> DefaultIn for ProtoEnum<E> {
    #[inline]
    fn default_in<A: ::allocator_api2::alloc::Allocator>(_alloc: A) -> Self {
        Self(E::proto_zero())
    }
}

impl<E: ProtoEnumStorage> DeallocateIn for ProtoEnum<E> {
    #[inline]
    unsafe fn deallocate_in<A: ::allocator_api2::alloc::Allocator>(self, _alloc: A) {}
}

impl<E: ProtoEnumStorage> ProtoEmpty for ProtoEnum<E> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0 == E::proto_zero()
    }
}

impl<E: ProtoEnumStorage> VarintProtoType for ProtoEnum<E> {
    type Value = E;

    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        E::decode_from_wire(ProtoInt32::decode_wire(raw)?)
    }

    fn encode_wire(value: Self::Value) -> u64 {
        ProtoInt32::encode_wire(value.to_wire())
    }
}

// ---------------------------------------------------------------------------
// Thin wrappers — payload + wire identity
// ---------------------------------------------------------------------------

macro_rules! proto_varint_wrapper {
    (
        $(#[$meta:meta])*
        $name:ident($inner:ty),
        decode = $decode:expr,
        encode = $encode:expr $(,)?
    ) => {
        $(#[$meta])*
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Deref, DerefMut, From, Into,
        )]
        #[repr(transparent)]
        pub struct $name(pub $inner);

        impl DefaultIn for $name {
            #[inline]
            fn default_in<A: ::allocator_api2::alloc::Allocator>(alloc: A) -> Self {
                Self(<$inner as DefaultIn>::default_in(alloc))
            }
        }

        impl DeallocateIn for $name {
            #[inline]
            unsafe fn deallocate_in<A: ::allocator_api2::alloc::Allocator>(self, alloc: A) {
                unsafe { <$inner as DeallocateIn>::deallocate_in(self.0, alloc) };
            }
        }

        impl ProtoEmpty for $name {
            #[inline]
            fn is_proto_empty(&self) -> bool {
                self.0.is_proto_empty()
            }
        }

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

proto_varint_wrapper! {
    ProtoUInt32(u32),
    decode = |raw| Varint::from_uint64(raw).try_to_uint32().map_err(DecodeError::from),
    encode = |value| Varint::from_uint32(value).to_uint64(),
}

proto_varint_wrapper! {
    ProtoUInt64(u64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_uint64()),
    encode = |value| Varint::from_uint64(value).to_uint64(),
}

proto_varint_wrapper! {
    /// Protobuf `int32`.
    ProtoInt32(i32),
    decode = |raw| Varint::from_uint64(raw).try_to_int32().map_err(DecodeError::from),
    encode = |value| Varint::from_int32(value).to_uint64(),
}

proto_varint_wrapper! {
    ProtoInt64(i64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_int64()),
    encode = |value| Varint::from_int64(value).to_uint64(),
}

proto_varint_wrapper! {
    /// Protobuf `sint32` — varint with ZigZag encoding.
    ProtoSint32(i32),
    decode = |raw| Varint::from_uint64(raw).try_to_sint32().map_err(DecodeError::from),
    encode = |value| Varint::from_sint32(value).to_uint64(),
}

proto_varint_wrapper! {
    /// Protobuf `sint64` — varint with ZigZag encoding.
    ProtoSint64(i64),
    decode = |raw| Ok(Varint::from_uint64(raw).to_sint64()),
    encode = |value| Varint::from_sint64(value).to_uint64(),
}

/// Protobuf `bool` type marker — varint 0 or 1.
///
/// Implements [`ScalarProtoType`](super::scalar::ScalarProtoType) with
/// `Slot = Self` (ZST). The logical `bool` is packed at `VALUE_BIT` in
/// [`MessageCommon`](crate::MessageCommon)'s bitvec; the field struct only
/// stores this marker for presence/init layout.
///
/// Interim: `VALUE_BIT` lives on this type marker for codegen stability. A
/// future cleanup should move the index to the field / layout side so the
/// marker is bit-index-free.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool<const VALUE_BIT: usize>;

impl<const VALUE_BIT: usize> DefaultIn for ProtoBool<VALUE_BIT> {
    #[inline]
    fn default_in<A: ::allocator_api2::alloc::Allocator>(_alloc: A) -> Self {
        Self
    }
}

impl<const VALUE_BIT: usize> DeallocateIn for ProtoBool<VALUE_BIT> {
    #[inline]
    unsafe fn deallocate_in<A: ::allocator_api2::alloc::Allocator>(self, _alloc: A) {}
}

impl<const VALUE_BIT: usize> VarintProtoType for ProtoBool<VALUE_BIT> {
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

/// Always [`WireType::Varint`] for singular field merge/encode checks.
pub(crate) const WIRE_TYPE: WireType = WireType::Varint;
