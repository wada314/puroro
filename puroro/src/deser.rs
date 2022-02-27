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
    fn deser_from_bytes_impl<'a>(
        &mut self,
        bytes: Iter,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<()>
    where
        Iter: Iterator<Item = IoResult<u8>> + ScopedIterator;
}

impl<MP, FieldsType, SharedType, Iter> DeserFromBytesImpl<Iter>
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Self: MatchFieldNumber<DeserOwnedFieldHandler<Iter>>,
{
    fn deser_from_bytes_impl<'a>(
        &mut self,
        bytes: Iter,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<()>
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
        Ok(())
    }
}

impl<MP, FieldsType, SharedType> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType> {
    pub fn deser_from_bytes<Iter>(&mut self, bytes: Iter, options: DeserOptions) -> Result<()>
    where
        Self: DeserFromBytesImpl<PosIter<Iter>>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        let pos_iter = PosIter::new(bytes);
        self.deser_from_bytes_impl(pos_iter, options, 0)
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
    type Scoped<'a>: ScopedIterator + Iterator<Item = Self::Item>
    where
        Self: 'a;
    fn scope(&mut self, new_len: usize) -> Self::Scoped<'_>;
}
impl<'a, I: Iterator> ScopedIterator for ScopedIter<'a, I>
where
    Self: 'a,
{
    type Scoped<'b>
    where
        Self: 'b,
    = ScopedIter<'b, I>;
    fn scope(&mut self, scope_len: usize) -> Self::Scoped<'_> {
        <ScopedIter<I>>::scope(self, scope_len)
    }
}
impl<I: Iterator> ScopedIterator for PosIter<I> {
    type Scoped<'a>
    where
        Self: 'a,
    = ScopedIter<'a, I>;
    fn scope(&mut self, new_len: usize) -> Self::Scoped<'_> {
        <ScopedIter<I>>::new(self, self.pos() + new_len)
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

pub struct ScopedIter<'a, I> {
    iter: &'a mut PosIter<I>,
    end_pos: usize,
}
impl<'a, I> ScopedIter<'a, I> {
    pub(crate) fn new(iter: &'a mut PosIter<I>, end_pos: usize) -> Self {
        Self { iter, end_pos }
    }
    pub(crate) fn scope(&mut self, scope_len: usize) -> ScopedIter<'_, I> {
        let pos = self.iter.pos();
        ScopedIter::new(self.iter, pos + scope_len)
    }
}
impl<'a, I> Iterator for ScopedIter<'a, I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.pos() >= self.end_pos {
            None
        } else {
            self.iter.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_size = self.iter.size_hint();
        (inner_size.0, Some(self.end_pos - self.iter.pos()))
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
