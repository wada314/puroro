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

use super::DeserMessageFromBytesIter;
use crate::internal::types::{FieldData, WireType};
use crate::internal::variant::Variant;
use crate::{ErrorKind, Result};
use ::std::convert::TryFrom;
use ::std::io::Result as IoResult;

pub fn deser_from_iter<Msg, I>(message: &mut Msg, input_iter: I) -> Result<()>
where
    Msg: ?Sized + DeserMessageFromBytesIter,
    I: Iterator<Item = IoResult<u8>>,
{
    let mut scoped_iter = ScopedIter::new(input_iter);
    deser_from_scoped_iter(message, &mut scoped_iter)
}

pub fn deser_from_scoped_iter<Msg, I>(message: &mut Msg, iter: &mut ScopedIter<I>) -> Result<()>
where
    Msg: ?Sized + DeserMessageFromBytesIter,
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

        <Msg as DeserMessageFromBytesIter>::deser_field(message, field_number, field_data)?;

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

/// Converts `Iterator<Item=IoResult<u8>>` into `Result<Variant, ErrorKind>`.
pub struct Variants<I> {
    iter: I,
}
impl<I> Variants<I> {
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
