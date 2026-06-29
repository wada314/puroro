//! Semantic protobuf types that share wire type [`WireType::Len`].
//!
//! Payload decode uses [`crate::decode`] helpers; LEN record framing uses
//! [`crate::encode::encode_len_field`].

use ::bytes::Buf;
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box as ABox;
use ::allocator_api2::vec::Vec as AVec;

use crate::decode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

/// Wire semantics for protobuf types encoded as length-delimited records.
pub trait LenProtoType {
    /// Owned storage in a generated message field (`Box<str, A>`, `Vec<u8, A>`, …).
    type Storage<A: Allocator>;

    /// Borrowed view returned by getters (`&str`, `&[u8]`, …).
    type Ref<'a, A: Allocator>
    where
        Self: 'a;

    /// Empty value used for IMPLICIT omit-on-encode and EXPLICIT unset slots.
    fn new_empty<A: Allocator + Clone>(alloc: A) -> Self::Storage<A>;

    /// `true` when the field should be omitted on encode (IMPLICIT presence).
    fn is_empty<A: Allocator>(value: &Self::Storage<A>) -> bool;

    /// Borrows the stored value for accessors.
    fn borrow<'a, A: Allocator>(value: &'a Self::Storage<A>) -> Self::Ref<'a, A>;

    /// Payload bytes for [`encode_len_field`](crate::encode::encode_len_field).
    fn as_bytes<A: Allocator>(value: &Self::Storage<A>) -> &[u8];

    /// Decodes one LEN payload (length varint + body consumed by helper).
    fn decode<B: Buf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError>;

    /// Builds storage from an already-decoded payload slice (setters).
    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError>;
}

/// Protobuf `string` — UTF-8 LEN payload stored as `Box<str, A>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoString;

impl LenProtoType for ProtoString {
    type Storage<A: Allocator> = ABox<str, A>;
    type Ref<'a, A: Allocator> = &'a str;

    fn new_empty<A: Allocator + Clone>(alloc: A) -> Self::Storage<A> {
        decode::str_to_box_in("", alloc)
    }

    fn is_empty<A: Allocator>(value: &Self::Storage<A>) -> bool {
        value.is_empty()
    }

    fn borrow<'a, A: Allocator>(value: &'a Self::Storage<A>) -> Self::Ref<'a, A> {
        value
    }

    fn as_bytes<A: Allocator>(value: &Self::Storage<A>) -> &[u8] {
        value.as_bytes()
    }

    fn decode<B: Buf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError> {
        decode::decode_string_in(buf, alloc)
    }

    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError> {
        let s = ::core::str::from_utf8(v).map_err(|_| DecodeError::InvalidUtf8)?;
        Ok(decode::str_to_box_in(s, alloc))
    }
}

/// Protobuf `bytes` — opaque LEN payload stored as `Vec<u8, A>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBytes;

impl LenProtoType for ProtoBytes {
    type Storage<A: Allocator> = AVec<u8, A>;
    type Ref<'a, A: Allocator> = &'a [u8];

    fn new_empty<A: Allocator + Clone>(alloc: A) -> Self::Storage<A> {
        AVec::new_in(alloc)
    }

    fn is_empty<A: Allocator>(value: &Self::Storage<A>) -> bool {
        value.is_empty()
    }

    fn borrow<'a, A: Allocator>(value: &'a Self::Storage<A>) -> Self::Ref<'a, A> {
        value
    }

    fn as_bytes<A: Allocator>(value: &Self::Storage<A>) -> &[u8] {
        value
    }

    fn decode<B: Buf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError> {
        decode::decode_bytes_in(buf, alloc)
    }

    fn store_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<Self::Storage<A>, DecodeError> {
        let mut vec = AVec::with_capacity_in(v.len(), alloc);
        vec.extend_from_slice(v);
        Ok(vec)
    }
}

/// Always [`WireType::Len`] for singular field merge/encode checks.
pub const WIRE_TYPE: WireType = WireType::Len;
