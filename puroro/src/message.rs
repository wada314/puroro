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

use crate::desc::FieldDescriptor;
use crate::{ErrorKind, Result};

pub trait MessageImpl<'msg, MD> {
    fn try_get_u32<FD>(&'msg self) -> Result<u32>
    where
        Self: MessageFieldGetter<'msg, FD, u32>,
    {
        <Self as MessageFieldGetter<FD, u32>>::try_get_field(&self)
    }
    fn try_get_str<FD>(&'msg self) -> Result<&str>
    where
        Self: MessageFieldGetter<'msg, FD, &'msg str>,
    {
        <Self as MessageFieldGetter<FD, &'msg str>>::try_get_field(&self)
    }
}
pub trait MessageFieldGetter<'msg, FD, R> {
    fn try_get_field(&'msg self) -> Result<R> {
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
