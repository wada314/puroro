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

use crate::desc::{FieldDefaultValue, StaticFieldDescriptor, StaticMessageDescriptor};
use crate::internal::owned::{OwnedRawFieldGetter, OwnedRawFields};
use crate::message::{Message, MessageFieldGetter};
use crate::tags::{self};

/// assume a proto like this as input:
/// message Person {
///     optional string name = 1;
///     optional uint32 age = 2;
///     optional Person partner = 4;
///     repeated string nicknames = 5;
///     repeated uint32 scores = 6;
///     repeated Person children = 3;
/// }

struct PersonOwnedRawFields {
    name: String,
    age: u32,
}
impl OwnedRawFields for PersonOwnedRawFields {}
impl OwnedRawFieldGetter<1> for PersonOwnedRawFields {
    type Type = String;
    fn get(&self) -> &Self::Type {
        &self.name
    }
}
impl OwnedRawFieldGetter<2> for PersonOwnedRawFields {
    type Type = u32;
    fn get(&self) -> &Self::Type {
        &self.age
    }
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
    const NUMBER: i32 = 4;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::None;
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::Message<PersonStaticMessageDescriptor>;
}

pub struct Person<M>(M);
impl<M: Message> Person<M>
where
    M: MessageFieldGetter<PersonStaticFieldDescriptor<1>, 1>,
    M: MessageFieldGetter<PersonStaticFieldDescriptor<2>, 2>,
{
    pub fn name(&self) -> &str {
        <M as Message>::try_get_str::<PersonStaticFieldDescriptor<1>, 1>(&self.0).unwrap()
    }
    pub fn age(&self) -> u32 {
        <M as Message>::try_get_u32::<PersonStaticFieldDescriptor<2>, 2>(&self.0).unwrap()
    }
}
