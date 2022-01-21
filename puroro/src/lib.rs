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

#![doc = include_str!("lib.md")]
#![feature(backtrace)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
// Allow using GAT in document sample code.
#![doc(test(attr(feature(generic_associated_types))))]

mod common_traits;
mod error;
pub mod internal;
pub mod repeated_field;
pub mod tags;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub use self::repeated_field::{AsRefRepeatedField, CloneThenIntoRepeatedField, RepeatedField};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

use ::std::marker::PhantomData;

pub struct Message<MP, ImplTag, Fields, Shared> {
    pub fields: Fields,
    pub shared: Shared,
    _phantom: PhantomData<(MP, ImplTag)>,
}
impl<MP, ImplTag, Fields, Shared> Default for Message<MP, ImplTag, Fields, Shared>
where
    Fields: Default,
    Shared: Default,
{
    fn default() -> Self {
        Self {
            fields: Default::default(),
            shared: Default::default(),
            _phantom: Default::default(),
        }
    }
}

// メモ
use internal::methods::GetOptFieldMethod;
use internal::SimpleShared;
use internal::{FieldProperties, MessageProperties};

// assume a proto like this:
// message Person {
//     optional string name = 1;
//     optional uint32 age = 2;
//     optional Person partner = 4;
//     repeated string nicknames = 5;
//     repeated uint32 scores = 6;
//     repeated Person children = 3;
// }
//
type Person =
    Message<PersonMessageProperties, tags::SimpleImpl, PersonFieldsContainer, SimpleShared<1>>;

impl_scalar_getters!(PersonMessageProperties, 1, name, name_opt);
impl_scalar_getters!(PersonMessageProperties, 2, age, age_opt);
impl_scalar_getters!(PersonMessageProperties, 4, partner, partner_opt);
impl_repeated_getters!(PersonMessageProperties, 3, children);
impl_repeated_getters!(PersonMessageProperties, 6, scores);
impl_repeated_getters!(PersonMessageProperties, 5, nicknames);

#[derive(Default)]
struct PersonFieldsContainer {
    name: String,
    age: u32,
    children: Vec<Person>,
    partner: Option<Box<Person>>,
    nicknames: Vec<String>,
    scores: Vec<u32>,
}
impl crate::internal::FieldsContainer for PersonFieldsContainer {}

impl_has_field!(PersonFieldsContainer, 1, String, name);
impl_has_field!(PersonFieldsContainer, 2, u32, age);
impl_has_field!(PersonFieldsContainer, 3, Vec<Person>, children);
impl_has_field!(PersonFieldsContainer, 4, Option<Box<Person>>, partner);
impl_has_field!(PersonFieldsContainer, 5, Vec<String>, nicknames);
impl_has_field!(PersonFieldsContainer, 6, Vec<u32>, scores);

struct PersonMessageProperties;
impl MessageProperties for PersonMessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    type Fields<const NUMBER: i32> = PersonFieldProperties<NUMBER>;
}
struct PersonFieldProperties<const FIELD_NUMBER: i32>;
impl FieldProperties for PersonFieldProperties<1> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Optional;
    type TypeTag = tags::String;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = "";
}
impl FieldProperties for PersonFieldProperties<2> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
    type LabelTag = tags::Optional;
    type TypeTag = tags::UInt32;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = 0;
}
impl FieldProperties for PersonFieldProperties<3> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Repeated;
    type TypeTag = tags::Message<PersonMessageProperties>;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = ();
}
impl FieldProperties for PersonFieldProperties<4> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Optional;
    type TypeTag = tags::Message<PersonMessageProperties>;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = ();
}
impl FieldProperties for PersonFieldProperties<6> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Repeated;
    type TypeTag = tags::String;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = "";
}
impl FieldProperties for PersonFieldProperties<5> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Repeated;
    type TypeTag = tags::UInt32;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType = 0;
}

fn test() {
    let p = Person::default();
    let _: Option<u32> = p.age_opt();
    let _: Option<&str> = p.name_opt();
    let _: Option<&Person> = p.partner_opt();
    let _: u32 = p.age();
    let _: &str = p.name();
    //let _: &Person = p.partner();
    let _: &Vec<u32> = p.scores();
    let _: &Vec<String> = p.nicknames();
    let _: &Vec<Person> = p.children();
}
