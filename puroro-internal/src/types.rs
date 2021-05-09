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

// For non-repeated types except Message (and probably Map? TBD).
#[derive(Debug, Clone)]
pub enum SliceViewScalarField<T: Clone> {
    NotFound,
    ValueAvailable(T),
}

#[derive(Debug, Clone)]
pub enum SliceViewMultipleField<'slice, T: Clone> {
    NotFound,
    // Only available for scalar Message field.
    // For String and Bytes, they can just use the last single field.
    MessageLocations {
        first: BytesSlice<'slice>,
        // number of the message field merged into this message instance.
        count: usize,
    },
    // Only available for Variant wire type.
    SinglePacked(FieldData<BytesSlice<'slice>>),
    MultipleFields {
        first: FieldData<BytesSlice<'slice>>,
        remaining_slice: &'slice [u8],
        count: usize,
    },
    _Phantom(PhantomData<T>),
}

enum SourceFields<'slice> {
    FieldsInSingleSlice {
        slice: &'slice [u8],
        count: usize,
        enclosing_slice: &'slice [u8],
    },
    FieldsInMultipleSlices {
        count: usize,
    }
}
