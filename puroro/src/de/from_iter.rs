use super::{FieldData, WireType};
use crate::tags;
use crate::variant::Variant;
use crate::ErrorKind;
use crate::{FieldTypeGen, FunctorForFieldMut, Message, Result};
use ::std::convert::TryFrom as _;
use ::std::io::Result as IoResult;

pub trait ImplIsDeserializableFromIter<LabelAndType>: FieldTypeGen<LabelAndType> {
    fn deser_field<I>(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        input: FieldData<I>,
    ) -> Result<()>
    where
        I: Iterator<Item = IoResult<u8>>;
}

fn deser_from_iter<Msg, I>(message: &mut Msg, input_iter: I) -> Result<()>
where
    Msg: crate::Message,
    I: Iterator<Item = IoResult<u8>>,
{
    let mut scoped_iter = ScopedIter::new(input_iter);
    deser_from_scoped_iter(message, &mut scoped_iter)
}

fn deser_from_scoped_iter<Msg, I>(message: &mut Msg, iter: &mut ScopedIter<I>) -> Result<()>
where
    Msg: crate::Message,
    I: Iterator<Item = IoResult<u8>>,
{
    match try_get_wire_type_and_field_number(iter)? {
        None => {
            // reached at the end of this message.
            Ok(())
        }
        Some((wire_type, field_number)) => {
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
                    FieldData::LengthDelimited(iter)
                }
                WireType::StartGroup | WireType::EndGroup => Err(ErrorKind::GroupNotSupported)?,
            };
            let mut functor = DeserFieldFunctor::<<Msg as Message>::ImplTypeTag, _>::new(
                wire_type.clone(),
                field_data,
            );
            message.apply_mut_to_field_with_number(field_number, functor)
        }
    }
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
    let key = Variant::decode_bytes(&mut peekable)?.to_i32()?;
    Ok(Some((WireType::try_from(key & 0x07)?, (key >> 3))))
}

struct ScopedIter<I> {
    iter: I,
    pos: usize,
    end_stack: Vec<usize>,
}
impl<I> ScopedIter<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: Vec::new(),
        }
    }
    fn new_with_len(iter: I, len: usize) -> Self {
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

struct DeserFieldFunctor<'a, ImplTag, I> {
    phantom: std::marker::PhantomData<ImplTag>,
    wire_type: WireType,
    field_data: FieldData<&'a mut I>,
}

impl<'a, ImplTag, I> DeserFieldFunctor<'a, ImplTag, I> {
    fn new(wire_type: WireType, field_data: FieldData<&'a mut I>) -> Self {
        Self {
            phantom: std::marker::PhantomData,
            wire_type,
            field_data,
        }
    }
}

impl<'a, ImplTag, I> FunctorForFieldMut for DeserFieldFunctor<'a, ImplTag, I> {
    type ImplTypeTag = ImplTag;
    fn apply_mut<LabelAndType>(
        &mut self,
        field: &mut <Self::ImplTypeTag as FieldTypeGen<LabelAndType>>::Type,
    ) -> Result<()>
    where
        Self::ImplTypeTag: FieldTypeGen<LabelAndType>,
    {
        todo!()
    }
}
