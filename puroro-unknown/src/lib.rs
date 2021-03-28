use ::puroro::Message;
use ::puroro::PuroroError;
use ::puroro_deserializer::stream::*;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, PuroroError>;
type DResult<T> = std::result::Result<T, DeserializeError>;

#[derive(Debug)]
enum Field {
    Variant(Variant),
    Value32([u8; 4]),
    Value64([u8; 8]),
    LengthDelimited(DelayedLengthDelimitedDeserializer),
}

#[derive(Debug)]
pub struct UnknownMessage {
    fields: HashMap<usize, Vec<Field>>,
}

impl UnknownMessage {
    fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    fn get_last_variant_field(&self, field_number: usize) -> Result<Variant> {
        if let Some(field) = self.fields.get(&field_number) {
            if let Some(last_field) = field.last() {
                match last_field {
                    Field::Variant(ref variant) => {
                        return Ok(variant.clone());
                    }
                    Field::LengthDelimited(ref dldd) => {
                        let mut variants = Vec::new();
                        dldd.deserialize_as_variants(|vs| {
                            variants = vs;
                            Ok(())
                        })
                        .map_err(|e| PuroroError::DeserializeError(Box::new(e)))?;
                        if let Some(variant) = variants.last() {
                            return Ok(variant.clone());
                        }
                    }
                    _ => {
                        // found fixed32 or fixed64 field
                        return Err(PuroroError::InvalidWireType);
                    }
                }
            }
        }
        Ok(Variant::default())
    }
}

impl MessageHandler for UnknownMessage {
    type Target = Self;

    fn finish(self) -> DResult<Self::Target> {
        Ok(self)
    }

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> DResult<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Variant(variant));
        Ok(())
    }

    fn deserialized_32bits(&mut self, field_number: usize, value: [u8; 4]) -> DResult<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Value32(value));
        Ok(())
    }

    fn deserialized_64bits(&mut self, field_number: usize, value: [u8; 8]) -> DResult<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::Value64(value));
        Ok(())
    }

    fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> DResult<()> {
        self.fields
            .entry(field_number)
            .or_insert(Vec::new())
            .push(Field::LengthDelimited(deserializer.leave_as_unknown()?));
        Ok(())
    }
}

impl Message for UnknownMessage {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self> {
        deserializer_from_bytes(iter)
            .deserialize(UnknownMessage::new())
            .map_err(|e| PuroroError::DeserializeError(Box::new(e)))
    }

    fn get_field_as_i32(&self, field_number: usize) -> Result<i32> {
        self.get_last_variant_field(field_number)?
            .to_i32()
            .map_err(|e| PuroroError::DeserializeError(Box::new(e)))
    }

    fn get_field_as_i64(&self, field_number: usize) -> Result<i64> {
        todo!()
    }

    fn get_field_as_si32(&self, field_number: usize) -> Result<i32> {
        todo!()
    }

    fn get_field_as_si64(&self, field_number: usize) -> Result<i64> {
        todo!()
    }

    fn get_field_as_u32(&self, field_number: usize) -> Result<u32> {
        todo!()
    }

    fn get_field_as_u64(&self, field_number: usize) -> Result<u64> {
        todo!()
    }

    fn collect_field_as_repeated_i32<T: std::iter::FromIterator<i32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_i64<T: std::iter::FromIterator<i64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_si32<T: std::iter::FromIterator<i32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_si64<T: std::iter::FromIterator<i64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_u32<T: std::iter::FromIterator<u32>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_u64<T: std::iter::FromIterator<u64>>(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn collect_field_as_str<S: std::iter::FromIterator<char>>(
        &self,
        field_number: usize,
    ) -> Result<S> {
        todo!()
    }

    fn collect_field_as_repeated_str<
        S: std::iter::FromIterator<char>,
        T: std::iter::FromIterator<S>,
    >(
        &self,
        field_number: usize,
    ) -> Result<T> {
        todo!()
    }

    fn get_field_as_message<T: Message>(&self, field_number: usize) -> Result<T> {
        todo!()
    }

    fn collect_field_as_repeated_message<T: Message, U: std::iter::FromIterator<T>>(
        &self,
        field_number: usize,
    ) -> Result<U> {
        todo!()
    }
}
