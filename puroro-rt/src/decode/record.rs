//! Incremental field-record scanner.
//!
//! `push` emits every **complete** record as metadata (and a small numeric
//! value). LEN payloads are **not** copied — callers slice `_wire` after
//! append via [`scanned_len_span`]. A group is one record from its start tag
//! through the matching end tag; the interior is [`scanned_group_body_span`].
//! An incomplete tag, length varint, fixed payload, LEN body, or group stays
//! in leftover. [`RecordScanner::finish`] errors if leftover is not empty.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::Buf;
use ::protobuf_core::{FieldNumber, Varint};
use ::puroro::wire_type;
use ::puroro::{DecodeError, WireType};

/// Byte range of one field payload inside a parent `_wire` buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireSpan {
    pub offset: usize,
    pub len: usize,
}

impl WireSpan {
    /// Returns the span as a subslice of `wire`.
    pub fn slice(self, wire: &[u8]) -> Result<&[u8], DecodeError> {
        let end = self
            .offset
            .checked_add(self.len)
            .ok_or(DecodeError::TruncatedMessage)?;
        wire.get(self.offset..end)
            .ok_or(DecodeError::TruncatedMessage)
    }
}

/// Decoded body of one [`ScannedRecord`].
///
/// LEN keeps only the payload length. A group keeps the interior length and
/// the end-tag length. The bytes live in `_wire` after append.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScannedValue {
    Varint(Varint),
    I32([u8; 4]),
    I64([u8; 8]),
    Len {
        payload_len: usize,
    },
    /// Start tag, interior, and matching end tag form the record.
    ///
    /// `body_len` excludes both tags. `end_tag_len` is the end-group tag.
    Group {
        body_len: usize,
        end_tag_len: usize,
    },
}

/// One complete record from [`RecordScanner::push`], with its position in
/// `leftover || chunk`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScannedRecord {
    pub field_number: FieldNumber,
    pub start: usize,
    pub len: usize,
    pub tag_len: usize,
    pub value: ScannedValue,
}

/// Full record (tag and body) in `_wire`, given `origin` of `leftover || chunk`.
pub fn scanned_record_span(origin: usize, rec: &ScannedRecord) -> Result<WireSpan, DecodeError> {
    let offset = origin
        .checked_add(rec.start)
        .ok_or(DecodeError::TruncatedMessage)?;
    Ok(WireSpan {
        offset,
        len: rec.len,
    })
}

/// Calls `f` with each LEN payload in a complete message body.
///
/// `payload` is one message body (not a length prefix). Non-LEN records,
/// including groups, are skipped. A truncated or illegal record is that
/// decode error. Group interiors are not visited here.
pub fn for_each_len_field(
    payload: &[u8],
    mut f: impl FnMut(u32, &[u8]) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    let mut scanner = RecordScanner::<Global>::new();
    let records = scanner.push(payload)?;
    scanner.finish()?;
    for rec in &records {
        if !matches!(rec.value, ScannedValue::Len { .. }) {
            continue;
        }
        let body = rec.len_span(0)?.slice(payload)?;
        f(rec.field_number.as_u32(), body)?;
    }
    Ok(())
}

/// LEN payload range in `_wire`, given `origin` of `leftover || chunk`.
///
/// The payload is the last `payload_len` bytes of the record (after tag + length).
pub fn scanned_len_span(origin: usize, rec: &ScannedRecord) -> Result<WireSpan, DecodeError> {
    rec.len_span(origin)
}

/// Group interior in `_wire`, given `origin` of `leftover || chunk`.
///
/// The span excludes the start tag and the matching end tag. Known group
/// fields are not applied yet; callers that need the bytes slice this span.
pub fn scanned_group_body_span(
    origin: usize,
    rec: &ScannedRecord,
) -> Result<WireSpan, DecodeError> {
    rec.group_body_span(origin)
}

/// LEN payload bytes in `wire`, given `origin` of `leftover || chunk`.
pub fn scanned_len_payload<'a>(
    origin: usize,
    rec: &ScannedRecord,
    wire: &'a [u8],
) -> Result<&'a [u8], DecodeError> {
    rec.len_span(origin)?.slice(wire)
}

impl ScannedRecord {
    /// LEN payload range in `_wire`.
    pub fn len_span(&self, origin: usize) -> Result<WireSpan, DecodeError> {
        let ScannedValue::Len { payload_len } = self.value else {
            return Err(DecodeError::InvalidTag);
        };
        let offset = origin
            .checked_add(self.start)
            .and_then(|s| s.checked_add(self.len))
            .and_then(|end| end.checked_sub(payload_len))
            .ok_or(DecodeError::TruncatedMessage)?;
        Ok(WireSpan {
            offset,
            len: payload_len,
        })
    }

    /// Group interior in `_wire`, excluding the start tag and the end tag.
    pub fn group_body_span(&self, origin: usize) -> Result<WireSpan, DecodeError> {
        let ScannedValue::Group { body_len, .. } = self.value else {
            return Err(DecodeError::InvalidTag);
        };
        let offset = origin
            .checked_add(self.start)
            .and_then(|start| start.checked_add(self.tag_len))
            .ok_or(DecodeError::TruncatedMessage)?;
        Ok(WireSpan {
            offset,
            len: body_len,
        })
    }
}

/// Leftover-holding scanner over vectored / chunked input.
#[derive(Debug)]
pub struct RecordScanner<A: Allocator = Global> {
    leftover: AllocVec<u8, A>,
}

impl<A: Allocator + Clone + Default> Default for RecordScanner<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl RecordScanner<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A: Allocator + Clone> Clone for RecordScanner<A> {
    fn clone(&self) -> Self {
        Self {
            leftover: self.leftover.clone(),
        }
    }
}

impl<A: Allocator> RecordScanner<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            leftover: AllocVec::new_in(alloc),
        }
    }

    /// Incomplete bytes waiting for the next [`push`](Self::push).
    pub fn leftover(&self) -> &[u8] {
        &self.leftover
    }

    /// `true` when an unfinished record is held.
    pub fn needs_more(&self) -> bool {
        !self.leftover.is_empty()
    }

    /// End of the logical stream. Non-empty leftover is truncated input.
    pub fn finish(&mut self) -> Result<(), DecodeError> {
        if self.leftover.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::TruncatedMessage)
        }
    }
}

impl<A: Allocator + Clone> RecordScanner<A> {
    /// Parse every complete field in `leftover || chunk`.
    ///
    /// Complete records are read in place. Only the unfinished tail is copied
    /// into leftover. Incomplete input is not an error here — call
    /// [`finish`](Self::finish) when no more bytes will arrive.
    pub fn push(&mut self, chunk: &[u8]) -> Result<AllocVec<ScannedRecord, A>, DecodeError> {
        let alloc = self.leftover.allocator().clone();
        let mut records = AllocVec::new_in(alloc);
        let src = TwoSlice {
            head: &self.leftover,
            tail: chunk,
        };
        let mut pos = 0usize;
        while let TryRecord::Done { rec } = try_record_at(&src, pos)? {
            pos += rec.len;
            records.push(rec);
        }
        keep_unparsed_tail(&mut self.leftover, chunk, pos);
        Ok(records)
    }
}

enum TryRecord {
    NeedMore,
    Done { rec: ScannedRecord },
}

enum TryVarint {
    NeedMore,
    Done { value: u64, len: usize },
}

/// `leftover || chunk` without concatenating complete records.
struct TwoSlice<'a> {
    head: &'a [u8],
    tail: &'a [u8],
}

impl<'a> TwoSlice<'a> {
    fn len(&self) -> usize {
        self.head.len() + self.tail.len()
    }

    fn get(&self, i: usize) -> Option<u8> {
        if i < self.head.len() {
            Some(self.head[i])
        } else {
            self.tail.get(i - self.head.len()).copied()
        }
    }

    fn copy_fixed<const N: usize>(&self, start: usize) -> Option<[u8; N]> {
        let mut out = [0u8; N];
        for (j, slot) in out.iter_mut().enumerate() {
            *slot = self.get(start + j)?;
        }
        Some(out)
    }

    fn buf_from(&self, start: usize) -> TwoSliceBuf<'a> {
        if start < self.head.len() {
            TwoSliceBuf {
                head: &self.head[start..],
                tail: self.tail,
            }
        } else {
            let at = start - self.head.len();
            TwoSliceBuf {
                head: &[],
                tail: self.tail.get(at..).unwrap_or(&[]),
            }
        }
    }
}

/// [`Buf`] view of [`TwoSlice`] from a start index.
struct TwoSliceBuf<'a> {
    head: &'a [u8],
    tail: &'a [u8],
}

impl Buf for TwoSliceBuf<'_> {
    fn remaining(&self) -> usize {
        self.head.len() + self.tail.len()
    }

    fn chunk(&self) -> &[u8] {
        if self.head.is_empty() {
            self.tail
        } else {
            self.head
        }
    }

    fn advance(&mut self, cnt: usize) {
        if cnt <= self.head.len() {
            self.head = &self.head[cnt..];
            return;
        }
        let into_tail = cnt - self.head.len();
        self.head = &[];
        self.tail = self.tail.get(into_tail..).unwrap_or(&[]);
    }
}

/// Keep `src[pos..]` in `leftover`. Complete prefix is dropped, not shifted
/// out of a concatenated buffer.
fn keep_unparsed_tail<A: Allocator + Clone>(
    leftover: &mut AllocVec<u8, A>,
    chunk: &[u8],
    pos: usize,
) {
    let head_len = leftover.len();
    let total = head_len + chunk.len();
    if pos >= total {
        leftover.clear();
        return;
    }
    if pos >= head_len {
        leftover.clear();
        leftover.extend_from_slice(&chunk[pos - head_len..]);
        return;
    }
    leftover.drain(..pos);
    leftover.extend_from_slice(chunk);
}

fn try_varint_at(src: &TwoSlice<'_>, start: usize) -> Result<TryVarint, DecodeError> {
    let mut buf = src.buf_from(start);
    let available = buf.remaining();
    match super::decode_varint(&mut buf) {
        Ok(value) => Ok(TryVarint::Done {
            value,
            len: available - buf.remaining(),
        }),
        Err(DecodeError::UnexpectedEof) => Ok(TryVarint::NeedMore),
        Err(err) => Err(err),
    }
}

fn try_record_at(src: &TwoSlice<'_>, start: usize) -> Result<TryRecord, DecodeError> {
    if start >= src.len() {
        return Ok(TryRecord::NeedMore);
    }
    let TryVarint::Done {
        value: raw_tag,
        len: tag_len,
    } = try_varint_at(src, start)?
    else {
        return Ok(TryRecord::NeedMore);
    };
    if raw_tag > u64::from(u32::MAX) {
        return Err(DecodeError::InvalidVarint);
    }
    let raw_tag = raw_tag as u32;
    let field_number = FieldNumber::try_new(raw_tag >> 3).map_err(|_| DecodeError::InvalidTag)?;
    let wire_type = wire_type::from_raw((raw_tag & 0b111) as u8)?;
    if wire_type == WireType::EGroup {
        return Err(DecodeError::InvalidTag);
    }

    let body_at = start + tag_len;
    let (value, body_len) = match wire_type {
        WireType::Varint => match try_varint_at(src, body_at)? {
            TryVarint::NeedMore => return Ok(TryRecord::NeedMore),
            TryVarint::Done { value, len } => {
                (ScannedValue::Varint(Varint::from_uint64(value)), len)
            }
        },
        WireType::Int64 => {
            let Some(bytes) = src.copy_fixed::<8>(body_at) else {
                return Ok(TryRecord::NeedMore);
            };
            (ScannedValue::I64(bytes), 8)
        }
        WireType::Int32 => {
            let Some(bytes) = src.copy_fixed::<4>(body_at) else {
                return Ok(TryRecord::NeedMore);
            };
            (ScannedValue::I32(bytes), 4)
        }
        WireType::Len => {
            let TryVarint::Done {
                value: payload_len,
                len: len_len,
            } = try_varint_at(src, body_at)?
            else {
                return Ok(TryRecord::NeedMore);
            };
            let payload_len = payload_len as usize;
            let body_len = len_len
                .checked_add(payload_len)
                .ok_or(DecodeError::TruncatedMessage)?;
            let payload_end = body_at + body_len;
            if src.len() < payload_end {
                return Ok(TryRecord::NeedMore);
            }
            (ScannedValue::Len { payload_len }, body_len)
        }
        WireType::SGroup => {
            let Some(frame) =
                super::group::measure_group(src.len(), |i| src.get(i), body_at, field_number)?
            else {
                return Ok(TryRecord::NeedMore);
            };
            (
                ScannedValue::Group {
                    body_len: frame.body_len,
                    end_tag_len: frame.end_tag_len,
                },
                frame.consumed,
            )
        }
        WireType::EGroup => unreachable!("rejected above"),
    };

    Ok(TryRecord::Done {
        rec: ScannedRecord {
            field_number,
            start,
            len: tag_len + body_len,
            tag_len,
            value,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::{encode_tag, encode_varint, encode_varint_field};
    use ::protobuf_core::Varint;
    use ::puroro::WireType;
    use ::std::vec::Vec;

    fn field(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).expect("test field number")
    }

    fn encode_len_field(number: u32, content: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_tag(field(number), WireType::Len, &mut buf);
        encode_varint(Varint::from_uint64(content.len() as u64), &mut buf);
        buf.extend_from_slice(content);
        buf
    }

    fn encode_fixed32_field(number: u32, value: u32) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_tag(field(number), WireType::Int32, &mut buf);
        buf.extend_from_slice(&value.to_le_bytes());
        buf
    }

    fn encode_fixed64_field(number: u32, value: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_tag(field(number), WireType::Int64, &mut buf);
        buf.extend_from_slice(&value.to_le_bytes());
        buf
    }

    #[test]
    fn empty_chunk_then_finish() {
        let mut s = RecordScanner::new();
        assert!(s.push(&[]).unwrap().is_empty());
        s.finish().unwrap();
        assert!(!s.needs_more());
    }

    #[test]
    fn one_varint_record() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(42), &mut wire);
        let mut s = RecordScanner::new();
        let recs = s.push(&wire).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].field_number, field(1));
        assert_eq!(recs[0].value, ScannedValue::Varint(Varint::from_uint64(42)));
        assert_eq!(recs[0].start, 0);
        assert_eq!(recs[0].len, wire.len());
    }

    #[test]
    fn mid_varint_value_needs_more() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(300), &mut wire);
        // 300 = 0xAC 0x02; tag is 0x08. Split after the first value byte.
        assert!(wire.len() >= 3);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..2]).unwrap().is_empty());
        assert!(s.needs_more());
        assert_eq!(s.finish(), Err(DecodeError::TruncatedMessage));
    }

    #[test]
    fn two_chunks_complete_varint() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(300), &mut wire);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..2]).unwrap().is_empty());
        let recs = s.push(&wire[2..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(
            recs[0].value,
            ScannedValue::Varint(Varint::from_uint64(300))
        );
    }

    #[test]
    fn mid_fixed32_needs_more() {
        let wire = encode_fixed32_field(2, 0xAABBCCDD);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..3]).unwrap().is_empty());
        assert_eq!(s.finish(), Err(DecodeError::TruncatedMessage));
    }

    #[test]
    fn two_chunks_complete_fixed64() {
        let wire = encode_fixed64_field(3, 7);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..4]).unwrap().is_empty());
        let recs = s.push(&wire[4..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].value, ScannedValue::I64(7u64.to_le_bytes()));
    }

    #[test]
    fn mid_len_body_needs_more() {
        let wire = encode_len_field(4, b"abcd");
        let mut s = RecordScanner::new();
        // tag + length + 2 content bytes
        assert!(s.push(&wire[..4]).unwrap().is_empty());
        assert_eq!(s.finish(), Err(DecodeError::TruncatedMessage));
    }

    #[test]
    fn two_chunks_complete_len() {
        let wire = encode_len_field(4, b"hello");
        let mid = 3;
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..mid]).unwrap().is_empty());
        let recs = s.push(&wire[mid..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].value, ScannedValue::Len { payload_len: 5 });
        let span = scanned_len_span(0, &recs[0]).unwrap();
        assert_eq!(&wire[span.offset..span.offset + span.len], b"hello");
    }

    #[test]
    fn empty_chunk_keeps_leftover() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(1), &mut wire);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..1]).unwrap().is_empty());
        assert!(s.push(&[]).unwrap().is_empty());
        assert!(s.needs_more());
    }

    #[test]
    fn two_records_one_chunk() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(1), &mut wire);
        encode_varint_field(field(2), Varint::from_uint64(2), &mut wire);
        let mut s = RecordScanner::new();
        let recs = s.push(&wire).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].field_number, field(1));
        assert_eq!(recs[1].field_number, field(2));
    }

    #[test]
    fn finish_after_complete_then_garbage_chunk() {
        let mut first = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(1), &mut first);
        let mut s = RecordScanner::new();
        assert_eq!(s.push(&first).unwrap().len(), 1);
        s.finish().unwrap();
        // A lone continuation byte is leftover, not a record.
        assert!(s.push(&[0x80]).unwrap().is_empty());
        assert_eq!(s.finish(), Err(DecodeError::TruncatedMessage));
    }

    fn push_group(buf: &mut Vec<u8>, number: u32, body: &[u8]) {
        encode_tag(field(number), WireType::SGroup, buf);
        buf.extend_from_slice(body);
        encode_tag(field(number), WireType::EGroup, buf);
    }

    #[test]
    fn unclosed_group_stays_in_leftover() {
        let mut buf = Vec::new();
        encode_tag(field(1), WireType::SGroup, &mut buf);
        let mut s = RecordScanner::new();
        assert!(s.push(&buf).unwrap().is_empty());
        assert_eq!(s.leftover(), buf.as_slice());
        assert_eq!(s.finish(), Err(DecodeError::TruncatedMessage));
    }

    #[test]
    fn bare_end_group_is_invalid() {
        let mut buf = Vec::new();
        encode_tag(field(1), WireType::EGroup, &mut buf);
        let mut s = RecordScanner::new();
        assert_eq!(s.push(&buf).err(), Some(DecodeError::InvalidTag));
    }

    #[test]
    fn group_is_one_record_and_keeps_the_following_field() {
        let mut inner = Vec::new();
        encode_varint_field(field(3), Varint::from_uint64(1), &mut inner);
        let mut nested = Vec::new();
        push_group(&mut nested, 1, &inner);
        let mut wire = Vec::new();
        push_group(&mut wire, 1, &nested);
        encode_varint_field(field(4), Varint::from_uint64(9), &mut wire);

        let mut s = RecordScanner::new();
        let recs = s.push(&wire).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 2);
        let ScannedValue::Group {
            body_len,
            end_tag_len,
        } = recs[0].value
        else {
            panic!("expected a group record");
        };
        assert_eq!(body_len, nested.len());
        assert_eq!(end_tag_len, 1);
        assert_eq!(recs[0].len, recs[0].tag_len + body_len + end_tag_len);
        let body = recs[0].group_body_span(0).unwrap().slice(&wire).unwrap();
        assert_eq!(body, nested.as_slice());
        assert_eq!(recs[1].field_number, field(4));
    }

    #[test]
    fn group_split_across_chunks_is_one_record() {
        let mut wire = Vec::new();
        push_group(&mut wire, 1, &[0x08, 0x01]);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..1]).unwrap().is_empty());
        let recs = s.push(&wire[1..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert!(matches!(recs[0].value, ScannedValue::Group { .. }));
    }

    #[test]
    fn mismatched_end_group_fails_the_scan() {
        let mut wire = Vec::new();
        encode_tag(field(1), WireType::SGroup, &mut wire);
        encode_tag(field(2), WireType::EGroup, &mut wire);
        let mut s = RecordScanner::new();
        assert_eq!(
            s.push(&wire).unwrap_err(),
            DecodeError::MismatchedEndGroup {
                expected: 1,
                got: 2
            }
        );
    }

    #[test]
    fn mid_tag_varint() {
        // Field 16 → tag 0x80 0x01.
        let mut wire = Vec::new();
        encode_varint_field(field(16), Varint::from_uint64(0), &mut wire);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..1]).unwrap().is_empty());
        let recs = s.push(&wire[1..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs[0].field_number, field(16));
    }

    #[test]
    fn leftover_is_only_unparsed_tail() {
        let mut first = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(1), &mut first);
        let mut second = Vec::new();
        encode_varint_field(field(2), Varint::from_uint64(2), &mut second);
        let mut third = Vec::new();
        encode_varint_field(field(16), Varint::from_uint64(0), &mut third);
        let mut chunk = first;
        chunk.extend_from_slice(&second);
        chunk.extend_from_slice(&third[..1]);
        let mut s = RecordScanner::new();
        let recs = s.push(&chunk).unwrap();
        assert_eq!(recs.len(), 2);
        assert_eq!(s.leftover(), &third[..1]);
    }

    #[test]
    fn record_spans_three_chunks() {
        let wire = encode_len_field(4, b"hello");
        assert!(wire.len() >= 3);
        let a = wire.len() / 3;
        let b = (2 * wire.len()) / 3;
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..a]).unwrap().is_empty());
        assert!(s.push(&wire[a..b]).unwrap().is_empty());
        let recs = s.push(&wire[b..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].value, ScannedValue::Len { payload_len: 5 });
    }

    #[test]
    fn overlong_varint_across_chunks() {
        let mut s = RecordScanner::new();
        assert!(s.push(&[0x80; 6]).unwrap().is_empty());
        assert_eq!(s.push(&[0x80; 4]).err(), Some(DecodeError::InvalidVarint));
        // The failing chunk is not consumed.
        assert_eq!(s.leftover(), &[0x80; 6]);
    }

    #[test]
    fn tag_above_u32_across_chunks() {
        // 2^32 as a tag varint: five bytes, value does not fit in `u32`.
        let mut s = RecordScanner::new();
        assert!(s.push(&[0x80, 0x80]).unwrap().is_empty());
        assert_eq!(
            s.push(&[0x80, 0x80, 0x10]).err(),
            Some(DecodeError::InvalidVarint)
        );
        assert_eq!(s.leftover(), &[0x80, 0x80]);
    }

    #[test]
    fn field_number_zero_and_unknown_wire_type() {
        let mut s = RecordScanner::new();
        assert_eq!(s.push(&[0x00]).err(), Some(DecodeError::InvalidTag));
        assert!(s.leftover().is_empty());
        // Field 1, wire type 7.
        assert_eq!(s.push(&[0x0f]).err(), Some(DecodeError::InvalidTag));
        assert!(s.leftover().is_empty());
    }

    #[test]
    fn invalid_tag_does_not_consume_earlier_record() {
        let mut wire = Vec::new();
        encode_varint_field(field(1), Varint::from_uint64(1), &mut wire);
        wire.push(0x0f);
        let mut s = RecordScanner::new();
        assert_eq!(s.push(&wire).err(), Some(DecodeError::InvalidTag));
        assert!(s.leftover().is_empty());
    }

    #[test]
    fn two_chunks_split_len_prefix() {
        let content = vec![0xab; 128];
        let wire = encode_len_field(4, &content);
        // tag, then the two-byte length 128 (`0x80 0x01`). Split inside that length.
        assert_eq!(&wire[..3], &[0x22, 0x80, 0x01]);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..2]).unwrap().is_empty());
        assert!(s.needs_more());
        let recs = s.push(&wire[2..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].value, ScannedValue::Len { payload_len: 128 });
    }

    #[test]
    fn two_chunks_complete_fixed32() {
        let wire = encode_fixed32_field(2, 0xAABBCCDD);
        let mut s = RecordScanner::new();
        assert!(s.push(&wire[..3]).unwrap().is_empty());
        let recs = s.push(&wire[3..]).unwrap();
        s.finish().unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(
            recs[0].value,
            ScannedValue::I32(0xAABBCCDDu32.to_le_bytes())
        );
    }
}
