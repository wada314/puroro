use crate::types::*;
use std::io::Read;

mod delayed;
mod impls;
mod iters;

pub use crate::variant::Variant;
pub use ::puroro::{PuroroError, Result};
pub use ::puroro::{RepeatedFieldCollector, RepeatedFieldHandler};

pub use delayed::DelayedLengthDelimitedDeserializer;

pub trait Deserializer {
    fn deserialize<H: MessageHandler>(self, handler: H) -> Result<H::Target>;
}
pub fn deserializer_from_read<R: Read>(read: R) -> impl Deserializer {
    impls::DeserializerImpl::<std::io::Bytes<R>>::new(read.bytes())
}
pub fn deserializer_from_bytes<I: Iterator<Item = std::io::Result<u8>>>(
    iter: I,
) -> impl Deserializer {
    impls::DeserializerImpl::<I>::new(iter)
}

pub trait LengthDelimitedDeserializer<'a>: Sized {
    fn deserialize_as_message<H: MessageHandler>(
        self,
        handler: H,
    ) -> Result<<H as MessageHandler>::Target>;

    fn deserialize_as_string<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = char>;
    fn deserialize_as_bytes<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = u8>;
    fn deserialize_as_variants<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = Variant>;

    type BytesIterator: Iterator<Item = Result<u8>>;
    fn deserialize_as_bytes_iter(self) -> Self::BytesIterator;

    type CharsIterator: Iterator<Item = Result<char>>;
    fn deserialize_as_chars_iter(self) -> Self::CharsIterator;

    type VariantsIterator: Iterator<Item = Result<Variant>>;
    fn deserialize_as_variants_iter(self) -> Self::VariantsIterator;

    // Delay the deserializing
    fn leave_as_unknown(self) -> Result<DelayedLengthDelimitedDeserializer>;
}

pub trait MessageHandler {
    type Target;
    fn finish(self) -> Result<Self::Target>;

    #[allow(unused_variables)]
    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    #[allow(unused_variables)]
    fn deserialized_32bits(&mut self, field_number: usize, value: [u8; 4]) -> Result<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    #[allow(unused_variables)]
    fn deserialized_64bits(&mut self, field_number: usize, value: [u8; 8]) -> Result<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }

    #[allow(unused_variables)]
    fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> Result<()> {
        // Providing a default implementation just for testing convenience.
        panic!("Please provide the implementation for every handler method!");
    }
}
