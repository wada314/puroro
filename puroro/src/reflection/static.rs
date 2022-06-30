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
use crate::{ErrorKind, Result};
use desc::FieldDescriptorExt;
use metako::{If, Number};
use typenum::ToInt;

use self::desc::MessageDescriptorExt;

pub trait Reflection {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool>;
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32>;
    type StringFieldType<'a, FD: FieldDescriptorExt>: AsRef<str>
    where
        Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>>;
    type MessageFieldType<'a, FD: FieldDescriptorExt>: Reflection
    where
        Self: 'a,
        FD::MaybeFieldMessageDescriptor: 'a + MessageDescriptorExt;
    fn get_message<FD: FieldDescriptorExt>(&self) -> Result<Self::MessageFieldType<'_, FD>>
    where
        FD::MaybeFieldMessageDescriptor: MessageDescriptorExt;
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
    type MessageFieldType<'a, FD: FieldDescriptorExt> = T::MessageFieldType<'a, FD>
    where
        Self: 'a,
        FD::MaybeFieldMessageDescriptor:'a + MessageDescriptorExt;
    fn get_message<FD: FieldDescriptorExt>(&self) -> Result<Self::MessageFieldType<'_, FD>>
    where
        FD::MaybeFieldMessageDescriptor: MessageDescriptorExt,
    {
        <T as Reflection>::get_message::<FD>(self)
    }
}

pub trait FdListAndFieldTypeList {
    type FieldList;
    fn get_uint32<FD: FieldDescriptorExt>(field_list: &Self::FieldList) -> Result<u32>;
}

impl FdListAndFieldTypeList for ((), ()) {
    type FieldList = ();

    fn get_uint32<FD: FieldDescriptorExt>(_field_list: &Self::FieldList) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<FD: FieldDescriptorExt, FDRest, FieldRest> FdListAndFieldTypeList
    for ((FD, FDRest), (u32, FieldRest))
where
    (FDRest, FieldRest): FdListAndFieldTypeList<FieldList = FieldRest>,
{
    type FieldList = (u32, FieldRest);

    fn get_uint32<ParamFD: FieldDescriptorExt>(field_list: &Self::FieldList) -> Result<u32> {
        if <FD::Number as ToInt<i32>>::to_int() == <ParamFD::Number as ToInt<i32>>::to_int() {
            Ok(field_list.0)
        } else {
            <(FDRest, FieldRest) as FdListAndFieldTypeList>::get_uint32::<ParamFD>(&field_list.1)
        }
    }
}
