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

use crate::internal::bool::{False, True};
use crate::internal::impls::owned::deser::{DeserOwnedFieldHandler};
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::{
    Bitfield, FieldHandlerBase, FieldHandlerMut, FieldProperties, GetField, GetFieldMut,
    MatchFieldNumber, MessageProperties, SharedBitfield,
};
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Result as IoResult;
use ::std::marker::PhantomData;

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

impl<MP, FieldsType, SharedType> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    MP: MessageProperties,
{
    pub fn deser_from_bytes<'a, Iter>(
        &'a mut self,
        bytes: Iter,
        options: DeserOptions,
    ) -> Result<()>
    where
        for<'b> Self:
            MatchFieldNumber<DeserOwnedFieldHandler<MP, FieldsType, SharedType, &'b mut Iter>>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        self.deser_from_bytes_impl(bytes, options, 0)
    }

    fn deser_from_bytes_impl<'a, Iter>(
        &'a mut self,
        mut bytes: Iter,
        options: DeserOptions,
        recursion_level: usize,
    ) -> Result<()>
    where
        for<'b> Self:
            MatchFieldNumber<DeserOwnedFieldHandler<MP, FieldsType, SharedType, &'b mut Iter>>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        while let Some((wire_type, number)) = try_get_wire_type_and_field_number(bytes.by_ref())? {
            let mut handler = DeserOwnedFieldHandler {
                bytes: bytes.by_ref(),
                wire_type,
                recursion_level,
                options: options.clone(),
                _phantom: PhantomData::<(MP, FieldsType, SharedType)>,
            };
            self.match_field_number_mut(number, &mut handler)?;
        }
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
