//! One scan of a contiguous message body for a borrowing view.
//!
//! Records are applied while the scanner's list is still live. Getters later
//! slice the caller's buffer; they do not walk it again.

use ::bytes::Buf;
use ::core::str;
use ::puroro::DecodeError;
use ::puroro_rt::Varint;
use ::puroro_rt::decode::{RecordScanner, ScannedRecord, ScannedValue, WireSpan, decode_varint};
use ::std::vec::Vec;

/// Scans `body` once. `origin` is `body`'s offset in the root buffer the view
/// borrows, so a nested LEN stores spans into that root.
pub(crate) fn for_each_record(
    body: &[u8],
    mut f: impl FnMut(&ScannedRecord) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    let mut scanner = RecordScanner::new();
    let records = scanner.push(body)?;
    scanner.finish()?;
    for rec in &records {
        f(rec)?;
    }
    Ok(())
}

pub(crate) fn len_span(origin: usize, rec: &ScannedRecord) -> Result<WireSpan, DecodeError> {
    rec.len_span(origin)
}

pub(crate) fn varint_i32(value: &ScannedValue) -> Result<i32, DecodeError> {
    match *value {
        ScannedValue::Varint(raw) => raw.try_to_int32().map_err(DecodeError::from),
        _ => Err(DecodeError::InvalidTag),
    }
}

pub(crate) fn varint_bool(value: &ScannedValue) -> Result<bool, DecodeError> {
    match *value {
        ScannedValue::Varint(raw) => Ok(raw.to_bool()),
        _ => Err(DecodeError::InvalidTag),
    }
}

pub(crate) fn fixed32(value: &ScannedValue) -> Result<u32, DecodeError> {
    match *value {
        ScannedValue::I32(bytes) => Ok(u32::from_le_bytes(bytes)),
        _ => Err(DecodeError::InvalidTag),
    }
}

pub(crate) fn fixed64_f64(value: &ScannedValue) -> Result<f64, DecodeError> {
    match *value {
        ScannedValue::I64(bytes) => Ok(f64::from_le_bytes(bytes)),
        _ => Err(DecodeError::InvalidTag),
    }
}

/// Appends every `int32` in a packed LEN. `span` is absolute in `root`.
pub(crate) fn extend_packed_i32(
    root: &[u8],
    span: WireSpan,
    out: &mut Vec<i32>,
) -> Result<(), DecodeError> {
    let mut rest = span.slice(root)?;
    while rest.has_remaining() {
        let raw = Varint::from_uint64(decode_varint(&mut rest)?);
        out.push(raw.try_to_int32().map_err(DecodeError::from)?);
    }
    Ok(())
}

/// Appends every `bool` in a packed LEN. `span` is absolute in `root`.
pub(crate) fn extend_packed_bool(
    root: &[u8],
    span: WireSpan,
    out: &mut Vec<bool>,
) -> Result<(), DecodeError> {
    let mut rest = span.slice(root)?;
    while rest.has_remaining() {
        let raw = Varint::from_uint64(decode_varint(&mut rest)?);
        out.push(raw.to_bool());
    }
    Ok(())
}

pub(crate) fn push_i32(
    root: &[u8],
    origin: usize,
    rec: &ScannedRecord,
    out: &mut Vec<i32>,
) -> Result<(), DecodeError> {
    match rec.value {
        ScannedValue::Varint(_) => out.push(varint_i32(&rec.value)?),
        ScannedValue::Len { .. } => extend_packed_i32(root, len_span(origin, rec)?, out)?,
        _ => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

pub(crate) fn push_bool(
    root: &[u8],
    origin: usize,
    rec: &ScannedRecord,
    out: &mut Vec<bool>,
) -> Result<(), DecodeError> {
    match rec.value {
        ScannedValue::Varint(_) => out.push(varint_bool(&rec.value)?),
        ScannedValue::Len { .. } => extend_packed_bool(root, len_span(origin, rec)?, out)?,
        _ => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

/// UTF-8 of one stored span. Absence is `Ok(None)`. A bad payload is
/// [`DecodeError::InvalidUtf8`] here, not at scan time.
pub(crate) fn utf8(root: &[u8], span: Option<WireSpan>) -> Result<Option<&str>, DecodeError> {
    let Some(span) = span else {
        return Ok(None);
    };
    str::from_utf8(span.slice(root)?)
        .map(Some)
        .map_err(|_| DecodeError::InvalidUtf8)
}
