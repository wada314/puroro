//! Wire-format decoding helpers used by generated field types.
//!
//! Varint decoding prefers a contiguous [`Buf::chunk`] scan; when a varint
//! spans chunk boundaries it falls back to [`protobuf_core::IteratorExtVarint`].
//! Tags are unpacked here into a validated [`FieldNumber`] plus [`WireType`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::protobuf_core::{
    FieldNumber, IteratorExtVarint, MAX_VARINT_BYTES, VARINT_CONTINUATION_BIT, VARINT_PAYLOAD_MASK,
    Varint,
};
use ::puroro::wire_type;
use ::puroro::{DecodeError, UnknownField, UnknownPayload, WireType};
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::encode;

struct BufVarintReader<'a, B: Buf> {
    buf: &'a mut B,
}

impl<B: Buf> Iterator for BufVarintReader<'_, B> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.has_remaining() {
            Some(self.buf.get_u8())
        } else {
            None
        }
    }
}

/// Try to decode a full varint from a contiguous slice.
///
/// - `Ok(Some((value, len)))` — complete varint of `len` bytes (1..=10)
/// - `Ok(None)` — not enough bytes in `chunk` (caller should use another source)
/// - `Err(InvalidVarint)` — 10 continuation bytes (too long)
fn try_decode_varint_from_slice(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    let mut value = 0u64;
    let mut shift = 0u32;

    for (i, &byte) in chunk.iter().take(MAX_VARINT_BYTES).enumerate() {
        value |= u64::from(byte & VARINT_PAYLOAD_MASK) << shift;
        if byte & VARINT_CONTINUATION_BIT == 0 {
            return Ok(Some((value, i + 1)));
        }
        shift += 7;
    }

    if chunk.len() >= MAX_VARINT_BYTES {
        Err(DecodeError::InvalidVarint)
    } else {
        Ok(None)
    }
}

/// Decodes a base-128 varint from `buf`.
pub fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    match try_decode_varint_from_slice(buf.chunk())? {
        Some((value, len)) => {
            buf.advance(len);
            Ok(value)
        }
        None => match (BufVarintReader { buf }).read_varint()? {
            Some(v) => Ok(v.to_uint64()),
            None => Err(DecodeError::UnexpectedEof),
        },
    }
}

/// Decodes a tag and returns `(field_number, wire_type)`.
///
/// Field-number range is validated via [`FieldNumber::try_new`]; wire type via
/// [`wire_type::from_raw`]. Does not build a [`protobuf_core::Tag`] intermediate.
pub fn decode_tag<B: Buf>(buf: &mut B) -> Result<(FieldNumber, WireType), DecodeError> {
    let raw = decode_varint(buf)?;
    // Tag values are u32 on the wire; preserve the previous InvalidVarint mapping
    // used when going through `Varint::try_to_uint32`.
    if raw > u64::from(u32::MAX) {
        return Err(DecodeError::InvalidVarint);
    }
    let raw = raw as u32;
    let field_number = FieldNumber::try_new(raw >> 3).map_err(|_| DecodeError::InvalidTag)?;
    let wire_type = wire_type::from_raw((raw & 0b111) as u8)?;
    Ok((field_number, wire_type))
}

/// Decodes one LEN payload into an [`UnmanagedVec<u8, A>`].
pub(crate) fn decode_bytes_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<UnmanagedVec<u8, A>, DecodeError> {
    use crate::fields::wire::wire_payload::LenPayload;

    Ok(LenPayload::decode_in(WireType::Len, buf, alloc)?.into_vec())
}

/// Decodes one LEN payload as UTF-8 into an [`UnmanagedString<A>`].
pub(crate) fn decode_string_in<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<UnmanagedString<A>, DecodeError> {
    use crate::fields::wire::wire_payload::LenPayload;

    let payload = LenPayload::decode_in(WireType::Len, buf, alloc.clone())?;
    match UnmanagedString::from_utf8(payload.into_vec()) {
        Ok(s) => Ok(s),
        Err(bytes) => {
            // SAFETY: `alloc` owns the buffer produced by `decode_in`.
            unsafe { bytes.deallocate(alloc) };
            Err(DecodeError::InvalidUtf8)
        }
    }
}

/// Skips one field payload for `wire_type` without preserving unknowns.
///
/// Used for map-entry interiors (and similar ephemeral messages) where unknown
/// fields are not round-tripped.
pub(crate) fn skip_field<B: Buf>(wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
    match wire_type {
        WireType::Varint => {
            decode_varint(buf)?;
        }
        WireType::Int64 => {
            if buf.remaining() < 8 {
                return Err(DecodeError::UnexpectedEof);
            }
            buf.advance(8);
        }
        WireType::Len => {
            let len = decode_varint(buf)? as usize;
            if buf.remaining() < len {
                return Err(DecodeError::TruncatedMessage);
            }
            buf.advance(len);
        }
        WireType::Int32 => {
            if buf.remaining() < 4 {
                return Err(DecodeError::UnexpectedEof);
            }
            buf.advance(4);
        }
        WireType::SGroup | WireType::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

/// Skips one field payload and appends its tag + wire bytes to `unknown_fields`.
///
/// Used by message `merge` unknown arms so unrecognized tags round-trip on encode.
pub fn skip_field_and_save<B: Buf, A: Allocator>(
    field_number: FieldNumber,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut UnmanagedVec<u8, A>,
    alloc: A,
) -> Result<(), DecodeError> {
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { unknown_fields.with_alloc(alloc) };
    encode::write_tag_to_vec(field_number, wire_type, &mut *g);

    match wire_type {
        WireType::Varint => {
            let v = decode_varint(buf)?;
            encode::write_varint_to_vec(Varint::from_uint64(v), &mut *g);
        }
        WireType::Int64 => {
            if buf.remaining() < 8 {
                return Err(DecodeError::UnexpectedEof);
            }
            let bytes = buf.copy_to_bytes(8);
            g.extend_from_slice(&bytes);
        }
        WireType::Len => {
            let len = decode_varint(buf)?;
            if buf.remaining() < len as usize {
                return Err(DecodeError::TruncatedMessage);
            }
            encode::write_varint_to_vec(Varint::from_uint64(len), &mut *g);
            let bytes = buf.copy_to_bytes(len as usize);
            g.extend_from_slice(&bytes);
        }
        WireType::Int32 => {
            if buf.remaining() < 4 {
                return Err(DecodeError::UnexpectedEof);
            }
            let bytes = buf.copy_to_bytes(4);
            g.extend_from_slice(&bytes);
        }
        WireType::SGroup | WireType::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

pub(crate) fn save_unknown_varint_field<A: Allocator>(
    field_number: FieldNumber,
    value: u64,
    unknown_fields: &mut UnmanagedVec<u8, A>,
    alloc: A,
) {
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { unknown_fields.with_alloc(alloc) };
    encode::write_tag_to_vec(field_number, WireType::Varint, &mut *g);
    encode::write_varint_to_vec(Varint::from_uint64(value), &mut *g);
}

/// Iterates unknown fields stored as a contiguous partial protobuf stream.
///
/// The buffer is expected to be written only by [`skip_field_and_save`] and
/// [`save_unknown_varint_field`]. On truncated or otherwise corrupt input the
/// iterator ends (infallible for callers).
#[inline]
pub fn iter_unknown_fields(bytes: &[u8]) -> UnknownFieldsIter<'_> {
    UnknownFieldsIter { rest: bytes }
}

/// Iterator over [`UnknownField`] values parsed from a preserved unknown-fields
/// wire blob.
#[derive(Debug, Clone)]
pub struct UnknownFieldsIter<'a> {
    rest: &'a [u8],
}

impl<'a> Iterator for UnknownFieldsIter<'a> {
    type Item = UnknownField<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut buf = self.rest;
        let (number, wire_type) = match decode_tag(&mut buf) {
            Ok(t) => t,
            Err(_) => {
                debug_assert!(false, "corrupt unknown-fields blob: bad tag");
                self.rest = &[];
                return None;
            }
        };
        let number = number.as_u32();

        let payload = match wire_type {
            WireType::Varint => {
                let v = match decode_varint(&mut buf) {
                    Ok(v) => v,
                    Err(_) => {
                        debug_assert!(false, "corrupt unknown-fields blob: bad varint");
                        self.rest = &[];
                        return None;
                    }
                };
                UnknownPayload::Varint(v)
            }
            WireType::Int64 => {
                if buf.len() < 8 {
                    debug_assert!(false, "corrupt unknown-fields blob: truncated I64");
                    self.rest = &[];
                    return None;
                }
                let mut le = [0u8; 8];
                le.copy_from_slice(&buf[..8]);
                buf = &buf[8..];
                UnknownPayload::Fixed64(u64::from_le_bytes(le))
            }
            WireType::Len => {
                let len = match decode_varint(&mut buf) {
                    Ok(v) => v as usize,
                    Err(_) => {
                        debug_assert!(false, "corrupt unknown-fields blob: bad LEN length");
                        self.rest = &[];
                        return None;
                    }
                };
                if buf.len() < len {
                    debug_assert!(false, "corrupt unknown-fields blob: truncated LEN");
                    self.rest = &[];
                    return None;
                }
                let payload = &buf[..len];
                buf = &buf[len..];
                UnknownPayload::Bytes(payload)
            }
            WireType::Int32 => {
                if buf.len() < 4 {
                    debug_assert!(false, "corrupt unknown-fields blob: truncated I32");
                    self.rest = &[];
                    return None;
                }
                let mut le = [0u8; 4];
                le.copy_from_slice(&buf[..4]);
                buf = &buf[4..];
                UnknownPayload::Fixed32(u32::from_le_bytes(le))
            }
            WireType::SGroup | WireType::EGroup => {
                debug_assert!(false, "groups are not preserved in unknown fields");
                self.rest = &[];
                return None;
            }
        };

        self.rest = buf;
        Some(UnknownField::new(number, payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bytes::Buf;

    /// `Buf` that exposes `first` and `rest` as separate chunks.
    struct SplitBuf<'a> {
        first: &'a [u8],
        rest: &'a [u8],
    }

    impl Buf for SplitBuf<'_> {
        fn remaining(&self) -> usize {
            self.first.len() + self.rest.len()
        }

        fn chunk(&self) -> &[u8] {
            if self.first.is_empty() {
                self.rest
            } else {
                self.first
            }
        }

        fn advance(&mut self, cnt: usize) {
            assert!(cnt <= self.remaining());
            if cnt <= self.first.len() {
                self.first = &self.first[cnt..];
            } else {
                let into_rest = cnt - self.first.len();
                self.first = &[];
                self.rest = &self.rest[into_rest..];
            }
        }
    }

    fn encode_u64(value: u64) -> Vec<u8> {
        let (bytes, count) = Varint::from_uint64(value).encode();
        bytes[..count].to_vec()
    }

    #[test]
    fn decode_varint_lengths() {
        for value in [0u64, 1, 127, 150, 1 << 28, u64::from(u32::MAX), u64::MAX] {
            let wire = encode_u64(value);
            let mut cursor = wire.as_slice();
            assert_eq!(decode_varint(&mut cursor).unwrap(), value);
            assert!(!cursor.has_remaining());
        }
    }

    #[test]
    fn decode_varint_too_long() {
        let wire = [0x80u8; 10];
        let mut cursor = wire.as_slice();
        assert!(matches!(
            decode_varint(&mut cursor),
            Err(DecodeError::InvalidVarint)
        ));
    }

    #[test]
    fn decode_varint_empty() {
        let mut cursor = [].as_slice();
        assert!(matches!(
            decode_varint(&mut cursor),
            Err(DecodeError::UnexpectedEof)
        ));
    }

    #[test]
    fn decode_varint_split_across_chunks() {
        // 150 == [0x96, 0x01], split after the first byte.
        let mut buf = SplitBuf {
            first: &[0x96],
            rest: &[0x01],
        };
        assert_eq!(decode_varint(&mut buf).unwrap(), 150);
        assert_eq!(buf.remaining(), 0);
    }

    #[test]
    fn decode_varint_split_then_more_data() {
        let mut buf = SplitBuf {
            first: &[0x96],
            rest: &[0x01, 0x07],
        };
        assert_eq!(decode_varint(&mut buf).unwrap(), 150);
        assert_eq!(decode_varint(&mut buf).unwrap(), 7);
    }
}
