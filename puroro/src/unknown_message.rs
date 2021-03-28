use puroro_deserializer::stream::{
    DelayedLengthDelimitedDeserializer, LengthDelimitedDeserializer, MessageHandler,
    Result as DResult, Variant,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Field {
    Variant(Variant),
    Value32([u8; 4]),
    Value64([u8; 8]),
    LengthDelimited(LengthDelimitedField),
}

#[derive(Debug)]
enum LengthDelimitedField {
    String(String),
    Bytes(Vec<u8>),
    Variants(Vec<Variant>),
    Message(UnknownMessage),
    Unknown(DelayedLengthDelimitedDeserializer),
}

#[derive(Debug)]
pub struct UnknownMessage {
    fields: HashMap<usize, Vec<Field>>,
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
            .push(Field::LengthDelimited(LengthDelimitedField::Unknown(
                deserializer.leave_as_unknown()?,
            )));
        Ok(())
    }
}
