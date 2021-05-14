use crate::variant::Variant;
use ::num_derive::FromPrimitive;

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
pub enum SliceViewFields<'slice> {
    FieldsInSingleSlice {
        /// A subslice of `enclosing_slice` starting from this field's item.
        slice: &'slice [u8],
        /// A total number of field items in the slice. Packed repeated field is counted as 1.
        count: usize,
        enclosing_slice: &'slice [u8],
    },
    FieldsInMultipleSlices {
        count: usize,
        first_enclosing_slice: &'slice [u8],
    },
}

