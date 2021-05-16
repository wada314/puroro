use std::convert::TryInto;
use std::io::Read;

use super::{DeserializableMessageFromIter, LdIter};
use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::ErrorKind;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait DeserializableMessageFromSlice<'slice> {
    fn met_field_at(
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

    pub fn deser_message<H: DeserializableMessageFromSlice<'slice>>(
        &self,
        handler: &mut H,
    ) -> Result<()> {
        let enclosing_slice = self.slice;
        let mut fields = self.fields();

        while let Some(rfield) = fields.next() {
            let field = rfield?;
            let mut ld_slice_from_this_field = self.clone();
            ld_slice_from_this_field.update_start_pos(field.slice);
            if !handler.met_field_at(
                field.data,
                field.number,
                ld_slice_from_this_field,
                enclosing_slice,
            )? {
                break;
            }
        }
        Ok(())
    }

    pub fn as_slice(&self) -> &'slice [u8] {
        self.slice
    }

    pub fn fields(&self) -> Fields<'slice> {
        Fields {
            ld_slice: self.clone(),
        }
    }

    pub fn update_start_pos(&mut self, new_slice: &'slice [u8]) {
        debug_assert!(self.as_slice().as_ptr_range().start <= new_slice.as_ptr_range().start);
        debug_assert_eq!(
            self.as_slice().as_ptr_range().end,
            new_slice.as_ptr_range().end
        );
        self.slice = new_slice;
    }

    pub fn get_slice_by(&self, sub_slice: &'slice [u8]) -> &'slice [u8] {
        debug_assert!(self.as_slice().as_ptr_range().start <= sub_slice.as_ptr_range().start);
        let mut length = (sub_slice.as_ptr() as usize) - (self.as_slice().as_ptr() as usize);
        length = usize::min(length, self.as_slice().len());
        self.as_slice().split_at(length).0
    }
}

pub struct FieldInSlice<'slice> {
    pub number: usize,
    pub data: FieldData<LdSlice<'slice>>,
    pub slice: &'slice [u8],
}

pub struct Fields<'slice> {
    ld_slice: LdSlice<'slice>,
}

impl<'slice> Fields<'slice> {
    fn try_next(&mut self) -> Result<Option<FieldInSlice<'slice>>> {
        let slice_from_this_field = self.ld_slice.as_slice();
        match self.try_get_wire_type_and_field_number(slice_from_this_field)? {
            None => {
                Ok(None) // end of slice
            }
            Some((wire_type, field_number, mut slice)) => {
                let (field_data, remaining_slice) = match wire_type {
                    WireType::Variant => {
                        let variant = Variant::decode_bytes(&mut slice.by_ref().bytes())?;
                        (FieldData::Variant(variant), slice)
                    }
                    WireType::LengthDelimited => {
                        let ld_length =
                            Variant::decode_bytes(&mut slice.by_ref().bytes())?.to_usize()?;
                        let (inner_slice, rest) = slice.split_at(ld_length);
                        (FieldData::LengthDelimited(LdSlice::new(inner_slice)), slice)
                    }
                    WireType::Bits32 => {
                        if slice.len() < 4 {
                            Err(ErrorKind::UnexpectedInputTermination)?;
                        }
                        let (bytes, remain) = slice.split_at(4);
                        (
                            FieldData::Bits32(
                                bytes
                                    .try_into()
                                    .map_err(|_| ErrorKind::UnexpectedInputTermination)?,
                            ),
                            remain,
                        )
                    }
                    WireType::Bits64 => {
                        if slice.len() < 8 {
                            Err(ErrorKind::UnexpectedInputTermination)?;
                        }
                        let (bytes, remain) = slice_from_this_field.split_at(8);
                        (
                            FieldData::Bits64(
                                bytes
                                    .try_into()
                                    .map_err(|_| ErrorKind::UnexpectedInputTermination)?,
                            ),
                            remain,
                        )
                    }
                    WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
                };
                let field_slice = self.ld_slice.get_slice_by(remaining_slice);
                self.ld_slice.update_start_pos(remaining_slice);
                Ok(Some(FieldInSlice {
                    number: field_number,
                    data: field_data,
                    slice: field_slice,
                }))
            }
        }
    }

    fn try_get_wire_type_and_field_number(
        &self,
        mut slice: &'slice [u8],
    ) -> Result<Option<(WireType, usize, &'slice [u8])>> {
        if slice.len() == 0 {
            return Ok(None);
        }
        let header = { Variant::decode_bytes(&mut slice.by_ref().bytes())?.to_usize()? };
        Ok(Some((
            WireType::from_usize(header & 0x07).ok_or(ErrorKind::InvalidWireType)?,
            (header >> 3),
            slice,
        )))
    }
}

impl<'slice> Iterator for Fields<'slice> {
    type Item = Result<FieldInSlice<'slice>>;

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

impl<'slice, 'a, T> DeserializableMessageFromSlice<'slice> for FromIterToFromSlice<'a, T>
where
    T: DeserializableMessageFromIter,
{
    fn met_field_at(
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
