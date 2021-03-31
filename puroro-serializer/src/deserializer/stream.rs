use crate::types::*;
use std::io::{Read, Result as IoResult};

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

    type BytesIterator: Iterator<Item = Result<u8>>;
    fn deserialize_as_bytes_iter(self) -> Self::BytesIterator;

    type CharsIterator: Iterator<Item = Result<char>>;
    fn deserialize_as_chars_iter(self) -> Self::CharsIterator;

    type VariantsIterator: Iterator<Item = Result<Variant>>;
    fn deserialize_as_variants_iter(self) -> Self::VariantsIterator;

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

    type BytesIterator = BytesIterator<std::io::Bytes<&'a [u8]>>;

    fn deserialize_as_bytes_iter(self) -> Self::BytesIterator {
        BytesIterator::new(self.contents.bytes())
    }

    type CharsIterator = CharsIterator2<std::io::Bytes<&'a [u8]>>;

    fn deserialize_as_chars_iter(self) -> Self::CharsIterator {
        CharsIterator2::new(self.contents.bytes())
    }

    type VariantsIterator = VariantsIterator2<std::io::Bytes<&'a [u8]>>;

    fn deserialize_as_variants_iter(self) -> Self::VariantsIterator {
        VariantsIterator2::new(self.contents.bytes())
    }
}

pub struct BytesIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}

impl<'a, I: Iterator<Item = IoResult<u8>>> BytesIterator<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for BytesIterator<I> {
    type Item = Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}
pub struct CharsIterator2<I: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<I>,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> CharsIterator2<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for CharsIterator2<I> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}
pub struct VariantsIterator2<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> VariantsIterator2<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for VariantsIterator2<I> {
    type Item = Result<Variant>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
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
