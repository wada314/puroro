use std::marker::PhantomData;

use crate::deser::BytesSlice;
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
pub enum SliceViewScalarField<'slice, T: Clone> {
    NotFound,
    // Ld stands for "Length Delimited", a wire type.
    // Only available for scalar String, Bytes, and Message fields.
    // Otherwise, the value is already decoded and stored while scanning.
    SingleLdLocation(FieldData<BytesSlice<'slice>>),
    // Only available for scalar Message field. For String and Bytes,
    // they can just use the last single (needs non-empty check in proto3) field.
    MultipleLdLocations {
        first: FieldData<BytesSlice<'slice>>,
        remaining_slice: &'slice [u8],
        count: usize,
    },
    // The data is available and already decoded.
    //  For numerical fields including Enum.
    ValueAvailable(T),
}

#[derive(Debug, Clone)]
pub enum SliceViewRepeatedField<'slice, T: Clone> {
    NotFound,
    // Only available for Variant wire type.
    SinglePacked(FieldData<BytesSlice<'slice>>),
    MultipleFields {
        first: FieldData<BytesSlice<'slice>>,
        remaining_slice: &'slice [u8],
        count: usize,
    },
    _Phantom(PhantomData<T>),
}
