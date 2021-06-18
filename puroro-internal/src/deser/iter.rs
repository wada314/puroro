use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::{ErrorKind, PuroroError, Result};
use std::io::Result as IoResult;

use ::num_traits::FromPrimitive;

pub trait MergeableMessageFromIter: Sized {
    fn met_field<'a, I>(
        &mut self,
        field: FieldData<&'a mut LdIter<I>>,
        field_number: usize,
    ) -> Result<bool>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;

    fn merge_from_iter<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        let mut ld_iter = LdIter::new(iter);
        ld_iter.merge_message(self)?;
        Ok(())
    }
}

pub struct LdIter<I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    iter: I,
    index: usize,
    end: Option<usize>,
}

impl<I> LdIter<I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            index: 0,
            end: None,
        }
    }

    pub fn len(&self) -> Option<usize> {
        self.end.clone().map(|e| e - self.index)
    }

    fn try_get_wire_type_and_field_number(&mut self) -> Result<Option<(WireType, usize)>> {
        let mut peekable = self.by_ref().peekable();
        if let None = peekable.peek() {
            // Found EOF at first byte. Successfull failure.
            return Ok(None);
        }
        let key = Variant::decode_bytes(&mut peekable)?.to_usize()?;
        Ok(Some((
            WireType::from_usize(key & 0x07).ok_or(ErrorKind::InvalidWireType)?,
            (key >> 3),
        )))
    }

    fn next_bytes<const BYTES: usize>(&mut self) -> Result<[u8; BYTES]> {
        let mut result = [0; BYTES];
        for i in 0..BYTES {
            result[i] = self
                .next()
                .transpose()?
                .ok_or(PuroroError::from(ErrorKind::UnexpectedInputTermination))?;
        }
        Ok(result)
    }

    pub fn merge_message<H: MergeableMessageFromIter>(&mut self, handler: &mut H) -> Result<()> {
        while let Some((wire_type, field_number)) = self.try_get_wire_type_and_field_number()? {
            let old_end_index = self.end;
            let field_data = match wire_type {
                WireType::Variant => {
                    let variant = Variant::decode_bytes(self)?;
                    FieldData::Variant(variant)
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::decode_bytes(self)?.to_usize()?;
                    self.end = Some(self.index + field_length);
                    FieldData::LengthDelimited(self.by_ref())
                }
                WireType::Bits32 => FieldData::Bits32(self.next_bytes::<4>()?),
                WireType::Bits64 => FieldData::Bits64(self.next_bytes::<8>()?),
                WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
            };
            let finish_loop = !handler.met_field(field_data, field_number)?;
            self.end = old_end_index;
            if finish_loop {
                break;
            }
        }
        Ok(())
    }

    pub fn bytes(&mut self) -> Bytes<&'_ mut Self> {
        Bytes::new(self)
    }

    pub fn chars(&mut self) -> Chars<&'_ mut Self> {
        Chars::new(self)
    }

    pub fn variants(&mut self) -> Variants<&'_ mut Self> {
        Variants::new(self.by_ref())
    }
}

impl<'a, I> Iterator for LdIter<I>
where
    I: Iterator<Item = ::std::io::Result<u8>>,
{
    type Item = std::io::Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end.unwrap_or(usize::MAX) {
            None
        } else {
            let result = self.iter.next();
            self.index += 1;
            result
        }
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<u8, ErrorKind>`.
pub struct Bytes<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> Bytes<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for Bytes<I> {
    type Item = Result<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<char, ErrorKind>`.
pub struct Chars<I: Iterator<Item = IoResult<u8>>> {
    iter: ::utf8_decode::UnsafeDecoder<I>,
}
impl<I: Iterator<Item = IoResult<u8>>> Chars<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter: ::utf8_decode::UnsafeDecoder::new(iter),
        }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for Chars<I> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ior| ior.map_err(|ioe| ioe.into()))
    }
}

/// Converts `Result<u8, std::io::IoError>` into `Result<Variant, ErrorKind>`.
pub struct Variants<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> Variants<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}
impl<I: Iterator<Item = IoResult<u8>>> Iterator for Variants<I> {
    type Item = Result<Variant>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable = self.iter.by_ref().peekable();
        if let None = peekable.peek() {
            return None;
        }
        Some(Variant::decode_bytes(&mut peekable))
    }
}
