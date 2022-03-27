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

//////////////////////////////////////////////////////////////

use crate::desc::{
    FieldDefaultValue, FieldDescriptor, MessageDescriptor, StaticFieldDescriptor,
    StaticMessageDescriptor,
};
use crate::tags::{self};
use crate::{ErrorKind, Result};
use ::std::marker::PhantomData;

trait GetFieldByNumber<MD, R> {
    fn field_by_number(&self, _: i32) -> Result<R>;
}

pub trait Message {
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
    fn try_get_message<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a dyn Message> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait MessageMut {}

/// assume a proto like this as input:
/// message Person {
///     optional string name = 1;
///     optional uint32 age = 2;
///     optional Person partner = 4;
///     repeated string nicknames = 5;
///     repeated uint32 scores = 6;
///     repeated Person children = 3;
/// }

struct OwnedMessageImpl<MD, F, const BITFIELD_U32_LEN: usize> {
    bitvec: ::bitvec::array::BitArray<::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
    fields: F,
    _phantom: PhantomData<MD>,
}

trait OwnedFields {
    type FieldType<const NUMBER: i32>;
    fn get<const NUMBER: i32>(&self) -> &<Self::FieldType<NUMBER> as OwnedFieldType>::Type
    where
        Self::FieldType<NUMBER>: OwnedFieldType;
}
trait OwnedFieldType {
    type Type;
}
struct PersonOwnedFields {
    name: String,
    age: u32,
}
struct PersonOwnedFieldType<const NUMBER: i32>();
impl OwnedFieldType for PersonOwnedFieldType<1> {
    type Type = String;
}
impl OwnedFieldType for PersonOwnedFieldType<2> {
    type Type = u32;
}

struct PersonStaticMessageDescriptor;
struct PersonStaticFieldDescriptor<const NUMBER: i32>;

impl StaticMessageDescriptor for PersonStaticMessageDescriptor {
    type Fields<const NUMBER: i32> = PersonStaticFieldDescriptor<NUMBER>;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<1> {
    const NUMBER: i32 = 1;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::String("John Doe");
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::String;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<2> {
    const NUMBER: i32 = 2;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::U32(14);
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::UInt32;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<3> {
    const NUMBER: i32 = 3;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::None;
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::Message<PersonStaticMessageDescriptor>;
}
