// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::internal::impls::owned::deser::DeserOwnedFieldHandler;
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::MatchFieldNumber;
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Result as IoResult;

#[derive(Clone)]
pub struct DeserOptions {
    pub recursion_limit: Option<usize>,
    pub do_utf8_check: bool,
}
impl Default for DeserOptions {
    fn default() -> Self {
        Self {
            recursion_limit: Some(30),
            do_utf8_check: true,
        }
    }
}

pub trait DeserFromBytesImpl<Iter> {
    fn deser_from_bytes_impl(
        &mut self,
        bytes: &mut ScopedIter<Iter>,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<()>;
}

impl<MP, FieldsType, SharedType, Iter> DeserFromBytesImpl<Iter>
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Iter: Iterator<Item = IoResult<u8>>,
    for<'a> Self: MatchFieldNumber<DeserOwnedFieldHandler<'a, Iter>>,
{
    fn deser_from_bytes_impl(
        &mut self,
        bytes: &mut ScopedIter<Iter>,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<()> {
        let mut handler = DeserOwnedFieldHandler {
            bytes,
            wire_type: WireType::Variant, // a random value
            recursion_level,
            options: options.clone(),
        };
        while let Some((wire_type, number)) =
            try_get_wire_type_and_field_number(&mut handler.bytes)?
        {
            handler.wire_type = wire_type;
            self.match_field_number_mut(number, &mut handler)?;
        }
        Ok(())
    }
}

impl<MP, FieldsType, SharedType> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType> {
    pub fn deser_from_bytes<'a, Iter>(
        &'a mut self,
        bytes: Iter,
        options: DeserOptions,
    ) -> Result<()>
    where
        Self: DeserFromBytesImpl<Iter>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        let mut scoped_iter = ScopedIter::new(bytes);
        self.deser_from_bytes_impl(&mut scoped_iter, options, 0)
    }
}

fn try_get_wire_type_and_field_number<I>(iter: I) -> Result<Option<(WireType, i32)>>
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
        (key >> 3)
            .try_into()
            .map_err(|_| ErrorKind::InvalidFieldNumber)?,
    )))
}

pub trait ScopedIterator<'a>: Iterator {
    type Scoped<'b>: ScopedIterator<'b> + Iterator<Item = Self::Item>
    where
        Self: 'a + 'b,
        'a: 'b;
    fn push_scope<'b>(&'a mut self, new_len: usize) -> Self::Scoped<'b>
    where
        'a: 'b;
}

pub struct ScopedIter<I> {
    iter: I,
    pos: usize,
    end_stack: Vec<usize>,
}
impl<I> ScopedIter<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: Vec::new(),
        }
    }
    pub(crate) fn push_scope(&mut self, new_len: usize) {
        if let Some(cur_end) = self.end_stack.last() {
            assert!(self.pos + new_len <= *cur_end);
        }
        self.end_stack.push(self.pos + new_len);
    }
    pub(crate) fn pop_scope(&mut self) {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_size = self.iter.size_hint();
        match self.end_stack.last() {
            Some(end) => (inner_size.0, Some(end - self.pos)),
            None => inner_size,
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
