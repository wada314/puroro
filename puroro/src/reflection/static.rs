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
use crate::Result;
use desc::FieldDescriptorExt;

pub trait Reflection {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool>;
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32>;
    type StringFieldType<'a, FD: FieldDescriptorExt>: AsRef<str> + Default
    where
        Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>>;
    type MessageFieldType<'a, FD: 'a + FieldDescriptorExt>: Reflection
    where
        Self: 'a;
    fn get_message<'a, FD: 'a + FieldDescriptorExt>(
        &'a self,
    ) -> Result<Self::MessageFieldType<'a, FD>>;
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
    type MessageFieldType<'a, FD: 'a + FieldDescriptorExt> = T::MessageFieldType<'a, FD>
    where
        Self: 'a;
    fn get_message<'a, FD: 'a + FieldDescriptorExt>(
        &'a self,
    ) -> Result<Self::MessageFieldType<'a, FD>> {
        <T as Reflection>::get_message::<FD>(self)
    }
}

impl<T: Reflection> Reflection for Option<T> {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool> {
        self.as_ref()
            .map_or(Ok(false), |t| <T as Reflection>::has_field::<FD>(t))
    }
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32> {
        self.as_ref().map_or(Ok(Default::default()), |t| {
            <T as Reflection>::get_uint32::<FD>(t)
        })
    }
    type StringFieldType<'a, FD: FieldDescriptorExt> = T::StringFieldType<'a, FD> where Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>> {
        self.as_ref().map_or(Ok(Default::default()), |t| {
            <T as Reflection>::get_string::<FD>(t)
        })
    }
    type MessageFieldType<'a, FD: 'a + FieldDescriptorExt> = Option<T::MessageFieldType<'a, FD>>
    where
        Self: 'a;
    fn get_message<'a, FD: 'a + FieldDescriptorExt>(
        &'a self,
    ) -> Result<Self::MessageFieldType<'a, FD>> {
        self.as_ref().map_or(Ok(None), |t| {
            <T as Reflection>::get_message::<FD>(t).map(|m| Some(m))
        })
    }
}

impl Reflection for () {
    fn has_field<FD: FieldDescriptorExt>(&self) -> Result<bool> {
        Ok(false)
    }
    fn get_uint32<FD: FieldDescriptorExt>(&self) -> Result<u32> {
        Ok(0)
    }

    type StringFieldType<'a, FD: FieldDescriptorExt> = &'a str
    where
        Self: 'a;
    fn get_string<FD: FieldDescriptorExt>(&self) -> Result<Self::StringFieldType<'_, FD>> {
        Ok("")
    }

    type MessageFieldType<'a, FD: 'a + FieldDescriptorExt> = ()
    where
        Self: 'a;
    fn get_message<'a, FD: 'a + FieldDescriptorExt>(
        &'a self,
    ) -> Result<Self::MessageFieldType<'a, FD>> {
        Ok(())
    }
}
