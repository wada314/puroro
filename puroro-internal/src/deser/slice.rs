use std::convert::TryInto;
use std::io::Read;

use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::ErrorKind;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait DeserializableMessageFromSlice {
    fn met_field_at<'slice>(
        &mut self,
        field: FieldData<LdSlice<'slice>>,
        field_number: usize,
        slice_from_this_field: &'slice [u8],
        enclosing_slice: &'slice [u8],
    ) -> Result<bool>;
}

/// A wrapper over slice which stores maybe multiple fields data.
/// Ld = Length delimited = wiretype==2
#[derive(Debug, Clone)]
pub struct LdSlice<'slice> {
    slice: &'slice [u8],
}
impl<'slice> LdSlice<'slice> {
    pub fn new(slice: &'slice [u8]) -> Self {
        Self { slice }
    }

    pub fn deser_message<H: DeserializableMessageFromSlice>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        let enclosing_slice = self.slice;
        let mut slice_from_this_field = self.slice;

        while let Some(field_number_and_data) = self.next() {
            let (field_number, field_data) = field_number_and_data?;
            if !handler.met_field_at(
                field_data,
                field_number,
                slice_from_this_field,
                enclosing_slice,
            )? {
                break;
            }
            slice_from_this_field = self.slice;
        }
        Ok(())
    }

    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        if self.slice.len() == 0 {
            return Ok(None);
        }
        let key = { Variant::decode_bytes(&mut self.bytes())?.to_usize()? };
        Ok(Some((
            WireType::from_usize(key & 0x07).ok_or(ErrorKind::InvalidWireType)?,
            (key >> 3),
        )))
    }

    pub fn bytes(&mut self) -> std::io::Bytes<&mut &'slice [u8]> {
        self.slice.by_ref().bytes()
    }

    pub fn as_slice(&self) -> &'slice [u8] {
        self.slice
    }
}

impl<'slice> Iterator for LdSlice<'slice> {
    type Item = Result<(usize, FieldData<LdSlice<'slice>>)>;

    fn next(&mut self) -> Option<Self::Item> {
        (|| {
            Ok(match self.try_get_wire_type_and_field_number()? {
                None => {
                    None // end of slice
                }
                Some((wire_type, field_number)) => {
                    let field_data = match wire_type {
                        WireType::Variant => {
                            let variant = Variant::decode_bytes(&mut self.bytes())?;
                            FieldData::Variant(variant)
                        }
                        WireType::LengthDelimited => {
                            let field_length =
                                Variant::decode_bytes(&mut self.bytes())?.to_usize()?;
                            let (inner_slice, rest) = self.slice.split_at(field_length);
                            self.slice = rest;
                            FieldData::LengthDelimited(LdSlice::new(inner_slice))
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
                        WireType::StartGroup | WireType::EndGroup => {
                            Err(ErrorKind::GroupNotSupported)?
                        }
                    };
                    Some((field_number, field_data))
                }
            })
        })()
        .transpose()
    }
}
