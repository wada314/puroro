//! Codec for copy-inline numerical protobuf **types** (`int32` / `ProtoInt32`,
//! `fixed64` / `ProtoFixed64`, open/closed enums, … — not Len types such as
//! string / bytes / message, and not [`ProtoBool`](super::varint::ProtoBool)).
//!
//! Maps [`Value`](NumericalType::Value) ↔ [`CopyWirePayload`](super::wire_payload::CopyWirePayload).
//! Tagged encode goes through [`EncodeType`](super::encode_type::EncodeType).
//! Packed repeated encode is derived in
//! [`PackableRepeatedElement`](super::repeated_element::PackableRepeatedElement).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES, Varint};

use ::puroro::{DecodeError, WireType};

use crate::decode;
use crate::fields::shared::ProtoEmpty;
use crate::fields::shared::value_slot::AddressableSlot;

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

/// Copy-inline numerical protobuf **type** marker (e.g. `ProtoInt32`,
/// `ProtoFixed64`, `ProtoEnum<…>` — not `bool` / `string` / `bytes` / message).
pub trait NumericalType: Sized {
    /// Singular slot / repeated element / written value.
    type Value: Copy + Default + ProtoEmpty + AddressableSlot;

    /// Wire-shape body for this proto type (e.g. [`VarintPayload`] for `int32`).
    type Raw: CopyWirePayload;

    fn to_raw(value: Self::Value) -> Self::Raw;

    fn from_raw(raw: Self::Raw) -> Result<Self::Value, DecodeError>;

    /// Decode one singular occurrence after the tag has been read.
    ///
    /// For varint markers, [`DecodeError::UnknownClosedEnum`] may be returned
    /// (singular merge parks it; repeated merge propagates the error).
    #[inline]
    fn decode_wire_value(
        wire_type: WireType,
        buf: &mut impl Buf,
    ) -> Result<Self::Value, DecodeError> {
        Self::from_raw(Self::Raw::decode(wire_type, buf)?)
    }

    /// Untagged wire-body length of one value (no tag).
    #[inline]
    fn payload_len(value: Self::Value) -> usize {
        Self::to_raw(value).encoded_len()
    }

    /// Write untagged wire-body bytes of one value (no tag).
    #[inline]
    fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
        Self::to_raw(value).encode(buf);
    }

    /// Merge one wire occurrence into `push` (expanded or packed).
    fn merge_occurrence(
        wire_type: WireType,
        buf: &mut impl Buf,
        mut push: impl FnMut(Self::Value),
    ) -> Result<(), DecodeError> {
        match Self::Raw::WIRE_TYPE {
            WireType::Varint => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(Self::decode_wire_value(WireType::Varint, &mut sub)?);
                    }
                    Ok(())
                }
                WireType::Varint => {
                    push(Self::decode_wire_value(WireType::Varint, buf)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Int32 => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    if !len.is_multiple_of(FIXED32_BYTES) {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(Self::decode_wire_value(WireType::Int32, &mut sub)?);
                    }
                    Ok(())
                }
                WireType::Int32 => {
                    push(Self::decode_wire_value(WireType::Int32, buf)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Int64 => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    if !len.is_multiple_of(FIXED64_BYTES) {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(Self::decode_wire_value(WireType::Int64, &mut sub)?);
                    }
                    Ok(())
                }
                WireType::Int64 => {
                    push(Self::decode_wire_value(WireType::Int64, buf)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Len | WireType::SGroup | WireType::EGroup => {
                unreachable!("numerical Raw is never Len or group")
            }
        }
    }
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
    decode = |raw| Varint::from_uint64(raw).try_to_uint32().map_err(DecodeError::from),
    encode = |value| Varint::from_uint32(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoUInt64,
    u64,
    decode = |raw| Ok(Varint::from_uint64(raw).to_uint64()),
    encode = |value| Varint::from_uint64(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoInt32,
    i32,
    decode = |raw| Varint::from_uint64(raw).try_to_int32().map_err(DecodeError::from),
    encode = |value| Varint::from_int32(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoInt64,
    i64,
    decode = |raw| Ok(Varint::from_uint64(raw).to_int64()),
    encode = |value| Varint::from_int64(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoSInt32,
    i32,
    decode = |raw| Varint::from_uint64(raw).try_to_sint32().map_err(DecodeError::from),
    encode = |value| Varint::from_sint32(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoSInt64,
    i64,
    decode = |raw| Ok(Varint::from_uint64(raw).to_sint64()),
    encode = |value| Varint::from_sint64(value).to_uint64(),
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

impl<E: OpenEnum> NumericalType for ProtoEnum<E, Open> {
    type Value = E;
    type Raw = VarintPayload;

    #[inline]
    fn to_raw(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()).to_uint64())
    }

    #[inline]
    fn from_raw(raw: VarintPayload) -> Result<E, DecodeError> {
        let i = Varint::from_uint64(raw.0)
            .try_to_int32()
            .map_err(DecodeError::from)?;
        Ok(E::from(i))
    }
}

impl<E: ClosedEnum> NumericalType for ProtoEnum<E, Closed> {
    type Value = E;
    type Raw = VarintPayload;

    #[inline]
    fn to_raw(value: E) -> VarintPayload {
        VarintPayload(Varint::from_int32(value.to_wire()).to_uint64())
    }

    #[inline]
    fn from_raw(raw: VarintPayload) -> Result<E, DecodeError> {
        let wire = Varint::from_uint64(raw.0)
            .try_to_int32()
            .map_err(DecodeError::from)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum { raw: raw.0 })
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
        T::payload_len(value)
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: T::Value, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        T::encode_payload(value, buf);
    }
}
