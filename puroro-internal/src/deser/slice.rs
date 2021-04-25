use std::convert::TryInto;
use std::io::Read;

use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::ErrorKind;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait DeserializableFromSlice {
    fn deserialize(&mut self, slice: &[u8]) -> Result<()>;
}
pub trait DeserializableMessageFromSlice {
    fn met_field(&mut self, field: FieldData<&[u8]>, field_number: usize) -> Result<()>;
}

pub struct BytesSlice<'slice> {
    slice: &'slice [u8],
}
impl<'slice> BytesSlice<'slice> {
    pub fn deser_message<H: DeserializableMessageFromSlice>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        while let Some((wire_type, field_number)) = self.try_get_wire_type_and_field_number()? {
            let field_data = match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(&mut self.slice.bytes())?;
                    FieldData::Variant(variant)
                }
                WireType::LengthDelimited => {
                    let field_length =
                        Variant::decode_bytes(&mut self.slice.bytes())?.to_usize()?;
                    let (inner_slice, rest) = self.slice.split_at(field_length);
                    self.slice = rest;
                    FieldData::LengthDelimited(inner_slice)
                }
                WireType::Bits32 => {
                    if self.slice.len() < 4 {
                        Err(ErrorKind::UnexpectedInputTermination)?;
                    }
                    let (bytes, rest) = self.slice.split_at(4);
                    self.slice = rest;
                    FieldData::Bits32(
                        bytes
                            .try_into()
                            .map_err(|_| ErrorKind::UnexpectedInputTermination)?,
                    )
                }
                WireType::Bits64 => {
                    if self.slice.len() < 8 {
                        Err(ErrorKind::UnexpectedInputTermination)?;
                    }
                    let (bytes, rest) = self.slice.split_at(8);
                    self.slice = rest;
                    FieldData::Bits64(
                        bytes
                            .try_into()
                            .map_err(|_| ErrorKind::UnexpectedInputTermination)?,
                    )
                }
                WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
            };
            handler.met_field(field_data, field_number)?;
        }
        Ok(())
    }

    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        if self.slice.len() == 0 {
            return Ok(None);
        }
        let key = { Variant::decode_bytes(&mut self.slice.by_ref().bytes())?.to_usize()? };
        Ok(Some((
            WireType::from_usize(key & 0x07).ok_or(ErrorKind::InvalidWireType)?,
            (key >> 3),
        )))
    }
}
