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

use crate::internal::{
    FieldHandlerMut, FieldProperties, GetField, GetFieldMut, MatchFieldNumber, MessageProperties,
};
use crate::tags;
use crate::{MessageImpl, Result};
use ::std::io;
use ::std::marker::PhantomData;

pub struct DeserSimpleImpl<MP, FieldsType, SharedType, Iter> {
    bytes: Iter,
    _phantom: PhantomData<(MP, FieldsType, SharedType)>,
}

impl<MP, FieldsType, SharedType, Iter> FieldHandlerMut
    for DeserSimpleImpl<MP, FieldsType, SharedType, Iter>
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
    pub fn deser_from_bytes<Iter>(&mut self, bytes: Iter) -> Result<()>
    where
        Self: MatchFieldNumber<DeserSimpleImpl<MP, Fields, Shared, Iter>>,
        Iter: Iterator<Item = io::Result<u8>>,
    {
        let mut deser = DeserSimpleImpl {
            bytes,
            _phantom: PhantomData,
        };
        self.match_field_number_mut(1, &mut deser)?;
        todo!();
        Ok(())
    }
}
