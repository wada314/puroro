use super::{FieldData, WireType};
use crate::tags;
use crate::variant::Variant;
use crate::{FieldTypeGen, FunctorForFieldMut, Message, Result};
use ::std::convert::TryFrom as _;
use ::std::io::Result as IoResult;

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
            let mut functor = DeserFieldFunctor::<<Msg as Message>::ImplTypeTag>::new();
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

struct DeserFieldFunctor<ImplTag> {
    phantom: std::marker::PhantomData<ImplTag>,
}

impl<ImplTag> DeserFieldFunctor<ImplTag> {
    fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

impl<ImplTag> FunctorForFieldMut for DeserFieldFunctor<ImplTag> {
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
