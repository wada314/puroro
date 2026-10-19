//! Numerical protobuf **types** via [`Numerical`]`<C>` ([`NumericalType`] codec).
//!
//! Covers `int32` / [`ProtoInt32`], `bool` / [`ProtoBool`], `fixed64` /
//! [`ProtoFixed64`], open/closed enums, … — not Len types such as string /
//! bytes / message.
//!
//! Method 1 style: blankets go on [`Numerical`]`<C>`, so they do not collide
//! with [`LenScalar`](super::len::LenScalar) blankets. [`NumericalType`] is the
//! codec (parallel to [`LenCodec`](super::len::LenCodec)); prefer the
//! [`ProtoInt32`] / … aliases in generated code.
//!
//! [`NativeType`](NumericalType::NativeType) is the host-language copy value used
//! for encode/decode and field get/set — not necessarily the singular struct
//! slot type (`AddressableSlot` lives on
//! [`PayloadAccess`](super::singular_type::PayloadAccess); singular
//! [`ProtoBool`] may also use [`BitPacked`](crate::BitPacked)).
//! Maps `NativeType` ↔ [`WireBody`](NumericalType::WireBody)
//! ([`CopyWirePayload`](super::wire_payload::CopyWirePayload)).

use ::core::convert::TryFrom;
use ::core::marker::PhantomData;

use ::protobuf_core::Varint;

use ::puroro::DecodeError;

use super::wire_payload::{CopyWirePayload, Fixed32Payload, Fixed64Payload, VarintPayload};

// ---------------------------------------------------------------------------
// Enum kind markers
// ---------------------------------------------------------------------------

/// Open-enum kind marker for [`ProtoEnum`] (`enum_type = OPEN`).
pub struct Open;

/// Closed-enum kind marker for [`ProtoEnum`] (`enum_type = CLOSED`).
pub struct Closed;

/// Generated enum newtype storage: wire `i32` plus a known default.
pub trait ProtoEnumStorage: Copy + PartialEq + Default + 'static {
    /// Numeric value written on the wire.
    fn to_wire(self) -> i32;
}

/// Open enum: unknown wire values are retained via [`From`]`<i32>`.
pub trait OpenEnum: ProtoEnumStorage + From<i32> {}

/// Closed enum: unknown wire values fail [`TryFrom`]`<i32>` and become unknowns.
pub trait ClosedEnum: ProtoEnumStorage + TryFrom<i32, Error = i32> {}

// Enum storage types are `Copy` + `Default`; `CloneIn` / `DefaultIn` /
// `DeallocateIn` come from `unmanaged` blankets.

/// Numerical protobuf type marker, parametrised by [`NumericalType`] codec.
///
/// Public so [`ProtoInt32`] / … aliases can be crate-root re-exports; prefer
/// those aliases in generated code. Not re-exported from the crate root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Numerical<C>(PhantomData<C>);

/// Codec for a numerical protobuf type (`int32`, `bool`, open enum, …).
///
/// `pub` because it appears in bounds of public trait impls on [`Numerical`]
/// (same reason [`LenCodec`](super::len::LenCodec) is public). Not re-exported
/// from the crate root.
pub trait NumericalType: Sized {
    /// Host-language value for encode/decode and field get/set (not necessarily
    /// the singular struct slot type).
    type NativeType: Copy + Default + PartialEq;

    /// Wire-shape body for this proto type (e.g. [`VarintPayload`] for `int32`).
    type WireBody: CopyWirePayload;

    fn to_wire_body(value: Self::NativeType) -> Self::WireBody;

    fn from_wire_body(wire_body: Self::WireBody) -> Result<Self::NativeType, DecodeError>;
}

// ---------------------------------------------------------------------------
// Varint codecs + aliases
// ---------------------------------------------------------------------------

macro_rules! varint_numerical {
    ($codec:ident, $alias:ident, $inner:ty, decode = $decode:expr, encode = $encode:expr $(,)?) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $codec;

        /// Protobuf type marker (see [`Numerical`] visibility note).
        pub type $alias = Numerical<$codec>;

        impl NumericalType for $codec {
            type NativeType = $inner;
            type WireBody = VarintPayload;

            #[inline]
            fn to_wire_body(value: $inner) -> VarintPayload {
                VarintPayload(($encode)(value))
            }

            #[inline]
            fn from_wire_body(wire_body: VarintPayload) -> Result<$inner, DecodeError> {
                ($decode)(wire_body.0)
            }
        }
    };
}

varint_numerical! {
    UInt32Codec,
    ProtoUInt32,
    u32,
    decode = |raw: Varint| raw.try_to_uint32().map_err(DecodeError::from),
    encode = Varint::from_uint32,
}

varint_numerical! {
    UInt64Codec,
    ProtoUInt64,
    u64,
    decode = |raw: Varint| Ok(raw.to_uint64()),
    encode = Varint::from_uint64,
}

varint_numerical! {
    Int32Codec,
    ProtoInt32,
    i32,
    decode = |raw: Varint| raw.try_to_int32().map_err(DecodeError::from),
    encode = Varint::from_int32,
}

varint_numerical! {
    Int64Codec,
    ProtoInt64,
    i64,
    decode = |raw: Varint| Ok(raw.to_int64()),
    encode = Varint::from_int64,
}

varint_numerical! {
    SInt32Codec,
    ProtoSInt32,
    i32,
    decode = |raw: Varint| raw.try_to_sint32().map_err(DecodeError::from),
    encode = Varint::from_sint32,
}

varint_numerical! {
    SInt64Codec,
    ProtoSInt64,
    i64,
    decode = |raw: Varint| Ok(raw.to_sint64()),
    encode = Varint::from_sint64,
}

varint_numerical! {
    BoolCodec,
    ProtoBool,
    bool,
    decode = |raw: Varint| Ok(raw.to_bool()),
    encode = Varint::from_bool,
}

// ---------------------------------------------------------------------------
// Enum codec + alias
// ---------------------------------------------------------------------------

/// Open/closed enum codec for [`ProtoEnum`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EnumCodec<E, K>(PhantomData<(E, K)>);

/// Allocator-free enum type marker (`Open` / `Closed`).
pub type ProtoEnum<E, K> = Numerical<EnumCodec<E, K>>;

impl<E: OpenEnum> NumericalType for EnumCodec<E, Open> {
    type NativeType = E;
    type WireBody = VarintPayload;

    #[inline]
    fn to_wire_body(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()))
    }

    #[inline]
    fn from_wire_body(wire_body: VarintPayload) -> Result<E, DecodeError> {
        let i = wire_body.0.try_to_int32().map_err(DecodeError::from)?;
        Ok(E::from(i))
    }
}

impl<E: ClosedEnum> NumericalType for EnumCodec<E, Closed> {
    type NativeType = E;
    type WireBody = VarintPayload;

    #[inline]
    fn to_wire_body(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()))
    }

    #[inline]
    fn from_wire_body(wire_body: VarintPayload) -> Result<E, DecodeError> {
        let wire = wire_body.0.try_to_int32().map_err(DecodeError::from)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum {
            raw: wire_body.0.to_uint64(),
        })
    }
}

// ---------------------------------------------------------------------------
// Fixed32 / Fixed64 codecs + aliases
// ---------------------------------------------------------------------------

macro_rules! fixed32_numerical {
    ($codec:ident, $alias:ident, $inner:ty) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $codec;

        /// Protobuf type marker (see [`Numerical`] visibility note).
        pub type $alias = Numerical<$codec>;

        impl NumericalType for $codec {
            type NativeType = $inner;
            type WireBody = Fixed32Payload;

            #[inline]
            fn to_wire_body(value: $inner) -> Fixed32Payload {
                Fixed32Payload(value.to_le_bytes())
            }

            #[inline]
            fn from_wire_body(wire_body: Fixed32Payload) -> Result<$inner, DecodeError> {
                Ok(<$inner>::from_le_bytes(wire_body.0))
            }
        }
    };
}

macro_rules! fixed64_numerical {
    ($codec:ident, $alias:ident, $inner:ty) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $codec;

        /// Protobuf type marker (see [`Numerical`] visibility note).
        pub type $alias = Numerical<$codec>;

        impl NumericalType for $codec {
            type NativeType = $inner;
            type WireBody = Fixed64Payload;

            #[inline]
            fn to_wire_body(value: $inner) -> Fixed64Payload {
                Fixed64Payload(value.to_le_bytes())
            }

            #[inline]
            fn from_wire_body(wire_body: Fixed64Payload) -> Result<$inner, DecodeError> {
                Ok(<$inner>::from_le_bytes(wire_body.0))
            }
        }
    };
}

fixed32_numerical!(Fixed32Codec, ProtoFixed32, u32);
fixed32_numerical!(SFixed32Codec, ProtoSFixed32, i32);
fixed32_numerical!(FloatCodec, ProtoFloat, f32);
fixed64_numerical!(Fixed64Codec, ProtoFixed64, u64);
fixed64_numerical!(SFixed64Codec, ProtoSFixed64, i64);
fixed64_numerical!(DoubleCodec, ProtoDouble, f64);
