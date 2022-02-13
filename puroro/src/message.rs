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

use crate::tags;
use crate::{AsMessageImplRef, DefaultIn, Result};
use ::std::io;
use ::std::marker::PhantomData;

use crate::internal::{FieldProperties, GetField, GetFieldMut, MessageProperties};

pub struct MessageImpl<MP, ImplTag, Fields, Shared> {
    pub(crate) fields: Fields,
    pub(crate) shared: Shared,
    _phantom: PhantomData<(MP, ImplTag)>,
}

impl<MP, ImplTag, Fields, Shared> MessageImpl<MP, ImplTag, Fields, Shared> {
    pub fn from_raw_parts(fields: Fields, shared: Shared) -> Self {
        Self {
            fields,
            shared,
            _phantom: PhantomData,
        }
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
        Ok(())
    }
}

pub trait FieldHandler {
    type MP;
    type FieldsType;
    type SharedType;
    fn handle_mut<const NUMBER: i32>(
        &mut self,
        field: &mut <Self::FieldsType as GetField<NUMBER>>::Type,
        shared: &mut Self::SharedType,
    ) -> Result<()>
    where
        Self::FieldsType: GetFieldMut<NUMBER>,
        Self::MP: MessageProperties,
        <Self::MP as MessageProperties>::Fields<NUMBER>: FieldProperties;
}

pub struct DeserSimpleImpl<MP, FieldsType, SharedType, Iter> {
    bytes: Iter,
    _phantom: PhantomData<(MP, FieldsType, SharedType)>,
}

impl<MP, FieldsType, SharedType, Iter> FieldHandler
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

pub trait MatchFieldNumber<FH: FieldHandler> {
    fn match_field_number_mut(&mut self, number: i32, handler: &mut FH) -> Result<()>;
}

impl<MP, ImplTag, Fields, Shared> Default for MessageImpl<MP, ImplTag, Fields, Shared>
where
    Fields: Default,
    Shared: Default,
{
    fn default() -> Self {
        Self {
            fields: Default::default(),
            shared: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<MP, ImplTag, Fields, Shared, Alloc> DefaultIn for MessageImpl<MP, ImplTag, Fields, Shared>
where
    Fields: DefaultIn<AllocatorType = Alloc>,
    Shared: DefaultIn<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type AllocatorType = <Shared as DefaultIn>::AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        Self {
            fields: DefaultIn::default_in(Clone::clone(&alloc)),
            shared: DefaultIn::default_in(Clone::clone(&alloc)),
            _phantom: Default::default(),
        }
    }
}

impl<MP, ImplTag, Fields, Shared> AsMessageImplRef for MessageImpl<MP, ImplTag, Fields, Shared> {
    type MessageImplType = MessageImpl<MP, ImplTag, Fields, Shared>;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        self
    }
}
