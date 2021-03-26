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
        length: usize,
    ) -> Result<()> {
        // Providing a default implementation just for testing.
        panic!("Please provide the implementation for every handler method!");
    }
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
