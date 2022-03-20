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
use ::std::any::Any;
use ::std::marker::PhantomData;

pub trait GenericMessage {
    fn try_get_field(&self, number: i32) -> Result<GenericFieldWrapper<'_>>;
}

pub struct GenericFieldWrapper<'msg> {
    exclusive: &'msg dyn GenericField,
    shared: &'msg dyn GenericShared,
    number: i32,
}
impl<'msg> GenericFieldWrapper<'msg> {
    pub fn try_get_u32(&self) -> Result<u32> {
        self.exclusive.try_get_u32(self.shared, self.number)
    }
    pub fn try_get_str(&self) -> Result<&str> {
        self.exclusive.try_get_str(self.shared, self.number)
    }
    pub fn try_get_message(&self) -> Result<&dyn GenericMessage> {
        self.exclusive.try_get_message(self.shared, self.number)
    }
}
impl<'msg> From<&'msg u32> for GenericFieldWrapper<'msg> {
    fn from(val: &'msg u32) -> Self {
        Self {
            exclusive: val,
            shared: &(),
            number: 0,
        }
    }
}

pub trait GenericField {
    fn try_get_u32(&self, _: &dyn GenericShared, _: i32) -> Result<u32> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_str(&self, _: &dyn GenericShared, _: i32) -> Result<&str> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_message(&self, _: &dyn GenericShared, _: i32) -> Result<&dyn GenericMessage> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

pub trait GenericShared: Any {
    fn try_get_bitfield(&self) -> Result<&dyn Bitfield> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

impl GenericField for u32 {
    fn try_get_u32(&self, _: &dyn GenericShared, _: i32) -> Result<u32> {
        Ok(*self)
    }
}

impl<'msg> GenericField for String {
    fn try_get_str(&self, _: &dyn GenericShared, _: i32) -> Result<&str> {
        Ok(&self)
    }
}

pub struct MessageField<M>(Option<Box<M>>);
impl<M: GenericMessage> GenericField for MessageField<M> {
    fn try_get_message(&self, _: &dyn GenericShared, _: i32) -> Result<&dyn GenericMessage> {
        Ok(self.0.as_ref().expect("FIXME").as_ref())
    }
}

pub struct OptionProtoStructDummyField<M>(PhantomData<M>);
impl<M> GenericField for OptionProtoStructDummyField<M> {
    fn try_get_u32(&self, shared: &dyn GenericShared, _: i32) -> Result<u32> {
        shared.downcast_ref::<OptionProtoStructShared<M>>()
    }
    fn try_get_str(&self, _: &dyn GenericShared, _: i32) -> Result<&str> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_message(&self, _: &dyn GenericShared, _: i32) -> Result<&dyn GenericMessage> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

impl GenericShared for () {}

struct OptionProtoStructShared<M>(Option<M>);
impl<M: 'static> GenericShared for OptionProtoStructShared<M> {}
