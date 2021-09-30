use crate::internal::variant::Variant;
use crate::{ErrorKind, PuroroError};
use ::std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[derive(Debug, Clone)]
pub enum WireType {
    Variant = 0,
    Bits64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bits32 = 5,
}

impl<T> FieldData<T> {
    pub fn as_mut(&mut self) -> FieldData<&mut T> {
        match self {
            FieldData::Variant(x) => FieldData::Variant(x.clone()),
            FieldData::LengthDelimited(x) => FieldData::LengthDelimited(x),
            FieldData::Bits32(x) => FieldData::Bits32(x.clone()),
            FieldData::Bits64(x) => FieldData::Bits64(x.clone()),
        }
    }

    pub fn map<U, F>(self, f: F) -> FieldData<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            FieldData::Variant(x) => FieldData::Variant(x),
            FieldData::LengthDelimited(x) => FieldData::LengthDelimited((f)(x)),
            FieldData::Bits32(x) => FieldData::Bits32(x),
            FieldData::Bits64(x) => FieldData::Bits64(x),
        }
    }
}

impl TryFrom<i32> for WireType {
    type Error = PuroroError;

    fn try_from(value: i32) -> ::std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::Bits64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Bits32,
            _ => Err(ErrorKind::InvalidWireType(value))?,
        })
    }
}
