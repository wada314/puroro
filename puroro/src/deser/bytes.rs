use super::iters::{CharsIterator, VariantsIterator};
use super::BytesIterator;
use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::PuroroError;
use crate::Result;
use ::num_traits::FromPrimitive;

pub trait BytesIter: Sized + Iterator<Item = ::std::io::Result<u8>> {
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

    fn deser_message<H: DeserializeMessageFromBytesEventHandler>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        while let Some((wire_type, field_number)) = self.try_get_wire_type_and_field_number()? {
            let field_data = match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(self)?;
                    FieldData::Variant(variant)
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::decode_bytes(self)?.to_usize()?;
                    let inner_bytes = self.by_ref().take(field_length);
                    FieldData::LengthDelimited(inner_bytes)
                }
                WireType::Bits32 => FieldData::Bits32(self.next_bytes::<4>()?),
                WireType::Bits64 => FieldData::Bits64(self.next_bytes::<8>()?),
                WireType::StartGroup | WireType::EndGroup => Err(PuroroError::GroupNotSupported)?,
            };
            handler.met_field(field_data, field_number)?;
        }
        Ok(())
    }

    fn bytes(&mut self) -> BytesIterator<'_, Self> {
        BytesIterator::new(self)
    }

    fn chars(&mut self) -> CharsIterator<'_, Self> {
        CharsIterator::new(self)
    }

    fn variants(&mut self) -> VariantsIterator<'_, Self> {
        VariantsIterator::new(self)
    }
}
// blanket implementation
impl<T> BytesIter for T where T: Sized + Iterator<Item = ::std::io::Result<u8>> {}

pub trait DeserializableFromBytes {
    fn deserialize<B: BytesIter>(iter: B) -> Self;
}

pub trait DeserializeMessageFromBytesEventHandler {
    fn met_field<B: BytesIter>(&mut self, field: FieldData<B>, field_number: usize) -> Result<()>;
}
