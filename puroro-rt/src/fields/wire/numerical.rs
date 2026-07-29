//! Codec for copy-inline numerical protobuf **types** (`int32` / `ProtoInt32`,
//! `fixed64` / `ProtoFixed64`, open/closed enums, … — not Len types such as
//! string / bytes / message, and not [`ProtoBool`](super::varint::ProtoBool)).
//!
//! Tagged encode goes through [`EncodeType`](super::encode_type::EncodeType) /
//! [`encode_field`](super::encode_type::encode_field); this trait owns decode and
//! single-value payload. Packed repeated encode is derived in
//! [`PackableRepeatedElement`](super::repeated_element::PackableRepeatedElement).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES, Varint};

use ::puroro::{DecodeError, WireType};

use crate::decode;
use crate::encode;
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

/// Wire-type family for a [`NumericalType`] marker: `Varint` (e.g. `int32` /
/// `sint32`), `Fixed32` (e.g. `fixed32` / `float`), or `Fixed64` (e.g. `fixed64` /
/// `double`).
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

/// Copy-inline numerical protobuf **type** marker (e.g. `ProtoInt32`,
/// `ProtoFixed64`, `ProtoEnum<…>` — not `bool` / `string` / `bytes` / message).
pub trait NumericalType: Sized {
    /// Singular slot / repeated element / written value.
    type Value: Copy + Default + ProtoEmpty + AddressableSlot;

    const WIRE: NumericalWireKind;

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
}

impl<E: ClosedEnum> NumericalType for ProtoEnum<E, Closed> {
    type Value = E;
    const WIRE: NumericalWireKind = NumericalWireKind::Varint;

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
        }
    };
}

macro_rules! impl_fixed64_numerical {
    ($marker:ty, $inner:ty) => {
        impl NumericalType for $marker {
            type Value = $inner;
            const WIRE: NumericalWireKind = NumericalWireKind::Fixed64;

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
