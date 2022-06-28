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

pub mod desc;
pub mod owned;
use crate::tags;
use crate::Result;
use desc::FieldDescriptorExt;

use self::desc::MessageDescriptorExt;

pub trait Reflection {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool>;
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32>;
    type StringFieldType<'a, FD: FieldDescriptorExt>: AsRef<str> where Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>>;
    type MessageFieldType<'a, FD>: Reflection
    where
        FD: FieldDescriptorExt,
        FD::MaybeFieldMessageDescriptor: 'a + MessageDescriptorExt;
}

impl<T: Reflection> Reflection for &'_ T {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool> {
        <T as Reflection>::has_field::<FD>(self)
    }
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32> {
        <T as Reflection>::get_uint32::<FD>(self)
    }
    type StringFieldType<'a, FD: FieldDescriptorExt> = T::StringFieldType<'a, FD> where Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>> {
        <T as Reflection>::get_string::<FD>(self)
    }
    type MessageFieldType<'a, FD>  = T::MessageFieldType<'a, FD>
    where
        FD: FieldDescriptorExt,
        FD::MaybeFieldMessageDescriptor:
            'a + MessageDescriptorExt;
}
