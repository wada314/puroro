//! Wire-format decoding helpers and the [`MessageDecode`] trait.
//!
//! Varint and tag decoding delegate to [`protobuf_core`] (`Varint`, `Tag`,
//! [`IteratorExtVarint`]). `bytes::Buf` adapters live here.

use ::bytes::Buf;
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box as ABox;
use ::allocator_api2::vec::Vec as AVec;
use ::protobuf_core::{IteratorExtVarint, Tag, Varint};

use crate::encode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// Implemented by every generated message type.
pub trait MessageDecode: Sized {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut msg = Self::default();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }
}

// ---------------------------------------------------------------------------
// Buf → byte iterator (for protobuf-core varint readers)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Tag / varint decoding (protobuf-core)
// ---------------------------------------------------------------------------

/// Decodes a base-128 varint from `buf`.
pub fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    match (BufVarintReader { buf }).read_varint()? {
        Some(v) => Ok(v.to_uint64()),
        None => Err(DecodeError::UnexpectedEof),
    }
}

/// Decodes a tag and returns `(field_number, wire_type)`.
pub fn decode_tag<B: Buf>(buf: &mut B) -> Result<(u32, WireType), DecodeError> {
    let raw = decode_varint(buf)?;
    let tag = Tag::try_from(Varint::from_uint64(raw))?;
    Ok((u32::from(tag.field_number), tag.wire_type))
}

// ---------------------------------------------------------------------------
// LEN-payload helpers
// ---------------------------------------------------------------------------

pub fn decode_bytes_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<AVec<u8, A>, DecodeError> {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    let mut v = AVec::<u8, A>::with_capacity_in(len, alloc);
    let mut remaining = len;
    while remaining > 0 {
        let chunk = buf.chunk();
        let to_copy = chunk.len().min(remaining);
        v.extend_from_slice(&chunk[..to_copy]);
        buf.advance(to_copy);
        remaining -= to_copy;
    }
    Ok(v)
}

pub fn decode_string_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<ABox<str, A>, DecodeError> {
    let v = decode_bytes_in(buf, alloc)?;
    ::core::str::from_utf8(&v).map_err(|_| DecodeError::InvalidUtf8)?;
    Ok(bytes_vec_into_str_box(v))
}

fn bytes_vec_into_str_box<A: Allocator>(v: AVec<u8, A>) -> ABox<str, A> {
    let boxed_bytes: ABox<[u8], A> = v.into_boxed_slice();
    let (ptr, alloc) = ABox::into_raw_with_allocator(boxed_bytes);
    // SAFETY: UTF-8 validated at the call site.
    unsafe { ABox::from_raw_in(ptr as *mut str, alloc) }
}

pub fn str_to_box_in<A: Allocator>(s: &str, alloc: A) -> ABox<str, A> {
    let mut v = AVec::<u8, A>::with_capacity_in(s.len(), alloc);
    v.extend_from_slice(s.as_bytes());
    bytes_vec_into_str_box(v)
}

// ---------------------------------------------------------------------------
// Unknown-field handling
// ---------------------------------------------------------------------------

pub fn skip_field_and_save<B: Buf, A: Allocator>(
    field_number: u32,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut AVec<u8, A>,
) -> Result<(), DecodeError> {
    let tag = encode::tag_to_u64_for_unknown(field_number, wire_type);
    encode::write_varint_to_vec(tag, unknown_fields);

    match wire_type {
        WireType::Varint => {
            let v = decode_varint(buf)?;
            encode::write_varint_to_vec(v, unknown_fields);
        }
        WireType::Int64 => {
            if buf.remaining() < 8 {
                return Err(DecodeError::UnexpectedEof);
            }
            let bytes = buf.copy_to_bytes(8);
            unknown_fields.extend_from_slice(&bytes);
        }
        WireType::Len => {
            let len = decode_varint(buf)?;
            if buf.remaining() < len as usize {
                return Err(DecodeError::TruncatedMessage);
            }
            encode::write_varint_to_vec(len, unknown_fields);
            let bytes = buf.copy_to_bytes(len as usize);
            unknown_fields.extend_from_slice(&bytes);
        }
        WireType::Int32 => {
            if buf.remaining() < 4 {
                return Err(DecodeError::UnexpectedEof);
            }
            let bytes = buf.copy_to_bytes(4);
            unknown_fields.extend_from_slice(&bytes);
        }
        WireType::SGroup | WireType::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

pub fn save_unknown_varint_field<A: Allocator>(
    field_number: u32,
    value: u64,
    unknown_fields: &mut AVec<u8, A>,
) {
    let tag = encode::tag_to_u64_for_unknown(field_number, WireType::Varint);
    encode::write_varint_to_vec(tag, unknown_fields);
    encode::write_varint_to_vec(value, unknown_fields);
}

pub fn skip_field<B: Buf>(wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
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
