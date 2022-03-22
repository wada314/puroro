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

//! assume a proto like this as input:
//! message Person {
//!     optional string name = 1;
//!     optional uint32 age = 2;
//!     optional Person partner = 4;
//!     repeated string nicknames = 5;
//!     repeated uint32 scores = 6;
//!     repeated Person children = 3;
//! }
//!

//////////////////////////////////////////////////////////////

use crate::internal::Bitfield;
use crate::{ErrorKind, Result};
use ::std::marker::PhantomData;

pub struct MessageDescriptor {
    fields: &'static [FieldDescriptor],
}

impl MessageDescriptor {
    pub fn field(&self, number: i32) -> Result<&FieldDescriptor> {
        debug_assert!(self.fields.is_sorted_by_key(|f| f.number));
        Ok(self
            .fields
            .binary_search_by_key(&number, |f| f.number)
            .map(|index| &self.fields[index])
            .map_err(|_| ErrorKind::ReflectionError)?)
    }
}

enum FieldDefaultValue {
    None,
    U32(u32),
    String(&'static str),
}

pub struct FieldDescriptor {
    number: i32,
    default_value: FieldDefaultValue,
}

impl FieldDescriptor {
    pub fn default_value_u32(&self) -> Result<u32> {
        match &self.default_value {
            FieldDefaultValue::U32(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    pub fn default_value_str(&self) -> Result<&'static str> {
        match &self.default_value {
            FieldDefaultValue::String(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
}

pub trait GenericMessage {
    fn try_get_u32<'a>(&'a self, _: &'a FieldDescriptor) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_repeated_u32_boxed<'a>(
        &'a self,
        _: &'a FieldDescriptor,
    ) -> Result<Box<dyn 'a + Iterator<Item = u32>>> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_str<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a str> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_message<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a dyn GenericMessage> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub struct DefaultProtoStruct<'a> {
    desc: &'a MessageDescriptor,
}
impl<'a> GenericMessage for DefaultProtoStruct<'a> {}
