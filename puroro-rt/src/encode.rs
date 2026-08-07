//! Wire-format encoding helpers used by generated field types.
//!
//! Varint encoding delegates to [`protobuf_core::Varint`]. Tags are packed as
//! `(field_number << 3) | wire_type` without re-validating field numbers that
//! the catalog / generated code already constrains. `BufMut` adapters live here
//! because generated code targets `bytes` buffers.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::BufMut;
use ::protobuf_core::Varint;
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

/// Pack a tag without `FieldNumber` / `Tag` validation.
///
/// Callers must pass a field number in `[1, 2^29 - 1]` (catalog constants and
/// previously decoded tags). Debug builds assert the range.
#[inline]
fn tag_to_u64(field_number: u32, wire_type: WireType) -> u64 {
    debug_assert!(
        (1..=0x1FFF_FFFF).contains(&field_number),
        "field number out of protobuf range: {field_number}"
    );
    (u64::from(field_number) << 3) | u64::from(u8::from(wire_type))
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

/// Writes a varint into an allocator-aware byte vector.
pub(crate) fn write_varint_to_vec<A: Allocator>(v: u64, buf: &mut AllocVec<u8, A>) {
    let (bytes, count) = Varint::from_uint64(v).encode();
    buf.extend_from_slice(&bytes[..count]);
}
