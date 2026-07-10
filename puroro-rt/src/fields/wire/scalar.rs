//! Unified wire/storage semantics for singular scalar fields (varint + LEN).
//!
//! [`VarintProtoType`](super::varint::VarintProtoType) and
//! [`LenProtoType`](super::len::LenProtoType) remain for repeated-field helpers;
//! singular fields go through [`ScalarProtoType`] so one
//! [`SingularField`](crate::fields::singular::field::SingularField) wrapper
//! covers both wire families.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::DerefMut;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::{DefaultIn, DeallocateIn, ProtoEmpty};

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::varint::{
    ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64,
    ProtoUInt32, ProtoUInt64, VarintProtoType,
};

/// Wire + storage semantics for a singular scalar protobuf type.
///
/// Implemented once per protobuf *type* (not per message field). Drives
/// [`SingularField`](crate::fields::singular::field::SingularField) encode /
/// decode / accessor behaviour for both varint and LEN payloads.
pub trait ScalarProtoType {
    /// Owned storage in a generated message field (`i32`, `UnmanagedString`, …).
    type Storage: DefaultIn + DeallocateIn + ProtoEmpty;

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

    /// Borrows (or copies) the stored value for accessors.
    fn borrow<'a>(storage: &'a Self::Storage) -> Self::Ref<'a>;

    /// Builds a mutable accessor handle from storage + allocator.
    fn with_mut<'a, A: Allocator + 'a>(
        storage: &'a mut Self::Storage,
        alloc: A,
    ) -> Self::Mut<'a, A>;

    /// Wire byte length of one tagged occurrence of `storage`.
    fn encoded_len(field: u32, storage: &Self::Storage) -> usize;

    /// Encodes one tagged occurrence of `storage`.
    fn encode<B: BufMut>(field: u32, storage: &Self::Storage, buf: &mut B);

    /// Decodes one occurrence after the tag has been read (`wire_type` checked here).
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Storage, DecodeError>;
}

// ---------------------------------------------------------------------------
// Varint markers
// ---------------------------------------------------------------------------

macro_rules! impl_varint_scalar {
    ($ty:ty) => {
        impl ScalarProtoType for $ty {
            type Storage = <Self as VarintProtoType>::Value;
            type Ref<'a> = <Self as VarintProtoType>::Value;
            type Mut<'a, A: Allocator + 'a> = &'a mut <Self as VarintProtoType>::Value;
            const WIRE_TYPE: WireType = WireType::Varint;

            #[inline]
            fn borrow<'a>(storage: &'a Self::Storage) -> Self::Ref<'a> {
                *storage
            }

            #[inline]
            fn with_mut<'a, A: Allocator + 'a>(
                storage: &'a mut Self::Storage,
                _alloc: A,
            ) -> Self::Mut<'a, A> {
                storage
            }

            #[inline]
            fn encoded_len(field: u32, storage: &Self::Storage) -> usize {
                encode::encoded_len_varint_field(field, <Self as VarintProtoType>::encode_wire(*storage))
            }

            #[inline]
            fn encode<B: BufMut>(field: u32, storage: &Self::Storage, buf: &mut B) {
                encode::encode_varint_field(
                    field,
                    <Self as VarintProtoType>::encode_wire(*storage),
                    buf,
                );
            }

            #[inline]
            fn decode<B: Buf, A: Allocator>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
            ) -> Result<Self::Storage, DecodeError> {
                if wire_type != WireType::Varint {
                    return Err(DecodeError::InvalidTag);
                }
                let raw = decode::decode_varint(buf)?;
                <Self as VarintProtoType>::decode_wire(raw)
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
impl_varint_scalar!(ProtoBool);

impl<E: ProtoEnumStorage> ScalarProtoType for ProtoEnum<E> {
    type Storage = E;
    type Ref<'a> = E where E: 'a;
    type Mut<'a, A: Allocator + 'a> = &'a mut E where E: 'a;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn borrow<'a>(storage: &'a Self::Storage) -> Self::Ref<'a> {
        *storage
    }

    #[inline]
    fn with_mut<'a, A: Allocator + 'a>(
        storage: &'a mut Self::Storage,
        _alloc: A,
    ) -> Self::Mut<'a, A> {
        storage
    }

    #[inline]
    fn encoded_len(field: u32, storage: &Self::Storage) -> usize {
        encode::encoded_len_varint_field(field, <Self as VarintProtoType>::encode_wire(*storage))
    }

    #[inline]
    fn encode<B: BufMut>(field: u32, storage: &Self::Storage, buf: &mut B) {
        encode::encode_varint_field(field, <Self as VarintProtoType>::encode_wire(*storage), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
    ) -> Result<Self::Storage, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        <Self as VarintProtoType>::decode_wire(raw)
    }
}

// ---------------------------------------------------------------------------
// LEN markers
// ---------------------------------------------------------------------------

macro_rules! impl_len_scalar {
    ($ty:ty) => {
        impl ScalarProtoType for $ty {
            type Storage = <Self as LenProtoType>::Storage;
            type Ref<'a> = <Self as LenProtoType>::Ref<'a>;
            type Mut<'a, A: Allocator + 'a> = <Self as LenProtoType>::Mut<'a, A>;
            const WIRE_TYPE: WireType = WireType::Len;

            #[inline]
            fn borrow<'a>(storage: &'a Self::Storage) -> Self::Ref<'a> {
                <Self as LenProtoType>::borrow(storage)
            }

            #[inline]
            fn with_mut<'a, A: Allocator + 'a>(
                storage: &'a mut Self::Storage,
                alloc: A,
            ) -> Self::Mut<'a, A> {
                <Self as LenProtoType>::with_alloc(storage, alloc)
            }

            #[inline]
            fn encoded_len(field: u32, storage: &Self::Storage) -> usize {
                encode::encoded_len_len_field(field, <Self as LenProtoType>::as_bytes(storage).len())
            }

            #[inline]
            fn encode<B: BufMut>(field: u32, storage: &Self::Storage, buf: &mut B) {
                encode::encode_len_field(field, <Self as LenProtoType>::as_bytes(storage), buf);
            }

            #[inline]
            fn decode<B: Buf, A: Allocator>(
                wire_type: WireType,
                buf: &mut B,
                alloc: A,
            ) -> Result<Self::Storage, DecodeError> {
                if wire_type != WireType::Len {
                    return Err(DecodeError::InvalidTag);
                }
                <Self as LenProtoType>::decode(buf, alloc)
            }
        }
    };
}

impl_len_scalar!(ProtoString);
impl_len_scalar!(ProtoBytes);
