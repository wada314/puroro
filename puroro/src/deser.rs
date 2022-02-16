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

use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::{
    FieldHandlerMut, FieldProperties, GetField, GetFieldMut, MatchFieldNumber, MessageProperties,
};
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Result as IoResult;
use ::std::marker::PhantomData;

pub struct DeserSimpleFieldHandler<MP, FieldsType, SharedType, Iter> {
    bytes: Iter,
    wire_type: WireType,
    _phantom: PhantomData<(MP, FieldsType, SharedType)>,
}

impl<MP, FieldsType, SharedType, Iter> FieldHandlerMut
    for DeserSimpleFieldHandler<MP, FieldsType, SharedType, Iter>
where
    MP: MessageProperties,
{
    type MP = MP;
    type FieldsType = FieldsType;
    type SharedType = SharedType;

    fn handle_mut<const NUMBER: i32>(
        &mut self,
        field: &mut <Self::FieldsType as GetField<NUMBER>>::Type,
        shared: &mut Self::SharedType,
    ) -> Result<()>
    where
        Self::FieldsType: GetFieldMut<NUMBER>,
        Self::MP: MessageProperties,
        <Self::MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    {
        todo!()
    }
}

impl<MP, Fields, Shared> MessageImpl<MP, tags::SimpleImpl, Fields, Shared>
where
    MP: MessageProperties,
{
    pub fn deser_from_bytes<Iter>(&mut self, mut bytes: Iter) -> Result<()>
    where
        Self: MatchFieldNumber<DeserSimpleFieldHandler<MP, Fields, Shared, Iter>>,
        Iter: Iterator<Item = IoResult<u8>>,
    {
        loop {
            match try_get_wire_type_and_field_number(bytes.by_ref()) {
                Ok(Some((wire_type, number))) => {
                    let mut deser = DeserSimpleFieldHandler {
                        bytes: bytes.by_ref(),
                        wire_type,
                        _phantom: PhantomData,
                    };
                    self.match_field_number_mut(number, &mut deser)?;
                }
                Ok(None) => {
                    // end of input iterator, correct exit
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}

fn try_get_wire_type_and_field_number<I>(mut iter: I) -> Result<Option<(WireType, i32)>>
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
