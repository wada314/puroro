//! Incremental field-record scanner.
//!
//! `push` emits every **complete** [`Field`]. An incomplete tag, length
//! varint, fixed payload, or LEN body stays in leftover. [`RecordScanner::finish`]
//! errors if leftover is not empty.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::ops::Deref;
use ::protobuf_core::{
    Field, FieldNumber, FieldValue, MAX_VARINT_BYTES, VARINT_CONTINUATION_BIT, VARINT_PAYLOAD_MASK,
    Varint,
};
use ::puroro::wire_type;
use ::puroro::{DecodeError, ScopedBuf, WireType};
use ::std::vec::Vec;

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

/// One complete record from [`RecordScanner::push`], with its position in
/// `leftover || chunk`.
#[derive(Debug)]
pub struct ScannedRecord<A: Allocator> {
    pub field: Field<AllocVec<u8, A>>,
    pub start: usize,
    pub len: usize,
}

impl<A: Allocator> Deref for ScannedRecord<A> {
    type Target = Field<AllocVec<u8, A>>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

/// LEN payload range in `_wire`, given `origin` of `leftover || chunk`.
///
/// The payload is the last `Len` bytes of the record (after tag + length).
pub fn scanned_len_span<A: Allocator>(
    origin: usize,
    rec: &ScannedRecord<A>,
) -> Result<WireSpan, DecodeError> {
    match &rec.field.value {
        FieldValue::Len(payload) => Ok(WireSpan {
            offset: origin + rec.start + rec.len - payload.len(),
            len: payload.len(),
        }),
        _ => Err(DecodeError::InvalidTag),
    }
}

/// LEN payload bytes from a scanned record (content only).
pub fn scanned_len_payload<A: Allocator>(rec: &ScannedRecord<A>) -> Result<&[u8], DecodeError> {
    match &rec.field.value {
        FieldValue::Len(payload) => Ok(payload.as_slice()),
        _ => Err(DecodeError::InvalidTag),
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
    pub fn push(&mut self, chunk: &[u8]) -> Result<AllocVec<ScannedRecord<A>, A>, DecodeError> {
        let alloc = self.leftover.allocator().clone();
        let mut records = AllocVec::new_in(alloc.clone());
        let src = TwoSlice {
            head: &self.leftover,
            tail: chunk,
        };
        let mut pos = 0usize;
        while let TryRecord::Done { field, len } = try_record_at(&src, pos, alloc.clone())? {
            records.push(ScannedRecord {
                field,
                start: pos,
                len,
            });
            pos += len;
        }
        keep_unparsed_tail(&mut self.leftover, chunk, pos);
        Ok(records)
    }
}

/// Reconstructs the post-tag body of a scanned [`Field`] for catalog `merge`.
///
/// [`FieldValue::Len`] is payload only; packed / LEN catalog paths still read a
/// length varint, so that arm prefixes the length before calling `merge`.
pub fn merge_scanned_field<L, F>(field: &Field<L>, merge: F) -> Result<(), DecodeError>
where
    L: AsRef<[u8]>,
    F: FnOnce(WireType, &mut ScopedBuf<'_, &[u8]>) -> Result<(), DecodeError>,
{
    match &field.value {
        FieldValue::Varint(v) => {
            let (bytes, n) = v.encode();
            let mut slice = &bytes[..n];
            let mut scoped = ScopedBuf::new(&mut slice);
            merge(WireType::Varint, &mut scoped)
        }
        FieldValue::I32(bytes) => {
            let mut slice = bytes.as_slice();
            let mut scoped = ScopedBuf::new(&mut slice);
            merge(WireType::Int32, &mut scoped)
        }
        FieldValue::I64(bytes) => {
            let mut slice = bytes.as_slice();
            let mut scoped = ScopedBuf::new(&mut slice);
            merge(WireType::Int64, &mut scoped)
        }
        FieldValue::Len(payload) => {
            let payload = payload.as_ref();
            let (len_bytes, n) = Varint::from_uint64(payload.len() as u64).encode();
            let total = n + payload.len();
            let mut body = Vec::with_capacity(total);
            body.extend_from_slice(&len_bytes[..n]);
            body.extend_from_slice(payload);
            let mut slice = body.as_slice();
            let mut scoped = ScopedBuf::new(&mut slice);
            merge(WireType::Len, &mut scoped)
        }
    }
}

enum TryRecord<A: Allocator> {
    NeedMore,
    Done {
        field: Field<AllocVec<u8, A>>,
        len: usize,
    },
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

    fn copy_range<A: Allocator + Clone>(
        &self,
        start: usize,
        end: usize,
        alloc: A,
    ) -> AllocVec<u8, A> {
        debug_assert!(start <= end && end <= self.len());
        let mut v = AllocVec::with_capacity_in(end - start, alloc);
        if let Some(slice) = self.contiguous(start, end) {
            v.extend_from_slice(slice);
            return v;
        }
        for i in start..end {
            v.push(self.get(i).expect("range in bounds"));
        }
        v
    }

    fn contiguous(&self, start: usize, end: usize) -> Option<&'a [u8]> {
        if end <= self.head.len() {
            Some(&self.head[start..end])
        } else if start >= self.head.len() {
            let s = start - self.head.len();
            let e = end - self.head.len();
            Some(&self.tail[s..e])
        } else {
            None
        }
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
    let mut value = 0u64;
    for i in 0..MAX_VARINT_BYTES {
        let Some(byte) = src.get(start + i) else {
            return Ok(TryVarint::NeedMore);
        };
        value |= u64::from(byte & VARINT_PAYLOAD_MASK) << (7 * i);
        if byte & VARINT_CONTINUATION_BIT == 0 {
            return Ok(TryVarint::Done { value, len: i + 1 });
        }
    }
    Err(DecodeError::InvalidVarint)
}

fn try_record_at<A: Allocator + Clone>(
    src: &TwoSlice<'_>,
    start: usize,
    alloc: A,
) -> Result<TryRecord<A>, DecodeError> {
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
    if matches!(wire_type, WireType::SGroup | WireType::EGroup) {
        return Err(DecodeError::InvalidTag);
    }

    let body_at = start + tag_len;
    let (value, body_len) = match wire_type {
        WireType::Varint => match try_varint_at(src, body_at)? {
            TryVarint::NeedMore => return Ok(TryRecord::NeedMore),
            TryVarint::Done { value, len } => (FieldValue::from_uint64(value), len),
        },
        WireType::Int64 => {
            let Some(bytes) = src.copy_fixed::<8>(body_at) else {
                return Ok(TryRecord::NeedMore);
            };
            (FieldValue::I64(bytes), 8)
        }
        WireType::Int32 => {
            let Some(bytes) = src.copy_fixed::<4>(body_at) else {
                return Ok(TryRecord::NeedMore);
            };
            (FieldValue::I32(bytes), 4)
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
            let payload_at = body_at + len_len;
            let payload_end = body_at + body_len;
            if src.len() < payload_end {
                return Ok(TryRecord::NeedMore);
            }
            (
                FieldValue::Len(src.copy_range(payload_at, payload_end, alloc)),
                body_len,
            )
        }
        WireType::SGroup | WireType::EGroup => unreachable!("rejected above"),
    };

    Ok(TryRecord::Done {
        field: Field::new(field_number, value),
        len: tag_len + body_len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::{encode_tag, encode_varint, encode_varint_field};
    use ::protobuf_core::Varint;
    use ::puroro::WireType;

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

    fn copy_slice_in<A: Allocator + Clone>(bytes: &[u8], alloc: A) -> AllocVec<u8, A> {
        let mut v = AllocVec::with_capacity_in(bytes.len(), alloc);
        v.extend_from_slice(bytes);
        v
    }

    fn len_bytes(bytes: &[u8]) -> FieldValue<AllocVec<u8, Global>> {
        FieldValue::Len(copy_slice_in(bytes, Global))
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
        assert_eq!(
            recs[0].field,
            Field::new(field(1), FieldValue::from_uint64(42))
        );
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
        assert_eq!(recs[0].value, FieldValue::from_uint64(300));
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
        assert_eq!(recs[0].value, FieldValue::from_fixed64(7));
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
        assert_eq!(recs[0].value, len_bytes(b"hello"));
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

    #[test]
    fn group_tag_is_invalid() {
        let mut buf = Vec::new();
        encode_tag(field(1), WireType::SGroup, &mut buf);
        let mut s = RecordScanner::new();
        assert_eq!(s.push(&buf).err(), Some(DecodeError::InvalidTag));
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
        assert_eq!(recs[0].value, len_bytes(b"hello"));
    }
}
