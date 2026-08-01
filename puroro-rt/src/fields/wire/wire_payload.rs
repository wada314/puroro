//! Untagged body for a protobuf **wire type** (e.g. `Varint`, `Len`).
//!
//! This is not a protobuf **type** such as `int32` / [`ProtoInt32`](super::varint::ProtoInt32)
//! or `string` / [`ProtoString`](super::len::ProtoString). Proto types map onto these
//! shapes via [`NumericalType`](super::numerical::NumericalType) / [`EncodeType`](super::encode_type::EncodeType).
//!
//! A [`WirePayload`] is the complete tag-free body for its [`WIRE_TYPE`](WirePayload::WIRE_TYPE):
//! for `Len`, that includes the length varint plus content bytes.
//!
//! # Visibility
//!
//! Len helpers are `pub(crate)`. Copy numerical payloads stay `pub` (but are not
//! re-exported from the crate root) because [`NumericalType::WireBody`](super::numerical::NumericalType::WireBody)
//! names them in a public trait — `pub(crate)` there is E0446.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::{Buf, BufMut};
use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES, Varint};
use ::puroro::{DecodeError, Message, WireType};
use ::unmanaged::UnmanagedVec;

use crate::decode;
use crate::encode;

/// Untagged body for a wire type (`Varint`, `Int32`, `Int64`, or `Len`).
#[doc(hidden)]
pub trait WirePayload {
    /// Wire type of this body (e.g. `WireType::Varint`, `WireType::Len`).
    const WIRE_TYPE: WireType;

    /// Encoded byte length of this body (no tag).
    fn encoded_len(&self) -> usize;

    /// Writes this body (no tag).
    fn encode(&self, buf: &mut impl BufMut);
}

/// Copy numerical wire bodies that decode without an allocator.
#[doc(hidden)]
pub trait CopyWirePayload: WirePayload + Copy + Sized {
    /// Reads one body after the tag has been consumed.
    fn decode(wire_type: WireType, buf: &mut impl Buf) -> Result<Self, DecodeError>;
}

/// Varint wire body (`WireType::Varint`) — [`Varint`] before proto-type mapping
/// (e.g. `int32` / `sint32` / `bool`).
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VarintPayload(pub Varint);

impl WirePayload for VarintPayload {
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn encoded_len(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        let (bytes, count) = self.0.encode();
        buf.put_slice(&bytes[..count]);
    }
}

impl CopyWirePayload for VarintPayload {
    #[inline]
    fn decode(wire_type: WireType, buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(Varint::from_uint64(decode::decode_varint(buf)?)))
    }
}

/// Fixed32-family wire body (`WireType::Int32`) — 4 LE bytes (`fixed32` / `float` / …).
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fixed32Payload(pub [u8; FIXED32_BYTES]);

impl WirePayload for Fixed32Payload {
    const WIRE_TYPE: WireType = WireType::Int32;

    #[inline]
    fn encoded_len(&self) -> usize {
        FIXED32_BYTES
    }

    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0);
    }
}

impl CopyWirePayload for Fixed32Payload {
    #[inline]
    fn decode(wire_type: WireType, buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if wire_type != WireType::Int32 {
            return Err(DecodeError::InvalidTag);
        }
        if buf.remaining() < FIXED32_BYTES {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut bytes = [0u8; FIXED32_BYTES];
        buf.copy_to_slice(&mut bytes);
        Ok(Self(bytes))
    }
}

/// Fixed64-family wire body (`WireType::Int64`) — 8 LE bytes (`fixed64` / `double` / …).
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fixed64Payload(pub [u8; FIXED64_BYTES]);

impl WirePayload for Fixed64Payload {
    const WIRE_TYPE: WireType = WireType::Int64;

    #[inline]
    fn encoded_len(&self) -> usize {
        FIXED64_BYTES
    }

    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0);
    }
}

impl CopyWirePayload for Fixed64Payload {
    #[inline]
    fn decode(wire_type: WireType, buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if wire_type != WireType::Int64 {
            return Err(DecodeError::InvalidTag);
        }
        if buf.remaining() < FIXED64_BYTES {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut bytes = [0u8; FIXED64_BYTES];
        buf.copy_to_slice(&mut bytes);
        Ok(Self(bytes))
    }
}

/// Borrowed Len wire body for encode: length varint + `content`
/// (e.g. UTF-8 bytes of a `string`, or raw `bytes`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LenPayloadRef<'a> {
    pub content: &'a [u8],
}

impl WirePayload for LenPayloadRef<'_> {
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn encoded_len(&self) -> usize {
        encode::encoded_len_varint(self.content.len() as u64) + self.content.len()
    }

    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        encode::encode_varint(self.content.len() as u64, buf);
        buf.put_slice(self.content);
    }
}

/// Len wire body for encode from a nested message: length varint + `Message::encode_raw`
/// (no temporary buffer).
#[derive(Clone, Copy, Debug)]
pub(crate) struct MessageLenRef<'a, M: Message> {
    pub message: &'a M,
}

impl<M: Message> WirePayload for MessageLenRef<'_, M> {
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn encoded_len(&self) -> usize {
        let n = self.message.encoded_len();
        encode::encoded_len_varint(n as u64) + n
    }

    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        let n = self.message.encoded_len();
        encode::encode_varint(n as u64, buf);
        self.message.encode_raw(buf);
    }
}

/// Owned Len content after decode (length prefix already consumed).
pub(crate) struct LenPayload<A: Allocator> {
    content: UnmanagedVec<u8, A>,
}

impl<A: Allocator> LenPayload<A> {
    /// Reads a `Len` wire body into owned bytes.
    pub(crate) fn decode_in(
        wire_type: WireType,
        buf: &mut impl Buf,
        alloc: A,
    ) -> Result<Self, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut vec = AllocVec::<u8, A>::with_capacity_in(len, alloc);
        let mut remaining = len;
        while remaining > 0 {
            let chunk = buf.chunk();
            let to_copy = chunk.len().min(remaining);
            vec.extend_from_slice(&chunk[..to_copy]);
            buf.advance(to_copy);
            remaining -= to_copy;
        }
        Ok(Self {
            content: UnmanagedVec::from_vec(vec),
        })
    }

    #[inline]
    pub(crate) fn into_vec(self) -> UnmanagedVec<u8, A> {
        self.content
    }
}
