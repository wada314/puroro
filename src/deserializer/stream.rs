use crate::types::*;
use std::io::{Read, Result as IoResult};

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

pub trait LengthDelimitedDeserializer: Sized + Iterator<Item = IoResult<u8>> {
    fn index(&self) -> usize;
    // Note: Don't use ExactSizeIterator because the iterator might terminate in middle.
    fn length(&self) -> Option<usize>;

    fn deserialize_as_message<H: Handler>(self, handler: H) -> Result<<H as Handler>::Target>;

    fn deserialize_as_string<H>(mut self, handler: H) -> Result<()>
    where
        H: RepeatedFieldHandler<char>,
    {
        let start_pos = self.index();
        if let Some(length) = self.length() {
            let iter = CharsIterator::new(self.by_ref().take(length));
            handler.handle(iter)?;
        } else {
            let iter = CharsIterator::new(self.by_ref());
            handler.handle(iter)?;
        }
        let end_pos = self.index();
        if let Some(length) = self.length() {
            if end_pos - start_pos == length {
                Ok(())
            } else {
                Err(DeserializeError::InvalidStringLength)
            }
        } else {
            Ok(())
        }
    }
}
pub struct CharsIterator<T: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<T>,
}
impl<T: Iterator<Item = IoResult<u8>>> CharsIterator<T> {
    pub fn new(iter: T) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<T: Iterator<Item = IoResult<u8>>> Iterator for CharsIterator<T> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map_err(|e| e.into()))
    }
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
