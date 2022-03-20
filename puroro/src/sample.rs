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

pub struct MessageDescriptor {}

pub trait GenericMessage {
    fn try_get_field(&self, number: i32) -> Result<GenericFieldWrapper<'_>>;
}

pub struct GenericFieldWrapper<'msg> {
    field: &'msg dyn GenericField,
    shared: &'msg dyn GenericShared,
    number: i32,
}
impl<'msg> GenericFieldWrapper<'msg> {
    pub fn try_get_u32(&self) -> Result<u32> {
        self.field.try_get_u32(self.shared, self.number)
    }
    pub fn try_get_str(&self) -> Result<&'msg str> {
        self.field.try_get_str(self.shared, self.number)
    }
    pub fn try_get_message(&self) -> Result<&'msg dyn GenericMessage> {
        self.field.try_get_message(self.shared, self.number)
    }
}
impl<'msg> From<&'msg u32> for GenericFieldWrapper<'msg> {
    fn from(val: &'msg u32) -> Self {
        Self {
            field: val,
            shared: &(),
            number: 0,
        }
    }
}

pub trait GenericField {
    fn try_get_u32<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<u32> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_str<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<&'a str> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: i32,
    ) -> Result<&'a dyn GenericMessage> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

pub trait GenericShared {
    fn try_get_bitfield(&self) -> Result<&dyn Bitfield> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_wrapped_option(&self) -> Result<Option<&dyn GenericMessage>> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

impl GenericField for u32 {
    fn try_get_u32<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<u32> {
        Ok(*self)
    }
}

impl<'msg> GenericField for String {
    fn try_get_str<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<&'a str> {
        Ok(&self)
    }
}

pub struct MessageField<M>(Option<Box<M>>);
impl<M: GenericMessage> GenericField for MessageField<M> {
    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: i32,
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
    fn try_get_field(&self, number: i32) -> Result<GenericFieldWrapper<'_>> {
        Ok(GenericFieldWrapper {
            field: &DEFAULT_PROTO_STRUCT_DUMMY_FIELD,
            shared: &(),
            number,
        })
    }
}

pub struct DefaultProtoStructDummyField();
const DEFAULT_PROTO_STRUCT_DUMMY_FIELD: DefaultProtoStructDummyField =
    DefaultProtoStructDummyField();
impl GenericField for DefaultProtoStructDummyField {
    fn try_get_u32<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<u32> {
        Ok(0 /* Need default value */)
    }
    fn try_get_str<'a>(&'a self, _: &'a dyn GenericShared, _: i32) -> Result<&'a str> {
        Ok("" /* Need default value */)
    }

    fn try_get_message<'a>(
        &'a self,
        _: &'a dyn GenericShared,
        _: i32,
    ) -> Result<&'a dyn GenericMessage> {
        Ok(&DEFAULT_PROTO_STRUCT)
    }
}

pub struct OptionProtoStructDummyField();
impl GenericField for OptionProtoStructDummyField {
    fn try_get_u32<'a>(&'a self, shared: &'a dyn GenericShared, number: i32) -> Result<u32> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(number))
            .transpose()?
            .map(|f| f.try_get_u32())
            .transpose()?
            .unwrap_or(0 /* need proper default value here */))
    }
    fn try_get_str<'a>(&'a self, shared: &'a dyn GenericShared, number: i32) -> Result<&'a str> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(number))
            .transpose()?
            .map(|f| f.try_get_str())
            .transpose()?
            .unwrap_or("" /* need proper default value here */))
    }
    fn try_get_message<'a>(
        &'a self,
        shared: &'a dyn GenericShared,
        number: i32,
    ) -> Result<&'a dyn GenericMessage> {
        Ok(shared
            .try_get_wrapped_option()?
            .map(|m| m.try_get_field(number))
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
