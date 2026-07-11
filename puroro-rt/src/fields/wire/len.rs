//! Semantic protobuf types that share wire type [`WireType::Len`].
//!
//! Payload decode uses [`crate::decode`] helpers; LEN record framing uses
//! [`crate::encode::encode_len_field`].
//!
//! # Singular vs repeated
//!
//! [`ProtoString`] / [`ProtoBytes`] are **thin wrappers** over allocator-less
//! storage. Singular fields store the wrapper
//! ([`ScalarProtoType`](super::scalar::ScalarProtoType)). Repeated fields keep
//! [`LenProtoType::Storage`] (the inner `UnmanagedString` / `UnmanagedVec<u8>`)
//! in the element buffer so `as_slice()` stays `&[UnmanagedString]` / …
//!
//! The allocator is supplied on every operation that (de)allocates, so a
//! generated message keeps a single allocator in
//! [`MessageCommon`](crate::fields::shared::MessageCommon).

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::unmanaged::string::StringGuard;
use ::unmanaged::vec::VecGuard;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::decode;
use crate::fields::shared::{DeallocateIn, DefaultIn, ProtoEmpty};
use ::puroro::DecodeError;
use ::puroro::WireType;

/// Wire semantics for protobuf types encoded as length-delimited records.
///
/// [`Storage`](Self::Storage) is the **element type for repeated fields**.
/// Singular fields store the thin wrapper type itself.
pub trait LenProtoType {
    /// Owned, allocator-less element storage for **repeated** fields
    /// (`UnmanagedString`, `UnmanagedVec<u8>`, …).
    type Storage: DefaultIn + DeallocateIn + ProtoEmpty;

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
    fn new_in<A: Allocator>(alloc: A) -> Self::Storage;

    /// `true` when the field should be omitted on encode (IMPLICIT presence).
    fn is_empty(value: &Self::Storage) -> bool;

    /// Returns the getter view of `value` (e.g. `&str`, `&[u8]`).
    fn get<'a>(value: &'a Self::Storage) -> Self::Ref<'a>;

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

/// Protobuf `string` — thin wrapper over [`UnmanagedString`].
#[repr(transparent)]
pub struct ProtoString(pub UnmanagedString);

impl ::core::fmt::Debug for ProtoString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtoString").field(&&*self.0).finish()
    }
}

impl DefaultIn for ProtoString {
    #[inline]
    fn default_in<A: Allocator>(alloc: A) -> Self {
        Self(UnmanagedString::new(alloc))
    }
}

impl DeallocateIn for ProtoString {
    #[inline]
    unsafe fn deallocate_in<A: Allocator>(self, alloc: A) {
        unsafe { self.0.deallocate(alloc) };
    }
}

impl ProtoEmpty for ProtoString {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl LenProtoType for ProtoString {
    type Storage = UnmanagedString;
    type Ref<'a> = &'a str;
    type Mut<'a, A: Allocator + 'a>
        = StringGuard<'a, A>
    where
        Self: 'a;

    fn new_in<A: Allocator>(alloc: A) -> Self::Storage {
        UnmanagedString::new(alloc)
    }

    fn is_empty(value: &Self::Storage) -> bool {
        value.is_empty()
    }

    fn get<'a>(value: &'a Self::Storage) -> Self::Ref<'a> {
        value
    }

    fn as_bytes(value: &Self::Storage) -> &[u8] {
        value.as_bytes()
    }

    fn with_alloc<'a, A: Allocator + 'a>(
        value: &'a mut Self::Storage,
        alloc: A,
    ) -> Self::Mut<'a, A> {
        // SAFETY: the caller always passes the same allocator that owns this
        // string's buffer.
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

/// Protobuf `bytes` — thin wrapper over [`UnmanagedVec<u8>`].
#[repr(transparent)]
pub struct ProtoBytes(pub UnmanagedVec<u8>);

impl ::core::fmt::Debug for ProtoBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtoBytes").field(&&*self.0).finish()
    }
}

impl DefaultIn for ProtoBytes {
    #[inline]
    fn default_in<A: Allocator>(alloc: A) -> Self {
        Self(UnmanagedVec::new(alloc))
    }
}

impl DeallocateIn for ProtoBytes {
    #[inline]
    unsafe fn deallocate_in<A: Allocator>(self, alloc: A) {
        unsafe { self.0.deallocate(alloc) };
    }
}

impl ProtoEmpty for ProtoBytes {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl LenProtoType for ProtoBytes {
    type Storage = UnmanagedVec<u8>;
    type Ref<'a> = &'a [u8];
    type Mut<'a, A: Allocator + 'a>
        = VecGuard<'a, u8, A>
    where
        Self: 'a;

    fn new_in<A: Allocator>(alloc: A) -> Self::Storage {
        UnmanagedVec::new(alloc)
    }

    fn is_empty(value: &Self::Storage) -> bool {
        value.is_empty()
    }

    fn get<'a>(value: &'a Self::Storage) -> Self::Ref<'a> {
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
pub(crate) const WIRE_TYPE: WireType = WireType::Len;
