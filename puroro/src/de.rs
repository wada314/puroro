pub mod from_iter;
use std::convert::TryFrom;

use crate::variant::Variant;
use crate::ErrorKind;
use crate::PuroroError;

#[derive(Debug, Clone)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum WireType {
    Variant = 0,
    Bits64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bits32 = 5,
}
impl TryFrom<i32> for WireType {
    type Error = PuroroError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::Variant,
            2 => WireType::Variant,
            3 => WireType::Variant,
            4 => WireType::Variant,
            5 => WireType::Variant,
            _ => Err(ErrorKind::InvalidWireType(value))?,
        })
    }
}
