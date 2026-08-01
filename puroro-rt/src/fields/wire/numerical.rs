//! Codec for numerical protobuf **types** (`int32` / `ProtoInt32`,
//! `bool` / [`ProtoBool`](super::varint::ProtoBool), `fixed64` / `ProtoFixed64`,
//! open/closed enums, … — not Len types such as string / bytes / message).
//!
//! [`NativeType`](NumericalType::NativeType) is the host-language copy value used for
//! encode/decode and field get/set — not necessarily the singular struct slot
//! type (`AddressableSlot` lives on [`PayloadAccess`](super::singular_type::PayloadAccess);
//! singular [`ProtoBool`](super::varint::ProtoBool) uses [`BitPacked`](crate::BitPacked)).
//! Maps `NativeType` ↔ [`WireBody`](NumericalType::WireBody)
//! ([`CopyWirePayload`](super::wire_payload::CopyWirePayload)).
//! Tagged encode is the [`EncodeType`](super::encode_type::EncodeType) blanket
//! over `NumericalType`. Packed repeated merge / encode live on
//! [`RepeatedElementMerge`](super::repeated_element::RepeatedElementMerge) /
//! [`PackableRepeatedElement`](super::repeated_element::PackableRepeatedElement).

use ::protobuf_core::Varint;

use ::puroro::DecodeError;

use crate::fields::shared::ProtoEmpty;

use super::fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSInt32,
    ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
use super::wire_payload::{CopyWirePayload, Fixed32Payload, Fixed64Payload, VarintPayload};

/// Numerical protobuf **type** marker (e.g. `ProtoInt32`, `ProtoBool`,
/// `ProtoFixed64`, `ProtoEnum<…>` — not `string` / `bytes` / message).
pub trait NumericalType: Sized {
    /// Host-language value for encode/decode and field get/set (not necessarily
    /// the singular struct slot type).
    type NativeType: Copy + Default + ProtoEmpty;

    /// Wire-shape body for this proto type (e.g. [`VarintPayload`] for `int32`).
    type WireBody: CopyWirePayload;

    fn to_wire_body(value: Self::NativeType) -> Self::WireBody;

    fn from_wire_body(wire_body: Self::WireBody) -> Result<Self::NativeType, DecodeError>;
}

// ---------------------------------------------------------------------------
// Varint numerics
// ---------------------------------------------------------------------------

macro_rules! impl_varint_numerical {
    ($marker:ty, $inner:ty, decode = $decode:expr, encode = $encode:expr $(,)?) => {
        impl NumericalType for $marker {
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

impl_varint_numerical! {
    ProtoUInt32,
    u32,
    decode = |raw: Varint| raw.try_to_uint32().map_err(DecodeError::from),
    encode = Varint::from_uint32,
}

impl_varint_numerical! {
    ProtoUInt64,
    u64,
    decode = |raw: Varint| Ok(raw.to_uint64()),
    encode = Varint::from_uint64,
}

impl_varint_numerical! {
    ProtoInt32,
    i32,
    decode = |raw: Varint| raw.try_to_int32().map_err(DecodeError::from),
    encode = Varint::from_int32,
}

impl_varint_numerical! {
    ProtoInt64,
    i64,
    decode = |raw: Varint| Ok(raw.to_int64()),
    encode = Varint::from_int64,
}

impl_varint_numerical! {
    ProtoSInt32,
    i32,
    decode = |raw: Varint| raw.try_to_sint32().map_err(DecodeError::from),
    encode = Varint::from_sint32,
}

impl_varint_numerical! {
    ProtoSInt64,
    i64,
    decode = |raw: Varint| Ok(raw.to_sint64()),
    encode = Varint::from_sint64,
}

impl_varint_numerical! {
    ProtoBool,
    bool,
    decode = |raw: Varint| Ok(raw.to_bool()),
    encode = Varint::from_bool,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

impl<E: OpenEnum> NumericalType for ProtoEnum<E, Open> {
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

impl<E: ClosedEnum> NumericalType for ProtoEnum<E, Closed> {
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
// Fixed32 / Fixed64
// ---------------------------------------------------------------------------

macro_rules! impl_fixed32_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
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

macro_rules! impl_fixed64_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
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

impl_fixed32_numerical!(ProtoFixed32, u32);
impl_fixed32_numerical!(ProtoSFixed32, i32);
impl_fixed32_numerical!(ProtoFloat, f32);
impl_fixed64_numerical!(ProtoFixed64, u64);
impl_fixed64_numerical!(ProtoSFixed64, i64);
impl_fixed64_numerical!(ProtoDouble, f64);
