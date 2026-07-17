//! Semantic protobuf types that share wire type [`WireType::Len`].
//!
//! Markers [`ProtoString`] / [`ProtoBytes`] are allocator-free. Singular and
//! repeated storage is [`UnmanagedString`] / [`UnmanagedVec`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::core::str;
use ::unmanaged::string::StringGuard;
use ::unmanaged::vec::VecGuard;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::decode;
use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DeallocateIn, DefaultIn};
use ::puroro::DecodeError;
use ::puroro::WireType;

/// Wire helpers for length-delimited markers (string / bytes).
pub trait LenProtoType {
    type Ref<'a>
    where
        Self: 'a;

    type Mut<'a, A: Allocator + Clone + 'a>
    where
        Self: 'a;

    type Storage<A: Allocator + Clone>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>;

    fn as_bytes<A: Allocator + Clone>(value: &Self::Storage<A>) -> &[u8];

    fn get<'a, A: Allocator + Clone>(value: &'a Self::Storage<A>) -> Self::Ref<'a>;

    fn with_alloc<'a, A: Allocator + Clone>(
        value: &'a mut Self::Storage<A>,
        alloc: A,
    ) -> Self::Mut<'a, A>;

    fn decode<A: Allocator + Clone, B: Buf>(
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError>;

    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError>;
}

/// Protobuf `string` type marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoString;

impl LenProtoType for ProtoString {
    type Ref<'a> = &'a str;
    type Mut<'a, A: Allocator + Clone + 'a> = StringGuard<'a, A>;
    type Storage<A: Allocator + Clone> = UnmanagedString<A>;

    #[inline]
    fn as_bytes<A: Allocator + Clone>(value: &Self::Storage<A>) -> &[u8] {
        value.as_bytes()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone>(value: &'a Self::Storage<A>) -> &'a str {
        value
    }

    #[inline]
    fn with_alloc<'a, A: Allocator + Clone>(
        value: &'a mut Self::Storage<A>,
        alloc: A,
    ) -> StringGuard<'a, A> {
        // SAFETY: caller passes the allocator that owns this buffer.
        unsafe { value.with_alloc(alloc) }
    }

    #[inline]
    fn decode<A: Allocator + Clone, B: Buf>(
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        decode::decode_string_in(buf, alloc)
    }

    #[inline]
    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        let s = str::from_utf8(v).map_err(|_| DecodeError::InvalidUtf8)?;
        Ok(decode::str_to_unmanaged_in(s, alloc))
    }
}

/// Protobuf `bytes` type marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBytes;

impl LenProtoType for ProtoBytes {
    type Ref<'a> = &'a [u8];
    type Mut<'a, A: Allocator + Clone + 'a> = VecGuard<'a, u8, A>;
    type Storage<A: Allocator + Clone> = UnmanagedVec<u8, A>;

    #[inline]
    fn as_bytes<A: Allocator + Clone>(value: &Self::Storage<A>) -> &[u8] {
        value
    }

    #[inline]
    fn get<'a, A: Allocator + Clone>(value: &'a Self::Storage<A>) -> &'a [u8] {
        value
    }

    #[inline]
    fn with_alloc<'a, A: Allocator + Clone>(
        value: &'a mut Self::Storage<A>,
        alloc: A,
    ) -> VecGuard<'a, u8, A> {
        // SAFETY: caller passes the allocator that owns this buffer.
        unsafe { value.with_alloc(alloc) }
    }

    #[inline]
    fn decode<A: Allocator + Clone, B: Buf>(
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        decode::decode_bytes_in(buf, alloc)
    }

    #[inline]
    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        Ok(decode::bytes_to_unmanaged_in(v, alloc))
    }
}

/// Always [`WireType::Len`] for singular field merge/encode checks.
pub(crate) const WIRE_TYPE: WireType = WireType::Len;
