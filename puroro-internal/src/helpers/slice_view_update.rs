use super::DoDefaultCheck;
use crate::deser::BytesSlice;
use crate::tags::FieldTypeTag;
use crate::tags::{self, FieldLabelTag};
use crate::types::{FieldData, SliceViewRepeatedField, SliceViewScalarField};
use crate::variant::Variant;
use crate::{ErrorKind, Result};

pub trait SliceViewUpdate<'slice, TypeTag, LabelTag>
where
    TypeTag: FieldTypeTag,
    LabelTag: FieldLabelTag,
{
    fn update(&mut self, field: FieldData<&'slice [u8]>) -> Result<()>;
}

impl<'slice> SliceViewUpdate<'slice, tags::Int32, tags::Required>
    for SliceViewScalarField<'slice, i32>
{
    fn update(&mut self, field: FieldData<&'slice [u8]>) -> Result<()> {
        // Even if the field exists, if the value is 0 and the field is proto3's
        // singular field then we should not update the value.
        let maybe_new_val = match field {
            FieldData::Variant(variant) => {
                if !tags::Required::DO_DEFAULT_CHECK || !variant.is_zero() {
                    Some(variant.to_native::<tags::Int32>()?)
                } else {
                    None
                }
            }
            FieldData::LengthDelimited(slice) => {
                let mut last_val = None;
                let mut variants = BytesSlice::new(slice).variants().peekable();
                if let None = variants.peek() {
                    Err(ErrorKind::ZeroLengthPackedField)?
                }
                for rvariant in variants {
                    let variant = rvariant?;
                    if !tags::Required::DO_DEFAULT_CHECK || !variant.is_zero() {
                        last_val = Some(variant.to_native::<tags::Int32>()?);
                    }
                }
                last_val
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        };
        if let Some(new_val) = maybe_new_val {
            match self {
                SliceViewScalarField::NotFound | SliceViewScalarField::ValueAvailable(_) => {
                    *self = SliceViewScalarField::ValueAvailable(new_val);
                }
                _ => Err(ErrorKind::InvalidSliceViewType)?,
            }
        }
        Ok(())
    }
}

impl<'slice> SliceViewUpdate<'slice, tags::String, tags::Required>
    for SliceViewScalarField<'slice, &'slice str>
{
    fn update(&mut self, field: FieldData<&'slice [u8]>) -> Result<()> {
        let maybe_str = match field {
            FieldData::LengthDelimited(bytes) => {
                // TODO: Using a nightly feature str_internals. Maybe remove it
                match core::str::lossy::Utf8Lossy::from_bytes(bytes)
                    .chunks()
                    .next()
                {
                    Some(chunk) => {
                        if chunk.valid.len() == bytes.len() {
                            Some(chunk.valid)
                        } else {
                            Err(ErrorKind::InvalidUtf8)?
                        }
                    }
                    None => {
                        if tags::Required::DO_DEFAULT_CHECK {
                            None
                        } else {
                            Some("")
                        }
                    }
                }
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        };
        if let Some(string) = maybe_str {
            match self {
                SliceViewScalarField::NotFound | SliceViewScalarField::ValueAvailable(_) => {
                    *self = SliceViewScalarField::ValueAvailable(string);
                }
                _ => Err(ErrorKind::InvalidSliceViewType)?,
            }
        }
        Ok(())
    }
}

impl<'slice> SliceViewUpdate<'slice, tags::Float, tags::Required>
    for SliceViewScalarField<'slice, f32>
{
    fn update(&mut self, field: FieldData<&'slice [u8]>) -> Result<()> {
        let maybe_new_val = match field {
            FieldData::Bits32(array) => {
                if !tags::Required::DO_DEFAULT_CHECK || !array.iter().all(|x| *x == 0) {
                    Some(f32::from_le_bytes(array))
                } else {
                    None
                }
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        };
        if let Some(new_val) = maybe_new_val {
            match self {
                SliceViewScalarField::NotFound | SliceViewScalarField::ValueAvailable(_) => {
                    *self = SliceViewScalarField::ValueAvailable(new_val);
                }
                _ => Err(ErrorKind::InvalidSliceViewType)?,
            }
        }
        Ok(())
    }
}
