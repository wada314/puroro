//! Semantic protobuf types that share wire type [`WireType::Len`].
//!
//! Payload decode uses [`crate::decode`] helpers; LEN record framing uses
//! [`crate::encode::encode_len_field`].
//!
//! # Singular vs repeated
//!
//! [`ProtoString`] / [`ProtoBytes`] are thin wrappers over allocator-typed
//! storage. Singular fields store the wrapper
//! ([`ProtoType`](super::proto_type::ProtoType)). Repeated fields keep
//! [`LenProtoType::Storage`] (the inner `UnmanagedString` / `UnmanagedVec<u8>`)
//! in the element buffer so `as_slice()` stays `&[UnmanagedString]` / …
//!
//! The allocator is supplied on every operation that (de)allocates, so a
//! generated message keeps a single allocator in
//! [`MessageCommon`](crate::fields::shared::MessageCommon).

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::core::fmt;
use ::core::str;
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
    /// Allocator type used by this protobuf marker and its storage.
    type Alloc: Allocator + Clone;

    /// Owned element storage for **repeated** fields.
    type Storage: DefaultIn<Alloc = Self::Alloc> + DeallocateIn<Alloc = Self::Alloc> + ProtoEmpty;

    /// Borrowed view returned by getters (`&str`, `&[u8]`, …).
    type Ref<'a>
    where
        Self: 'a;

    /// Growable handle returned by `_mut` accessors, borrowing the storage
    /// together with an allocator `A`. Dereferences to the full owning type
    /// (`String<A>`, `Vec<u8, A>`) and writes back on drop.
    type Mut<'a>
    where
        Self: 'a;

    /// Empty value used for IMPLICIT omit-on-encode and EXPLICIT unset slots.
    fn new_in(alloc: Self::Alloc) -> Self::Storage;

    /// `true` when the field should be omitted on encode (IMPLICIT presence).
    fn is_empty(value: &Self::Storage) -> bool;

    /// Returns the getter view of `value` (e.g. `&str`, `&[u8]`).
    fn get<'a>(value: &'a Self::Storage) -> Self::Ref<'a>;

    /// Payload bytes for [`encode_len_field`](crate::encode::encode_len_field).
    fn as_bytes(value: &Self::Storage) -> &[u8];

    /// Borrows the storage together with `alloc`, yielding a growable guard.
    fn with_alloc<'a>(value: &'a mut Self::Storage, alloc: Self::Alloc) -> Self::Mut<'a>;

    /// Decodes one LEN payload (length varint + body consumed by helper).
    fn decode<B: Buf>(buf: &mut B, alloc: Self::Alloc) -> Result<Self::Storage, DecodeError>;

    /// Builds storage from an already-decoded payload slice (setters).
    fn store_from_slice(v: &[u8], alloc: Self::Alloc) -> Result<Self::Storage, DecodeError>;

    /// Drops the value and frees its backing allocation through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns `value`'s buffer.
    unsafe fn deallocate(value: Self::Storage, alloc: Self::Alloc);
}

/// Protobuf `string` — thin wrapper over [`UnmanagedString`].
#[repr(transparent)]
pub struct ProtoString<A: Allocator>(pub UnmanagedString<A>);

impl<A: Allocator> fmt::Debug for ProtoString<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ProtoString").field(&&*self.0).finish()
    }
}

impl<A: Allocator + Clone> DefaultIn for ProtoString<A> {
    type Alloc = A;

    #[inline]
    fn default_in(alloc: A) -> Self {
        Self(UnmanagedString::new(alloc))
    }
}

impl<A: Allocator + Clone> DeallocateIn for ProtoString<A> {
    type Alloc = A;

    #[inline]
    unsafe fn deallocate_in(self, alloc: A) {
        unsafe { self.0.deallocate(alloc) };
    }
}

impl<A: Allocator> ProtoEmpty for ProtoString<A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<A: Allocator + Clone> LenProtoType for ProtoString<A> {
    type Alloc = A;
    type Storage = UnmanagedString<A>;
    type Ref<'a>
        = &'a str
    where
        Self: 'a;
    type Mut<'a>
        = StringGuard<'a, A>
    where
        Self: 'a,
        A: 'a;

    fn new_in(alloc: A) -> Self::Storage {
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

    fn with_alloc<'a>(value: &'a mut Self::Storage, alloc: A) -> Self::Mut<'a> {
        // SAFETY: the caller always passes the same allocator that owns this
        // string's buffer.
        unsafe { value.with_alloc(alloc) }
    }

    fn decode<B: Buf>(buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError> {
        decode::decode_string_in(buf, alloc)
    }

    fn store_from_slice(v: &[u8], alloc: A) -> Result<Self::Storage, DecodeError> {
        let s = str::from_utf8(v).map_err(|_| DecodeError::InvalidUtf8)?;
        Ok(decode::str_to_unmanaged_in(s, alloc))
    }

    unsafe fn deallocate(value: Self::Storage, alloc: A) {
        // SAFETY: forwarded to the caller's obligation.
        unsafe { value.deallocate(alloc) };
    }
}

/// Protobuf `bytes` — thin wrapper over [`UnmanagedVec<u8>`].
#[repr(transparent)]
pub struct ProtoBytes<A: Allocator>(pub UnmanagedVec<u8, A>);

impl<A: Allocator> fmt::Debug for ProtoBytes<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ProtoBytes").field(&&*self.0).finish()
    }
}

impl<A: Allocator + Clone> DefaultIn for ProtoBytes<A> {
    type Alloc = A;

    #[inline]
    fn default_in(alloc: A) -> Self {
        Self(UnmanagedVec::new(alloc))
    }
}

impl<A: Allocator + Clone> DeallocateIn for ProtoBytes<A> {
    type Alloc = A;

    #[inline]
    unsafe fn deallocate_in(self, alloc: A) {
        unsafe { self.0.deallocate(alloc) };
    }
}

impl<A: Allocator> ProtoEmpty for ProtoBytes<A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<A: Allocator + Clone> LenProtoType for ProtoBytes<A> {
    type Alloc = A;
    type Storage = UnmanagedVec<u8, A>;
    type Ref<'a>
        = &'a [u8]
    where
        Self: 'a;
    type Mut<'a>
        = VecGuard<'a, u8, A>
    where
        Self: 'a,
        A: 'a;

    fn new_in(alloc: A) -> Self::Storage {
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

    fn with_alloc<'a>(value: &'a mut Self::Storage, alloc: A) -> Self::Mut<'a> {
        // SAFETY: the caller always passes the same allocator that owns this
        // vector's buffer.
        unsafe { value.with_alloc(alloc) }
    }

    fn decode<B: Buf>(buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError> {
        decode::decode_bytes_in(buf, alloc)
    }

    fn store_from_slice(v: &[u8], alloc: A) -> Result<Self::Storage, DecodeError> {
        Ok(decode::bytes_to_unmanaged_in(v, alloc))
    }

    unsafe fn deallocate(value: Self::Storage, alloc: A) {
        // SAFETY: forwarded to the caller's obligation.
        unsafe { value.deallocate(alloc) };
    }
}

/// Always [`WireType::Len`] for singular field merge/encode checks.
pub(crate) const WIRE_TYPE: WireType = WireType::Len;
