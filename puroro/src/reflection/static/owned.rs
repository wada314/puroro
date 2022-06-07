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

use super::desc::{FieldDescriptor, MessageDescriptor};
use super::Reflection;
use crate::tags;
use crate::Result;
use ::metako::*;
pub use md_fd_into_owned_type::MdFdIntoOwnedType;

mod boxed_message;
mod md_fd_into_owned_type;

pub struct OwnedMessage<MD>
where
    MD: MessageDescriptor,
{
    pub fields: MD::GetOwnedFieldList,
}

impl<MD> OwnedMessage<MD>
where
    MD: MessageDescriptor,
{
    pub fn get_message<FD: FieldDescriptor>(&self) -> Result<&OwnedMessage<<FD::Type as tags::FieldTypeTag>::MessageDescriptor>>
    where
        <FD::Type as tags::FieldTypeTag>::MessageDescriptor: MessageDescriptor,
        list::Map<MdFdIntoOwnedType>: Func<<<FD::Type as tags::FieldTypeTag>::MessageDescriptor as MessageDescriptor>::GetFieldListAsMdFd>,
    {
        todo!()
    }
}

impl<MD> Default for OwnedMessage<MD>
where
    MD: MessageDescriptor,
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
    MD: MessageDescriptor,
    list::Map<MdFdIntoOwnedType>: Func<MD::GetFieldListAsMdFd>,
{
    fn has_field<FD: super::desc::FieldDescriptor>(&self) -> crate::Result<bool> {
        todo!()
    }

    fn get_uint32<FD: super::desc::FieldDescriptor>(&self) -> crate::Result<u32> {
        todo!()
    }

    fn get_string<FD: super::desc::FieldDescriptor>(&self) -> crate::Result<&str> {
        todo!()
    }

    // type MessageFieldType<FD: FieldDescriptor>;
}
