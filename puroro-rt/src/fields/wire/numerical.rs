//! Codec for numerical protobuf **types** (`int32` / `ProtoInt32`,
//! `fixed64` / `ProtoFixed64`, open/closed enums, … — not Len types such as
//! string / bytes / message, and not [`ProtoBool`](super::varint::ProtoBool)).
//!
//! [`Value`](NumericalType::Value) is the **logical** copy value used for
//! encode/decode and field get/set — not necessarily the singular struct slot
//! type (`AddressableSlot` lives on [`PayloadAccess`](super::singular_type::PayloadAccess)).
//! Maps `Value` ↔ [`CopyWirePayload`](super::wire_payload::CopyWirePayload).
//! Tagged encode goes through [`EncodeType`](super::encode_type::EncodeType).
//! Packed repeated merge / encode live on
//! [`RepeatedElementMerge`](super::repeated_element::RepeatedElementMerge) /
//! [`PackableRepeatedElement`](super::repeated_element::PackableRepeatedElement).
//!
//! [`ProtoBool`](super::varint::ProtoBool) stays outside this trait for now
//! (singular bit-pack; repeated / encode are handwritten).

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::protobuf_core::Varint;

use ::puroro::{DecodeError, WireType};

use crate::fields::shared::ProtoEmpty;

use super::encode_type::EncodeType;
use super::fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSInt32,
    ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
use super::wire_payload::{
    CopyWirePayload, Fixed32Payload, Fixed64Payload, VarintPayload, WirePayload,
};

/// Numerical protobuf **type** marker (e.g. `ProtoInt32`, `ProtoFixed64`,
/// `ProtoEnum<…>` — not `bool` / `string` / `bytes` / message).
pub trait NumericalType: Sized {
    /// Logical value for encode/decode and field get/set (not necessarily the
    /// singular struct slot type).
    type Value: Copy + Default + ProtoEmpty;

    /// Wire-shape body for this proto type (e.g. [`VarintPayload`] for `int32`).
    type Raw: CopyWirePayload;

    fn to_raw(value: Self::Value) -> Self::Raw;

    fn from_raw(raw: Self::Raw) -> Result<Self::Value, DecodeError>;
}

// ---------------------------------------------------------------------------
// Varint numerics
// ---------------------------------------------------------------------------

macro_rules! impl_varint_numerical {
    ($marker:ty, $inner:ty, decode = $decode:expr, encode = $encode:expr $(,)?) => {
        impl NumericalType for $marker {
            type Value = $inner;
            type Raw = VarintPayload;

            #[inline]
            fn to_raw(value: $inner) -> VarintPayload {
                VarintPayload(($encode)(value))
            }

            #[inline]
            fn from_raw(raw: VarintPayload) -> Result<$inner, DecodeError> {
                ($decode)(raw.0)
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

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

impl<E: OpenEnum> NumericalType for ProtoEnum<E, Open> {
    type Value = E;
    type Raw = VarintPayload;

    #[inline]
    fn to_raw(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()))
    }

    #[inline]
    fn from_raw(raw: VarintPayload) -> Result<E, DecodeError> {
        let i = raw.0.try_to_int32().map_err(DecodeError::from)?;
        Ok(E::from(i))
    }
}

impl<E: ClosedEnum> NumericalType for ProtoEnum<E, Closed> {
    type Value = E;
    type Raw = VarintPayload;

    #[inline]
    fn to_raw(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()))
    }

    #[inline]
    fn from_raw(raw: VarintPayload) -> Result<E, DecodeError> {
        let wire = raw.0.try_to_int32().map_err(DecodeError::from)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum {
            raw: raw.0.to_uint64(),
        })
    }
}

// ---------------------------------------------------------------------------
// Fixed32 / Fixed64
// ---------------------------------------------------------------------------

macro_rules! impl_fixed32_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
            type Value = $inner;
            type Raw = Fixed32Payload;

            #[inline]
            fn to_raw(value: $inner) -> Fixed32Payload {
                Fixed32Payload(value.to_le_bytes())
            }

            #[inline]
            fn from_raw(raw: Fixed32Payload) -> Result<$inner, DecodeError> {
                Ok(<$inner>::from_le_bytes(raw.0))
            }
        }
    };
}

macro_rules! impl_fixed64_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
            type Value = $inner;
            type Raw = Fixed64Payload;

            #[inline]
            fn to_raw(value: $inner) -> Fixed64Payload {
                Fixed64Payload(value.to_le_bytes())
            }

            #[inline]
            fn from_raw(raw: Fixed64Payload) -> Result<$inner, DecodeError> {
                Ok(<$inner>::from_le_bytes(raw.0))
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

impl<T: NumericalType> EncodeType for T {
    type View<'a, A: Allocator + Clone>
        = T::Value
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = <T::Raw as WirePayload>::WIRE_TYPE;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: T::Value) -> usize
    where
        Self: 'a,
    {
        T::to_raw(value).encoded_len()
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: T::Value, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        T::to_raw(value).encode(buf);
    }
}
