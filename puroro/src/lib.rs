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
use internal::impls::option::{OptionFields, OptionShared};

use ::std::marker::PhantomData;

pub struct MessageImpl<MP, ImplTag, Fields, Shared> {
    pub fields: Fields,
    pub shared: Shared,
    _phantom: PhantomData<(MP, ImplTag)>,
}
impl<MP, ImplTag, Fields, Shared> Default for MessageImpl<MP, ImplTag, Fields, Shared>
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
trait AsMessageRef {
    type MessageType;
    fn as_message_ref(&self) -> &Self::MessageType;
}
impl<MP, ImplTag, Fields, Shared> AsMessageRef for MessageImpl<MP, ImplTag, Fields, Shared> {
    type MessageType = MessageImpl<MP, ImplTag, Fields, Shared>;
    fn as_message_ref(&self) -> &Self::MessageType {
        self
    }
}
impl<'a, T> AsMessageRef for &'a T
where
    T: AsMessageRef,
{
    type MessageType = T::MessageType;
    fn as_message_ref(&self) -> &Self::MessageType {
        <T as AsMessageRef>::as_message_ref(*self)
    }
}

// メモ
use internal::SimpleShared;
use internal::{FieldProperties, MessageProperties};
use std::ops::Deref;

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
    MessageImpl<PersonMessageProperties, tags::SimpleImpl, PersonFieldsContainer, SimpleShared<1>>;

use internal::methods::{GetFieldMethod, GetOptFieldMethod};

struct PersonStruct<
    ImplTag = tags::SimpleImpl,
    FieldsType = PersonFieldsContainer,
    SharedType = SimpleShared<1>,
>(MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>);
impl<ImplTag, FieldsType, SharedType> AsMessageRef
    for PersonStruct<ImplTag, FieldsType, SharedType>
{
    type MessageType = MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>;
    fn as_message_ref(&self) -> &Self::MessageType {
        &self.0
    }
}
impl<ImplTag, FieldsType, SharedType> Deref for PersonStruct<ImplTag, FieldsType, SharedType> {
    type Target = MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag, FieldsType, SharedType> Default for PersonStruct<ImplTag, FieldsType, SharedType>
where
    MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>: Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

/*type PersonOption<T> =
    MessageImpl<PersonMessageProperties, tags::OptionImpl, OptionFields, OptionShared<T>>;
*/
type PersonOption<T> = PersonStruct<tags::OptionImpl, OptionFields, OptionShared<T>>;
impl<T> From<MessageImpl<PersonMessageProperties, tags::OptionImpl, OptionFields, OptionShared<T>>>
    for PersonOption<T>
{
    fn from(
        m: MessageImpl<PersonMessageProperties, tags::OptionImpl, OptionFields, OptionShared<T>>,
    ) -> Self {
        PersonStruct(m)
    }
}
impl<T> From<Option<T>> for PersonOption<T> {
    fn from(inner: Option<T>) -> Self {
        PersonStruct(Into::<MessageImpl<_, _, _, _>>::into(inner))
    }
}

trait PersonTrait
where
    Self: AsMessageRef,
    for<'a> <Self as AsMessageRef>::MessageType: GetFieldMethod<'a, 1>
        + GetOptFieldMethod<'a, 1>
        + GetFieldMethod<'a, 2>
        + GetOptFieldMethod<'a, 2>
        + GetFieldMethod<'a, 3>
        + GetFieldMethod<'a, 4>
        + GetOptFieldMethod<'a, 4>
        + GetFieldMethod<'a, 5>
        + GetFieldMethod<'a, 6>,
{
    define_opt_getter!(fn name_opt(1));
    define_getter!(fn name(1));
    define_opt_getter!(fn age_opt(2));
    define_getter!(fn age(2));
    define_getter!(fn children(3));
    define_opt_getter!(fn partner_opt(4));
    define_getter!(fn partner(4));
    define_getter!(fn nicknames(5));
    define_getter!(fn scores(6));
}

impl_scalar_getters2!(PersonStruct, 1, name, name_opt);
impl_scalar_getters2!(PersonStruct, 2, age, age_opt);
impl_scalar_getters2!(PersonStruct, 4, partner, partner_opt);
impl_repeated_getters2!(PersonStruct, 3, children);
impl_repeated_getters2!(PersonStruct, 5, nicknames);
impl_repeated_getters2!(PersonStruct, 6, scores);

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
    children: Vec<PersonStruct>,
    partner: Option<Box<PersonStruct>>,
    nicknames: Vec<String>,
    scores: Vec<u32>,
}
impl crate::internal::FieldsContainer for PersonFieldsContainer {}

impl_has_field!(PersonFieldsContainer, 1, String, name);
impl_has_field!(PersonFieldsContainer, 2, u32, age);
impl_has_field!(PersonFieldsContainer, 3, Vec<PersonStruct>, children);
impl_has_field!(PersonFieldsContainer, 4, Option<Box<PersonStruct>>, partner);
impl_has_field!(PersonFieldsContainer, 5, Vec<String>, nicknames);
impl_has_field!(PersonFieldsContainer, 6, Vec<u32>, scores);

struct PersonMessageProperties;
impl MessageProperties for PersonMessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    type Fields<const NUMBER: i32> = PersonFieldProperties<NUMBER>;
}
impl_field_properties!(PersonFieldProperties<1>, Optional, String, "", 0);
impl_field_properties!(PersonFieldProperties<2>, Optional, UInt32, 0, 0);
impl_field_properties!(
    PersonFieldProperties<3>,
    Repeated,
    Message<PersonMessageProperties>,
    (),
    0
);
impl_field_properties!(
    PersonFieldProperties<4>,
    Optional,
    Message<PersonMessageProperties>,
    (),
    0
);
impl_field_properties!(PersonFieldProperties<5>, Repeated, String, "", 0);
impl_field_properties!(PersonFieldProperties<6>, Repeated, UInt32, 0, 0);
struct PersonFieldProperties<const FIELD_NUMBER: i32>;

fn test() {
    let p = Person::default();

    let _: Option<u32> = p.age_opt();
    let _: Option<&str> = p.name_opt();
    let _: Option<&PersonStruct> = p.partner_opt();
    let _: u32 = p.age();
    let _: &str = p.name();
    let _: &[u32] = p.scores();
    let _: &[String] = p.nicknames();
    let _: &[PersonStruct] = p.children();

    let partner: PersonOption<&PersonStruct> = p.partner();
    let _: Option<u32> = partner.age_opt();
    let _: Option<&PersonStruct> = partner.partner_opt();
    let _: u32 = partner.age();
    let _: PersonOption<&PersonStruct> = partner.partner();
    let _: &[u32] = partner.scores();
    let _: &[String] = partner.nicknames();
    let _: &[PersonStruct] = partner.children();
}
