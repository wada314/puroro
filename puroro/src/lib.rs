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
pub mod message;
pub mod repeated_field;
pub mod tags;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub use self::message::MessageImpl;
pub use self::repeated_field::{AsRefRepeatedField, CloneThenIntoRepeatedField, RepeatedField};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

use ::std::marker::PhantomData;

// メモ

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
use internal::impls::option::{MessageInOptionTrait, OptionShared};
use internal::methods::{GetFieldMethod, GetOptFieldMethod};
use internal::EmptyFields;
use internal::MessageProperties;
use internal::{ImplProperties, SimpleShared};
use std::ops::Deref;

pub struct PersonSimpleImplProperties<
    FieldsType = PersonFieldsContainer,
    SharedType = SimpleShared<1>,
>(PhantomData<(FieldsType, SharedType)>);
pub type PersonBumpImplProperties<'bump> = PersonSimpleImplProperties<
    PersonBumpFieldsContainer<'bump>,
    internal::impls::bumpalo::BumpShared<'bump, 1>,
>;

impl<FieldsType, SharedType> ImplProperties for PersonSimpleImplProperties<FieldsType, SharedType> {
    type ImplTag = tags::SimpleImpl;
    type FieldsType = FieldsType;
    type SharedType = SharedType;
}

pub struct OptionImplProperties<T>(PhantomData<T>);
impl<T> ImplProperties for OptionImplProperties<T> {
    type ImplTag = tags::OptionImpl;
    type FieldsType = EmptyFields;
    type SharedType = OptionShared<T>;
}

pub struct Person<Impl = PersonSimpleImplProperties>(
    MessageImpl<PersonMessageProperties, Impl::ImplTag, Impl::FieldsType, Impl::SharedType>,
)
where
    Impl: ImplProperties;
pub type PersonBump<'bump> = Person<PersonBumpImplProperties<'bump>>;

impl<Impl> Person<Impl>
where
    Self: Default,
    Impl: ImplProperties,
{
    pub fn new() -> Self {
        Default::default()
    }
}
impl<Impl> Person<Impl>
where
    Self: DefaultIn,
    Impl: ImplProperties,
{
    pub fn new_in(alloc: <Self as DefaultIn>::AllocatorType) -> Self {
        DefaultIn::default_in(alloc)
    }
}

impl<Impl> Person<Impl>
where
    Impl: ImplProperties,
{
    pub fn from_raw_parts(fields: Impl::FieldsType, shared: Impl::SharedType) -> Self {
        Self(MessageImpl::from_raw_parts(fields, shared))
    }
}

impl<Impl, Alloc> DefaultIn for Person<Impl>
where
    Impl: ImplProperties,
    MessageImpl<PersonMessageProperties, Impl::ImplTag, Impl::FieldsType, Impl::SharedType>:
        DefaultIn<AllocatorType = Alloc>,
{
    type AllocatorType = Alloc;
    fn default_in(alloc: Alloc) -> Self {
        Self(MessageImpl::default_in(alloc))
    }
}

impl<Impl> AsMessageImplRef for Person<Impl>
where
    Impl: ImplProperties,
{
    type MessageImplType =
        MessageImpl<PersonMessageProperties, Impl::ImplTag, Impl::FieldsType, Impl::SharedType>;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        &self.0
    }
}
impl<Impl> AsMessageRef for Person<Impl>
where
    Impl: ImplProperties,
{
    type MessageType = Person<Impl>;
    fn as_message_ref(&self) -> &Self::MessageType {
        self
    }
}

impl<Impl> Deref for Person<Impl>
where
    Impl: ImplProperties,
{
    type Target =
        MessageImpl<PersonMessageProperties, Impl::ImplTag, Impl::FieldsType, Impl::SharedType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<Impl> Default for Person<Impl>
where
    Impl: ImplProperties,
    MessageImpl<PersonMessageProperties, Impl::ImplTag, Impl::FieldsType, Impl::SharedType>:
        Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T> From<Option<T>> for Person<OptionImplProperties<T>>
where
    T: AsMessageImplRef,
{
    fn from(opt: Option<T>) -> Self {
        Person::from_raw_parts(EmptyFields::default(), opt.into())
    }
}

impl<T, ImplTag, FieldsType, SharedType> MessageInOptionTrait<PersonMessageProperties> for Option<T>
where
    T: AsMessageImplRef<
        MessageImplType = MessageImpl<PersonMessageProperties, ImplTag, FieldsType, SharedType>,
    >,
{
    type WrappedOptionMessage = Person<OptionImplProperties<T>>;
    fn into_message(self) -> Self::WrappedOptionMessage {
        Person::from_raw_parts(EmptyFields::default(), self.into())
    }
}

trait PersonTrait<'a>
where
    Self: AsMessageImplRef,
    <Self as AsMessageImplRef>::MessageImplType: GetFieldMethod<'a, 1>
        + GetOptFieldMethod<'a, 1>
        + GetFieldMethod<'a, 2>
        + GetOptFieldMethod<'a, 2>
        + GetFieldMethod<'a, 3>
        + GetFieldMethod<'a, 4>
        + GetOptFieldMethod<'a, 4>
        + GetFieldMethod<'a, 5>
        + GetFieldMethod<'a, 6>,
{
    define_opt_getter!(fn name_opt<1>(&'a self));
    define_getter!(fn name<1>(&'a self));
    define_opt_getter!(fn age_opt<2>(&'a self));
    define_getter!(fn age<2>(&'a self));
    define_getter!(fn children<3>(&'a self));
    define_opt_getter!(fn partner_opt<4>(&'a self));
    define_getter!(fn partner<4>(&'a self));
    define_getter!(fn nicknames<5>(&'a self));
    define_getter!(fn scores<6>(&'a self));
}

impl<'a, T> PersonTrait<'a> for T
where
    T: AsMessageImplRef,
    <T as AsMessageImplRef>::MessageImplType: GetFieldMethod<'a, 1>
        + GetOptFieldMethod<'a, 1>
        + GetFieldMethod<'a, 2>
        + GetOptFieldMethod<'a, 2>
        + GetFieldMethod<'a, 3>
        + GetFieldMethod<'a, 4>
        + GetOptFieldMethod<'a, 4>
        + GetFieldMethod<'a, 5>
        + GetFieldMethod<'a, 6>,
{
}

define_fields_container! {
    #[derive(Default)]
    struct PersonFieldsContainer {
        name: String = 1,
        age: u32 = 2,
        children: Vec<Person> = 3,
        partner: Option<Box<Person>> = 4,
        nicknames: Vec<String> = 5,
        scores: Vec<u32> = 6,
    }
}

use crate::bumpalo::collections::{String as BString, Vec as BVec};
use crate::internal::{NoAllocBumpBox, NoAllocBumpString, NoAllocBumpVec};

define_fields_container! {
    #[derive(Default)]
    struct PersonBumpFieldsContainer<'bump> {
        name: NoAllocBumpString = 1,
        age: u32 = 2,
        children: NoAllocBumpVec<PersonBump<'bump>> = 3,
        partner: Option<NoAllocBumpBox<PersonBump<'bump>>> = 4,
        nicknames: NoAllocBumpVec<BString<'bump>> = 5,
        scores: NoAllocBumpVec<u32> = 6,
    }
}

pub struct PersonMessageProperties;
impl MessageProperties for PersonMessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
    type Fields<const NUMBER: i32> = PersonFieldProperties<NUMBER>;
}

pub struct PersonFieldProperties<const FIELD_NUMBER: i32>;
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

impl_getter!(Person, pub fn name<1>(&self));
impl_getter!(Person, pub fn age<2>(&self));
impl_getter!(Person, pub fn children<3>(&self));
impl_getter!(Person, pub fn partner<4>(&self));
impl_getter!(Person, pub fn nicknames<5>(&self));
impl_getter!(Person, pub fn scores<6>(&self));
impl_opt_getter!(Person, pub fn name_opt<1>(&self));
impl_opt_getter!(Person, pub fn age_opt<2>(&self));
impl_opt_getter!(Person, pub fn partner_opt<4>(&self));

fn test() {
    let person: Person = Person::new();

    let _: Option<u32> = person.age_opt();
    let _: Option<&str> = person.name_opt();
    let _: Option<&Person> = person.partner_opt();
    let _: u32 = person.age();
    let _: &str = person.name();
    let partner: Person<_> = person.partner();
    let _: &[u32] = person.scores();
    let _: &[String] = person.nicknames();
    let _: &[Person] = person.children();

    let _: Option<u32> = partner.age_opt();
    let _: Option<&str> = partner.name_opt();
    let _: Option<&Person> = partner.partner_opt();
    let _: u32 = partner.age();
    let _: &str = partner.name();
    let _: Person<_> = partner.partner();
    let _: &[u32] = partner.scores();
    let _: &[String] = partner.nicknames();
    let _: &[Person] = partner.children();

    let _: u32 = <Person<_> as PersonTrait>::age(&person);

    let bump = bumpalo::Bump::new();
    let bperson = PersonBump::new_in(&bump);
    let _: Option<u32> = bperson.age_opt();
    let _: Option<&str> = bperson.name_opt();
    //let _: Option<&PersonBump> = bperson.partner_opt();
    let _: u32 = bperson.age();
    let _: &str = bperson.name();
    //let bpartner: Person<_> = bperson.partner();
    //let _: &[u32] = bperson.scores();
    //let _: &[String] = bperson.nicknames();
    //let _: &[Person] = bperson.children();
}
