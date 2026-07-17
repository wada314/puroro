//! Semantic protobuf types that share wire type [`WireType::Varint`].
//!
//! Wire encode/decode uses [`protobuf_core::Varint`] — puroro does not duplicate
//! zigzag or varint byte logic here.
//!
//! # Singular vs repeated
//!
//! Each `Proto*` type is a **thin wrapper** over its payload (`ProtoInt32(i32)`,
//! …). Singular fields ([`ProtoType`](super::proto_type::ProtoType))
//! store the wrapper itself. Repeated fields use [`VarintProtoType::Value`]
//! (the inner primitive / enum) in the element buffer so `as_slice()` stays
//! `&[i32]` / `&[E]`.

use ::core::convert::TryFrom;
use ::core::marker::PhantomData;
use ::core::ops::{Deref, DerefMut};

use ::allocator_api2::alloc::Allocator;
use ::protobuf_core::Varint;

use ::puroro::DecodeError;

use crate::fields::shared::{DeallocateIn, DefaultIn, ProtoEmpty};

// ---------------------------------------------------------------------------
// Core trait (repeated + shared wire helpers)
// ---------------------------------------------------------------------------

/// Wire semantics for a protobuf type encoded as a base-128 varint.
///
/// [`Value`](Self::Value) is the **element type for repeated fields** and the
/// inner payload of the thin wrapper. Singular fields store the wrapper type
/// itself (see [`ProtoType`](super::proto_type::ProtoType)).
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

/// Marker for [`features.enum_type = OPEN`](https://protobuf.dev/editions/features/#enum_type).
pub struct Open;

/// Marker for [`features.enum_type = CLOSED`](https://protobuf.dev/editions/features/#enum_type).
pub struct Closed;

/// Shared wire/storage basics for a generated protobuf enum newtype.
///
/// Open vs closed is expressed by implementing [`OpenEnum`] or [`ClosedEnum`]
/// and wrapping as [`ProtoEnum<E, Open, A>`] / [`ProtoEnum<E, Closed, A>`].
pub trait ProtoEnumStorage: Copy + PartialEq + 'static {
    fn proto_zero() -> Self;
    fn to_wire(self) -> i32;
}

/// Open protobuf enum — any `i32` is a valid wire/storage value (`From<i32>`).
///
/// Spec ([Enum Behavior](https://protobuf.dev/programming-guides/enum/)): an
/// unrecognized wire value is stored in the field; accessors report the field
/// as set and return a value representing that integer.
pub trait OpenEnum: ProtoEnumStorage + From<i32> {}

/// Closed protobuf enum — only known values are valid (`TryFrom<i32>`).
///
/// Unknown wire values yield [`DecodeError::UnknownClosedEnum`]; singular
/// [`merge`](crate::fields::singular::field::SingularFieldMut::merge) catches
/// that and appends the raw varint to unknown fields.
///
/// Spec ([Enum Behavior](https://protobuf.dev/programming-guides/enum/)): an
/// unrecognized wire value is stored in the message's unknown field set;
/// accessors report the field as unset and return the enum default.
pub trait ClosedEnum: ProtoEnumStorage + TryFrom<i32, Error = i32> {}

/// Thin wrapper around a generated enum newtype `E` for singular fields.
///
/// `K` is [`Open`] or [`Closed`]. Repeated enum fields (if any) still store bare
/// `E` via [`VarintProtoType::Value`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ProtoEnum<E, K, A: Allocator>(pub E, PhantomData<(K, A)>);

impl<E, K, A: Allocator> ProtoEnum<E, K, A> {
    #[inline]
    pub const fn new(value: E) -> Self {
        Self(value, PhantomData)
    }
}

impl<E, K, A: Allocator> Deref for ProtoEnum<E, K, A> {
    type Target = E;

    #[inline]
    fn deref(&self) -> &E {
        &self.0
    }
}

impl<E, K, A: Allocator> DerefMut for ProtoEnum<E, K, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut E {
        &mut self.0
    }
}

impl<E, K, A: Allocator> From<E> for ProtoEnum<E, K, A> {
    #[inline]
    fn from(value: E) -> Self {
        Self::new(value)
    }
}

impl<E: Default, K, A: Allocator> Default for ProtoEnum<E, K, A> {
    #[inline]
    fn default() -> Self {
        Self::new(E::default())
    }
}

impl<E: ProtoEnumStorage, K, A: Allocator + Clone> DefaultIn for ProtoEnum<E, K, A> {
    type Alloc = A;

    #[inline]
    fn default_in(_alloc: A) -> Self {
        Self::new(E::proto_zero())
    }
}

impl<E: ProtoEnumStorage, K, A: Allocator + Clone> DeallocateIn for ProtoEnum<E, K, A> {
    type Alloc = A;

    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {}
}

impl<E: ProtoEnumStorage, K, A: Allocator> ProtoEmpty for ProtoEnum<E, K, A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0 == E::proto_zero()
    }
}

impl<E: OpenEnum, A: Allocator> VarintProtoType for ProtoEnum<E, Open, A> {
    type Value = E;

    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        Ok(E::from(<ProtoInt32<A> as VarintProtoType>::decode_wire(
            raw,
        )?))
    }

    #[inline]
    fn encode_wire(value: Self::Value) -> u64 {
        <ProtoInt32<A> as VarintProtoType>::encode_wire(value.to_wire())
    }
}

impl<E: ClosedEnum, A: Allocator> VarintProtoType for ProtoEnum<E, Closed, A> {
    type Value = E;

    /// Known values succeed. Unknown values return
    /// [`DecodeError::UnknownClosedEnum`] so singular `merge` can divert `raw`
    /// into unknown fields
    /// ([spec](https://protobuf.dev/programming-guides/enum/)).
    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError> {
        let wire = <ProtoInt32<A> as VarintProtoType>::decode_wire(raw)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum { raw })
    }

    #[inline]
    fn encode_wire(value: Self::Value) -> u64 {
        <ProtoInt32<A> as VarintProtoType>::encode_wire(value.to_wire())
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
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name<A: ::allocator_api2::alloc::Allocator>(pub $inner, PhantomData<A>);

        impl<A: ::allocator_api2::alloc::Allocator> $name<A> {
            #[inline]
            pub const fn new(value: $inner) -> Self {
                Self(value, PhantomData)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> ::core::ops::Deref for $name<A> {
            type Target = $inner;

            #[inline]
            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> ::core::ops::DerefMut for $name<A> {
            #[inline]
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> From<$inner> for $name<A> {
            #[inline]
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + Clone> DefaultIn for $name<A> {
            type Alloc = A;

            #[inline]
            fn default_in(_alloc: A) -> Self {
                Self::new(0 as $inner)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + Clone> DeallocateIn for $name<A> {
            type Alloc = A;

            #[inline]
            unsafe fn deallocate_in(self, _alloc: A) {}
        }

        impl<A: ::allocator_api2::alloc::Allocator> ProtoEmpty for $name<A> {
            #[inline]
            fn is_proto_empty(&self) -> bool {
                self.0 == 0
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> VarintProtoType for $name<A> {
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
/// Implements [`ProtoType`](super::proto_type::ProtoType) with
/// `Slot = Self` (ZST). The logical `bool` is packed at `VALUE_BIT` in
/// [`MessageCommon`](crate::MessageCommon)'s bitvec; the field struct only
/// stores this marker for presence/init layout.
///
/// **Singular / oneof vs repeated (future):** this bit-packed form (and the
/// `VALUE_BIT` const generic) is only for singular and oneof `bool`. A
/// `repeated bool` must store plain `bool` elements in the repeated buffer and
/// must **not** take a MessageCommon bit index. Do not reuse this marker as-is
/// for repeated; keep scalar (bit-packed) and repeated (element `bool`) as
/// distinct type paths. Interim: `VALUE_BIT` still lives on this marker for
/// codegen stability; a later cleanup should move the index to the
/// field / layout side so the singular marker can be bit-index-free too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool<A: Allocator, const VALUE_BIT: usize>(PhantomData<A>);

impl<A: Allocator + Clone, const VALUE_BIT: usize> DefaultIn for ProtoBool<A, VALUE_BIT> {
    type Alloc = A;

    #[inline]
    fn default_in(_alloc: A) -> Self {
        Self(PhantomData)
    }
}

impl<A: Allocator + Clone, const VALUE_BIT: usize> DeallocateIn for ProtoBool<A, VALUE_BIT> {
    type Alloc = A;

    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {}
}

impl<A: Allocator, const VALUE_BIT: usize> VarintProtoType for ProtoBool<A, VALUE_BIT> {
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
