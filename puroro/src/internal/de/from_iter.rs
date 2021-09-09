use super::DeserFieldsFromBytesIter;
use crate::types::{FieldData, WireType};
use crate::variant::Variant;
use crate::Message;
use crate::{ErrorKind, Result};
use ::std::convert::TryFrom;
use ::std::io::Result as IoResult;

pub fn deser_from_iter<Msg, I>(message: &mut Msg, input_iter: I) -> Result<()>
where
    Msg: Message + DeserFieldsFromBytesIter,
    I: Iterator<Item = IoResult<u8>>,
{
    let mut scoped_iter = ScopedIter::new(input_iter);
    deser_from_scoped_iter(message, &mut scoped_iter)
}

pub fn deser_from_scoped_iter<Msg, I>(message: &mut Msg, iter: &mut ScopedIter<I>) -> Result<()>
where
    Msg: Message + DeserFieldsFromBytesIter,
    I: Iterator<Item = IoResult<u8>>,
{
    while let Some((wire_type, field_number)) = try_get_wire_type_and_field_number(iter)? {
        fn byte<I: Iterator<Item = IoResult<u8>>>(iter: &mut I) -> Result<u8> {
            Ok(iter.next().ok_or(ErrorKind::UnexpectedInputTermination)??)
        }
        let field_data = match wire_type {
            WireType::Variant => FieldData::Variant(Variant::decode_bytes(iter)?),
            WireType::Bits32 => {
                FieldData::Bits32([byte(iter)?, byte(iter)?, byte(iter)?, byte(iter)?])
            }
            WireType::Bits64 => FieldData::Bits64([
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
                byte(iter)?,
            ]),
            WireType::LengthDelimited => {
                let length = usize::try_from(Variant::decode_bytes(iter)?.to_i32()?)
                    .map_err(|_| ErrorKind::InvalidFieldLength)?;
                iter.push_scope(length);
                FieldData::LengthDelimited(&mut *iter)
            }
            WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
        };

        <Msg as DeserFieldsFromBytesIter>::deser_field(message, field_number, field_data)?;

        if let WireType::LengthDelimited = wire_type {
            iter.pop_scope();
        }
    }
    Ok(())
}

fn try_get_wire_type_and_field_number<I>(iter: &mut I) -> Result<Option<(WireType, i32)>>
where
    I: Iterator<Item = IoResult<u8>>,
{
    let mut peekable = iter.peekable();
    if let None = peekable.peek() {
        // Found EOF at first byte. Successfull failure.
        return Ok(None);
    }
    let key = Variant::decode_bytes(&mut peekable)?.to_u32()?;
    Ok(Some((
        WireType::try_from((key & 0x07) as i32)?,
        <i32 as TryFrom<u32>>::try_from(key >> 3).map_err(|_| ErrorKind::InvalidFieldNumber)?,
    )))
}

pub struct ScopedIter<I> {
    iter: I,
    pos: usize,
    end_stack: Vec<usize>,
}
impl<I> ScopedIter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: Vec::new(),
        }
    }
    pub fn new_with_len(iter: I, len: usize) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: vec![len],
        }
    }
    fn push_scope(&mut self, new_len: usize) {
        if let Some(cur_end) = self.end_stack.last() {
            assert!(self.pos + new_len <= *cur_end);
        }
        self.end_stack.push(self.pos + new_len);
    }
    fn pop_scope(&mut self) {
        if let Some(cur_end) = self.end_stack.last() {
            assert_eq!(self.pos, *cur_end);
        }
        self.end_stack.pop().unwrap();
    }
}
impl<I> Iterator for ScopedIter<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.end_stack.last() {
            Some(end) => {
                if self.pos < *end {
                    self.pos += 1;
                    self.iter.next()
                } else {
                    None
                }
            }
            None => {
                self.pos += 1;
                self.iter.next()
            }
        }
    }
}

#[test]
fn test_scoped_iter() {
    let s1 = ScopedIter::new("abcdefg".chars());
    assert_eq!("abcdefg", s1.collect::<String>());

    let s2 = ScopedIter::new_with_len("abcdefg".chars(), 5);
    assert_eq!("abcde", s2.collect::<String>());

    let mut s3 = ScopedIter::new("abcdefg".chars());
    s3.push_scope(5);
    assert_eq!("abcde", s3.collect::<String>());

    let mut s4 = ScopedIter::new("abcdefg".chars());
    assert_eq!(Some('a'), s4.next());
    s4.push_scope(5);
    assert_eq!("bcdef", s4.by_ref().collect::<String>());
    s4.pop_scope();
    assert_eq!("g", s4.by_ref().collect::<String>());
}

/// Converts `Result<u8, std::io::IoError>` into `Result<Variant, ErrorKind>`.
pub struct Variants<I: Iterator<Item = IoResult<u8>>> {
    iter: I,
}
impl<I: Iterator<Item = IoResult<u8>>> Variants<I> {
    pub fn new(iter: I) -> Self {
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
