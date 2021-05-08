use crate::deser::BytesSlice;
use crate::tags;
use crate::tags::FieldTypeTag;
use crate::types::{FieldData, SliceViewRepeatedField, SliceViewScalarField};
use crate::variant::Variant;
use crate::{ErrorKind, Result};

pub trait SliceViewUpdate<TypeTag: FieldTypeTag> {
    fn update(&mut self, field: FieldData<&[u8]>) -> Result<()>;
}

impl<'slice> SliceViewUpdate<tags::Int32> for SliceViewScalarField<'slice, i32> {
    fn update(&mut self, field: FieldData<&[u8]>) -> Result<()> {
        let new_val = match field {
            FieldData::Variant(variant) => variant.to_native::<tags::Int32>()?,
            FieldData::LengthDelimited(slice) => {
                let wrapped_slice = BytesSlice::new(slice);
                todo!()
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        };
        Ok(())
    }
}
