//! Wire-format decoding helpers used by generated field types.
//!
//! [`decode_varint_with_raw`] is the production varint read ([`ChunkScanHot4`]).
//! [`decode_varint`] calls it and drops the wire bytes. [`decode_tag`] uses
//! [`decode_varint_with`] with [`ChunkScanLikely1`]. Tags are unpacked here
//! into a validated [`FieldNumber`] plus [`WireType`].

mod group;
mod lazy_profile;
mod lazy_scan;
mod record;
mod slice_cursor;
mod varint;
mod wire_buf;

pub use self::slice_cursor::{SliceCursor, contiguous_remaining};

pub(crate) use group::{copy_group_body, measure_group};

pub use self::lazy_profile::{Borrowed, LazyProfile, Owned};
pub use self::lazy_scan::{Eager, Lazy, LazyMessage, LazyScan, MessageScan};
pub use self::record::{
    RecordScanner, ScannedRecord, ScannedValue, WireSpan, for_each_len_field,
    scanned_group_body_span, scanned_len_payload, scanned_len_span, scanned_record_span,
};
pub use self::varint::{
    ByteIterator, BytePair, ByteQuad, ChunkScan, ChunkScan1, ChunkScan2, ChunkScan4, ChunkScanHot2,
    ChunkScanHot4, ChunkScanHot8, ChunkScanHot10, ChunkScanHotAdapt, ChunkScanLikely1,
    ChunkScanMsb, VarintDecoder,
};
pub use self::wire_buf::{WireAppend, WireBuf};

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::protobuf_core::{FieldNumber, MAX_VARINT_BYTES, Varint};
use ::puroro::wire_type;
use ::puroro::{DecodeBuf, DecodeError, UnknownField, UnknownPayload, WireType};
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::unknown_fields::{UnknownFields, UnknownStore};

use crate::encode;

/// Decodes a base-128 varint and returns the decoded value plus its wire bytes.
///
/// This is the production read ([`ChunkScanHot4`]). [`decode_varint`] calls it
/// and drops the slice. `raw` is scratch. The returned slice borrows it and is
/// the exact encoding consumed, including a non-canonical one.
#[inline]
pub fn decode_varint_with_raw<'a, B: Buf>(
    buf: &mut B,
    raw: &'a mut [u8; MAX_VARINT_BYTES],
) -> Result<(Varint, &'a [u8]), DecodeError> {
    self::varint::decode_varint_with_raw(buf, raw)
}

/// Decodes a base-128 varint from `buf`.
///
/// Calls [`decode_varint_with_raw`] and drops the wire bytes. The `u64` is
/// [`Varint::to_uint64`] of the decoded value.
#[inline]
pub fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    let mut raw = [0u8; MAX_VARINT_BYTES];
    Ok(decode_varint_with_raw(buf, &mut raw)?.0.to_uint64())
}

/// Decodes a base-128 varint from `buf` using algorithm `D`.
///
/// Prefer [`decode_varint`] for ordinary values; use this when a different
/// decoder is appropriate (e.g. tags via [`ChunkScanLikely1`]).
#[inline]
pub fn decode_varint_with<D: VarintDecoder, B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    D::decode_varint(buf)
}

/// Decodes a tag and returns `(field_number, wire_type)`.
///
/// Uses [`ChunkScanLikely1`] for the tag varint (mostly 1-byte field numbers).
/// Field-number range is validated via [`FieldNumber::try_new`]; wire type via
/// [`wire_type::from_raw`]. Does not build a [`protobuf_core::Tag`] intermediate.
#[inline]
pub fn decode_tag<B: Buf>(buf: &mut B) -> Result<(FieldNumber, WireType), DecodeError> {
    let raw = decode_varint_with::<ChunkScanLikely1, _>(buf)?;
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

/// Next field tag, or the end of this message body.
///
/// `end_group` is `None` for a top-level or length-delimited body. A bare
/// end-group is [`DecodeError::InvalidTag`]. No more bytes is `Ok(None)`.
///
/// `Some(field)` reads a group body whose start tag is already consumed. The
/// matching end-group is consumed and returns `Ok(None)`. A different
/// end-group is [`DecodeError::MismatchedEndGroup`]. Running out of bytes
/// before that end-group is [`DecodeError::TruncatedMessage`].
#[inline]
pub fn next_tag<B: Buf>(
    buf: &mut B,
    end_group: Option<FieldNumber>,
) -> Result<Option<(FieldNumber, WireType)>, DecodeError> {
    if !buf.has_remaining() {
        return if end_group.is_some() {
            Err(DecodeError::TruncatedMessage)
        } else {
            Ok(None)
        };
    }
    let (number, wire_type) = decode_tag(buf)?;
    if wire_type != WireType::EGroup {
        return Ok(Some((number, wire_type)));
    }
    match end_group {
        Some(expected) if expected == number => Ok(None),
        Some(expected) => Err(DecodeError::MismatchedEndGroup {
            expected: expected.as_u32(),
            got: number.as_u32(),
        }),
        None => Err(DecodeError::InvalidTag),
    }
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
///
/// Invalid UTF-8 returns [`DecodeError::InvalidUtf8`]. Used for
/// `utf8_validation=VERIFY` (`ProtoString`). `NONE` strings decode as bytes.
pub(crate) fn decode_string_in<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<UnmanagedString<A>, DecodeError> {
    use crate::fields::wire::wire_payload::LenPayload;

    let payload = LenPayload::decode_in(WireType::Len, buf, alloc.clone())?;
    let bytes = payload.into_vec();
    match UnmanagedString::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(bytes) => {
            // SAFETY: `alloc` owns the buffer produced by `decode_in`.
            unsafe { bytes.deallocate(&alloc) };
            Err(DecodeError::InvalidUtf8)
        }
    }
}

/// One length-delimited body, either a contiguous slice or a limited reader.
///
/// [`read_len_body`] advances the outer buffer after [`LenBody::Slice`] returns.
/// [`LenBody::Limited`] is already scoped; its limit pops when the reader returns.
pub(crate) enum LenBody<'a, B: ?Sized> {
    /// `buf.chunk()` held the whole body. The outer buffer is not advanced yet.
    Slice(&'a [u8]),
    /// The body spans chunks. Reads stay inside the LEN length.
    Limited(&'a mut B),
}

/// Reads one LEN body and passes it to `read`.
///
/// A wire type other than [`WireType::Len`] is [`DecodeError::InvalidTag`] and
/// consumes nothing. The length varint is consumed before the remaining-length
/// check, so [`DecodeError::TruncatedMessage`] leaves that varint behind.
///
/// On the contiguous path, `read` runs against [`LenBody::Slice`], then this
/// function advances `len` bytes and returns that result. An `Err` from `read`
/// still advances. On the split path, `read` runs against [`LenBody::Limited`].
#[inline]
pub(crate) fn read_len_body<B, T, F>(
    buf: &mut B,
    wire_type: WireType,
    read: F,
) -> Result<T, DecodeError>
where
    B: DecodeBuf,
    F: FnOnce(LenBody<'_, B>) -> Result<T, DecodeError>,
{
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    if buf.chunk().len() >= len {
        let decoded = {
            let payload = &buf.chunk()[..len];
            read(LenBody::Slice(payload))
        };
        buf.advance(len);
        return decoded;
    }
    let mut guard = buf.push_limit_guard(len)?;
    read(LenBody::Limited(&mut *guard))
}

/// Skips one field payload for `wire_type` without preserving unknowns.
///
/// Used for map-entry interiors (and similar ephemeral messages) where unknown
/// fields are not round-tripped. A start-group consumes the body and the
/// matching end-group ([`group::skip_group`]). A bare end-group is
/// [`DecodeError::InvalidTag`].
pub(crate) fn skip_field<B: Buf>(
    field_number: FieldNumber,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
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
        WireType::SGroup => group::skip_group(field_number, buf)?,
        WireType::EGroup => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

/// Skips one field payload and appends its tag + wire bytes to a preserve store.
///
/// Used by message `merge` unknown arms. Discard messages call
/// [`skip_field_and_save_in`] instead.
pub fn skip_field_and_save<B: Buf, A: Allocator + Clone>(
    field_number: FieldNumber,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut UnknownFields<A>,
    alloc: A,
) -> Result<(), DecodeError> {
    skip_field_and_save_in(field_number, wire_type, buf, unknown_fields, alloc)
}

/// [`skip_field_and_save`] for any [`UnknownStore`] (preserve, discard, or reject).
///
/// Reject consumes a well-formed payload, then returns
/// [`DecodeError::UnrecognizedField`]. A truncated or illegal payload is that
/// wire error instead. Closed-enum diversion does not use this function.
pub fn skip_field_and_save_in<B: Buf, A: Allocator + Clone, U: UnknownStore<A>>(
    field_number: FieldNumber,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut U,
    alloc: A,
) -> Result<(), DecodeError> {
    let rejected = unknown_fields.on_unrecognized_tag(field_number.as_u32());
    if rejected.is_err() {
        skip_field(field_number, wire_type, buf)?;
        return rejected;
    }
    let Some(blob) = unknown_fields.append_blob_mut(alloc.clone()) else {
        return skip_field(field_number, wire_type, buf);
    };
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { blob.with_alloc(alloc) };
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
        WireType::SGroup => {
            group::copy_group(field_number, buf, |bytes| g.extend_from_slice(bytes))?;
        }
        WireType::EGroup => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

pub(crate) fn save_unknown_varint_field<A: Allocator + Clone, U: UnknownStore<A>>(
    field_number: FieldNumber,
    value: u64,
    unknown_fields: &mut U,
    alloc: A,
) {
    let Some(blob) = unknown_fields.append_blob_mut(alloc.clone()) else {
        return;
    };
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { blob.with_alloc(alloc) };
    encode::write_tag_to_vec(field_number, WireType::Varint, &mut *g);
    encode::write_varint_to_vec(Varint::from_uint64(value), &mut *g);
}

/// Iterates unknown fields stored as a contiguous partial protobuf stream.
///
/// The buffer is expected to be written only by [`skip_field_and_save`] and
/// `save_unknown_varint_field`. On truncated or otherwise corrupt input the
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
        let (field_number, wire_type) = match decode_tag(&mut buf) {
            Ok(t) => t,
            Err(_) => {
                debug_assert!(false, "corrupt unknown-fields blob: bad tag");
                self.rest = &[];
                return None;
            }
        };
        let number = field_number.as_u32();

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
            WireType::SGroup => {
                let frame =
                    match group::measure_group(buf.len(), |i| buf.get(i).copied(), 0, field_number)
                    {
                        Ok(Some(frame)) => frame,
                        _ => {
                            debug_assert!(false, "corrupt unknown-fields blob: bad group");
                            self.rest = &[];
                            return None;
                        }
                    };
                let payload = &buf[..frame.body_len];
                buf = &buf[frame.consumed..];
                UnknownPayload::Group(payload)
            }
            WireType::EGroup => {
                debug_assert!(false, "end-group tag is not an unknown field");
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
    use super::{LenBody, read_len_body};
    use ::bytes::Buf;
    use ::puroro::{DecodeError, ScopedBuf, WireType};

    /// [`Buf`] whose [`chunk`](Buf::chunk) is at most `max_chunk` bytes.
    struct Chunked<'a> {
        rest: &'a [u8],
        max_chunk: usize,
    }

    impl Buf for Chunked<'_> {
        fn remaining(&self) -> usize {
            self.rest.len()
        }

        fn chunk(&self) -> &[u8] {
            let n = self.max_chunk.min(self.rest.len());
            &self.rest[..n]
        }

        fn advance(&mut self, cnt: usize) {
            self.rest = &self.rest[cnt..];
        }
    }

    /// Length 3, body `AA BB CC`, trailing `DD`.
    const LEN3: [u8; 5] = [0x03, 0xAA, 0xBB, 0xCC, 0xDD];

    #[test]
    fn non_len_consumes_nothing() {
        let mut inner = Chunked {
            rest: &LEN3,
            max_chunk: 8,
        };
        let mut buf = ScopedBuf::new(&mut inner);
        let err = read_len_body(&mut buf, WireType::Varint, |_| -> Result<(), DecodeError> {
            panic!("reader must not run")
        })
        .unwrap_err();
        assert_eq!(err, DecodeError::InvalidTag);
        assert_eq!(buf.chunk(), &LEN3);
    }

    #[test]
    fn truncated_length_consumes_only_the_varint() {
        let data = [0x05, 0x01, 0x02];
        let mut inner = Chunked {
            rest: &data,
            max_chunk: 8,
        };
        let mut buf = ScopedBuf::new(&mut inner);
        let err = read_len_body(&mut buf, WireType::Len, |_| -> Result<(), DecodeError> {
            panic!("reader must not run")
        })
        .unwrap_err();
        assert_eq!(err, DecodeError::TruncatedMessage);
        assert_eq!(buf.chunk(), &[0x01, 0x02]);
    }

    #[test]
    fn contiguous_body_is_a_slice_and_advances_on_error() {
        let mut inner = Chunked {
            rest: &LEN3,
            max_chunk: 8,
        };
        let mut buf = ScopedBuf::new(&mut inner);
        let err = read_len_body(&mut buf, WireType::Len, |body| -> Result<(), DecodeError> {
            match body {
                LenBody::Slice(payload) => {
                    assert_eq!(payload, &[0xAA, 0xBB, 0xCC]);
                    Err(DecodeError::InvalidUtf8)
                }
                LenBody::Limited(_) => panic!("contiguous body took the limit path"),
            }
        })
        .unwrap_err();
        assert_eq!(err, DecodeError::InvalidUtf8);
        assert_eq!(buf.chunk(), &[0xDD]);
    }

    #[test]
    fn empty_body_is_an_empty_slice() {
        let data = [0x00, 0x7F];
        let mut inner = Chunked {
            rest: &data,
            max_chunk: 8,
        };
        let mut buf = ScopedBuf::new(&mut inner);
        let payload = read_len_body(&mut buf, WireType::Len, |body| match body {
            LenBody::Slice(payload) => Ok(payload.to_vec()),
            LenBody::Limited(_) => panic!("empty body took the limit path"),
        })
        .unwrap();
        assert_eq!(payload, Vec::<u8>::new());
        assert_eq!(buf.chunk(), &[0x7F]);
    }

    #[test]
    fn split_body_is_limited_and_unread_bytes_are_skipped() {
        let mut inner = Chunked {
            rest: &LEN3,
            max_chunk: 1,
        };
        let mut buf = ScopedBuf::new(&mut inner);
        let first = read_len_body(&mut buf, WireType::Len, |body| match body {
            LenBody::Slice(_) => panic!("split body took the slice path"),
            LenBody::Limited(limited) => {
                assert_eq!(limited.remaining(), 3);
                assert_eq!(limited.chunk(), &[0xAA]);
                limited.advance(1);
                Ok(limited.chunk()[0])
            }
        })
        .unwrap();
        assert_eq!(first, 0xBB);
        assert_eq!(buf.chunk(), &[0xDD]);
    }
}
