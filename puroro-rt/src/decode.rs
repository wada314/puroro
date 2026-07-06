//! Wire-format decoding helpers used by generated field types.
//!
//! Varint and tag decoding delegate to [`protobuf_core`] (`Varint`, `Tag`,
//! [`IteratorExtVarint`]). `bytes::Buf` adapters live here.

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::protobuf_core::{IteratorExtVarint, Tag, Varint};
use ::puroro::{DecodeError, WireType};
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

/// Decodes one LEN payload into an allocator-less [`UnmanagedVec<u8>`].
pub fn decode_bytes_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<UnmanagedVec<u8>, DecodeError> {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    let mut vec = ::allocator_api2::vec::Vec::<u8, A>::with_capacity_in(len, alloc);
    let mut remaining = len;
    while remaining > 0 {
        let chunk = buf.chunk();
        let to_copy = chunk.len().min(remaining);
        vec.extend_from_slice(&chunk[..to_copy]);
        buf.advance(to_copy);
        remaining -= to_copy;
    }
    Ok(UnmanagedVec::from_vec(vec))
}

/// Decodes one LEN payload as UTF-8 into an allocator-less [`UnmanagedString`].
pub fn decode_string_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<UnmanagedString, DecodeError> {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    let bytes = buf.copy_to_bytes(len);
    let s = ::core::str::from_utf8(&bytes).map_err(|_| DecodeError::InvalidUtf8)?;
    Ok(str_to_unmanaged_in(s, alloc))
}

/// Copies `s` into a freshly allocated [`UnmanagedString`] backed by the owned
/// `alloc` (its buffer is owned by allocator type `A`).
pub fn str_to_unmanaged_in<A: Allocator>(s: &str, alloc: A) -> UnmanagedString {
    UnmanagedString::from_string(::unmanaged::String::from_str_in(s, alloc))
}

/// Copies `v` into a freshly allocated [`UnmanagedVec<u8>`] backed by the owned
/// `alloc` (its buffer is owned by allocator type `A`).
pub fn bytes_to_unmanaged_in<A: Allocator>(v: &[u8], alloc: A) -> UnmanagedVec<u8> {
    let mut vec = ::allocator_api2::vec::Vec::<u8, A>::with_capacity_in(v.len(), alloc);
    vec.extend_from_slice(v);
    UnmanagedVec::from_vec(vec)
}

pub fn skip_field_and_save<B: Buf, A: Allocator>(
    field_number: u32,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut UnmanagedVec<u8>,
    alloc: A,
) -> Result<(), DecodeError> {
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { unknown_fields.with_alloc(alloc) };
    let tag = encode::tag_to_u64_for_unknown(field_number, wire_type);
    encode::write_varint_to_vec(tag, &mut *g);

    match wire_type {
        WireType::Varint => {
            let v = decode_varint(buf)?;
            encode::write_varint_to_vec(v, &mut *g);
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
            encode::write_varint_to_vec(len, &mut *g);
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

pub fn save_unknown_varint_field<A: Allocator>(
    field_number: u32,
    value: u64,
    unknown_fields: &mut UnmanagedVec<u8>,
    alloc: A,
) {
    // SAFETY: the owned `alloc` (an `alloc.clone()` from the caller) is
    // interchangeable with the allocator that owns this vector's buffer.
    let mut g = unsafe { unknown_fields.with_alloc(alloc) };
    let tag = encode::tag_to_u64_for_unknown(field_number, WireType::Varint);
    encode::write_varint_to_vec(tag, &mut *g);
    encode::write_varint_to_vec(value, &mut *g);
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
