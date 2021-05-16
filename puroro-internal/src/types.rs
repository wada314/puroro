use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::variant::Variant;
use crate::{internal_data, tags, InternalDataForSliceViewStruct};
use ::num_derive::FromPrimitive;
use ::puroro::RepeatedField;

#[non_exhaustive]
#[derive(FromPrimitive, Debug)]
pub enum WireType {
    Variant = 0,
    Bits64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bits32 = 5,
}

#[derive(Debug, Clone)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[derive(Debug, Clone)]
pub enum SliceViewField<'slice> {
    FieldInSingleSlice {
        /// A subslice of `enclosing_slice` starting from this field's item.
        ld_slice: LdSlice<'slice>,
        /// A total number of field items in the slice. Packed repeated field is counted as 1.
        count: usize,
        enclosing_ld_slice: LdSlice<'slice>,
    },
    FieldInMultipleSlices {
        count: usize,
        first_enclosing_ld_slice: LdSlice<'slice>,
    },
}

#[derive(Debug, Clone)]
pub struct RepeatedSliceViewField<'slice, 'p, TypeTag, T>
where
    TypeTag: tags::FieldTypeTag,
{
    field: &'p Option<SliceViewField<'slice>>,
    field_number: usize,
    internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    phantom: PhantomData<(TypeTag, T)>,
}

#[derive(Debug, Clone)]
pub struct RepeatedSliceViewFieldIter<'slice, 'p, TypeTag, T>
where
    TypeTag: tags::FieldTypeTag,
{
    container: &'p RepeatedSliceViewField<'slice, 'p, TypeTag, T>,
}
