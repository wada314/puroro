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
use crate::internal::owned::{OwnedMessageImpl, OwnedRawField, OwnedRawMessageField};
use crate::message::{AsMessageImplRef, AsMessageRef, MessageImpl, MessageScalarFieldGetter};
use crate::tags;

// assume a proto like this as input:
// message Person {
//     optional string name = 1;
//     optional uint32 age = 2;
//     optional Person partner = 4;
//     repeated string nicknames = 5;
//     repeated uint32 scores = 6;
//     repeated Person children = 3;
// }

#[derive(Default)]
struct PersonOwnedRawFields {
    name: String,
    age: u32,
    partner: Option<Box<Person>>,
}
impl OwnedRawField<PersonStaticFieldDescriptor<1>> for PersonOwnedRawFields {
    type Type = String;
    fn get(&self) -> &Self::Type {
        &self.name
    }
}
impl OwnedRawField<PersonStaticFieldDescriptor<2>> for PersonOwnedRawFields {
    type Type = u32;
    fn get(&self) -> &Self::Type {
        &self.age
    }
}
impl OwnedRawField<PersonStaticFieldDescriptor<4>> for PersonOwnedRawFields {
    type Type = Option<Box<Person>>;
    fn get(&self) -> &Self::Type {
        &self.partner
    }
}
impl OwnedRawMessageField<PersonStaticFieldDescriptor<4>> for PersonOwnedRawFields {
    type FieldMessageImpl = OwnedMessageImpl<PersonStaticMessageDescriptor, PersonOwnedRawFields>;
}

#[derive(Default)]
struct PersonStaticMessageDescriptor;
struct PersonStaticFieldDescriptor<const NUMBER: i32>;

impl StaticMessageDescriptor for PersonStaticMessageDescriptor {
    type Fields<const NUMBER: i32> = PersonStaticFieldDescriptor<NUMBER>;
    type OwnedBitfield = ::bitvec::array::BitArray<::bitvec::order::Lsb0, [u32; 1]>;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<1> {
    const NUMBER: i32 = 1;
    const DEFAULT_VALUE: Option<FieldDefaultValue> = Some(FieldDefaultValue::String("John Doe"));
    const OWNED_HASFIELD_BITFIELD_INDEX: Option<usize> = Some(0);
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::String;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<2> {
    const NUMBER: i32 = 2;
    const DEFAULT_VALUE: Option<FieldDefaultValue> = Some(FieldDefaultValue::U32(14));
    const OWNED_HASFIELD_BITFIELD_INDEX: Option<usize> = Some(1);
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::UInt32;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<4> {
    const NUMBER: i32 = 4;
    const DEFAULT_VALUE: Option<FieldDefaultValue> = None;
    const OWNED_HASFIELD_BITFIELD_INDEX: Option<usize> = None;
    type FieldLabelTag = tags::Optional;
    type FieldTypeTag = tags::Message<PersonStaticMessageDescriptor>;
}

#[derive(Default)]
pub struct Person<M = OwnedMessageImpl<PersonStaticMessageDescriptor, PersonOwnedRawFields>>(M);
impl<'msg, M> Person<M>
where
    M: MessageImpl<'msg, PersonStaticMessageDescriptor>
        + MessageScalarFieldGetter<'msg, PersonStaticFieldDescriptor<1>, GetterReturnType = &'msg str>
        + MessageScalarFieldGetter<'msg, PersonStaticFieldDescriptor<2>, GetterReturnType = u32>
        + MessageScalarFieldGetter<'msg, PersonStaticFieldDescriptor<4>>,
{
    pub fn name(&'msg self) -> &str {
        <M as MessageImpl<PersonStaticMessageDescriptor>>::try_get_str::<
            PersonStaticFieldDescriptor<1>,
        >(&self.0)
        .unwrap()
    }
    pub fn age(&'msg self) -> u32 {
        <M as MessageImpl<PersonStaticMessageDescriptor>>::try_get_u32::<
            PersonStaticFieldDescriptor<2>,
        >(&self.0)
        .unwrap()
    }
    pub fn partner(
        &'msg self,
    ) -> Person<
        <M as MessageScalarFieldGetter<'msg, PersonStaticFieldDescriptor<4>>>::GetterReturnType,
    > {
        Person(
            <M as MessageImpl<PersonStaticMessageDescriptor>>::try_get_msg::<
                PersonStaticFieldDescriptor<4>,
                _,
            >(&self.0)
            .unwrap(),
        )
    }
}

impl<M> AsMessageRef for Person<M> {
    type MessageType = Self;
    fn as_message_ref(&self) -> &Self::MessageType {
        self
    }
}
impl<M> AsMessageImplRef for Person<M> {
    type MessageImplType = M;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        &self.0
    }
}

#[test]
fn test() {
    let person: Person = Person::default();
    assert_eq!("John Doe", person.name());
    assert_eq!(14, person.age());
}
