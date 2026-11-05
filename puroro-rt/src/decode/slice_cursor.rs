//! Slice cursor for contiguous eager decode.
//!
//! Modeled on protobuf-core's slice field reader: the cursor is the remaining
//! `&[u8]`, and a length-delimited payload is a subslice of that input.
//! Varints go through [`super::decode_tag`] / [`super::decode_varint`]
//! ([`super::ChunkScanLikely1`] for tags). This is not a second wire codec;
//! [`protobuf_core::AsRefExtProtobuf`] stays the reference reader.

use ::bytes::Buf;
use ::puroro::{DecodeBuf, DecodeError};
use ::std::vec::Vec;

/// Remaining bytes of one contiguous message body.
///
/// `limits` holds the suffix that sits outside each nested LEN frame, innermost
/// last. The hot path never pushes a frame: length-delimited messages split a
/// subslice and recurse instead. [`DecodeBuf::push_limit`] is the multi-chunk
/// fallback and the hand-written adapter.
pub struct SliceCursor<'a> {
    rest: &'a [u8],
    limits: Vec<&'a [u8]>,
}

impl<'a> SliceCursor<'a> {
    /// Cursor over the whole of `slice`.
    #[inline]
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            rest: slice,
            limits: Vec::new(),
        }
    }
}

impl Buf for SliceCursor<'_> {
    #[inline]
    fn remaining(&self) -> usize {
        self.rest.len()
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.rest
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        let rem = self.rest.len();
        assert!(
            cnt <= rem,
            "cannot advance past slice cursor: cnt={cnt}, remaining={rem}"
        );
        self.rest = &self.rest[cnt..];
    }
}

impl DecodeBuf for SliceCursor<'_> {
    #[inline]
    fn push_limit(&mut self, len: usize) -> Result<(), DecodeError> {
        if self.rest.len() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let (frame, suffix) = self.rest.split_at(len);
        self.limits.push(suffix);
        self.rest = frame;
        Ok(())
    }

    #[inline]
    fn pop_limit(&mut self) {
        let suffix = self
            .limits
            .pop()
            .expect("pop_limit called with empty limit stack");
        // Unread bytes in the frame are skipped. `suffix` is the parent body.
        self.rest = suffix;
    }
}

/// When `buf`'s remaining bytes are one contiguous chunk, return that slice
/// without advancing.
///
/// `Some(&[])` when nothing remains. `None` when the remainder spans chunks.
#[inline]
pub fn contiguous_remaining<B: Buf>(buf: &B) -> Option<&[u8]> {
    let rem = buf.remaining();
    if rem == 0 {
        return Some(&[]);
    }
    let chunk = buf.chunk();
    if chunk.len() < rem {
        return None;
    }
    Some(&chunk[..rem])
}

#[cfg(test)]
mod tests {
    use super::SliceCursor;
    use crate::decode::{decode_tag, decode_varint};
    use ::bytes::Buf;
    use ::protobuf_core::{AsRefExtProtobuf, FieldValue};
    use ::puroro::{DecodeBuf, DecodeError, WireType};

    fn assert_matches_core(bytes: &[u8]) {
        let mut cursor = SliceCursor::new(bytes);
        let mut fields = bytes.read_protobuf_fields();
        loop {
            if !cursor.has_remaining() {
                assert!(fields.next().is_none(), "{bytes:?}");
                break;
            }
            let (number, wire_type) = decode_tag(&mut cursor)
                .unwrap_or_else(|err| panic!("{err:?} at rest {:?} of {bytes:?}", cursor.chunk()));
            let field = fields.next().unwrap().unwrap();
            assert_eq!(field.field_number.as_u32(), number.as_u32(), "{bytes:?}");
            match (wire_type, field.value) {
                (WireType::Varint, FieldValue::Varint(value)) => {
                    assert_eq!(decode_varint(&mut cursor).unwrap(), value.to_uint64());
                }
                (WireType::Int32, FieldValue::I32(raw)) => {
                    let mut got = [0u8; 4];
                    got.copy_from_slice(&cursor.chunk()[..4]);
                    cursor.advance(4);
                    assert_eq!(got, raw);
                }
                (WireType::Int64, FieldValue::I64(raw)) => {
                    let mut got = [0u8; 8];
                    got.copy_from_slice(&cursor.chunk()[..8]);
                    cursor.advance(8);
                    assert_eq!(got, raw);
                }
                (WireType::Len, FieldValue::Len(payload)) => {
                    let len = decode_varint(&mut cursor).unwrap() as usize;
                    assert_eq!(&cursor.chunk()[..len], payload);
                    cursor.advance(len);
                }
                (WireType::SGroup, FieldValue::SGroup) | (WireType::EGroup, FieldValue::EGroup) => {
                }
                (wire_type, value) => panic!("mismatch {wire_type:?} vs {value:?} in {bytes:?}"),
            }
        }
    }

    #[test]
    fn cursor_matches_protobuf_core_on_mixed_fields() {
        let cases: &[&[u8]] = &[
            &[],
            &[0x08, 0x01],
            &[0x08, 0x96, 0x01],
            // Field 16, wire type varint, value 1. Tag is two bytes.
            &[0x80, 0x01, 0x01],
            // Field 2048, wire type varint, value 1. Tag is three bytes.
            &[0x80, 0x80, 0x01, 0x01],
            // Overlong two-byte encoding of varint 1 after a one-byte tag.
            &[0x08, 0x81, 0x00],
            &[0x0d, 0x00, 0x00, 0x80, 0x3f],
            &[0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f],
            &[0x12, 0x03, b'h', b'i', b'!'],
            // Field 1, LEN, payload is field 1 = 1 and field 2 = 2.
            &[0x0a, 0x04, 0x08, 0x01, 0x10, 0x02],
            &[0x08, 0x01, 0x12, 0x00, 0x1d, 0xff, 0xff, 0xff, 0xff],
            // Start-group, inner varint, end-group.
            &[0x0b, 0x10, 0x01, 0x0c],
        ];
        for bytes in cases {
            assert_matches_core(bytes);
        }
    }

    #[test]
    fn push_limit_skips_unread_and_restores_parent() {
        let mut cursor = SliceCursor::new(&[0x01, 0x02, 0x03, 0x04]);
        cursor.push_limit(3).unwrap();
        assert_eq!(cursor.chunk(), &[0x01, 0x02, 0x03]);
        cursor.advance(1);
        cursor.push_limit(1).unwrap();
        assert_eq!(cursor.chunk(), &[0x02]);
        cursor.pop_limit();
        assert_eq!(cursor.chunk(), &[0x03]);
        cursor.pop_limit();
        assert_eq!(cursor.chunk(), &[0x04]);
    }

    #[test]
    fn push_limit_truncated() {
        let mut cursor = SliceCursor::new(&[0x01]);
        assert_eq!(cursor.push_limit(2), Err(DecodeError::TruncatedMessage));
    }
}
