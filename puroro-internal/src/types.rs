use std::marker::PhantomData;

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
    SingleLocation(FieldData<&'slice [u8]>),
    MultipleLocations { start: &'slice [u8], count: usize },
    ValueAvailable(T),
    NotExist,
}

pub enum SliceRefRepeatedField<'slice, T> {
    Unchecked,
    SinglePacked(FieldData<&'slice [u8]>),
    MultipleFields { start: &'slice [u8], count: usize },
    NotExist,
    _Phantom(PhantomData<T>),
}
