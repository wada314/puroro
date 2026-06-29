//! Wire-format decoding helpers and the [`MessageDecode`] trait.
//!
//! # Design note
//!
//! The primary operation is **merging**: `merge_from` reads fields from the
//! wire and merges them into an existing (possibly default) message. A full
//! `decode` is simply `Default::default()` followed by `merge_from`. The
//! separation makes it easy to implement proto3's "last one wins" semantics for
//! duplicate singular fields and "concatenate" semantics for repeated fields.

use ::bytes::Buf;
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box as ABox;
use ::allocator_api2::vec::Vec as AVec;
use crate::error::DecodeError;
use crate::wire_type::WireType;

// ---------------------------------------------------------------------------
// Core trait
// ---------------------------------------------------------------------------

/// Implemented by every generated message type.
///
/// Like [`MessageEncode`](crate::encode::MessageEncode), this trait is **not**
/// object-safe due to the generic `B` parameter on `merge_from`.
pub trait MessageDecode: Sized {
    /// Reads fields from `buf` and merges them into `self`.
    ///
    /// - Singular scalar fields: the last value seen wins.
    /// - Singular message fields: values are recursively merged.
    /// - Repeated fields: each occurrence appends to the list.
    /// - Unknown fields are stored verbatim for round-trip preservation.
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    /// Decodes a complete message from `buf`.
    ///
    /// This is a convenience wrapper available when `Self: Default`. It creates
    /// a default instance and merges the buffer into it.
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
// Tag / varint decoding
// ---------------------------------------------------------------------------

/// Decodes a base-128 varint from `buf`, consuming exactly as many bytes as
/// the encoding uses.
///
/// Returns [`DecodeError::UnexpectedEof`] if the buffer is exhausted before
/// the varint terminates, and [`DecodeError::InvalidVarint`] if the varint
/// exceeds 10 bytes (which would overflow a `u64`).
pub fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof);
        }
        let byte = buf.get_u8();
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 70 {
            return Err(DecodeError::InvalidVarint);
        }
    }
}

/// Decodes a tag and returns `(field_number, wire_type)`.
pub fn decode_tag<B: Buf>(buf: &mut B) -> Result<(u32, WireType), DecodeError> {
    let raw = decode_varint(buf)?;
    let wire_type = WireType::from_raw((raw & 0x07) as u8)?;
    let field_number = (raw >> 3) as u32;
    Ok((field_number, wire_type))
}

// ZigZag decode helpers for sint32/sint64.

/// ZigZag-decodes a 32-bit wire value back to a signed integer.
#[inline]
pub fn unzigzag32(v: u64) -> i32 {
    let v = v as u32;
    ((v >> 1) as i32) ^ -((v & 1) as i32)
}

/// ZigZag-decodes a 64-bit wire value back to a signed integer.
#[inline]
pub fn unzigzag64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -((v & 1) as i64)
}

// ---------------------------------------------------------------------------
// LEN-payload helpers
// ---------------------------------------------------------------------------

/// Decodes a length-prefixed byte slice from `buf`, returning a freshly
/// allocated `Vec<u8, A>`.
///
/// Used for `bytes` fields and as the inner primitive for `decode_string_in`
/// and embedded message decoding.
pub fn decode_bytes_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<AVec<u8, A>, DecodeError> {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    let mut v = AVec::<u8, A>::with_capacity_in(len, alloc);
    // Iterate over Buf chunks to avoid an intermediate allocation.
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

/// Decodes a length-prefixed UTF-8 string from `buf`.
///
/// Returns a `Box<str, A>` to avoid the overhead of `String`'s capacity word
/// while still owning the data.
pub fn decode_string_in<B: Buf, A: Allocator>(
    buf: &mut B,
    alloc: A,
) -> Result<ABox<str, A>, DecodeError> {
    let v = decode_bytes_in(buf, alloc)?;
    // Validate UTF-8 before committing the allocation.
    ::core::str::from_utf8(&v).map_err(|_| DecodeError::InvalidUtf8)?;
    Ok(bytes_vec_into_str_box(v))
}

/// Converts `Vec<u8, A>` whose contents are valid UTF-8 into `Box<str, A>`.
///
/// # Safety
/// The caller must ensure `v` contains valid UTF-8.
fn bytes_vec_into_str_box<A: Allocator>(v: AVec<u8, A>) -> ABox<str, A> {
    let boxed_bytes: ABox<[u8], A> = v.into_boxed_slice();
    let (ptr, alloc) = ABox::into_raw_with_allocator(boxed_bytes);
    // SAFETY: We verified UTF-8 validity at the call site; [u8] and str share
    // the same fat-pointer representation (ptr + byte length).
    unsafe { ABox::from_raw_in(ptr as *mut str, alloc) }
}

/// Creates a `Box<str, A>` by copying from a `&str`.
///
/// This is a convenience helper for setters that accept `&str`.
pub fn str_to_box_in<A: Allocator>(s: &str, alloc: A) -> ABox<str, A> {
    let mut v = AVec::<u8, A>::with_capacity_in(s.len(), alloc);
    v.extend_from_slice(s.as_bytes());
    bytes_vec_into_str_box(v)
}

// ---------------------------------------------------------------------------
// Unknown-field handling
// ---------------------------------------------------------------------------

/// Reads one unknown field's payload from `buf` and appends both tag and
/// payload verbatim to `unknown_fields`, preserving round-trip fidelity.
///
/// On encountering deprecated group tags (`SGroup`/`EGroup`) the function
/// returns an error rather than attempting to parse them.
pub fn skip_field_and_save<B: Buf, A: Allocator>(
    field_number: u32,
    wire_type: WireType,
    buf: &mut B,
    unknown_fields: &mut AVec<u8, A>,
) -> Result<(), DecodeError> {
    // Re-serialise the tag so the unknown bytes remain a valid protobuf stream.
    let tag = ((field_number as u64) << 3) | (wire_type as u64);
    crate::encode::write_varint_to_vec(tag, unknown_fields);

    match wire_type {
        WireType::Varint => {
            let v = decode_varint(buf)?;
            crate::encode::write_varint_to_vec(v, unknown_fields);
        }
        WireType::I64 => {
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
            crate::encode::write_varint_to_vec(len, unknown_fields);
            let bytes = buf.copy_to_bytes(len as usize);
            unknown_fields.extend_from_slice(&bytes);
        }
        WireType::I32 => {
            if buf.remaining() < 4 {
                return Err(DecodeError::UnexpectedEof);
            }
            let bytes = buf.copy_to_bytes(4);
            unknown_fields.extend_from_slice(&bytes);
        }
        WireType::SGroup | WireType::EGroup => {
            // Deprecated; we do not attempt to recurse into group payloads.
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

/// Saves a VARINT field into an unknown-fields buffer (e.g. closed enum unknown variant).
pub fn save_unknown_varint_field<A: Allocator>(
    field_number: u32,
    value: u64,
    unknown_fields: &mut AVec<u8, A>,
) {
    let tag = ((field_number as u64) << 3) | (WireType::Varint as u64);
    crate::encode::write_varint_to_vec(tag, unknown_fields);
    crate::encode::write_varint_to_vec(value, unknown_fields);
}

/// Skips (and discards) one unknown field's payload without saving it.
///
/// Used by message types that have unknown-field preservation disabled.
pub fn skip_field<B: Buf>(wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
    match wire_type {
        WireType::Varint => {
            decode_varint(buf)?;
        }
        WireType::I64 => {
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
        WireType::I32 => {
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
