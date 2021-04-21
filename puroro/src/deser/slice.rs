use std::convert::TryInto;
use std::io::Read;

use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::PuroroError;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait DeserializableFromSlice {
    fn deserialize(&mut self, slice: &[u8]) -> Result<()>;
}
pub trait DeserializeMessageFromSliceEventHandler {
    fn met_field(&mut self, field: FieldData<&[u8]>, field_number: usize) -> Result<()>;
}
pub struct BytesHandlerToSliceHandler<T: super::bytes::DeserializeMessageFromBytesEventHandler>(T);
impl<T> DeserializeMessageFromSliceEventHandler for BytesHandlerToSliceHandler<T>
where
    T: super::bytes::DeserializeMessageFromBytesEventHandler,
{
    fn met_field(&mut self, field: FieldData<&[u8]>, field_number: usize) -> Result<()> {
        let field_with_iter = match field {
            FieldData::Variant(v) => FieldData::Variant(v),
            FieldData::LengthDelimited(slice) => FieldData::LengthDelimited(slice.bytes()),
            FieldData::Bits32(b) => FieldData::Bits32(b),
            FieldData::Bits64(b) => FieldData::Bits64(b),
        };
        self.0.met_field(field_with_iter, field_number)
    }
}

pub trait BytesSlice: Sized + std::io::Read {
    fn deser_message<H: DeserializeMessageFromSliceEventHandler>(
        &mut self,
        handler: &mut H,
    ) -> Result<()>;
}
impl<'a> BytesSlice for &'a [u8] {
    fn deser_message<H: DeserializeMessageFromSliceEventHandler>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        while let Some((new_slice, wire_type, field_number)) =
            try_get_wire_type_and_field_number(self)?
        {
            *self = new_slice;
            let field_data = match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(&mut self.bytes())?;
                    FieldData::Variant(variant)
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::decode_bytes(&mut self.bytes())?.to_usize()?;
                    let (inner_slice, rest) = self.split_at(field_length);
                    *self = rest;
                    FieldData::LengthDelimited(inner_slice)
                }
                WireType::Bits32 => {
                    if self.len() < 4 {
                        Err(PuroroError::UnexpectedInputTermination)?;
                    }
                    let (bytes, rest) = self.split_at(4);
                    *self = rest;
                    FieldData::Bits32(
                        bytes
                            .try_into()
                            .map_err(|_| PuroroError::UnexpectedInputTermination)?,
                    )
                }
                WireType::Bits64 => {
                    if self.len() < 8 {
                        Err(PuroroError::UnexpectedInputTermination)?;
                    }
                    let (bytes, rest) = self.split_at(8);
                    *self = rest;
                    FieldData::Bits64(
                        bytes
                            .try_into()
                            .map_err(|_| PuroroError::UnexpectedInputTermination)?,
                    )
                }
                WireType::StartGroup | WireType::EndGroup => Err(PuroroError::GroupNotSupported)?,
            };
            handler.met_field(field_data, field_number)?;
        }
        Ok(())
    }
}

fn try_get_wire_type_and_field_number(slice: &[u8]) -> Result<Option<(&[u8], WireType, usize)>> {
    if slice.len() == 0 {
        return Ok(None);
    }
    let key = { Variant::decode_bytes(&mut slice.bytes())?.to_usize()? };
    Ok(Some((
        slice,
        WireType::from_usize(key & 0x07).ok_or(PuroroError::InvalidWireType)?,
        (key >> 3),
    )))
}
