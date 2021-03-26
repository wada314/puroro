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
    fn deserialize_as_message<H: Handler>(
        self,
        opt_length: Option<usize>,
        handler: H,
    ) -> Result<<H as Handler>::Target>;

    fn deserialize_as_string(self) -> Result<()>;
    fn deserialize_as_packed_variants(self) -> Result<()>;
}

pub trait Handler {
    type Target;
    fn finish(self) -> Result<Self::Target>;

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
        #[cfg(not(test))]
        compile_error!("Please provide the implementation for every handler method!");
        Ok(())
    }

    fn deserialize_length_delimited_field<D: LengthDelimitedDeserializer>(
        &mut self,
        deserializer: D,
        field_number: usize,
        length: usize,
    ) -> Result<()>;
}

pub(crate) struct IndexedIterator<I> {
    index: usize,
    iter: I,
}
impl<I> Iterator for IndexedIterator<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iter.next()
    }
}
impl<I> IndexedIterator<I> {
    fn new(iter: I) -> Self {
        IndexedIterator { index: 0, iter }
    }
    fn index(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A handler to test the implementation of `Deserializer` and `LengthDelimitedDeserializer`.
    #[derive(Default)]
    pub(crate) struct MockHandler<T: Default, D: LengthDelimitedDeserializer> {
        pub(crate) value: T,
        pub(crate) deserialized_variant: Option<Box<dyn Fn(&mut T, usize, Variant) -> Result<()>>>,

        pub(crate) deserialize_length_delimited_field:
            Option<Box<dyn Fn(&mut T, D, usize, usize) -> Result<()>>>,
    }

    impl<T: Default, D: LengthDelimitedDeserializer> MockHandler<T, D> {
        pub(crate) fn new() -> Self {
            Self {
                value: Default::default(),
                deserialized_variant: None,
                deserialize_length_delimited_field: None,
            }
        }
    }
    impl<T: Default, D: LengthDelimitedDeserializer> Handler for MockHandler<T, D> {
        type Target = T;

        fn finish(self) -> Result<Self::Target> {
            Ok(self.value)
        }
        fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
            let value_mut = &mut self.value;
            self.deserialized_variant
                .as_ref()
                .map_or(Ok(()), |f| (f)(value_mut, field_number, variant))
        }

        fn deserialize_length_delimited_field<D: LengthDelimitedDeserializer>(
            &mut self,
            deserializer: D,
            field_number: usize,
            length: usize,
        ) -> Result<()> {
            let value_mut = &mut self.value;
            self.deserialize_length_delimited_field
                .as_ref()
                .map_or(Ok(()), |f| {
                    (f)(value_mut, deserializer, field_number, length)
                })
        }
    }
}
