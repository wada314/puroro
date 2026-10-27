//! Wire-format decoding helpers used by generated field types.
//!
//! Varint decoding is pluggable via [`VarintDecoder`]. [`decode_varint`] uses
//! [`ChunkScanHot4`]; [`decode_tag`] uses [`decode_varint_with`] with
//! [`ChunkScanLikely1`]. Tags are unpacked here into a validated [`FieldNumber`]
//! plus [`WireType`].

mod lazy_scan;
mod record;
mod shared_wire;
mod varint;

pub use self::lazy_scan::{Eager, Lazy, LazyMessage, LazyScan, MessageScan};
pub use self::record::{
    RecordScanner, ScannedRecord, ScannedValue, WireSpan, merge_scanned_field, scanned_group_span,
    scanned_len_payload, scanned_len_span,
};
pub use self::shared_wire::SharedWire;
pub use self::varint::{
    ByteIterator, BytePair, ByteQuad, ChunkScan, ChunkScan1, ChunkScan2, ChunkScan4, ChunkScanHot2,
    ChunkScanHot4, ChunkScanHot8, ChunkScanHot10, ChunkScanHotAdapt, ChunkScanLikely1,
    ChunkScanMsb, VarintDecoder,
};

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::Buf;
use ::protobuf_core::{FieldNumber, Varint};
use ::puroro::wire_type;
use ::puroro::{DecodeError, UnknownField, UnknownPayload, WireType};
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use crate::unknown_fields::{UnknownFields, UnknownStore};

use crate::encode;

/// Decodes a base-128 varint from `buf` with [`ChunkScanHot4`].
#[inline]
pub fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    decode_varint_with::<ChunkScanHot4, _>(buf)
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

/// Skips one field payload for `wire_type` without preserving unknowns.
///
/// Used for map-entry interiors (and similar ephemeral messages) where unknown
/// fields are not round-tripped. After a [`WireType::SGroup`] tag,
/// `field_number` is the start-group field number; the matching end-group is
/// consumed. A lone [`WireType::EGroup`] is [`DecodeError::InvalidTag`].
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
        WireType::SGroup => skip_group_body(field_number, buf)?,
        WireType::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

/// After an `SGroup` tag for `field` was consumed, discard nested fields until
/// the matching `EGroup` (nested groups allowed).
fn skip_group_body<B: Buf>(field: FieldNumber, buf: &mut B) -> Result<(), DecodeError> {
    take_group_body(field, buf, &mut DiscardGroupBody)
}

/// After an `SGroup` tag for `field` was consumed, copy the group **body**
/// (bytes between start and end tags; neither tag included) into `out`, and
/// consume the matching `EGroup`. Nested groups are copied intact.
pub fn take_group_body<B: Buf, O: GroupBodyOut>(
    field: FieldNumber,
    buf: &mut B,
    out: &mut O,
) -> Result<(), DecodeError> {
    let mut stack = ::std::vec![field];
    loop {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof);
        }
        let (num, wt) = decode_tag(buf)?;
        match wt {
            WireType::EGroup => {
                let Some(&expected) = stack.last() else {
                    return Err(DecodeError::InvalidTag);
                };
                if num != expected {
                    return Err(DecodeError::InvalidTag);
                }
                stack.pop();
                if stack.is_empty() {
                    return Ok(());
                }
                out.write_tag(num, WireType::EGroup);
            }
            WireType::SGroup => {
                stack.push(num);
                out.write_tag(num, WireType::SGroup);
            }
            WireType::Varint => {
                out.write_tag(num, WireType::Varint);
                let v = decode_varint(buf)?;
                out.write_varint(v);
            }
            WireType::Int64 => {
                out.write_tag(num, WireType::Int64);
                if buf.remaining() < 8 {
                    return Err(DecodeError::UnexpectedEof);
                }
                let bytes = buf.copy_to_bytes(8);
                out.write_bytes(&bytes);
            }
            WireType::Int32 => {
                out.write_tag(num, WireType::Int32);
                if buf.remaining() < 4 {
                    return Err(DecodeError::UnexpectedEof);
                }
                let bytes = buf.copy_to_bytes(4);
                out.write_bytes(&bytes);
            }
            WireType::Len => {
                out.write_tag(num, WireType::Len);
                let len = decode_varint(buf)?;
                if buf.remaining() < len as usize {
                    return Err(DecodeError::TruncatedMessage);
                }
                out.write_varint(len);
                let bytes = buf.copy_to_bytes(len as usize);
                out.write_bytes(&bytes);
            }
        }
    }
}

/// Sink for [`take_group_body`] (copy or discard).
pub trait GroupBodyOut {
    fn write_tag(&mut self, field: FieldNumber, wire_type: WireType);
    fn write_varint(&mut self, value: u64);
    fn write_bytes(&mut self, bytes: &[u8]);
}

struct DiscardGroupBody;

impl GroupBodyOut for DiscardGroupBody {
    #[inline]
    fn write_tag(&mut self, _field: FieldNumber, _wire_type: WireType) {}
    #[inline]
    fn write_varint(&mut self, _value: u64) {}
    #[inline]
    fn write_bytes(&mut self, _bytes: &[u8]) {}
}

impl<A: Allocator> GroupBodyOut for AllocVec<u8, A> {
    #[inline]
    fn write_tag(&mut self, field: FieldNumber, wire_type: WireType) {
        encode::write_tag_to_vec(field, wire_type, self);
    }
    #[inline]
    fn write_varint(&mut self, value: u64) {
        encode::write_varint_to_vec(Varint::from_uint64(value), self);
    }
    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
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
            // Start tag already written; append body then matching end tag.
            take_group_body(field_number, buf, &mut *g)?;
            encode::write_tag_to_vec(field_number, WireType::EGroup, &mut *g);
        }
        WireType::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
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
            WireType::SGroup => {
                let field = match FieldNumber::try_new(number) {
                    Ok(f) => f,
                    Err(_) => {
                        debug_assert!(false, "corrupt unknown-fields blob: bad group number");
                        self.rest = &[];
                        return None;
                    }
                };
                let body = match group_body_slice(field, &mut buf) {
                    Ok(b) => b,
                    Err(_) => {
                        debug_assert!(false, "corrupt unknown-fields blob: bad group");
                        self.rest = &[];
                        return None;
                    }
                };
                UnknownPayload::Group(body)
            }
            WireType::EGroup => {
                debug_assert!(false, "orphan end-group in unknown-fields blob");
                self.rest = &[];
                return None;
            }
        };

        self.rest = buf;
        Some(UnknownField::new(number, payload))
    }
}

/// Advances `buf` past a group body started with `field`, returning the body
/// subslice (excluding start/end tags). `buf` must be positioned just after
/// the start-group tag; on success it points just after the matching end tag.
fn group_body_slice<'a>(field: FieldNumber, buf: &mut &'a [u8]) -> Result<&'a [u8], DecodeError> {
    let start = *buf;
    let mut stack = ::std::vec![field];
    loop {
        if buf.is_empty() {
            return Err(DecodeError::UnexpectedEof);
        }
        let before_tag = *buf;
        let (num, wt) = decode_tag(buf)?;
        match wt {
            WireType::EGroup => {
                let Some(&expected) = stack.last() else {
                    return Err(DecodeError::InvalidTag);
                };
                if num != expected {
                    return Err(DecodeError::InvalidTag);
                }
                stack.pop();
                if stack.is_empty() {
                    let body_len = start.len() - before_tag.len();
                    return Ok(&start[..body_len]);
                }
            }
            WireType::SGroup => {
                stack.push(num);
            }
            WireType::Varint => {
                decode_varint(buf)?;
            }
            WireType::Int64 => {
                if buf.len() < 8 {
                    return Err(DecodeError::UnexpectedEof);
                }
                *buf = &buf[8..];
            }
            WireType::Int32 => {
                if buf.len() < 4 {
                    return Err(DecodeError::UnexpectedEof);
                }
                *buf = &buf[4..];
            }
            WireType::Len => {
                let len = decode_varint(buf)? as usize;
                if buf.len() < len {
                    return Err(DecodeError::TruncatedMessage);
                }
                *buf = &buf[len..];
            }
        }
    }
}

#[cfg(test)]
mod group_tests {
    use ::allocator_api2::alloc::Global;
    use ::bytes::Bytes;
    use ::core::ops::Deref;
    use ::protobuf_core::FieldNumber;

    use super::{
        AllocVec, decode_tag, iter_unknown_fields, skip_field, skip_field_and_save, take_group_body,
    };
    use crate::encode;
    use crate::unknown_fields::UnknownFields;
    use ::puroro::{DecodeError, UnknownPayload, WireType};
    use ::unmanaged::DeallocateIn;

    fn field(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).unwrap()
    }

    fn empty_group_then_varint() -> Vec<u8> {
        // field 1 SGroup, field 1 EGroup, field 2 = 7
        let mut v = Vec::new();
        encode::encode_tag(field(1), WireType::SGroup, &mut v);
        encode::encode_tag(field(1), WireType::EGroup, &mut v);
        encode::encode_tag(field(2), WireType::Varint, &mut v);
        encode::encode_varint(::protobuf_core::Varint::from_uint64(7), &mut v);
        v
    }

    fn group_with_varint_body() -> Vec<u8> {
        // field 1 SGroup, field 2 = 150, field 1 EGroup
        let mut v = Vec::new();
        encode::encode_tag(field(1), WireType::SGroup, &mut v);
        encode::encode_tag(field(2), WireType::Varint, &mut v);
        encode::encode_varint(::protobuf_core::Varint::from_uint64(150), &mut v);
        encode::encode_tag(field(1), WireType::EGroup, &mut v);
        v
    }

    #[test]
    fn skip_empty_group_then_next_field() {
        let bytes = empty_group_then_varint();
        let mut buf = bytes.as_slice();
        let (num, wt) = decode_tag(&mut buf).unwrap();
        assert_eq!(num.as_u32(), 1);
        assert_eq!(wt, WireType::SGroup);
        skip_field(num, wt, &mut buf).unwrap();
        let (num, wt) = decode_tag(&mut buf).unwrap();
        assert_eq!(num.as_u32(), 2);
        assert_eq!(wt, WireType::Varint);
    }

    #[test]
    fn take_group_body_copies_inner_only() {
        let bytes = group_with_varint_body();
        let mut buf = bytes.as_slice();
        let (num, wt) = decode_tag(&mut buf).unwrap();
        assert_eq!(wt, WireType::SGroup);
        let mut body = AllocVec::new_in(Global);
        take_group_body(num, &mut buf, &mut body).unwrap();
        assert!(buf.is_empty());
        // body is field 2 = 150 only
        let mut expected = AllocVec::new_in(Global);
        encode::write_tag_to_vec(field(2), WireType::Varint, &mut expected);
        encode::write_varint_to_vec(::protobuf_core::Varint::from_uint64(150), &mut expected);
        assert_eq!(body.as_slice(), expected.as_slice());
    }

    #[test]
    fn mismatched_end_group_is_invalid_tag() {
        // field 1 SGroup, field 2 EGroup
        let mut bytes = Vec::new();
        encode::encode_tag(field(1), WireType::SGroup, &mut bytes);
        encode::encode_tag(field(2), WireType::EGroup, &mut bytes);
        let mut buf = bytes.as_slice();
        let (num, wt) = decode_tag(&mut buf).unwrap();
        assert_eq!(skip_field(num, wt, &mut buf), Err(DecodeError::InvalidTag));
    }

    #[test]
    fn preserve_and_iter_unknown_group() {
        let bytes = group_with_varint_body();
        let mut buf = Bytes::from(bytes);
        let (num, wt) = decode_tag(&mut buf).unwrap();
        let mut unknowns = UnknownFields::<Global>::default();
        skip_field_and_save(num, wt, &mut buf, &mut unknowns, Global).unwrap();

        let blob = unknowns.deref().to_vec();
        let fields: Vec<_> = iter_unknown_fields(&blob).collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].number(), 1);
        assert_eq!(fields[0].wire_type(), WireType::SGroup);
        match fields[0].payload() {
            UnknownPayload::Group(body) => {
                let mut expected = AllocVec::new_in(Global);
                encode::write_tag_to_vec(field(2), WireType::Varint, &mut expected);
                encode::write_varint_to_vec(
                    ::protobuf_core::Varint::from_uint64(150),
                    &mut expected,
                );
                assert_eq!(*body, expected.as_slice());
            }
            other => panic!("expected Group, got {other:?}"),
        }
        // SAFETY: Global owns this store's buffer.
        unsafe { unknowns.deallocate_in(&Global) };
    }
}
