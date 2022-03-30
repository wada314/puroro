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

use crate::desc::{FieldDescriptor, StaticMessageDescriptor};
use crate::{ErrorKind, Result};

pub trait Message {
    type Descriptor: StaticMessageDescriptor;
    fn try_get_u32<'a, FD, const NUMBER: i32>(&'a self) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_repeated_u32_boxed<'a, FD, const NUMBER: i32>(
        &'a self,
    ) -> Result<Box<dyn 'a + Iterator<Item = u32>>> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_str<'a, FD, const NUMBER: i32>(&'a self) -> Result<&'a str> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait DynMessage {
    fn try_get_u32_dyn<'a>(&'a self, _: &'a FieldDescriptor) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_repeated_u32_dyn_boxed<'a>(
        &'a self,
        _: &'a FieldDescriptor,
    ) -> Result<Box<dyn 'a + Iterator<Item = u32>>> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_str_dyn<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a str> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_message_dyn<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a dyn DynMessage> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait MessageMut {}
