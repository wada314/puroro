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

use crate::bumpalo::boxed::Box as BBox;
use crate::bumpalo::collections::{String as BString, Vec as BVec};
use crate::internal::impls::either::{EitherImplProperties, IntoEitherMessage};
use crate::internal::impls::merged::{IntoMergedMessage, MergedImplProperties};
use crate::internal::impls::option::{IntoOptionMessage, OptionImplProperties};
use crate::internal::methods::{GetFieldMethod, GetOptFieldMethod};
use crate::internal::{EmptyFields, ImplProperties, MessageProperties, SimpleShared};
use crate::*;
use ::std::marker::PhantomData;
use ::std::ops::{Deref, DerefMut};

use crate::internal::GetFieldMut;
use crate::message::*;

impl<FieldsType, SharedType, FH> MatchFieldNumber<FH>
    for MessageImpl<PersonMessageProperties, tags::SimpleImpl, FieldsType, SharedType>
where
    FH: FieldHandler<FieldsType = FieldsType, SharedType = SharedType>,
    FieldsType: GetFieldMut<1>,
    FieldsType: GetFieldMut<2>,
    FieldsType: GetFieldMut<3>,
    FieldsType: GetFieldMut<4>,
    FieldsType: GetFieldMut<5>,
    FieldsType: GetFieldMut<6>,
{
    fn match_field_number_mut(&mut self, number: i32, handler: &mut FH) -> Result<()> {
        match number {
            1 => self.call_handler_mut::<_, 1>(handler),
            2 => self.call_handler_mut::<_, 2>(handler),
            3 => self.call_handler_mut::<_, 3>(handler),
            4 => self.call_handler_mut::<_, 4>(handler),
            5 => self.call_handler_mut::<_, 5>(handler),
            6 => self.call_handler_mut::<_, 6>(handler),
            _ => Err(ErrorKind::UnknownFieldNumber)?,
        }
    }
}
impl<FieldsType, SharedType>
    MessageImpl<PersonMessageProperties, tags::SimpleImpl, FieldsType, SharedType>
{
    fn call_handler_mut<FH, const NUMBER: i32>(&mut self, handler: &mut FH) -> Result<()>
    where
        FieldsType: GetFieldMut<NUMBER>,
        FH: FieldHandler<FieldsType = FieldsType, SharedType = SharedType>,
    {
        handler.handle_mut::<NUMBER>(
            &mut GetFieldMut::<NUMBER>::get_field_mut(&mut self.fields),
            &mut self.shared,
        )
    }
}

////////////////////////////////////////////////

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
impl<Impl> AsMessageImplMut for Person<Impl>
where
    Impl: ImplProperties,
{
    fn as_message_impl_mut(&mut self) -> &mut Self::MessageImplType {
        &mut self.0
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
impl<Impl> AsMessageMut for Person<Impl>
where
    Impl: ImplProperties,
{
    fn as_message_mut(&mut self) -> &mut Self::MessageType {
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
impl<Impl> DerefMut for Person<Impl>
where
    Impl: ImplProperties,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
impl<T, U> From<(T, U)> for Person<MergedImplProperties<T, U>>
where
    T: AsMessageImplRef,
    U: AsMessageImplRef,
{
    fn from(tuple: (T, U)) -> Self {
        Person::from_raw_parts(EmptyFields::default(), tuple.into())
    }
}
impl<T, U> From<Either<T, U>> for Person<EitherImplProperties<T, U>>
where
    T: AsMessageImplRef,
    U: AsMessageImplRef,
{
    fn from(either: Either<T, U>) -> Self {
        Person::from_raw_parts(EmptyFields::default(), either.into())
    }
}

impl<T> IntoOptionMessage<PersonMessageProperties> for Option<T> {
    type OptionMessage = Person<OptionImplProperties<T>>;
    fn into_message(self) -> Self::OptionMessage {
        Person::from_raw_parts(EmptyFields::default(), self.into())
    }
}

impl<T, U> IntoEitherMessage<PersonMessageProperties> for Either<T, U> {
    type EitherMessage = Person<EitherImplProperties<T, U>>;
    fn into_message(self) -> Self::EitherMessage {
        Person::from_raw_parts(EmptyFields::default(), self.into())
    }
}

impl<T, U> IntoMergedMessage<PersonMessageProperties> for (T, U) {
    type MergedMessage = Person<MergedImplProperties<T, U>>;
    fn into_message(self) -> Self::MergedMessage {
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
    // define_get_opt!(fn name_opt<1>(&'a self));
    // define_get!(fn name<1>(&'a self));
    // define_get_opt!(fn age_opt<2>(&'a self));
    // define_get!(fn age<2>(&'a self));
    // define_get!(fn children<3>(&'a self));
    // define_get_opt!(fn partner_opt<4>(&'a self));
    // define_get!(fn partner<4>(&'a self));
    // define_get!(fn nicknames<5>(&'a self));
    // define_get!(fn scores<6>(&'a self));
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
impl DefaultIn for PersonFieldsContainer {
    type AllocatorType = ();
    fn default_in(_: Self::AllocatorType) -> Self {
        Default::default()
    }
}

define_fields_container! {
    struct PersonBumpFieldsContainer<'bump> {
        name: BString<'bump> = 1,
        age: u32 = 2,
        children: BVec<'bump, PersonBump<'bump>> = 3,
        partner: Option<BBox<'bump, PersonBump<'bump>>> = 4,
        nicknames: BVec<'bump, BString<'bump>> = 5,
        scores: BVec<'bump, u32> = 6,
    }
}
impl<'bump> DefaultIn for PersonBumpFieldsContainer<'bump> {
    type AllocatorType = &'bump crate::bumpalo::Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        Self {
            name: DefaultIn::default_in(alloc),
            age: Default::default(),
            children: DefaultIn::default_in(alloc),
            partner: Default::default(),
            nicknames: DefaultIn::default_in(alloc),
            scores: DefaultIn::default_in(alloc),
            _phantom: Default::default(),
        }
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

impl_get!(Person, pub fn name<1>(&self));
impl_get!(Person, pub fn age<2>(&self));
impl_get!(Person, pub fn children<3>(&self));
impl_get!(Person, pub fn partner<4>(&self));
impl_get!(Person, pub fn nicknames<5>(&self));
impl_get!(Person, pub fn scores<6>(&self));
impl_get_opt!(Person, pub fn name_opt<1>(&self));
impl_get_opt!(Person, pub fn age_opt<2>(&self));
impl_get_opt!(Person, pub fn partner_opt<4>(&self));
impl_get_slice!(Person, pub fn children_slice<3>(&self));
impl_get_slice!(Person, pub fn nicknames_slice<5>(&self));
impl_get_slice!(Person, pub fn scores_slice<6>(&self));
impl_get_mut!(Person, pub fn name_mut<1>(&mut self));
impl_get_mut!(Person, pub fn age_mut<2>(&mut self));
impl_get_mut!(Person, pub fn children_mut<3>(&mut self));
impl_get_mut!(Person, pub fn partner_mut<4>(&mut self));
impl_get_mut!(Person, pub fn nicknames_mut<5>(&mut self));
impl_get_mut!(Person, pub fn scores_mut<6>(&mut self));
impl_has!(Person, pub fn has_name<1>(&self));
impl_has!(Person, pub fn has_age<2>(&self));
impl_has!(Person, pub fn has_partner<4>(&self));

#[allow(unused)]
fn test() {
    let mut person: Person = Person::new();

    *person.age_mut() = 20;
    *person.name_mut() = "卑弥呼".to_string();
    person.partner_mut();

    let _: Option<u32> = person.age_opt();
    let _: Option<&str> = person.name_opt();
    let _: Option<&Person> = person.partner_opt();
    let _: bool = person.has_age();
    let _: bool = person.has_name();
    let _: bool = person.has_partner();
    let _: u32 = person.age();
    let _: &str = person.name();
    let partner: Person<_> = person.partner();
    let _: &[u32] = person.scores_slice();
    let _: &[String] = person.nicknames_slice();
    let _: &[Person] = person.children_slice();
    for x in person.scores() {
        let _: u32 = x;
    }
    for x in person.nicknames() {
        let _: &String = x;
    }
    for x in person.children() {
        let _: &Person = x;
    }

    // ################ option ################
    let _: Option<u32> = partner.age_opt();
    let _: Option<&str> = partner.name_opt();
    let _: Option<&Person> = partner.partner_opt();
    let _: bool = partner.has_age();
    let _: bool = partner.has_name();
    let _: bool = partner.has_partner();
    let _: u32 = partner.age();
    let _: &str = partner.name();
    let _: Person<_> = partner.partner();
    let _: &[u32] = partner.scores_slice();
    let _: &[String] = partner.nicknames_slice();
    let _: &[Person] = partner.children_slice();
    for x in partner.scores() {
        let _: u32 = x;
    }
    for x in partner.nicknames() {
        let _: &String = x;
    }
    for x in partner.children() {
        let _: &Person = x;
    }

    // let _: u32 = <Person<_> as PersonTrait>::age(&person);

    // ################ bumpalo ################
    let bump = bumpalo::Bump::new();
    let mut bperson = PersonBump::new_in(&bump);

    *bperson.age_mut() = 20;
    bperson.name_mut().push_str("卑弥呼");
    bperson.partner_mut();

    let _: Option<u32> = bperson.age_opt();
    let _: Option<&str> = bperson.name_opt();
    let _: Option<&PersonBump> = bperson.partner_opt();
    let _: bool = bperson.has_age();
    let _: bool = bperson.has_name();
    let _: bool = bperson.has_partner();
    let _: u32 = bperson.age();
    let _: &str = bperson.name();
    let bpartner: Person<_> = bperson.partner();
    let _: &[u32] = bperson.scores_slice();
    let _: &[BString] = bperson.nicknames_slice();
    let _: &[PersonBump] = bperson.children_slice();

    let _: Option<u32> = bpartner.age_opt();
    let _: Option<&str> = bpartner.name_opt();
    let _: Option<&PersonBump> = bpartner.partner_opt();
    let _: u32 = bpartner.age();
    let _: &str = bpartner.name();
    let _: Person<_> = bpartner.partner();
    let _: &[u32] = bpartner.scores_slice();
    let _: &[BString] = bpartner.nicknames_slice();
    let _: &[PersonBump] = bpartner.children_slice();
    for x in bperson.scores() {
        let _: u32 = x;
    }
    for x in bperson.nicknames() {
        let _: &str = AsRef::<str>::as_ref(&x);
    }
    for x in bperson.children() {
        let _: &Person<_> = x.as_message_ref();
    }

    // ################ either ################
    let eperson: Person<_> = Either::<&Person<_>, &Person>::Left(&partner).into();

    let _: Option<u32> = eperson.age_opt();
    let _: Option<&str> = eperson.name_opt();
    let _: Option<Person<_>> = eperson.partner_opt();
    let _: bool = eperson.has_age();
    let _: bool = eperson.has_name();
    let _: bool = eperson.has_partner();
    let _: u32 = eperson.age();
    let _: &str = eperson.name();
    let epartner: Person<_> = eperson.partner();
    let _: &[u32] = eperson.scores_slice();
    let _: &[String] = eperson.nicknames_slice();
    let _: &[Person] = eperson.children_slice();
    for x in eperson.scores() {
        let _: u32 = x;
    }
    for x in eperson.nicknames() {
        let _: &str = AsRef::<str>::as_ref(&x);
    }
    for x in eperson.children() {
        let _: &Person<_> = x.as_message_ref();
    }

    // ################ merged ################
    let mperson: Person<_> = (&person, &bperson).into();

    let _: Option<u32> = mperson.age_opt();
    let _: Option<&str> = mperson.name_opt();
    let _: Option<Person<_>> = mperson.partner_opt();
    let _: bool = mperson.has_age();
    let _: bool = mperson.has_name();
    let _: bool = mperson.has_partner();
    let _: u32 = mperson.age();
    let _: &str = mperson.name();
    let mpartner: Person<_> = mperson.partner();
    for x in mperson.scores() {
        let _: u32 = x;
    }
    for x in mperson.nicknames() {
        let _: &str = AsRef::<str>::as_ref(&x);
    }
    for x in mperson.children() {
        let _: &Person<_> = x.as_message_ref();
    }
}
