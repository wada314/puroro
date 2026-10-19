//! Wire-format encoding helpers used by generated field types.
//!
//! Varint encoding delegates to [`protobuf_core::Varint`]. Tags are packed as
//! `(field_number << 3) | wire_type` into a [`Varint`]. Length prefixes stay
//! as plain integers converted at the call site via [`Varint::from_uint64`].
//! `BufMut` adapters live here because generated code targets `bytes` buffers.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::BufMut;
use ::protobuf_core::{FieldNumber, Varint};
use ::puroro::WireType;

/// Validated [`FieldNumber`] from a catalog / const-generic field number.
///
/// Catalog `const FIELD: u32` values are in range by construction; invalid
/// `FIELD` fails at compile time when this is evaluated in a const context.
#[inline]
pub const fn field_number_const<const FIELD: u32>() -> FieldNumber {
    match FieldNumber::try_new(FIELD) {
        Ok(n) => n,
        Err(_) => panic!("catalog field number out of protobuf range"),
    }
}

/// Returns the number of bytes needed to encode `v` as a base-128 varint.
#[inline]
pub(crate) fn encoded_len_varint(v: Varint) -> usize {
    v.varint_size()
}

/// Writes `v` as a base-128 varint to `buf`.
#[inline]
pub(crate) fn encode_varint<B: BufMut>(v: Varint, buf: &mut B) {
    let (bytes, count) = v.encode();
    buf.put_slice(&bytes[..count]);
}

/// Returns the number of bytes needed to encode a tag for the given field number.
#[inline]
pub(crate) fn encoded_len_tag(field_number: FieldNumber, wire_type: WireType) -> usize {
    encoded_len_varint(tag_varint(field_number, wire_type))
}

/// Writes the tag (field_number + wire_type pair) to `buf`.
#[inline]
pub(crate) fn encode_tag<B: BufMut>(field_number: FieldNumber, wire_type: WireType, buf: &mut B) {
    encode_varint(tag_varint(field_number, wire_type), buf);
}

/// Pack a tag into a [`Varint`] numeric payload.
#[inline]
fn tag_varint(field_number: FieldNumber, wire_type: WireType) -> Varint {
    Varint::from_uint64((u64::from(field_number.as_u32()) << 3) | u64::from(u8::from(wire_type)))
}

/// Writes a varint field (tag + value) to `buf`.
#[inline]
pub fn encode_varint_field<B: BufMut>(field_number: FieldNumber, v: Varint, buf: &mut B) {
    encode_tag(field_number, WireType::Varint, buf);
    encode_varint(v, buf);
}

/// Returns the encoded byte length of a LEN field (tag + length varint + payload).
#[inline]
pub(crate) fn encoded_len_len_field(field_number: FieldNumber, payload_len: usize) -> usize {
    encoded_len_tag(field_number, WireType::Len)
        + encoded_len_varint(Varint::from_uint64(payload_len as u64))
        + payload_len
}

/// Writes a varint into an allocator-aware byte vector.
pub(crate) fn write_varint_to_vec<A: Allocator>(v: Varint, buf: &mut AllocVec<u8, A>) {
    let (bytes, count) = v.encode();
    buf.extend_from_slice(&bytes[..count]);
}

/// Writes a tag into an allocator-aware byte vector (unknown-field path).
#[inline]
pub(crate) fn write_tag_to_vec<A: Allocator>(
    field_number: FieldNumber,
    wire_type: WireType,
    buf: &mut AllocVec<u8, A>,
) {
    write_varint_to_vec(tag_varint(field_number, wire_type), buf);
}
