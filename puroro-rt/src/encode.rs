//! Wire-format encoding helpers used by generated field types.
//!
//! Varint and tag encoding delegate to [`protobuf_core`] (`Varint`, `Tag`).
//! `BufMut` adapters live here because generated code targets `bytes` buffers.

use ::bytes::BufMut;
use ::protobuf_core::{FieldNumber, Tag, Varint};
use ::puroro::WireType;

/// Encoded tag as a raw varint numeric value (used when re-serialising unknown fields).
#[inline]
pub(crate) fn tag_to_u64_for_unknown(field_number: u32, wire_type: WireType) -> u64 {
    tag_to_u64(field_number, wire_type)
}

/// Returns the number of bytes needed to encode `v` as a base-128 varint.
#[inline]
pub(crate) fn encoded_len_varint(v: u64) -> usize {
    Varint::from_uint64(v).varint_size()
}

/// Writes `v` as a base-128 varint to `buf`.
#[inline]
pub(crate) fn encode_varint<B: BufMut>(v: u64, buf: &mut B) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.put_slice(&bytes[..count]);
}

/// Returns the number of bytes needed to encode a tag for the given field number.
#[inline]
pub(crate) fn encoded_len_tag(field_number: u32, wire_type: WireType) -> usize {
    encoded_len_varint(tag_to_u64(field_number, wire_type))
}

/// Writes the tag (field_number + wire_type pair) to `buf`.
#[inline]
pub(crate) fn encode_tag<B: BufMut>(field_number: u32, wire_type: WireType, buf: &mut B) {
    encode_varint(tag_to_u64(field_number, wire_type), buf);
}

#[inline]
fn tag_to_u64(field_number: u32, wire_type: WireType) -> u64 {
    let field_number =
        FieldNumber::try_new(field_number).expect("generated code uses valid field numbers");
    Tag {
        field_number,
        wire_type,
    }
    .to_encoded()
    .to_uint64()
}

/// Returns the encoded byte length of a varint field (tag + value).
#[inline]
pub(crate) fn encoded_len_varint_field(field_number: u32, v: u64) -> usize {
    encoded_len_tag(field_number, WireType::Varint) + encoded_len_varint(v)
}

/// Writes a varint field (tag + value) to `buf`.
#[inline]
pub fn encode_varint_field<B: BufMut>(field_number: u32, v: u64, buf: &mut B) {
    encode_tag(field_number, WireType::Varint, buf);
    encode_varint(v, buf);
}

/// Returns the encoded byte length of a LEN field (tag + length varint + payload).
#[inline]
pub(crate) fn encoded_len_len_field(field_number: u32, payload_len: usize) -> usize {
    encoded_len_tag(field_number, WireType::Len)
        + encoded_len_varint(payload_len as u64)
        + payload_len
}

/// Writes a LEN field (tag + length + payload bytes) to `buf`.
#[inline]
pub(crate) fn encode_len_field<B: BufMut>(field_number: u32, payload: &[u8], buf: &mut B) {
    encode_tag(field_number, WireType::Len, buf);
    encode_varint(payload.len() as u64, buf);
    buf.put_slice(payload);
}

pub(crate) fn encoded_len_packed_varint_field<T, F>(field_number: u32, values: &[T], to_u64: F) -> usize
where
    F: Fn(&T) -> u64,
{
    if values.is_empty() {
        return 0;
    }
    let payload_len: usize = values.iter().map(|v| encoded_len_varint(to_u64(v))).sum();
    encoded_len_len_field(field_number, payload_len)
}

pub(crate) fn encode_packed_varint_field<B: BufMut, T, F>(
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

/// Writes a varint into an allocator-aware byte vector.
pub(crate) fn write_varint_to_vec<A: ::allocator_api2::alloc::Allocator>(
    v: u64,
    buf: &mut ::allocator_api2::vec::Vec<u8, A>,
) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.extend_from_slice(&bytes[..count]);
}
