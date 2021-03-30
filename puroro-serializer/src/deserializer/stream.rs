use crate::types::*;
use std::io::Read;

mod impls;

pub use ::puroro::{RepeatedFieldCollector, RepeatedFieldHandler};

pub use crate::variant::Variant;
pub use ::puroro::{PuroroError, Result};

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

    // Delay the deserializing
    fn leave_as_unknown(self) -> Result<DelayedLengthDelimitedDeserializer>;
}

#[derive(Debug, Clone)]
pub struct DelayedLengthDelimitedDeserializer {
    contents: Vec<u8>,
}
impl DelayedLengthDelimitedDeserializer {
    fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }
    pub fn bytes(&self) -> std::io::Bytes<&[u8]> {
        self.contents.bytes()
    }
}
impl<'a> LengthDelimitedDeserializer<'a> for &'a DelayedLengthDelimitedDeserializer {
    fn deserialize_as_message<H: MessageHandler>(
        self,
        handler: H,
    ) -> Result<<H as MessageHandler>::Target> {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_message(handler)
    }

    fn deserialize_as_string<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = char>,
    {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_string(handler)
    }

    fn deserialize_as_bytes<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = u8>,
    {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_bytes(handler)
    }

    fn deserialize_as_variants<H>(self, handler: H) -> Result<H::Output>
    where
        H: RepeatedFieldHandler<Item = Variant>,
    {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_variants(handler)
    }

    fn leave_as_unknown(self) -> Result<DelayedLengthDelimitedDeserializer> {
        Ok(self.clone())
    }
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
