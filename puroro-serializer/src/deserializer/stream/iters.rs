use crate::variant::Variant;
use crate::Result;
use std::io::Result as IoResult;

pub struct BytesIterator<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}

impl<'a, I: Iterator<Item = IoResult<u8>>> BytesIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
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
    pub(crate) fn new(iter: I) -> Self {
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
    pub(crate) fn new(iter: I) -> Self {
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
