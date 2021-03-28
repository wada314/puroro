use puroro_deserializer::stream::{
    DelayedLengthDelimitedDeserializer, LengthDelimitedDeserializer, MessageHandler,
    Result as DResult, Variant,
};
use std::collections::HashMap;

pub enum PuroroError {}
pub type Result<T> = std::result::Result<T, PuroroError>;

pub trait Message {
    type StringType: AsRef<str>;
    fn get_field_as_str(&self, field_number: usize) -> Result<Self::StringType>;
    fn get_field_as_i32(&self, field_number: usize) -> Result<i32>;
    fn get_field_as_message(
        &self,
        field_number: usize,
    ) -> Result<&dyn Message<StringType = Self::StringType>>;
}
pub struct Merged<T: Message, U: Message> {
    prior: T,
    later: U,
}
pub fn merge<T: Message, U: Message>(prior: T, later: U) -> Merged<T, U> {
    Merged { prior, later }
}

enum Field {
    Variant(Variant),
    Value32([u8; 4]),
    Value64([u8; 8]),
    LengthDelimited(),
}
enum LengthDelimitedField {
    String(String),
    Bytes(Vec<u8>),
    Variants(Vec<Variant>),
    Unknown(DelayedLengthDelimitedDeserializer),
}

struct UnknownMessage {
    fields: HashMap<usize, Variant>,
}

impl MessageHandler for UnknownMessage {
    type Target = Self;

    fn finish(self) -> DResult<Self::Target> {
        Ok(self)
    }

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> DResult<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    fn deserialized_32bits(&mut self, field_number: usize, value: [u8; 4]) -> DResult<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    fn deserialized_64bits(&mut self, field_number: usize, value: [u8; 8]) -> DResult<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> DResult<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
