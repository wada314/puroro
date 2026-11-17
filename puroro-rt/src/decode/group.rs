//! Group framing on puroro's cursors.
//!
//! The start-group tag is already consumed. Nested groups keep a stack of
//! field numbers, the same rule as `protobuf_core::until_group_ends`: an
//! end-group must match the start-group currently open. A depth counter that
//! ignores those numbers is not used.
//!
//! [`measure_group`] reports byte ranges for the scanner and the unknown-field
//! iterator. [`skip_group`] and [`copy_group`] advance a [`Buf`](bytes::Buf).
//! Copying writes the raw interior and the end tag. It does not re-encode
//! fields. Known-field group encoding is a separate path and panics.

use ::bytes::Buf;
use ::protobuf_core::{FieldNumber, MAX_VARINT_BYTES};
use ::puroro::wire_type;
use ::puroro::{DecodeError, RECURSION_LIMIT, WireType};

/// Byte sizes of one group after its start tag has been consumed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GroupFrame {
    /// Interior plus the matching end tag.
    pub consumed: usize,
    /// Interior only. Excludes the start tag and the end tag.
    pub body_len: usize,
    /// Size of the matching end-group tag.
    pub end_tag_len: usize,
}

/// Field numbers of groups that are still open. Index 0 is the outermost.
struct OpenGroups {
    stack: [u32; RECURSION_LIMIT],
    len: usize,
}

impl OpenGroups {
    fn new(outer: u32) -> Self {
        let mut stack = [0; RECURSION_LIMIT];
        stack[0] = outer;
        Self { stack, len: 1 }
    }

    /// Accounts for one tag inside the group.
    ///
    /// Returns whether this tag closed the outermost group.
    fn on_tag(&mut self, number: u32, wire: WireType) -> Result<bool, DecodeError> {
        match wire {
            WireType::SGroup => {
                if self.len >= RECURSION_LIMIT {
                    return Err(DecodeError::RecursionLimitExceeded);
                }
                self.stack[self.len] = number;
                self.len += 1;
                Ok(false)
            }
            WireType::EGroup => {
                let expected = self.stack[self.len - 1];
                if number != expected {
                    return Err(DecodeError::MismatchedEndGroup {
                        expected,
                        got: number,
                    });
                }
                self.len -= 1;
                Ok(self.len == 0)
            }
            _ => Ok(false),
        }
    }
}

/// Measures one group in an indexable buffer.
///
/// `get` reads absolute indexes in `0..len`. `start` is the first byte after
/// the consumed start tag. `Ok(None)` means the group is not finished inside
/// this buffer (more chunks may arrive). A mismatched end-group, an illegal
/// tag, or a varint longer than 10 bytes is `Err`.
pub(crate) fn measure_group(
    len: usize,
    mut get: impl FnMut(usize) -> Option<u8>,
    start: usize,
    field_number: FieldNumber,
) -> Result<Option<GroupFrame>, DecodeError> {
    let mut groups = OpenGroups::new(field_number.as_u32());
    let mut pos = start;
    loop {
        let Some((raw, tag_len)) = varint_at(len, &mut get, pos)? else {
            return Ok(None);
        };
        let (number, wire) = split_tag(raw)?;
        let after_tag = pos + tag_len;
        if groups.on_tag(number, wire)? {
            let consumed = after_tag - start;
            return Ok(Some(GroupFrame {
                consumed,
                body_len: consumed - tag_len,
                end_tag_len: tag_len,
            }));
        }
        pos = after_tag;
        pos = match wire {
            WireType::SGroup | WireType::EGroup => pos,
            WireType::Varint => match varint_at(len, &mut get, pos)? {
                Some((_, n)) => pos + n,
                None => return Ok(None),
            },
            WireType::Int32 => match skip_fixed(len, pos, 4) {
                Some(end) => end,
                None => return Ok(None),
            },
            WireType::Int64 => match skip_fixed(len, pos, 8) {
                Some(end) => end,
                None => return Ok(None),
            },
            WireType::Len => match len_end(len, &mut get, pos)? {
                Some(end) => end,
                None => return Ok(None),
            },
        };
    }
}

/// Skips one group body and its matching end tag.
///
/// The start tag for `field_number` is already consumed.
pub(crate) fn skip_group<B: Buf>(
    field_number: FieldNumber,
    buf: &mut B,
) -> Result<(), DecodeError> {
    walk_group(field_number, buf, |_| {})
}

/// Copies one group body and its matching end tag into `emit`.
///
/// `emit` must copy the bytes. The slice is borrowed from `buf`'s current
/// chunk and is invalid after the next read. The start tag is not copied.
pub(crate) fn copy_group<B: Buf>(
    field_number: FieldNumber,
    buf: &mut B,
    emit: impl FnMut(&[u8]),
) -> Result<(), DecodeError> {
    walk_group(field_number, buf, emit)
}

fn walk_group<B: Buf>(
    field_number: FieldNumber,
    buf: &mut B,
    mut emit: impl FnMut(&[u8]),
) -> Result<(), DecodeError> {
    let mut groups = OpenGroups::new(field_number.as_u32());
    let mut scratch = [0u8; MAX_VARINT_BYTES];
    loop {
        if !buf.has_remaining() {
            return Err(DecodeError::TruncatedMessage);
        }
        let (tag, raw) = super::decode_varint_with_raw(buf, &mut scratch)?;
        let (number, wire) = split_tag(tag.to_uint64())?;
        let closed = groups.on_tag(number, wire)?;
        emit(raw);
        if closed {
            return Ok(());
        }
        match wire {
            WireType::SGroup | WireType::EGroup => {}
            WireType::Varint => {
                let (_, raw) = super::decode_varint_with_raw(buf, &mut scratch)?;
                emit(raw);
            }
            WireType::Int32 => {
                copy_exact(buf, 4, DecodeError::UnexpectedEof, &mut emit)?;
            }
            WireType::Int64 => {
                copy_exact(buf, 8, DecodeError::UnexpectedEof, &mut emit)?;
            }
            WireType::Len => {
                let (len_value, raw) = super::decode_varint_with_raw(buf, &mut scratch)?;
                emit(raw);
                let Ok(len) = usize::try_from(len_value.to_uint64()) else {
                    return Err(DecodeError::TruncatedMessage);
                };
                copy_exact(buf, len, DecodeError::TruncatedMessage, &mut emit)?;
            }
        }
    }
}

fn split_tag(raw: u64) -> Result<(u32, WireType), DecodeError> {
    if raw > u64::from(u32::MAX) {
        return Err(DecodeError::InvalidVarint);
    }
    let raw = raw as u32;
    let number = FieldNumber::try_new(raw >> 3).map_err(|_| DecodeError::InvalidTag)?;
    let wire = wire_type::from_raw((raw & 0b111) as u8)?;
    Ok((number.as_u32(), wire))
}

/// One byte of an indexable buffer, so [`super::decode_varint`] can read it.
///
/// `chunk` is at most one byte. The byte at `pos` is loaded before it is
/// observed.
struct IndexBuf<'a, F> {
    get: &'a mut F,
    pos: usize,
    end: usize,
    byte: [u8; 1],
}

impl<'a, F: FnMut(usize) -> Option<u8>> IndexBuf<'a, F> {
    fn new(get: &'a mut F, pos: usize, end: usize) -> Self {
        let mut buf = Self {
            get,
            pos,
            end,
            byte: [0],
        };
        buf.load();
        buf
    }

    fn load(&mut self) {
        if self.pos >= self.end {
            return;
        }
        match (self.get)(self.pos) {
            Some(byte) => self.byte[0] = byte,
            None => self.end = self.pos,
        }
    }
}

impl<F: FnMut(usize) -> Option<u8>> Buf for IndexBuf<'_, F> {
    fn remaining(&self) -> usize {
        self.end.saturating_sub(self.pos)
    }

    fn chunk(&self) -> &[u8] {
        if self.pos < self.end { &self.byte } else { &[] }
    }

    fn advance(&mut self, cnt: usize) {
        self.pos += cnt;
        self.load();
    }
}

fn varint_at(
    len: usize,
    get: &mut impl FnMut(usize) -> Option<u8>,
    start: usize,
) -> Result<Option<(u64, usize)>, DecodeError> {
    let mut buf = IndexBuf::new(get, start, len);
    let available = buf.remaining();
    match super::decode_varint(&mut buf) {
        Ok(value) => Ok(Some((value, available - buf.remaining()))),
        Err(DecodeError::UnexpectedEof) => Ok(None),
        Err(err) => Err(err),
    }
}

fn skip_fixed(len: usize, pos: usize, n: usize) -> Option<usize> {
    let end = pos.checked_add(n)?;
    if end > len { None } else { Some(end) }
}

fn len_end(
    len: usize,
    get: &mut impl FnMut(usize) -> Option<u8>,
    pos: usize,
) -> Result<Option<usize>, DecodeError> {
    let Some((payload_len, prefix_len)) = varint_at(len, get, pos)? else {
        return Ok(None);
    };
    let Ok(payload_len) = usize::try_from(payload_len) else {
        return Err(DecodeError::TruncatedMessage);
    };
    let Some(payload_at) = pos.checked_add(prefix_len) else {
        return Err(DecodeError::TruncatedMessage);
    };
    let Some(end) = payload_at.checked_add(payload_len) else {
        return Err(DecodeError::TruncatedMessage);
    };
    if end > len { Ok(None) } else { Ok(Some(end)) }
}

fn copy_exact<B: Buf>(
    buf: &mut B,
    mut len: usize,
    short: DecodeError,
    emit: &mut impl FnMut(&[u8]),
) -> Result<(), DecodeError> {
    if buf.remaining() < len {
        return Err(short);
    }
    while len > 0 {
        let chunk = buf.chunk();
        if chunk.is_empty() {
            return Err(DecodeError::UnexpectedEof);
        }
        let n = chunk.len().min(len);
        emit(&chunk[..n]);
        buf.advance(n);
        len -= n;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{copy_group, measure_group, skip_group};
    use crate::encode::{encode_tag, encode_varint, encode_varint_field};
    use ::bytes::Buf;
    use ::protobuf_core::{FieldNumber, Varint};
    use ::puroro::{DecodeError, WireType};
    use ::std::vec::Vec;

    fn field(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).expect("test field number")
    }

    fn push_group(buf: &mut Vec<u8>, number: u32, body: &[u8]) {
        encode_tag(field(number), WireType::SGroup, buf);
        buf.extend_from_slice(body);
        encode_tag(field(number), WireType::EGroup, buf);
    }

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

    #[test]
    fn empty_group_is_only_the_end_tag() {
        let mut wire = Vec::new();
        push_group(&mut wire, 1, &[]);
        let frame = measure_group(wire.len(), |i| wire.get(i).copied(), 1, field(1))
            .unwrap()
            .unwrap();
        assert_eq!(frame.body_len, 0);
        assert_eq!(frame.end_tag_len, 1);
        assert_eq!(frame.consumed, 1);

        let mut rest = &wire[1..];
        skip_group(field(1), &mut rest).unwrap();
        assert!(!rest.has_remaining());
    }

    #[test]
    fn nested_same_field_number_closes_in_order() {
        let mut inner = Vec::new();
        encode_varint_field(field(3), Varint::from_uint64(1), &mut inner);
        let mut body = Vec::new();
        push_group(&mut body, 1, &inner);
        let mut wire = Vec::new();
        push_group(&mut wire, 1, &body);
        encode_varint_field(field(4), Varint::from_uint64(9), &mut wire);

        let frame = measure_group(wire.len(), |i| wire.get(i).copied(), 1, field(1))
            .unwrap()
            .unwrap();
        assert_eq!(&wire[1..1 + frame.body_len], body.as_slice());
        let mut rest = &wire[1..];
        skip_group(field(1), &mut rest).unwrap();
        assert_eq!(rest, &wire[1 + frame.consumed..]);
    }

    #[test]
    fn mismatched_inner_end_group_is_an_error() {
        let mut wire = Vec::new();
        encode_tag(field(1), WireType::SGroup, &mut wire);
        encode_tag(field(2), WireType::SGroup, &mut wire);
        encode_tag(field(1), WireType::EGroup, &mut wire);
        let err = measure_group(wire.len(), |i| wire.get(i).copied(), 1, field(1)).unwrap_err();
        assert_eq!(
            err,
            DecodeError::MismatchedEndGroup {
                expected: 2,
                got: 1
            }
        );
        let mut rest = &wire[1..];
        assert_eq!(
            skip_group(field(1), &mut rest).unwrap_err(),
            DecodeError::MismatchedEndGroup {
                expected: 2,
                got: 1
            }
        );
    }

    #[test]
    fn unclosed_group_needs_more_bytes_on_a_slice_and_truncates_on_a_buf() {
        let mut wire = Vec::new();
        encode_tag(field(1), WireType::SGroup, &mut wire);
        encode_varint_field(field(2), Varint::from_uint64(1), &mut wire);
        assert!(
            measure_group(wire.len(), |i| wire.get(i).copied(), 1, field(1))
                .unwrap()
                .is_none()
        );
        let mut rest = &wire[1..];
        assert_eq!(
            skip_group(field(1), &mut rest).unwrap_err(),
            DecodeError::TruncatedMessage
        );
    }

    #[test]
    fn one_hundred_nested_groups_exceed_the_recursion_limit() {
        let mut wire = Vec::new();
        for _ in 0..100 {
            encode_tag(field(1), WireType::SGroup, &mut wire);
        }
        let err = measure_group(wire.len(), |i| wire.get(i).copied(), 0, field(1)).unwrap_err();
        assert_eq!(err, DecodeError::RecursionLimitExceeded);
    }

    #[test]
    fn copy_matches_the_raw_tail_across_one_byte_chunks() {
        let mut body = Vec::new();
        encode_tag(field(2), WireType::Int64, &mut body);
        body.extend_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
        encode_tag(field(3), WireType::Len, &mut body);
        encode_varint(Varint::from_uint64(3), &mut body);
        body.extend_from_slice(b"abc");
        let mut wire = Vec::new();
        push_group(&mut wire, 7, &body);
        wire.push(0x99);

        let mut copied = Vec::new();
        let mut chunked = Chunked {
            rest: &wire[1..],
            max_chunk: 1,
        };
        copy_group(field(7), &mut chunked, |bytes| {
            copied.extend_from_slice(bytes)
        })
        .unwrap();
        let frame = measure_group(wire.len(), |i| wire.get(i).copied(), 1, field(7))
            .unwrap()
            .unwrap();
        assert_eq!(copied, wire[1..1 + frame.consumed]);
        assert_eq!(chunked.rest, &[0x99]);
    }

    #[test]
    fn copy_keeps_a_noncanonical_varint() {
        // Field 2, varint 1, encoded as `81 00` rather than `01`.
        let body = [0x10, 0x81, 0x00];
        let mut wire = Vec::new();
        push_group(&mut wire, 1, &body);
        let mut copied = Vec::new();
        let mut rest = &wire[1..];
        copy_group(field(1), &mut rest, |bytes| copied.extend_from_slice(bytes)).unwrap();
        assert_eq!(copied, wire[1..]);
        assert!(!rest.has_remaining());
    }
}
