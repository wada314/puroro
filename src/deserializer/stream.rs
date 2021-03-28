use crate::types::*;
use std::io::Read;

pub mod error;
mod impls;
pub mod variant;

pub use error::{DeserializeError, Result};
pub use variant::Variant;

pub trait Deserializer {
    fn deserialize<H: Handler>(self, handler: H) -> Result<H::Target>;
}
pub fn deserializer_from_read<R: Read>(read: R) -> impl Deserializer {
    impls::DeserializerImpl::<std::io::Bytes<R>>::new(read.bytes())
}

pub trait LengthDelimitedDeserializer: Sized {
    fn deserialize_as_message<H: Handler>(self, handler: H) -> Result<<H as Handler>::Target>;

    fn deserialize_as_string<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<char>;
    fn deserialize_as_bytes<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<u8>;
    fn deserialize_as_variants<H>(self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<Variant>;
}

pub trait Handler {
    type Target;
    fn finish(self) -> Result<Self::Target>;

    #[allow(unused_variables)]
    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
        // Providing a default implementation just for testing.
        panic!("Please provide the implementation for every handler method!");
    }

    #[allow(unused_variables)]
    fn deserialize_length_delimited_field<D: LengthDelimitedDeserializer>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> Result<()> {
        // Providing a default implementation just for testing.
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
