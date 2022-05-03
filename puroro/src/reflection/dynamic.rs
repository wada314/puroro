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

pub trait DynamicReflection {
    fn has_field(&self, fd: &FieldDescriptor) -> Result<bool>;
    fn get_uint32(&self, fd: &FieldDescriptor) -> Result<u32>;
    fn get_string(&self, fd: &FieldDescriptor) -> Result<&str>;
    fn get_message(&self, fd: &FieldDescriptor) -> Result<&dyn DynamicReflection>;
}

impl<T> DynamicReflection for &T
where
    T: DynamicReflection,
{
    fn has_field(&self, fd: &FieldDescriptor) -> Result<bool> {
        <T as DynamicReflection>::has_field(*self, fd)
    }

    fn get_uint32(&self, fd: &FieldDescriptor) -> Result<u32> {
        <T as DynamicReflection>::get_uint32(*self, fd)
    }

    fn get_string(&self, fd: &FieldDescriptor) -> Result<&str> {
        <T as DynamicReflection>::get_string(*self, fd)
    }

    fn get_message(&self, fd: &FieldDescriptor) -> Result<&dyn DynamicReflection> {
        <T as DynamicReflection>::get_message(*self, fd)
    }
}
