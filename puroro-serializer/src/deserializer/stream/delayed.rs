use std::io::Read;

use super::impls;
use super::iters::{BytesIterator, CharsIterator2, VariantsIterator2};
use super::{LengthDelimitedDeserializer, MessageHandler, RepeatedFieldHandler};
use crate::variant::Variant;
use crate::Result;

#[derive(Debug, Clone)]
pub struct DelayedLengthDelimitedDeserializer {
    contents: Vec<u8>,
}
impl DelayedLengthDelimitedDeserializer {
    pub(crate) fn new(contents: Vec<u8>) -> Self {
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
