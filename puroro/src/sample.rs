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
    fn try_get_field<'a>(&'a self, desc: &'a FieldDescriptor) -> Result<GenericFieldWrapper<'a>>;
}

pub struct GenericFieldWrapper<'a> {
    field: &'a dyn GenericField,
    shared: &'a dyn GenericShared,
    desc: &'a FieldDescriptor,
}
impl<'a> GenericFieldWrapper<'a> {
    pub fn try_get_u32(&self) -> Result<u32> {
        self.field.try_get_u32(self.shared, self.desc)
    }
    pub fn try_get_str(&self) -> Result<&str> {
        self.field.try_get_str(self.shared, self.desc)
    }
    pub fn try_get_message(&self) -> Result<&dyn GenericMessage> {
        self.field.try_get_message(self.shared, self.desc)
    }

    pub fn from_u32(field: &'a u32, desc: &'a FieldDescriptor) -> Self {
        Self {
            field,
            shared: &(),
            desc,
        }
    }
}

pub trait GenericField {
    fn try_get_u32<'a>(&'a self, _: &'a dyn GenericShared, _: &'a FieldDescriptor) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_str<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: &'a FieldDescriptor,
    ) -> Result<&'a str> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: &'a FieldDescriptor,
    ) -> Result<&'a dyn GenericMessage> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait GenericShared {
    fn try_get_bitfield(&self) -> Result<&dyn Bitfield> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_wrapped_option(&self) -> Result<Option<&dyn GenericMessage>> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl GenericField for u32 {
    fn try_get_u32<'a>(&'a self, _: &'a dyn GenericShared, _: &'a FieldDescriptor) -> Result<u32> {
        Ok(*self)
    }
}

impl<'a> GenericField for String {
    fn try_get_str<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: &'a FieldDescriptor,
    ) -> Result<&'a str> {
        Ok(&self)
    }
}

pub struct MessageField<M>(Option<Box<M>>);
impl<M: GenericMessage> GenericField for MessageField<M> {
    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: &'a FieldDescriptor,
    ) -> Result<&'a dyn GenericMessage> {
        Ok(self
            .0
            .as_ref()
            .map(|m| m.as_ref() as &dyn GenericMessage)
            .unwrap_or(&DEFAULT_PROTO_STRUCT))
    }
}

pub struct DefaultProtoStruct();
const DEFAULT_PROTO_STRUCT: DefaultProtoStruct = DefaultProtoStruct();
impl GenericMessage for DefaultProtoStruct {
    fn try_get_field<'a>(&'a self, desc: &'a FieldDescriptor) -> Result<GenericFieldWrapper<'a>> {
        Ok(GenericFieldWrapper {
            field: &DEFAULT_PROTO_STRUCT_DUMMY_FIELD,
            shared: &(),
            desc,
        })
    }
}

pub struct DefaultProtoStructDummyField();
const DEFAULT_PROTO_STRUCT_DUMMY_FIELD: DefaultProtoStructDummyField =
    DefaultProtoStructDummyField();
impl GenericField for DefaultProtoStructDummyField {
    fn try_get_u32<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        desc: &'a FieldDescriptor,
    ) -> Result<u32> {
        Ok(desc.default_value_u32()?)
    }
    fn try_get_str<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        desc: &'a FieldDescriptor,
    ) -> Result<&'a str> {
        Ok(desc.default_value_str()?)
    }

    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: &'a FieldDescriptor,
    ) -> Result<&'a dyn GenericMessage> {
        Ok(&DEFAULT_PROTO_STRUCT)
    }
}

pub struct OptionProtoStructDummyField();
impl GenericField for OptionProtoStructDummyField {
    fn try_get_u32<'a>(
        &'a self,
        shared: &'a dyn GenericShared,
        desc: &'a FieldDescriptor,
    ) -> Result<u32> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(desc))
            .transpose()?
            .map(|f| f.try_get_u32())
            .unwrap_or_else(|| desc.default_value_u32())?)
    }
    fn try_get_str<'a>(
        &'a self,
        shared: &'a dyn GenericShared,
        desc: &'a FieldDescriptor,
    ) -> Result<&'a str> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(desc))
            .transpose()?
            .map(|f| f.try_get_str())
            .unwrap_or_else(|| desc.default_value_str())?)
    }
    fn try_get_message<'a>(
        &'a self,
        shared: &'a dyn GenericShared,
        desc: &'a FieldDescriptor,
    ) -> Result<&'a dyn GenericMessage> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(desc))
            .transpose()?
            .map(|f| f.try_get_message())
            .transpose()?
            .unwrap_or_else(|| &DEFAULT_PROTO_STRUCT))
    }
}

impl GenericShared for () {}

struct OptionProtoStructShared<M>(Option<M>);
impl<M: GenericMessage> GenericShared for OptionProtoStructShared<M> {
    fn try_get_wrapped_option(&self) -> Result<Option<&dyn GenericMessage>> {
        Ok(self.0.as_ref().map(|x| x as &dyn GenericMessage))
    }
}
