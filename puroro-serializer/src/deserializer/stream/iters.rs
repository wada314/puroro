use super::{impls::LengthDelimitedDeserializerImpl, Field, LengthDelimitedDeserializer};
use crate::types::WireType;
use crate::tags::Variant;
use crate::{PuroroError, Result};
use ::num_traits::FromPrimitive;
use std::io::Result as IoResult;

/// Converts `Result<u8, std::io::IoError>` into `Result<u8, PuroroError>`.
pub struct BytesIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> BytesIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for BytesIterator<I> {
    type Item = Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<char, PuroroError>`.
pub struct CharsIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<I>,
}
impl<I: Iterator<Item = IoResult<u8>>> CharsIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for CharsIterator<I> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<Variant, PuroroError>`.
pub struct VariantsIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> VariantsIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for VariantsIterator<I> {
    type Item = Result<Variant>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
    }
}
