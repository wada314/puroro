use super::iters::{Chars, Variants};
use super::Bytes;
use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::PuroroError;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait DeserializableFromBytes {
    fn deserialize<I>(&mut self, iter: &mut I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        let bytes_iter = BytesIter::new(iter);
        self.deserialize_from_bytes_iter(bytes_iter)
    }
    fn deserialize_from_bytes_iter<'a, I>(&mut self, bytes_iter: BytesIter<'a, I>) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserializableMessageFromIter {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: FieldData<&'a mut BytesIter<'b, I>>,
        field_number: usize,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
pub struct BytesHandlerToSliceHandler<T: DeserializableMessageFromIter>(T);
impl<T> super::slice::DeserializableMessageFromSlice for BytesHandlerToSliceHandler<T>
where
    T: DeserializableMessageFromIter,
{
    fn met_field<'a>(&mut self, field: FieldData<&'a [u8]>, field_number: usize) -> Result<()> {
        use std::io::Read;
        type BytesIterBoundType<'a> = BytesIter<'a, std::io::Bytes<&'a [u8]>>;
        match field {
            FieldData::Variant(v) => self
                .0
                .met_field::<BytesIterBoundType<'a>>(FieldData::Variant(v), field_number),
            FieldData::LengthDelimited(slice) => {
                let mut bytes = slice.bytes();
                let mut bytes_iter = BytesIter::new(&mut bytes);
                let field_data = FieldData::LengthDelimited(&mut bytes_iter);
                self.0.met_field(field_data, field_number)
            }
            FieldData::Bits32(b) => self
                .0
                .met_field::<BytesIterBoundType<'a>>(FieldData::Bits32(b), field_number),
            FieldData::Bits64(b) => self
                .0
                .met_field::<BytesIterBoundType<'a>>(FieldData::Bits64(b), field_number),
        }
    }
}

pub struct BytesIter<'a, I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    iter: &'a mut I,
    index: usize,
    end: usize,
}

impl<'a, I> BytesIter<'a, I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    pub fn new(iter: &'a mut I) -> Self {
        Self {
            iter,
            index: 0,
            end: usize::MAX,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.index
    }

    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        let mut peekable = self.by_ref().peekable();
        if let None = peekable.peek() {
            // Found EOF at first byte. Successfull failure.
            return Ok(None);
        }
        let key = Variant::decode_bytes(&mut peekable)?.to_usize()?;
        Ok(Some((
            WireType::from_usize(key & 0x07).ok_or(PuroroError::InvalidWireType)?,
            (key >> 3),
        )))
    }

    fn next_bytes<const BYTES: usize>(&mut self) -> Result<[u8; BYTES]> {
        let mut result = [0; BYTES];
        for i in 0..BYTES {
            result[i] = self
                .next()
                .ok_or(PuroroError::UnexpectedInputTermination)??;
        }
        Ok(result)
    }

    pub fn deser_message<H: DeserializableMessageFromIter>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        while let Some((wire_type, field_number)) = self.try_get_wire_type_and_field_number()? {
            let old_end_index = self.end;
            let field_data = match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(self)?;
                    FieldData::Variant(variant)
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::decode_bytes(self)?.to_usize()?;
                    self.end = self.index + field_length;
                    FieldData::LengthDelimited(self.by_ref())
                }
                WireType::Bits32 => FieldData::Bits32(self.next_bytes::<4>()?),
                WireType::Bits64 => FieldData::Bits64(self.next_bytes::<8>()?),
                WireType::StartGroup | WireType::EndGroup => Err(PuroroError::GroupNotSupported)?,
            };
            handler.met_field(field_data, field_number)?;
            self.end = old_end_index;
        }
        Ok(())
    }

    pub fn bytes(&mut self) -> Bytes<'_, Self> {
        Bytes::new(self)
    }

    pub fn chars(&mut self) -> Chars<'_, Self> {
        Chars::new(self)
    }

    pub fn variants(&mut self) -> Variants<'_, Self> {
        Variants::new(self)
    }
}

impl<'a, I> Iterator for BytesIter<'a, I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    type Item = std::io::Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            None
        } else {
            let result = self.iter.next();
            self.index += 1;
            result
        }
    }
}
