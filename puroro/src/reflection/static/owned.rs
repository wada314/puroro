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

mod md_fd_into_owned_type;

use super::desc::{FieldDescriptorExt, MessageDescriptorExt};
use super::Reflection;
use crate::Result;
use ::metako::*;
pub use md_fd_into_owned_type::MdFdIntoOwnedTypeFunctor;

pub struct OwnedMessage<MD>
where
    MD: MessageDescriptorExt,
{
    pub fields: MD::GetOwnedFieldList,
}

impl<MD> OwnedMessage<MD>
where
    MD: MessageDescriptorExt,
{
    pub fn get_message<FD: FieldDescriptorExt>(
        &self,
    ) -> Result<&OwnedMessage<FD::MaybeFieldMessageDescriptor>>
    where
        FD::MaybeFieldMessageDescriptor: MessageDescriptorExt,
    {
        todo!()
    }
}

impl<MD> Default for OwnedMessage<MD>
where
    MD: MessageDescriptorExt,
    MD::GetOwnedFieldList: Default,
{
    fn default() -> Self {
        Self {
            fields: Default::default(),
        }
    }
}

impl<MD> Reflection for OwnedMessage<MD>
where
    MD: MessageDescriptorExt,
{
    fn has_field<FD: FieldDescriptorExt>(&self) -> crate::Result<bool> {
        todo!()
    }

    fn get_uint32<FD: FieldDescriptorExt>(&self) -> crate::Result<u32> {
        todo!()
    }

    type StringFieldType<'a, FD: FieldDescriptorExt> = &'a str where Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>> {
        todo!()
    }

    type MessageFieldType<'a, FD: FieldDescriptorExt> = &'a OwnedMessage<FD::MaybeFieldMessageDescriptor>
    where
        Self: 'a,
        FD::MaybeFieldMessageDescriptor:
            'a + MessageDescriptorExt;
    fn get_message<FD: FieldDescriptorExt>(&self) -> Result<Self::MessageFieldType<'_, FD>>
    where
        FD::MaybeFieldMessageDescriptor: MessageDescriptorExt,
    {
        todo!()
    }
}
