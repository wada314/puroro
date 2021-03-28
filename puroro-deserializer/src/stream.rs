use crate::types::*;
use std::io::Read;

pub mod error;
mod impls;
pub mod variant;

pub use error::{DeserializeError, Result};
pub use variant::Variant;

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

    fn deserialize_as_string<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<char>;
    fn deserialize_as_bytes<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<u8>;
    fn deserialize_as_variants<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<Variant>;

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

    fn deserialize_as_string<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<char>,
    {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_string(handler)
    }

    fn deserialize_as_bytes<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<u8>,
    {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_bytes(handler)
    }

    fn deserialize_as_variants<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<Variant>,
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

pub trait RepeatedFieldHandler<T> {
    fn handle<I: Iterator<Item = Result<T>>>(self, iter: I) -> Result<()>;
}
impl<F: FnOnce(String) -> Result<()>> RepeatedFieldHandler<char> for F {
    fn handle<I: Iterator<Item = Result<char>>>(self, iter: I) -> Result<()> {
        (self)(iter.collect::<Result<String>>()?)
    }
}
impl<F: FnOnce(Vec<u8>) -> Result<()>> RepeatedFieldHandler<u8> for F {
    fn handle<I: Iterator<Item = Result<u8>>>(self, iter: I) -> Result<()> {
        (self)(iter.collect::<Result<Vec<u8>>>()?)
    }
}
impl<F: FnOnce(Vec<Variant>) -> Result<()>> RepeatedFieldHandler<Variant> for F {
    fn handle<I: Iterator<Item = Result<Variant>>>(self, iter: I) -> Result<()> {
        (self)(iter.collect::<Result<Vec<Variant>>>()?)
    }
}
