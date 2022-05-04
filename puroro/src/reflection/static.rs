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
use crate::Result;
use desc::FieldDescriptor;

pub trait Reflection {
    fn has_field<FD: FieldDescriptor>(&self) -> Result<bool>;
    fn get_uint32<FD: FieldDescriptor>(&self) -> Result<u32>;
    fn get_string<FD: FieldDescriptor>(&self) -> Result<&str>;
    type ChildReflection<'a, FD>
    where
        Self: 'a,
        FD: FieldDescriptor;
    fn get_message<FD: FieldDescriptor>(&self) -> Result<Self::ChildReflection<'_, FD>>;
}

impl<T: Reflection> Reflection for &'_ T {
    fn has_field<FD: FieldDescriptor>(&self) -> Result<bool> {
        <T as Reflection>::has_field::<FD>(self)
    }
    fn get_uint32<FD: FieldDescriptor>(&self) -> Result<u32> {
        <T as Reflection>::get_uint32::<FD>(self)
    }
    fn get_string<FD: FieldDescriptor>(&self) -> Result<&str> {
        <T as Reflection>::get_string::<FD>(self)
    }
    type ChildReflection<'a, FD>
    = <T as Reflection>::ChildReflection<'a, FD>
    where
        Self: 'a,
        FD: FieldDescriptor;
    fn get_message<FD: FieldDescriptor>(&self) -> Result<Self::ChildReflection<'_, FD>> {
        <T as Reflection>::get_message::<FD>(self)
    }
}
