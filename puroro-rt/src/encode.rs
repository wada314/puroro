//! Wire-format encoding helpers used by generated field types.
//!
//! Varint and tag encoding delegate to [`protobuf_core`] (`Varint`, `Tag`).
//! `BufMut` adapters live here because generated code targets `bytes` buffers.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
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

/// Returns the encoded byte length of a fixed32 / float field (tag + 4 LE bytes).
#[inline]
pub(crate) fn encoded_len_fixed32_field(field_number: u32) -> usize {
    encoded_len_tag(field_number, WireType::Int32) + ::protobuf_core::FIXED32_BYTES
}

/// Writes a fixed32 / float field (tag + 4 LE bytes) to `buf`.
#[inline]
pub(crate) fn encode_fixed32_field<B: BufMut>(field_number: u32, bytes: [u8; 4], buf: &mut B) {
    encode_tag(field_number, WireType::Int32, buf);
    buf.put_slice(&bytes);
}

/// Returns the encoded byte length of a fixed64 / double field (tag + 8 LE bytes).
#[inline]
pub(crate) fn encoded_len_fixed64_field(field_number: u32) -> usize {
    encoded_len_tag(field_number, WireType::Int64) + ::protobuf_core::FIXED64_BYTES
}

/// Writes a fixed64 / double field (tag + 8 LE bytes) to `buf`.
#[inline]
pub(crate) fn encode_fixed64_field<B: BufMut>(field_number: u32, bytes: [u8; 8], buf: &mut B) {
    encode_tag(field_number, WireType::Int64, buf);
    buf.put_slice(&bytes);
}

/// Writes a varint into an allocator-aware byte vector.
pub(crate) fn write_varint_to_vec<A: Allocator>(v: u64, buf: &mut AllocVec<u8, A>) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.extend_from_slice(&bytes[..count]);
}
