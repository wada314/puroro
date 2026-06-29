//! Wire-format encoding helpers and the [`MessageEncode`] trait.
//!
//! Varint and tag encoding delegate to [`protobuf_core`] (`Varint`, `Tag`).
//! `BufMut` adapters live here because generated code targets `bytes` buffers.

use ::bytes::BufMut;
use ::protobuf_core::{FieldNumber, Tag, Varint, FIXED32_BYTES, FIXED64_BYTES};

use crate::wire_type::WireType;

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// Implemented by every generated message type.
pub trait MessageEncode {
    fn encoded_len(&self) -> usize;

    fn encode_raw<B: BufMut>(&self, buf: &mut B);

    fn encode_to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.encoded_len());
        self.encode_raw(&mut v);
        v
    }

    fn encode_to_bytes(&self) -> ::bytes::Bytes {
        ::bytes::Bytes::from(self.encode_to_vec())
    }
}

// ---------------------------------------------------------------------------
// Varint encoding (protobuf-core)
// ---------------------------------------------------------------------------

/// Returns the number of bytes needed to encode `v` as a base-128 varint.
#[inline]
pub fn encoded_len_varint(v: u64) -> usize {
    Varint::from_uint64(v).varint_size()
}

/// Writes `v` as a base-128 varint to `buf`.
#[inline]
pub fn encode_varint<B: BufMut>(v: u64, buf: &mut B) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.put_slice(&bytes[..count]);
}

// ---------------------------------------------------------------------------
// Tag encoding (protobuf-core)
// ---------------------------------------------------------------------------

/// Encoded tag as a raw varint numeric value (used when re-serialising unknown fields).
#[inline]
pub(crate) fn tag_to_u64_for_unknown(field_number: u32, wire_type: WireType) -> u64 {
    tag_to_u64(field_number, wire_type)
}

/// Returns the number of bytes needed to encode a tag for the given field number.
#[inline]
pub fn encoded_len_tag(field_number: u32, wire_type: WireType) -> usize {
    encoded_len_varint(tag_to_u64(field_number, wire_type))
}

/// Writes the tag (field_number + wire_type pair) to `buf`.
#[inline]
pub fn encode_tag<B: BufMut>(field_number: u32, wire_type: WireType, buf: &mut B) {
    encode_varint(tag_to_u64(field_number, wire_type), buf);
}

#[inline]
fn tag_to_u64(field_number: u32, wire_type: WireType) -> u64 {
    let field_number = FieldNumber::try_new(field_number)
        .expect("generated code uses valid field numbers");
    Tag {
        field_number,
        wire_type,
    }
    .to_encoded()
    .to_uint64()
}

// ---------------------------------------------------------------------------
// Per-wire-type field helpers
// ---------------------------------------------------------------------------

/// Returns the encoded byte length of a varint field (tag + value).
#[inline]
pub fn encoded_len_varint_field(field_number: u32, v: u64) -> usize {
    encoded_len_tag(field_number, WireType::Varint) + encoded_len_varint(v)
}

/// Writes a varint field (tag + value) to `buf`.
#[inline]
pub fn encode_varint_field<B: BufMut>(field_number: u32, v: u64, buf: &mut B) {
    encode_tag(field_number, WireType::Varint, buf);
    encode_varint(v, buf);
}

// ---- Int32 wire type (fixed32, sfixed32, float) ----

/// Returns the encoded byte length of an Int32 field (tag + 4 bytes).
#[inline]
pub fn encoded_len_i32_field(field_number: u32) -> usize {
    encoded_len_tag(field_number, WireType::Int32) + FIXED32_BYTES
}

/// Writes an Int32 field (tag + raw little-endian 32-bit value) to `buf`.
#[inline]
pub fn encode_i32_field<B: BufMut>(field_number: u32, v: u32, buf: &mut B) {
    encode_tag(field_number, WireType::Int32, buf);
    buf.put_u32_le(v);
}

// ---- Int64 wire type (fixed64, sfixed64, double) ----

/// Returns the encoded byte length of an Int64 field (tag + 8 bytes).
#[inline]
pub fn encoded_len_i64_field(field_number: u32) -> usize {
    encoded_len_tag(field_number, WireType::Int64) + FIXED64_BYTES
}

/// Writes an Int64 field (tag + raw little-endian 64-bit value) to `buf`.
#[inline]
pub fn encode_i64_field<B: BufMut>(field_number: u32, v: u64, buf: &mut B) {
    encode_tag(field_number, WireType::Int64, buf);
    buf.put_u64_le(v);
}

// ---- LEN fields (string, bytes, embedded messages, packed repeated) ----

/// Returns the encoded byte length of a LEN field (tag + length varint + payload).
#[inline]
pub fn encoded_len_len_field(field_number: u32, payload_len: usize) -> usize {
    encoded_len_tag(field_number, WireType::Len)
        + encoded_len_varint(payload_len as u64)
        + payload_len
}

/// Writes a LEN field (tag + length + payload bytes) to `buf`.
#[inline]
pub fn encode_len_field<B: BufMut>(field_number: u32, payload: &[u8], buf: &mut B) {
    encode_tag(field_number, WireType::Len, buf);
    encode_varint(payload.len() as u64, buf);
    buf.put_slice(payload);
}

// ---- Packed repeated fields ----

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

#[inline]
pub fn encoded_len_packed_i32_field(field_number: u32, values: &[u32]) -> usize {
    if values.is_empty() {
        0
    } else {
        encoded_len_len_field(field_number, values.len() * FIXED32_BYTES)
    }
}

pub fn encode_packed_i32_field<B: BufMut>(field_number: u32, values: &[u32], buf: &mut B) {
    if values.is_empty() {
        return;
    }
    encode_tag(field_number, WireType::Len, buf);
    encode_varint((values.len() * FIXED32_BYTES) as u64, buf);
    for &v in values {
        buf.put_u32_le(v);
    }
}

#[inline]
pub fn encoded_len_packed_i64_field(field_number: u32, values: &[u64]) -> usize {
    if values.is_empty() {
        0
    } else {
        encoded_len_len_field(field_number, values.len() * FIXED64_BYTES)
    }
}

pub fn encode_packed_i64_field<B: BufMut>(field_number: u32, values: &[u64], buf: &mut B) {
    if values.is_empty() {
        return;
    }
    encode_tag(field_number, WireType::Len, buf);
    encode_varint((values.len() * FIXED64_BYTES) as u64, buf);
    for &v in values {
        buf.put_u64_le(v);
    }
}

// ---------------------------------------------------------------------------
// Varint writing to allocator-aware Vec (unknown fields)
// ---------------------------------------------------------------------------

/// Writes a varint into an allocator-aware byte vector.
pub fn write_varint_to_vec<A: ::allocator_api2::alloc::Allocator>(
    v: u64,
    buf: &mut ::allocator_api2::vec::Vec<u8, A>,
) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.extend_from_slice(&bytes[..count]);
}
