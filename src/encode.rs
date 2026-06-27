//! Wire-format encoding helpers and the [`MessageEncode`] trait.
//!
//! # Design note
//!
//! `MessageEncode::encode_raw` is generic over `B: BufMut` rather than taking
//! `&mut dyn BufMut`. This avoids a virtual dispatch on the hot path and lets
//! the compiler monomorphise and inline field writes. As a consequence
//! `MessageEncode` is **not** object-safe; use `Box<dyn ErasedMessageEncode>`
//! (a future wrapper type) if you need type erasure.

use ::bytes::BufMut;
use crate::wire_type::WireType;

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// Implemented by every generated message type.
///
/// The two required methods — `encoded_len` and `encode_raw` — together cover
/// all encoding needs. Convenience methods (`encode_to_vec`, `encode_to_bytes`)
/// are provided as blanket implementations.
pub trait MessageEncode {
    /// Returns the exact number of bytes this message would occupy on the wire.
    ///
    /// Implementations must be consistent with [`encode_raw`](Self::encode_raw):
    /// calling `encode_raw` must write exactly this many bytes.
    fn encoded_len(&self) -> usize;

    /// Writes the message to `buf` in protobuf wire format **without** a
    /// length prefix.
    ///
    /// The caller is responsible for ensuring `buf` has at least
    /// `self.encoded_len()` bytes of remaining capacity.
    fn encode_raw<B: BufMut>(&self, buf: &mut B);

    /// Encodes the message and returns it as a freshly allocated `Vec<u8>`.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.encoded_len());
        self.encode_raw(&mut v);
        v
    }

    /// Encodes the message and returns it as a [`bytes::Bytes`] value.
    fn encode_to_bytes(&self) -> ::bytes::Bytes {
        ::bytes::Bytes::from(self.encode_to_vec())
    }
}

// ---------------------------------------------------------------------------
// Varint encoding
// ---------------------------------------------------------------------------

/// Returns the number of bytes needed to encode `v` as a base-128 varint.
#[inline]
pub fn encoded_len_varint(v: u64) -> usize {
    // Each 7-bit group requires one byte; the minimum is 1 byte.
    let bits = 64 - v.leading_zeros() as usize;
    (bits.max(1) + 6) / 7
}

/// Writes `v` as a base-128 varint to `buf`.
#[inline]
pub fn encode_varint<B: BufMut>(mut v: u64, buf: &mut B) {
    loop {
        let byte = (v & 0x7F) as u8;
        v >>= 7;
        if v == 0 {
            buf.put_u8(byte);
            break;
        } else {
            buf.put_u8(byte | 0x80);
        }
    }
}

// ---------------------------------------------------------------------------
// Tag encoding
// ---------------------------------------------------------------------------

/// Returns the number of bytes needed to encode a tag for the given field number.
#[inline]
pub fn encoded_len_tag(field_number: u32) -> usize {
    encoded_len_varint(tag_value(field_number, WireType::Varint))
}

/// Writes the tag (field_number + wire_type pair) to `buf`.
#[inline]
pub fn encode_tag<B: BufMut>(field_number: u32, wire_type: WireType, buf: &mut B) {
    encode_varint(tag_value(field_number, wire_type), buf);
}

#[inline]
fn tag_value(field_number: u32, wire_type: WireType) -> u64 {
    ((field_number as u64) << 3) | (wire_type as u64)
}

// ---------------------------------------------------------------------------
// Per-wire-type field helpers
// ---------------------------------------------------------------------------

// ---- VARINT fields (int32, int64, uint32, uint64, sint32, sint64, bool, enum) ----

/// Returns the encoded byte length of a varint field (tag + value).
#[inline]
pub fn encoded_len_varint_field(field_number: u32, v: u64) -> usize {
    encoded_len_tag(field_number) + encoded_len_varint(v)
}

/// Writes a varint field (tag + value) to `buf`.
#[inline]
pub fn encode_varint_field<B: BufMut>(field_number: u32, v: u64, buf: &mut B) {
    encode_tag(field_number, WireType::Varint, buf);
    encode_varint(v, buf);
}

// ZigZag helpers for sint32/sint64.

/// ZigZag-encodes a signed 32-bit integer.
#[inline]
pub fn zigzag32(v: i32) -> u64 {
    ((v << 1) ^ (v >> 31)) as u32 as u64
}

/// ZigZag-encodes a signed 64-bit integer.
#[inline]
pub fn zigzag64(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

// ---- I32 fields (fixed32, sfixed32, float) ----

/// Returns the encoded byte length of an I32 field (tag + 4 bytes).
#[inline]
pub fn encoded_len_i32_field(field_number: u32) -> usize {
    encoded_len_tag(field_number) + 4
}

/// Writes an I32 field (tag + raw little-endian 32-bit value) to `buf`.
#[inline]
pub fn encode_i32_field<B: BufMut>(field_number: u32, v: u32, buf: &mut B) {
    encode_tag(field_number, WireType::I32, buf);
    buf.put_u32_le(v);
}

// ---- I64 fields (fixed64, sfixed64, double) ----

/// Returns the encoded byte length of an I64 field (tag + 8 bytes).
#[inline]
pub fn encoded_len_i64_field(field_number: u32) -> usize {
    encoded_len_tag(field_number) + 8
}

/// Writes an I64 field (tag + raw little-endian 64-bit value) to `buf`.
#[inline]
pub fn encode_i64_field<B: BufMut>(field_number: u32, v: u64, buf: &mut B) {
    encode_tag(field_number, WireType::I64, buf);
    buf.put_u64_le(v);
}

// ---- LEN fields (string, bytes, embedded messages, packed repeated) ----

/// Returns the encoded byte length of a LEN field (tag + length varint + payload).
#[inline]
pub fn encoded_len_len_field(field_number: u32, payload_len: usize) -> usize {
    encoded_len_tag(field_number) + encoded_len_varint(payload_len as u64) + payload_len
}

/// Writes a LEN field (tag + length + payload bytes) to `buf`.
#[inline]
pub fn encode_len_field<B: BufMut>(field_number: u32, payload: &[u8], buf: &mut B) {
    encode_tag(field_number, WireType::Len, buf);
    encode_varint(payload.len() as u64, buf);
    buf.put_slice(payload);
}

// ---- Packed repeated fields ----
//
// Packed fields encode all values inside a single LEN record.
// Only repeated fields whose element type uses VARINT, I32, or I64 wire type
// can be packed. Strings and embedded messages cannot.

/// Returns the encoded byte length of a packed repeated varint field.
///
/// This covers `repeated int32`, `repeated uint64`, `repeated bool`, etc.
pub fn encoded_len_packed_varint_field<T, F>(
    field_number: u32,
    values: &[T],
    to_u64: F,
) -> usize
where
    F: Fn(&T) -> u64,
{
    if values.is_empty() {
        return 0;
    }
    let payload_len: usize = values.iter().map(|v| encoded_len_varint(to_u64(v))).sum();
    encoded_len_len_field(field_number, payload_len)
}

/// Writes a packed repeated varint field to `buf`.
pub fn encode_packed_varint_field<B: BufMut, T, F>(
    field_number: u32,
    values: &[T],
    to_u64: F,
    buf: &mut B,
) where
    F: Fn(&T) -> u64,
{
    if values.is_empty() {
        return;
    }
    let payload_len: usize = values.iter().map(|v| encoded_len_varint(to_u64(v))).sum();
    encode_tag(field_number, WireType::Len, buf);
    encode_varint(payload_len as u64, buf);
    for v in values {
        encode_varint(to_u64(v), buf);
    }
}

/// Returns the encoded byte length of a packed repeated I32 field.
#[inline]
pub fn encoded_len_packed_i32_field(field_number: u32, values: &[u32]) -> usize {
    if values.is_empty() {
        0
    } else {
        encoded_len_len_field(field_number, values.len() * 4)
    }
}

/// Writes a packed repeated I32 field to `buf`.
pub fn encode_packed_i32_field<B: BufMut>(field_number: u32, values: &[u32], buf: &mut B) {
    if values.is_empty() {
        return;
    }
    encode_tag(field_number, WireType::Len, buf);
    encode_varint((values.len() * 4) as u64, buf);
    for &v in values {
        buf.put_u32_le(v);
    }
}

/// Returns the encoded byte length of a packed repeated I64 field.
#[inline]
pub fn encoded_len_packed_i64_field(field_number: u32, values: &[u64]) -> usize {
    if values.is_empty() {
        0
    } else {
        encoded_len_len_field(field_number, values.len() * 8)
    }
}

/// Writes a packed repeated I64 field to `buf`.
pub fn encode_packed_i64_field<B: BufMut>(field_number: u32, values: &[u64], buf: &mut B) {
    if values.is_empty() {
        return;
    }
    encode_tag(field_number, WireType::Len, buf);
    encode_varint((values.len() * 8) as u64, buf);
    for &v in values {
        buf.put_u64_le(v);
    }
}

// ---------------------------------------------------------------------------
// Varint writing to allocator-aware Vec (used internally for unknown fields)
// ---------------------------------------------------------------------------

/// Writes a varint directly into an allocator-aware byte vector.
///
/// This is a lower-level variant of [`encode_varint`] used when appending to
/// the unknown-fields buffer, which is `Vec<u8, A>` and does not implement
/// `bytes::BufMut`.
pub fn write_varint_to_vec<A: ::allocator_api2::alloc::Allocator>(
    mut v: u64,
    buf: &mut ::allocator_api2::vec::Vec<u8, A>,
) {
    loop {
        let byte = (v & 0x7F) as u8;
        v >>= 7;
        if v == 0 {
            buf.push(byte);
            break;
        } else {
            buf.push(byte | 0x80);
        }
    }
}
