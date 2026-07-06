//! Semantic protobuf types that share wire type [`WireType::Len`].
//!
//! Payload decode uses [`crate::decode`] helpers; LEN record framing uses
//! [`crate::encode::encode_len_field`].
//!
//! Storage is allocator-less ([`UnmanagedString`] / [`UnmanagedVec`]): the
//! allocator is supplied on every operation that (de)allocates, so a generated
//! message keeps a single allocator in [`MessageCommon`](crate::fields::shared::MessageCommon).

use ::bytes::Buf;
use ::allocator_api2::alloc::Allocator;
use ::unmanaged::string::StringGuard;
use ::unmanaged::vec::VecGuard;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::decode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

/// Wire semantics for protobuf types encoded as length-delimited records.
pub trait LenProtoType {
    /// Owned, allocator-less storage in a generated message field
    /// (`UnmanagedString`, `UnmanagedVec<u8>`, …).
    type Storage;

    /// Borrowed view returned by getters (`&str`, `&[u8]`, …).
    type Ref<'a>
    where
        Self: 'a;

    /// Growable handle returned by `_mut` accessors, borrowing the storage
    /// together with an allocator `A`. Dereferences to the full owning type
    /// (`String<A>`, `Vec<u8, A>`) and writes back on drop.
    type Mut<'a, A: Allocator + 'a>
    where
        Self: 'a;

    /// Empty value used for IMPLICIT omit-on-encode and EXPLICIT unset slots.
    fn new_empty<A: Allocator>(alloc: A) -> Self::Storage;

    /// `true` when the field should be omitted on encode (IMPLICIT presence).
    fn is_empty(value: &Self::Storage) -> bool;

    /// Borrows the stored value for accessors.
    fn borrow<'a>(value: &'a Self::Storage) -> Self::Ref<'a>;

    /// Payload bytes for [`encode_len_field`](crate::encode::encode_len_field).
    fn as_bytes(value: &Self::Storage) -> &[u8];

    /// Borrows the storage together with `alloc`, yielding a growable guard.
    fn with_alloc<'a, A: Allocator + 'a>(
        value: &'a mut Self::Storage,
        alloc: A,
    ) -> Self::Mut<'a, A>;

    /// Decodes one LEN payload (length varint + body consumed by helper).
    fn decode<B: Buf, A: Allocator>(buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError>;

    /// Builds storage from an already-decoded payload slice (setters).
    fn store_from_slice<A: Allocator>(v: &[u8], alloc: A) -> Result<Self::Storage, DecodeError>;

    /// Drops the value and frees its backing allocation through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns `value`'s buffer.
    unsafe fn deallocate<A: Allocator>(value: Self::Storage, alloc: A);
}

/// Protobuf `string` — UTF-8 LEN payload stored as [`UnmanagedString`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoString;

impl LenProtoType for ProtoString {
    type Storage = UnmanagedString;
    type Ref<'a> = &'a str;
    type Mut<'a, A: Allocator + 'a>
        = StringGuard<'a, A>
    where
        Self: 'a;

    fn new_empty<A: Allocator>(alloc: A) -> Self::Storage {
        UnmanagedString::new(alloc)
    }

    fn is_empty(value: &Self::Storage) -> bool {
        value.is_empty()
    }

    fn borrow<'a>(value: &'a Self::Storage) -> Self::Ref<'a> {
        value
    }

    fn as_bytes(value: &Self::Storage) -> &[u8] {
        value.as_bytes()
    }

    fn with_alloc<'a, A: Allocator + 'a>(
        value: &'a mut Self::Storage,
        alloc: A,
    ) -> Self::Mut<'a, A> {
        // SAFETY: the caller (`SingularLenField`) always passes the same
        // allocator that owns this string's buffer.
        unsafe { value.with_alloc(alloc) }
    }

    fn decode<B: Buf, A: Allocator>(buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError> {
        decode::decode_string_in(buf, alloc)
    }

    fn store_from_slice<A: Allocator>(v: &[u8], alloc: A) -> Result<Self::Storage, DecodeError> {
        let s = ::core::str::from_utf8(v).map_err(|_| DecodeError::InvalidUtf8)?;
        Ok(decode::str_to_unmanaged_in(s, alloc))
    }

    unsafe fn deallocate<A: Allocator>(value: Self::Storage, alloc: A) {
        // SAFETY: forwarded to the caller's obligation.
        unsafe { value.deallocate(alloc) };
    }
}

/// Protobuf `bytes` — opaque LEN payload stored as [`UnmanagedVec<u8>`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBytes;

impl LenProtoType for ProtoBytes {
    type Storage = UnmanagedVec<u8>;
    type Ref<'a> = &'a [u8];
    type Mut<'a, A: Allocator + 'a>
        = VecGuard<'a, u8, A>
    where
        Self: 'a;

    fn new_empty<A: Allocator>(alloc: A) -> Self::Storage {
        UnmanagedVec::new(alloc)
    }

    fn is_empty(value: &Self::Storage) -> bool {
        value.is_empty()
    }

    fn borrow<'a>(value: &'a Self::Storage) -> Self::Ref<'a> {
        value
    }

    fn as_bytes(value: &Self::Storage) -> &[u8] {
        value
    }

    fn with_alloc<'a, A: Allocator + 'a>(
        value: &'a mut Self::Storage,
        alloc: A,
    ) -> Self::Mut<'a, A> {
        // SAFETY: the caller always passes the same allocator that owns this
        // vector's buffer.
        unsafe { value.with_alloc(alloc) }
    }

    fn decode<B: Buf, A: Allocator>(buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError> {
        decode::decode_bytes_in(buf, alloc)
    }

    fn store_from_slice<A: Allocator>(v: &[u8], alloc: A) -> Result<Self::Storage, DecodeError> {
        Ok(decode::bytes_to_unmanaged_in(v, alloc))
    }

    unsafe fn deallocate<A: Allocator>(value: Self::Storage, alloc: A) {
        // SAFETY: forwarded to the caller's obligation.
        unsafe { value.deallocate(alloc) };
    }
}

/// Always [`WireType::Len`] for singular field merge/encode checks.
pub const WIRE_TYPE: WireType = WireType::Len;
