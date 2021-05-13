use std::convert::TryInto;
use std::io::Read;

use super::{DeserializableMessageFromIter, LdIter};
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

    pub fn deser_message<H: DeserializableMessageFromSlice>(&self, handler: &mut H) -> Result<()> {
        let enclosing_slice = self.slice;
        let mut slice_from_this_field = self.slice;
        let mut fields = self.fields();

        while let Some(field_number_and_data) = fields.next() {
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

    pub fn as_slice(&self) -> &'slice [u8] {
        self.slice
    }

    pub fn fields(&self) -> impl Iterator<Item = Result<(usize, FieldData<LdSlice<'slice>>)>> {
        Fields {
            slice: self.as_slice(),
        }
    }
}

struct Fields<'slice> {
    slice: &'slice [u8],
}

impl<'slice> Fields<'slice> {
    fn try_next(&mut self) -> Result<Option<(usize, FieldData<LdSlice<'slice>>)>> {
        Ok(match self.try_get_wire_type_and_field_number()? {
            None => {
                None // end of slice
            }
            Some((wire_type, field_number)) => {
                let field_data = match wire_type {
                    WireType::Variant => {
                        let variant = Variant::decode_bytes(&mut self.slice.by_ref().bytes())?;
                        FieldData::Variant(variant)
                    }
                    WireType::LengthDelimited => {
                        let field_length =
                            Variant::decode_bytes(&mut self.slice.by_ref().bytes())?.to_usize()?;
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
                    WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
                };
                Some((field_number, field_data))
            }
        })
    }

    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        if self.slice.len() == 0 {
            return Ok(None);
        }
        let header = { Variant::decode_bytes(&mut self.slice.by_ref().bytes())?.to_usize()? };
        Ok(Some((
            WireType::from_usize(header & 0x07).ok_or(ErrorKind::InvalidWireType)?,
            (header >> 3),
        )))
    }
}

impl<'slice> Iterator for Fields<'slice> {
    type Item = Result<(usize, FieldData<LdSlice<'slice>>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

pub struct FromIterToFromSlice<'a, T: DeserializableMessageFromIter>(&'a mut T);

impl<'a, T: DeserializableMessageFromIter> FromIterToFromSlice<'a, T> {
    pub fn new(from_iter: &'a mut T) -> Self {
        Self(from_iter)
    }
}

impl<'a, T> super::slice::DeserializableMessageFromSlice for FromIterToFromSlice<'a, T>
where
    T: DeserializableMessageFromIter,
{
    fn met_field_at<'slice>(
        &mut self,
        field: FieldData<LdSlice<'slice>>,
        field_number: usize,
        _: &'slice [u8],
        _: &'slice [u8],
    ) -> Result<bool> {
        let mut ld_iter;
        let field_data = match field {
            FieldData::Variant(v) => FieldData::Variant(v),
            FieldData::LengthDelimited(ld_slice) => {
                ld_iter = LdIter::new(ld_slice.slice.bytes());
                FieldData::LengthDelimited(&mut ld_iter)
            }
            FieldData::Bits32(b) => FieldData::Bits32(b),
            FieldData::Bits64(b) => FieldData::Bits64(b),
        };
        self.0.met_field(field_data, field_number)
    }
}
