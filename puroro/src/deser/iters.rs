use crate::variant::Variant;
use crate::Result;
use std::io::Result as IoResult;

/// Converts `Result<u8, std::io::IoError>` into `Result<u8, PuroroError>`.
pub struct Bytes<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: &'a mut I,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Bytes<'a, I> {
    pub fn new(iter: &'a mut I) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for Bytes<'a, I> {
    type Item = Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<char, PuroroError>`.
pub struct Chars<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<&'a mut I>,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Chars<'a, I> {
    pub(crate) fn new(iter: &'a mut I) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for Chars<'a, I> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<Variant, PuroroError>`.
pub struct Variants<'a, I: Iterator<Item = IoResult<u8>>> {
    iter: &'a mut I,
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Variants<'a, I> {
    pub(crate) fn new(iter: &'a mut I) -> Self {
        Self { iter }
    }
}
impl<'a, I: Iterator<Item = IoResult<u8>>> Iterator for Variants<'a, I> {
    type Item = Result<Variant>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
    }
}
