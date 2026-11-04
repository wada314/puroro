//! Adapt [`protobuf_core`] field iterators to catalog merges without re-encoding.
//!
//! Contiguous eager decode walks [`AsRefExtProtobuf`] fields and feeds
//! [`FieldValue`]s into `merge_parsed` on catalog views. Unknowns are saved
//! from the parsed [`Field`] directly.

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::protobuf_core::{Field, FieldValue, Varint};
use ::puroro::DecodeError;

use crate::decode::ScannedValue;
use crate::encode;
use crate::unknown_fields::UnknownStore;
use ::puroro::WireType;

/// Map a parsed scalar [`FieldValue`] into a [`ScannedValue`] for numeric merges.
#[inline]
pub fn field_value_to_scanned(value: &FieldValue<&[u8]>) -> Result<ScannedValue, DecodeError> {
    match *value {
        FieldValue::Varint(v) => Ok(ScannedValue::Varint(v)),
        FieldValue::I32(bytes) => Ok(ScannedValue::I32(bytes)),
        FieldValue::I64(bytes) => Ok(ScannedValue::I64(bytes)),
        FieldValue::Len(_) | FieldValue::SGroup | FieldValue::EGroup => {
            Err(DecodeError::InvalidTag)
        }
    }
}

/// Append a parsed field into an unknown-field store (preserve / discard / reject).
///
/// Mirrors [`super::skip_field_and_save_in`] without reading from a live buffer.
pub fn save_parsed_field_in<A: Allocator + Clone, U: UnknownStore<A>>(
    field: &Field<&[u8]>,
    unknown_fields: &mut U,
    alloc: A,
) -> Result<(), DecodeError> {
    unknown_fields.on_unrecognized_tag(field.field_number.as_u32())?;
    let Some(blob) = unknown_fields.append_blob_mut(alloc.clone()) else {
        return Ok(());
    };
    // SAFETY: `alloc` matches the vector's allocator.
    let mut g = unsafe { blob.with_alloc(alloc) };
    match &field.value {
        FieldValue::Varint(v) => {
            encode::write_tag_to_vec(field.field_number, WireType::Varint, &mut *g);
            encode::write_varint_to_vec(*v, &mut *g);
        }
        FieldValue::I32(bytes) => {
            encode::write_tag_to_vec(field.field_number, WireType::Int32, &mut *g);
            g.extend_from_slice(bytes);
        }
        FieldValue::I64(bytes) => {
            encode::write_tag_to_vec(field.field_number, WireType::Int64, &mut *g);
            g.extend_from_slice(bytes);
        }
        FieldValue::Len(payload) => {
            encode::write_tag_to_vec(field.field_number, WireType::Len, &mut *g);
            encode::write_varint_to_vec(Varint::from_uint64(payload.len() as u64), &mut *g);
            g.extend_from_slice(payload);
        }
        FieldValue::SGroup | FieldValue::EGroup => {
            return Err(DecodeError::InvalidTag);
        }
    }
    Ok(())
}

/// When `buf`'s remaining bytes are one contiguous chunk, walk them with
/// [`protobuf_core::AsRefExtProtobuf::read_protobuf_fields`].
///
/// Returns `Ok(true)` if this path ran (including `rem == 0`). Returns
/// `Ok(false)` when remaining bytes span multiple [`Buf::chunk`]s — the caller
/// should use the legacy tag loop. On success the cursor advances by `rem`.
pub fn try_for_each_contiguous_field<B, F>(
    buf: &mut B,
    mut on_field: F,
) -> Result<bool, DecodeError>
where
    B: Buf,
    F: FnMut(&Field<&[u8]>) -> Result<(), DecodeError>,
{
    let rem = buf.remaining();
    if rem == 0 {
        return Ok(true);
    }
    if buf.chunk().len() < rem {
        return Ok(false);
    }
    let slice = &buf.chunk()[..rem];
    for item in ::protobuf_core::AsRefExtProtobuf::read_protobuf_fields(slice) {
        on_field(&item?)?;
    }
    buf.advance(rem);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::protobuf_core::FieldValue;

    #[test]
    fn field_value_to_scanned_varint() {
        let v = Varint::from_uint64(150);
        assert_eq!(
            field_value_to_scanned(&FieldValue::Varint(v)).unwrap(),
            ScannedValue::Varint(v)
        );
    }

    #[test]
    fn len_is_not_a_scanned_scalar() {
        assert!(matches!(
            field_value_to_scanned(&FieldValue::Len(b"hi")),
            Err(DecodeError::InvalidTag)
        ));
    }
}
