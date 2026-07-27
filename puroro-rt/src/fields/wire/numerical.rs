//! Unified wire codec for copy-inline numerical markers (varint / fixed32 / fixed64 / enum).
//!
//! [`ProtoBool`](super::varint::ProtoBool), string, bytes, and message markers are
//! intentionally outside this trait (different singular slot shapes).
//!
//! Tagged encode goes through [`WirePayload`](super::wire_payload::WirePayload) /
//! [`encode_field`](super::wire_payload::encode_field); this trait owns decode,
//! single-value payload, and packed slice helpers.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES, Varint};

use ::puroro::{DecodeError, WireType};

use crate::decode;
use crate::encode;
use crate::fields::shared::ProtoEmpty;
use crate::fields::shared::value_slot::AddressableSlot;

use super::fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32,
    ProtoSint64, ProtoUInt32, ProtoUInt64,
};
use super::wire_payload::WirePayload;

/// Wire family for a [`NumericalType`] marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalWireKind {
    Varint,
    Fixed32,
    Fixed64,
}

#[inline]
const fn numerical_wire_type(kind: NumericalWireKind) -> WireType {
    match kind {
        NumericalWireKind::Varint => WireType::Varint,
        NumericalWireKind::Fixed32 => WireType::Int32,
        NumericalWireKind::Fixed64 => WireType::Int64,
    }
}

/// Copy-inline numerical protobuf type marker (not bool / string / bytes / message).
pub trait NumericalType: Sized {
    /// Singular slot / repeated element / written value.
    type Value: Copy + ProtoEmpty + AddressableSlot;

    const WIRE: NumericalWireKind;

    /// Protobuf empty / type-zero value.
    fn default_value() -> Self::Value;

    /// Decode one singular occurrence after the tag has been read.
    ///
    /// For varint markers, [`DecodeError::UnknownClosedEnum`] may be returned
    /// (singular merge parks it; repeated merge propagates the error).
    fn decode_wire_value(
        wire_type: WireType,
        buf: &mut impl Buf,
    ) -> Result<Self::Value, DecodeError>;

    /// Bare payload length of one value (no tag).
    fn payload_len(value: Self::Value) -> usize;

    /// Write bare payload bytes of one value (no tag).
    fn encode_payload(value: Self::Value, buf: &mut impl BufMut);

    /// Packed payload length (no tag / length prefix).
    fn packed_payload_len(values: &[Self::Value]) -> usize;

    /// Write packed payload bytes (no tag / length prefix).
    fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut);

    /// Merge one wire occurrence into `push` (expanded or packed).
    fn merge_occurrence(
        wire_type: WireType,
        buf: &mut impl Buf,
        mut push: impl FnMut(Self::Value),
    ) -> Result<(), DecodeError> {
        match Self::WIRE {
            NumericalWireKind::Varint => match wire_type {
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
            NumericalWireKind::Fixed32 => match wire_type {
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
            NumericalWireKind::Fixed64 => match wire_type {
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
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Varint numerics
// ---------------------------------------------------------------------------

macro_rules! impl_varint_numerical {
    ($marker:ty, $inner:ty, decode = $decode:expr, encode = $encode:expr $(,)?) => {
        impl NumericalType for $marker {
            type Value = $inner;
            const WIRE: NumericalWireKind = NumericalWireKind::Varint;

            #[inline]
            fn default_value() -> Self::Value {
                ::core::default::Default::default()
            }

            #[inline]
            fn decode_wire_value(
                wire_type: WireType,
                buf: &mut impl Buf,
            ) -> Result<Self::Value, DecodeError> {
                if wire_type != WireType::Varint {
                    return Err(DecodeError::InvalidTag);
                }
                let raw = decode::decode_varint(buf)?;
                ($decode)(raw)
            }

            #[inline]
            fn payload_len(value: Self::Value) -> usize {
                encode::encoded_len_varint(($encode)(value))
            }

            #[inline]
            fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
                encode::encode_varint(($encode)(value), buf);
            }

            #[inline]
            fn packed_payload_len(values: &[Self::Value]) -> usize {
                values
                    .iter()
                    .map(|v| <Self as NumericalType>::payload_len(*v))
                    .sum()
            }

            #[inline]
            fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut) {
                for v in values {
                    <Self as NumericalType>::encode_payload(*v, buf);
                }
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
    ProtoSint32,
    i32,
    decode = |raw| Varint::from_uint64(raw).try_to_sint32().map_err(DecodeError::from),
    encode = |value| Varint::from_sint32(value).to_uint64(),
}

impl_varint_numerical! {
    ProtoSint64,
    i64,
    decode = |raw| Ok(Varint::from_uint64(raw).to_sint64()),
    encode = |value| Varint::from_sint64(value).to_uint64(),
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[inline]
fn encode_enum_wire(value: i32) -> u64 {
    Varint::from_int32(value).to_uint64()
}

#[inline]
fn decode_enum_i32(raw: u64) -> Result<i32, DecodeError> {
    Varint::from_uint64(raw)
        .try_to_int32()
        .map_err(DecodeError::from)
}

impl<E: OpenEnum> NumericalType for ProtoEnum<E, Open> {
    type Value = E;
    const WIRE: NumericalWireKind = NumericalWireKind::Varint;

    #[inline]
    fn default_value() -> Self::Value {
        E::proto_zero()
    }

    #[inline]
    fn decode_wire_value(
        wire_type: WireType,
        buf: &mut impl Buf,
    ) -> Result<Self::Value, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Ok(E::from(decode_enum_i32(raw)?))
    }

    #[inline]
    fn payload_len(value: Self::Value) -> usize {
        encode::encoded_len_varint(encode_enum_wire(value.to_wire()))
    }

    #[inline]
    fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
        encode::encode_varint(encode_enum_wire(value.to_wire()), buf);
    }

    #[inline]
    fn packed_payload_len(values: &[Self::Value]) -> usize {
        values
            .iter()
            .map(|v| <Self as NumericalType>::payload_len(*v))
            .sum()
    }

    #[inline]
    fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut) {
        for v in values {
            <Self as NumericalType>::encode_payload(*v, buf);
        }
    }
}

impl<E: ClosedEnum> NumericalType for ProtoEnum<E, Closed> {
    type Value = E;
    const WIRE: NumericalWireKind = NumericalWireKind::Varint;

    #[inline]
    fn default_value() -> Self::Value {
        E::proto_zero()
    }

    #[inline]
    fn decode_wire_value(
        wire_type: WireType,
        buf: &mut impl Buf,
    ) -> Result<Self::Value, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let wire = decode_enum_i32(raw)?;
        E::try_from(wire).map_err(|_| DecodeError::UnknownClosedEnum { raw })
    }

    #[inline]
    fn payload_len(value: Self::Value) -> usize {
        encode::encoded_len_varint(encode_enum_wire(value.to_wire()))
    }

    #[inline]
    fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
        encode::encode_varint(encode_enum_wire(value.to_wire()), buf);
    }

    #[inline]
    fn packed_payload_len(values: &[Self::Value]) -> usize {
        values
            .iter()
            .map(|v| <Self as NumericalType>::payload_len(*v))
            .sum()
    }

    #[inline]
    fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut) {
        for v in values {
            <Self as NumericalType>::encode_payload(*v, buf);
        }
    }
}

// ---------------------------------------------------------------------------
// Fixed32 / Fixed64
// ---------------------------------------------------------------------------

macro_rules! impl_fixed32_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
            type Value = $inner;
            const WIRE: NumericalWireKind = NumericalWireKind::Fixed32;

            #[inline]
            fn default_value() -> Self::Value {
                ::core::default::Default::default()
            }

            #[inline]
            fn decode_wire_value(
                wire_type: WireType,
                buf: &mut impl Buf,
            ) -> Result<Self::Value, DecodeError> {
                if wire_type != WireType::Int32 {
                    return Err(DecodeError::InvalidTag);
                }
                Ok(<$inner>::from_le_bytes(read_fixed32(buf)?))
            }

            #[inline]
            fn payload_len(_value: Self::Value) -> usize {
                FIXED32_BYTES
            }

            #[inline]
            fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
                buf.put_slice(&value.to_le_bytes());
            }

            #[inline]
            fn packed_payload_len(values: &[Self::Value]) -> usize {
                values.len() * FIXED32_BYTES
            }

            #[inline]
            fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut) {
                for v in values {
                    <Self as NumericalType>::encode_payload(*v, buf);
                }
            }
        }
    };
}

macro_rules! impl_fixed64_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
            type Value = $inner;
            const WIRE: NumericalWireKind = NumericalWireKind::Fixed64;

            #[inline]
            fn default_value() -> Self::Value {
                ::core::default::Default::default()
            }

            #[inline]
            fn decode_wire_value(
                wire_type: WireType,
                buf: &mut impl Buf,
            ) -> Result<Self::Value, DecodeError> {
                if wire_type != WireType::Int64 {
                    return Err(DecodeError::InvalidTag);
                }
                Ok(<$inner>::from_le_bytes(read_fixed64(buf)?))
            }

            #[inline]
            fn payload_len(_value: Self::Value) -> usize {
                FIXED64_BYTES
            }

            #[inline]
            fn encode_payload(value: Self::Value, buf: &mut impl BufMut) {
                buf.put_slice(&value.to_le_bytes());
            }

            #[inline]
            fn packed_payload_len(values: &[Self::Value]) -> usize {
                values.len() * FIXED64_BYTES
            }

            #[inline]
            fn encode_packed_payload(values: &[Self::Value], buf: &mut impl BufMut) {
                for v in values {
                    <Self as NumericalType>::encode_payload(*v, buf);
                }
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

impl<T: NumericalType> WirePayload for T {
    type View<'a, A: Allocator + Clone>
        = T::Value
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = numerical_wire_type(T::WIRE);

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
