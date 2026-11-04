//! Adapt [`protobuf_core`] field iterators to the catalog `merge(wire_type, buf)` API.
//!
//! Eager merge historically reads payloads from a [`DecodeBuf`] after the tag.
//! [`AsRefExtProtobuf::read_protobuf_fields`] yields already-split
//! [`FieldValue`]s, so this module rebuilds a tiny wire body (stack for scalars;
//! length-prefix + borrow for Len) that existing merges can consume.

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::protobuf_core::{Field, FieldValue, MAX_VARINT_BYTES, Varint};
use ::puroro::{DecodeError, WireType};

use crate::encode;
use crate::unknown_fields::UnknownStore;

/// Wire body rebuilt from a parsed [`FieldValue`], as a [`Buf`].
///
/// Catalog `merge` still expects a length varint before a Len payload, so Len
/// values carry a stack-encoded prefix ahead of the borrowed payload.
pub enum ParsedWireBody<'a> {
    /// Scalar body (varint / fixed32 / fixed64) in `data[..len]`, cursor `pos`.
    Simple {
        data: [u8; MAX_VARINT_BYTES],
        len: usize,
        pos: usize,
    },
    /// Length-delimited: `prefix[..prefix_len]` then `payload`.
    Len {
        prefix: [u8; MAX_VARINT_BYTES],
        prefix_len: usize,
        prefix_pos: usize,
        payload: &'a [u8],
        payload_pos: usize,
    },
}

impl Buf for ParsedWireBody<'_> {
    #[inline]
    fn remaining(&self) -> usize {
        match self {
            Self::Simple { len, pos, .. } => len.saturating_sub(*pos),
            Self::Len {
                prefix_len,
                prefix_pos,
                payload,
                payload_pos,
                ..
            } => {
                prefix_len.saturating_sub(*prefix_pos) + payload.len().saturating_sub(*payload_pos)
            }
        }
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        match self {
            Self::Simple { data, len, pos } => &data[*pos..*len],
            Self::Len {
                prefix,
                prefix_len,
                prefix_pos,
                payload,
                payload_pos,
                ..
            } => {
                if *prefix_pos < *prefix_len {
                    &prefix[*prefix_pos..*prefix_len]
                } else {
                    &payload[*payload_pos..]
                }
            }
        }
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining(), "ParsedWireBody::advance past end");
        match self {
            Self::Simple { pos, .. } => *pos += cnt,
            Self::Len {
                prefix_len,
                prefix_pos,
                payload_pos,
                ..
            } => {
                let in_prefix = prefix_len.saturating_sub(*prefix_pos);
                if cnt <= in_prefix {
                    *prefix_pos += cnt;
                } else {
                    *prefix_pos = *prefix_len;
                    *payload_pos += cnt - in_prefix;
                }
            }
        }
    }
}

/// Rebuild a wire body for catalog [`merge`](crate::fields::singular::SingularFieldMut::merge).
///
/// Groups are rejected: the catalog does not preserve them.
pub fn parsed_wire_body<'a>(
    value: &FieldValue<&'a [u8]>,
) -> Result<(WireType, ParsedWireBody<'a>), DecodeError> {
    match value {
        FieldValue::Varint(v) => {
            let (bytes, len) = v.encode();
            Ok((
                WireType::Varint,
                ParsedWireBody::Simple {
                    data: bytes,
                    len,
                    pos: 0,
                },
            ))
        }
        FieldValue::I32(bytes) => {
            let mut data = [0u8; MAX_VARINT_BYTES];
            data[..4].copy_from_slice(bytes);
            Ok((
                WireType::Int32,
                ParsedWireBody::Simple {
                    data,
                    len: 4,
                    pos: 0,
                },
            ))
        }
        FieldValue::I64(bytes) => {
            let mut data = [0u8; MAX_VARINT_BYTES];
            data[..8].copy_from_slice(bytes);
            Ok((
                WireType::Int64,
                ParsedWireBody::Simple {
                    data,
                    len: 8,
                    pos: 0,
                },
            ))
        }
        FieldValue::Len(payload) => {
            let (prefix, prefix_len) = Varint::from_uint64(payload.len() as u64).encode();
            Ok((
                WireType::Len,
                ParsedWireBody::Len {
                    prefix,
                    prefix_len,
                    prefix_pos: 0,
                    payload,
                    payload_pos: 0,
                },
            ))
        }
        FieldValue::SGroup | FieldValue::EGroup => Err(DecodeError::InvalidTag),
    }
}

/// Append a parsed field into an unknown-field store (preserve / discard / reject).
///
/// Mirrors [`super::skip_field_and_save_in`] without reading from a live buffer.
pub fn save_parsed_field_in<A: Allocator + Clone, U: UnknownStore<A>>(
    field: &Field<&[u8]>,
    unknown_fields: &mut U,
    alloc: A,
) -> Result<(), DecodeError> {
    unknown_fields.on_unrecognized_tag(field.field_number.as_u32())?;
    let Some(blob) = unknown_fields.append_blob_mut(alloc.clone()) else {
        return Ok(());
    };
    // SAFETY: `alloc` matches the vector's allocator.
    let mut g = unsafe { blob.with_alloc(alloc) };
    match &field.value {
        FieldValue::Varint(v) => {
            encode::write_tag_to_vec(field.field_number, WireType::Varint, &mut *g);
            encode::write_varint_to_vec(*v, &mut *g);
        }
        FieldValue::I32(bytes) => {
            encode::write_tag_to_vec(field.field_number, WireType::Int32, &mut *g);
            g.extend_from_slice(bytes);
        }
        FieldValue::I64(bytes) => {
            encode::write_tag_to_vec(field.field_number, WireType::Int64, &mut *g);
            g.extend_from_slice(bytes);
        }
        FieldValue::Len(payload) => {
            encode::write_tag_to_vec(field.field_number, WireType::Len, &mut *g);
            encode::write_varint_to_vec(Varint::from_uint64(payload.len() as u64), &mut *g);
            g.extend_from_slice(payload);
        }
        FieldValue::SGroup | FieldValue::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

/// When `buf`'s remaining bytes are one contiguous chunk, walk them with
/// [`AsRefExtProtobuf::read_protobuf_fields`].
///
/// Returns `Ok(true)` if this path ran (including `rem == 0`). Returns
/// `Ok(false)` when remaining bytes span multiple [`Buf::chunk`]s — the caller
/// should use the legacy tag loop. On success the cursor advances by `rem`.
///
/// The callback owns the full `&mut self` borrow for catalog merges and unknown
/// saves (so it is not split across `MessageCommon` / field slots).
pub fn try_for_each_contiguous_field<B, F>(
    buf: &mut B,
    mut on_field: F,
) -> Result<bool, DecodeError>
where
    B: Buf,
    F: FnMut(&Field<&[u8]>) -> Result<(), DecodeError>,
{
    let rem = buf.remaining();
    if rem == 0 {
        return Ok(true);
    }
    if buf.chunk().len() < rem {
        return Ok(false);
    }
    let slice = &buf.chunk()[..rem];
    for item in ::protobuf_core::AsRefExtProtobuf::read_protobuf_fields(slice) {
        on_field(&item?)?;
    }
    buf.advance(rem);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::protobuf_core::FieldValue;

    use crate::decode::decode_varint;

    #[test]
    fn parsed_varint_round_trips_through_buf() {
        let v = Varint::from_uint64(150);
        let (wt, mut body) = parsed_wire_body(&FieldValue::Varint(v)).unwrap();
        assert_eq!(wt, WireType::Varint);
        assert_eq!(decode_varint(&mut body).unwrap(), 150);
        assert_eq!(body.remaining(), 0);
    }

    #[test]
    fn parsed_len_prefix_then_payload() {
        let payload = b"hi";
        let (wt, mut body) = parsed_wire_body(&FieldValue::Len(payload)).unwrap();
        assert_eq!(wt, WireType::Len);
        assert_eq!(decode_varint(&mut body).unwrap(), 2);
        assert_eq!(body.chunk(), b"hi");
        body.advance(2);
        assert_eq!(body.remaining(), 0);
    }

    #[test]
    fn groups_are_invalid_tags() {
        assert!(matches!(
            parsed_wire_body(&FieldValue::SGroup),
            Err(DecodeError::InvalidTag)
        ));
    }
}
