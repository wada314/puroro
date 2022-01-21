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
//     repeated Person children = 3;
// }
//
type Person =
    Message<PersonMessageProperties, tags::SimpleImpl, PersonFieldsContainer, SimpleShared<1>>;

impl<ImplTag, FieldsType, SharedType>
    Message<PersonMessageProperties, ImplTag, FieldsType, SharedType>
where
    for<'a> Self: crate::internal::methods::GetOptFieldMethod<'a, 2>,
{
    pub fn age_opt(
        &self,
    ) -> <Self as crate::internal::methods::GetOptFieldMethod<'_, 2>>::GetterType {
        <Self as crate::internal::methods::GetOptFieldMethod<'_, 2>>::get_opt(self)
    }
}

#[derive(Default)]
struct PersonFieldsContainer {
    name: String,
    age: u32,
    children: Vec<Person>,
}
impl crate::internal::FieldsContainer for PersonFieldsContainer {}
impl crate::internal::HasField<1> for PersonFieldsContainer {
    type Type = String;
    fn get(&self) -> &Self::Type {
        &self.name
    }
    fn get_mut(&mut self) -> &mut Self::Type {
        &mut self.name
    }
}
impl crate::internal::HasField<2> for PersonFieldsContainer {
    type Type = u32;
    fn get(&self) -> &Self::Type {
        &self.age
    }
    fn get_mut(&mut self) -> &mut Self::Type {
        &mut self.age
    }
}
impl crate::internal::HasField<3> for PersonFieldsContainer {
    type Type = Vec<Person>;
    fn get(&self) -> &Self::Type {
        &self.children
    }
    fn get_mut(&mut self) -> &mut Self::Type {
        &mut self.children
    }
}

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

fn test() {
    let p = Person::default();
    let i = p.age_opt();
}
