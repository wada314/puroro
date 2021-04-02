use std::io::{Bytes, Read, Result as IoResult};

use super::impls;
use super::iters::{BytesIterator, CharsIterator, VariantsIterator};
use super::{LengthDelimitedDeserializer, MessageHandler};
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
impl<'a> IntoIterator for &'a DelayedLengthDelimitedDeserializer {
    type Item = IoResult<u8>;
    type IntoIter = Bytes<&'a [u8]>;
    fn into_iter(self) -> Self::IntoIter {
        self.contents.bytes()
    }
}
impl<'a> LengthDelimitedDeserializer for &'a DelayedLengthDelimitedDeserializer {
    fn deserialize_as_message<H: MessageHandler>(
        self,
        handler: H,
    ) -> Result<<H as MessageHandler>::Target> {
        let mut iter = impls::IndexedIterator::new(self.contents.bytes());
        let deser =
            impls::LengthDelimitedDeserializerImpl::new(&mut iter, Some(self.contents.len()));
        deser.deserialize_as_message(handler)
    }

    type BytesIterator = BytesIterator<std::io::Bytes<&'a [u8]>>;
    fn deserialize_as_bytes(self) -> Self::BytesIterator {
        BytesIterator::new(self.contents.bytes())
    }

    type CharsIterator = CharsIterator<std::io::Bytes<&'a [u8]>>;
    fn deserialize_as_chars(self) -> Self::CharsIterator {
        CharsIterator::new(self.contents.bytes())
    }

    type VariantsIterator = VariantsIterator<std::io::Bytes<&'a [u8]>>;
    fn deserialize_as_variants(self) -> Self::VariantsIterator {
        VariantsIterator::new(self.contents.bytes())
    }

    fn leave_as_unknown(self) -> Result<DelayedLengthDelimitedDeserializer> {
        Ok(self.clone())
    }
}
