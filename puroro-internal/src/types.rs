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

pub enum SliceRefScalarField<'slice, T> {
    Unchecked,
    SingleLocation(FieldData<BytesSlice<'slice>>),
    MultipleLocations { start: &'slice [u8], count: usize },
    ValueAvailable(T),
    NotExist,
}

pub enum SliceRefRepeatedField<'slice, T> {
    Unchecked,
    SinglePacked(FieldData<BytesSlice<'slice>>),
    MultipleFields { start: &'slice [u8], count: usize },
    NotExist,
    _Phantom(PhantomData<T>),
}
