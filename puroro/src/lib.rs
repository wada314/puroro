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

pub trait AsMessageRef {
    type MessageType;
    fn as_message_ref(&self) -> &Self::MessageType;
}
pub trait AsMessageDeref {
    type DerefType<'a>: 'a + Deref<Target = Self::MessageType<'a>>
    where
        Self: 'a;
    type MessageType<'a>
    where
        Self: 'a;
    fn as_message_deref(&self) -> Self::DerefType<'_>;
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
impl<MP, ImplTag, Fields, Shared> AsMessageDeref for MessageImpl<MP, ImplTag, Fields, Shared> {
    type DerefType<'a>
    where
        Self: 'a,
    = &'a MessageImpl<MP, ImplTag, Fields, Shared>;
    type MessageType<'a>
    where
        Self: 'a,
    = MessageImpl<MP, ImplTag, Fields, Shared>;
    fn as_message_deref(&self) -> Self::DerefType<'_> {
        self
    }
}
impl<'a, T> AsMessageDeref for &'a T
where
    T: AsMessageDeref,
{
    type DerefType<'b>
    where
        Self: 'b,
    = T::DerefType<'b>;
    type MessageType<'b>
    where
        Self: 'b,
    = T::MessageType<'b>;
    fn as_message_deref(&self) -> Self::DerefType<'_> {
        <T as AsMessageDeref>::as_message_deref(self)
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
use internal::impls::option::{OptionFields, OptionShared};
use internal::methods::{GetFieldMethod, GetOptFieldMethod};
use internal::methods2::{GetFieldMethod2, GetOptFieldMethod2};
use internal::HasField;

struct PersonStruct<
    ImplTag = tags::SimpleImpl,
    FieldsType = PersonFieldsContainer,
    SharedType = SimpleShared<1>,
>(MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>);
type Person = PersonStruct;
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

trait PersonTrait
where
    Self: AsMessageDeref,
    for<'a> <Self as AsMessageDeref>::MessageType<'a>: GetFieldMethod2<1>
        + GetOptFieldMethod2<1>
        + GetFieldMethod2<2>
        + GetOptFieldMethod2<2>
        + GetFieldMethod2<3>
        + GetFieldMethod2<4>
        + GetOptFieldMethod2<4>
        + GetFieldMethod2<5>
        + GetFieldMethod2<6>,
{
    define_opt_getter2!(fn name_opt<1>(&self));
    define_getter2!(fn name<1>(&self));
    define_opt_getter2!(fn age_opt<2>(&self));
    define_getter2!(fn age<2>(&self));
    define_getter2!(fn children<3>(&self));
    define_opt_getter2!(fn partner_opt<4>(&self));
    define_getter2!(fn partner<4>(&self));
    define_getter2!(fn nicknames<5>(&self));
    define_getter2!(fn scores<6>(&self));
}

// impl_scalar_getters2!(PersonStruct, 1, name, name_opt);
// impl_scalar_getters2!(PersonStruct, 2, age, age_opt);
// impl_scalar_getters2!(PersonStruct, 4, partner, partner_opt);
// impl_repeated_getters2!(PersonStruct, 3, children);
// impl_repeated_getters2!(PersonStruct, 5, nicknames);
// impl_repeated_getters2!(PersonStruct, 6, scores);

impl PersonStruct<tags::SimpleImpl, PersonFieldsContainer, SimpleShared<1>>
where
    <Self as Deref>::Target: GetOptFieldMethod2<1>,
{
    pub fn name_opt2(&self) -> <<Self as Deref>::Target as GetOptFieldMethod2<1>>::GetterType<'_> {
        <<Self as Deref>::Target as GetOptFieldMethod2<1>>::get_opt(self.deref())
    }
}

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
    /*
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
    let _: &[PersonStruct] = partner.children();*/
}
