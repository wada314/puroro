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
        bytes: Iter,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<Iter>
    where
        Iter: Iterator<Item = IoResult<u8>> + ScopedIterator;
}

impl<MP, FieldsType, SharedType, Iter> DeserFromBytesImpl<Iter>
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Self: MatchFieldNumber<DeserOwnedFieldHandler<Iter>>,
{
    fn deser_from_bytes_impl(
        &mut self,
        bytes: Iter,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<Iter>
    where
        Iter: Iterator<Item = IoResult<u8>> + ScopedIterator,
    {
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
        Ok(bytes)
    }
}

impl<MP, FieldsType, SharedType> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType> {
    pub fn deser_from_bytes<Iter>(&mut self, bytes: Iter, options: DeserOptions) -> Result<()>
    where
        Self: DeserFromBytesImpl<ScopedIter<Iter>>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        let scoped_iter = ScopedIter::new(bytes);
        self.deser_from_bytes_impl(scoped_iter, options, 0)?;
        debug_assert!(!scoped_iter.is_in_scope());
        debug_assert!(scoped_iter.next().is_none());
        Ok(())
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

pub trait ScopedIterator: Iterator {
    fn push_scope(&mut self, new_len: usize);
    fn pop_scope(&mut self);
}
impl<I: Iterator> ScopedIterator for ScopedIter<I> {
    fn push_scope(&mut self, new_len: usize) {
        <ScopedIter<I>>::push_scope(self, new_len)
    }
    fn pop_scope(&mut self) {
        <ScopedIter<I>>::pop_scope(self)
    }
}
impl<'a, I: Iterator> ScopedIterator for &'a mut ScopedIter<I> {
    fn push_scope(&mut self, new_len: usize) {
        <ScopedIter<I>>::push_scope(self, new_len)
    }
    fn pop_scope(&mut self) {
        <ScopedIter<I>>::pop_scope(self)
    }
}

pub struct PosIter<I> {
    iter: I,
    pos: usize,
}
impl<I> PosIter<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, pos: 0 }
    }
    pub(crate) fn pos(&self) -> usize {
        self.pos
    }
}
impl<I: Iterator> Iterator for PosIter<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        self.iter.next()
    }
}

pub struct ScopedIter<I> {
    iter: PosIter<I>,
    end_pos: Vec<usize>,
}
impl<I> ScopedIter<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter: PosIter::new(iter),
            end_pos: Vec::new(),
        }
    }
    pub(crate) fn push_scope(&mut self, scope_len: usize) {
        let pos = self.iter.pos();
        self.end_pos.push(pos + scope_len);
    }
    pub(crate) fn pop_scope(&mut self) {
        let last_end_pos = self.end_pos.pop().expect("Non-corresponding push/pop");
        debug_assert_eq!(
            self.iter.pos(),
            last_end_pos,
            "The iter scope pop at unexpected location"
        );
    }
    pub(crate) fn is_in_scope(&self) -> bool {
        !self.end_pos.is_empty()
    }
}
impl<'a, I> Iterator for ScopedIter<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_pos) = self.end_pos.last() {
            if self.iter.pos() >= *last_pos {
                None
            } else {
                self.iter.next()
            }
        } else {
            self.iter.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_size = self.iter.size_hint();
        if let Some(last_pos) = self.end_pos.last() {
            (inner_size.0, Some(last_pos - self.iter.pos()))
        } else {
            inner_size
        }
    }
}

#[test]
fn test_scoped_iter() {
    let p1 = PosIter::new("abcdefg".chars());
    assert_eq!("abcdefg", p1.collect::<String>());

    let mut p2 = PosIter::new("abcdefg".chars());
    let s2 = p2.scope(4);
    assert_eq!("abcd", s2.collect::<String>());

    let mut p3 = PosIter::new("abcdefg".chars());
    assert_eq!(Some('a'), p3.next());
    let s3 = p3.scope(4);
    assert_eq!("bcde", s3.collect::<String>());
}
