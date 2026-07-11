//! Unified wire/storage semantics for singular scalar fields (varint + LEN).
//!
//! Each implementor is a **thin wrapper** over its payload (`ProtoInt32(i32)`,
//! `ProtoString(UnmanagedString)`, …). [`SingularField`] stores `T` directly.
//!
//! [`VarintProtoType`](super::varint::VarintProtoType) and
//! [`LenProtoType`](super::len::LenProtoType) remain for **repeated** fields,
//! which keep storing the inner [`VarintProtoType::Value`] /
//! [`LenProtoType::Storage`] so public slices stay `&[i32]` / `&[UnmanagedString]`.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::DerefMut;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::{ProtoEmpty, value_slot::AddressableSlot};

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::varint::{
    ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32,
    ProtoUInt64, VarintProtoType,
};

/// Wire + storage semantics for a singular scalar protobuf type.
///
/// The implementor **is** the value stored in a singular field (thin wrapper).
/// Drives [`SingularField`](crate::fields::singular::field::SingularField)
/// encode / decode / accessor behaviour for both varint and LEN payloads.
pub trait ScalarProtoType: AddressableSlot + ProtoEmpty + Sized {
    /// Borrowed / by-value view returned by getters (`i32`, `&str`, …).
    type Ref<'a>
    where
        Self: 'a;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`, …).
    type Mut<'a, A: Allocator + 'a>: DerefMut
    where
        Self: 'a;

    /// Expected wire type for a singular occurrence of this field.
    const WIRE_TYPE: WireType;

    /// Returns the getter view (by reference or by value).
    fn get(&self) -> Self::Ref<'_>;

    /// Builds a mutable accessor handle, taking an owned allocator clone when
    /// the payload is heap-backed.
    fn with_mut<'a, A: Allocator + 'a>(&'a mut self, alloc: A) -> Self::Mut<'a, A>;

    /// Wire byte length of one tagged occurrence.
    fn encoded_len(&self, field: u32) -> usize;

    /// Encodes one tagged occurrence.
    fn encode<B: BufMut>(&self, field: u32, buf: &mut B);

    /// Decodes one occurrence after the tag has been read (`wire_type` checked here).
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self, DecodeError>;
}

// ---------------------------------------------------------------------------
// Varint wrappers
// ---------------------------------------------------------------------------

macro_rules! impl_varint_scalar {
    ($ty:ty) => {
        impl ScalarProtoType for $ty {
            type Ref<'a> = <Self as VarintProtoType>::Value;
            type Mut<'a, A: Allocator + 'a> = &'a mut <Self as VarintProtoType>::Value;
            const WIRE_TYPE: WireType = WireType::Varint;

            #[inline]
            fn get(&self) -> Self::Ref<'_> {
                self.0
            }

            #[inline]
            fn with_mut<'a, A: Allocator + 'a>(&'a mut self, _alloc: A) -> Self::Mut<'a, A> {
                &mut self.0
            }

            #[inline]
            fn encoded_len(&self, field: u32) -> usize {
                encode::encoded_len_varint_field(
                    field,
                    <Self as VarintProtoType>::encode_wire(self.0),
                )
            }

            #[inline]
            fn encode<B: BufMut>(&self, field: u32, buf: &mut B) {
                encode::encode_varint_field(
                    field,
                    <Self as VarintProtoType>::encode_wire(self.0),
                    buf,
                );
            }

            #[inline]
            fn decode<B: Buf, A: Allocator>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
            ) -> Result<Self, DecodeError> {
                if wire_type != WireType::Varint {
                    return Err(DecodeError::InvalidTag);
                }
                let raw = decode::decode_varint(buf)?;
                Ok(Self::from(<Self as VarintProtoType>::decode_wire(raw)?))
            }
        }
    };
}

impl_varint_scalar!(ProtoUInt32);
impl_varint_scalar!(ProtoUInt64);
impl_varint_scalar!(ProtoInt32);
impl_varint_scalar!(ProtoInt64);
impl_varint_scalar!(ProtoSint32);
impl_varint_scalar!(ProtoSint64);

// `ProtoBool` is bit-packed: encode/merge go through `SingularField` + wire
// helpers, not `ScalarProtoType` on an addressable `&self` payload.

impl<E: ProtoEnumStorage> ScalarProtoType for ProtoEnum<E> {
    type Ref<'a>
        = E
    where
        E: 'a;
    type Mut<'a, A: Allocator + 'a>
        = &'a mut E
    where
        E: 'a;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn get(&self) -> E {
        self.0
    }

    #[inline]
    fn with_mut<'a, A: Allocator + 'a>(&'a mut self, _alloc: A) -> &'a mut E {
        &mut self.0
    }

    #[inline]
    fn encoded_len(&self, field: u32) -> usize {
        encode::encoded_len_varint_field(field, <Self as VarintProtoType>::encode_wire(self.0))
    }

    #[inline]
    fn encode<B: BufMut>(&self, field: u32, buf: &mut B) {
        encode::encode_varint_field(field, <Self as VarintProtoType>::encode_wire(self.0), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
    ) -> Result<Self, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Ok(Self(<Self as VarintProtoType>::decode_wire(raw)?))
    }
}

// ---------------------------------------------------------------------------
// LEN wrappers
// ---------------------------------------------------------------------------

impl ScalarProtoType for ProtoString {
    type Ref<'a> = &'a str;
    type Mut<'a, A: Allocator + 'a> = <Self as LenProtoType>::Mut<'a, A>;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn get(&self) -> &str {
        &self.0
    }

    #[inline]
    fn with_mut<'a, A: Allocator + 'a>(&'a mut self, alloc: A) -> Self::Mut<'a, A> {
        <Self as LenProtoType>::with_alloc(&mut self.0, alloc)
    }

    #[inline]
    fn encoded_len(&self, field: u32) -> usize {
        encode::encoded_len_len_field(field, self.0.as_bytes().len())
    }

    #[inline]
    fn encode<B: BufMut>(&self, field: u32, buf: &mut B) {
        encode::encode_len_field(field, self.0.as_bytes(), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}

impl ScalarProtoType for ProtoBytes {
    type Ref<'a> = &'a [u8];
    type Mut<'a, A: Allocator + 'a> = <Self as LenProtoType>::Mut<'a, A>;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn get(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    fn with_mut<'a, A: Allocator + 'a>(&'a mut self, alloc: A) -> Self::Mut<'a, A> {
        <Self as LenProtoType>::with_alloc(&mut self.0, alloc)
    }

    #[inline]
    fn encoded_len(&self, field: u32) -> usize {
        encode::encoded_len_len_field(field, self.0.len())
    }

    #[inline]
    fn encode<B: BufMut>(&self, field: u32, buf: &mut B) {
        encode::encode_len_field(field, &self.0, buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}
