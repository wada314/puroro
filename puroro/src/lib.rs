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

use ::std::ops::{Deref, DerefMut};

// Bumpalo wrapper
pub struct BumpaloOwned<T> {
    // The field order matters, `Drop` drops the field in decl order.
    t: T,
    bump: Box<crate::bumpalo::Bump>,
}
impl<T> BumpaloOwned<T> {
    pub fn bump(this: &BumpaloOwned<T>) -> &crate::bumpalo::Bump {
        &this.bump
    }
    pub fn inner(this: &BumpaloOwned<T>) -> &T {
        &this.t
    }
    pub fn inner_mut(this: &mut BumpaloOwned<T>) -> &mut T {
        &mut this.t
    }
}
impl<T> BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    pub fn new() -> Self {
        let bump = Box::new(crate::bumpalo::Bump::new());
        let t = crate::internal::BumpDefault::default_in(unsafe {
            ::std::mem::transmute(bump.as_ref())
        });
        Self { t, bump }
    }
}
impl<T> Default for BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Deref for BumpaloOwned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
impl<T> DerefMut for BumpaloOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<M, T> Message<M> for BumpaloOwned<T> where T: Message<M> {}

pub struct SimpleImpl;
pub struct BumpaloImpl;

// メモ
use internal::methods::{GetFieldMethod, GetOptFieldMethod};
use internal::Bitfield;
use internal::{FieldAndSharedRef, FieldProperties, MessageProperties};
use internal::{SimpleFields, SimpleShared};

// assume a proto like this:
// message Person {
//     optional string name = 1;
//     optional uint32 age = 2;
//     repeated Person children = 3;
// }
//
trait PersonFieldsType {
    type ImplTag;
    type NameType;
    type AgeType;
    type ChildrenType;
}
impl PersonFieldsType for SimpleFields {
    type ImplTag = tags::SimpleImpl;
    type NameType = String;
    type AgeType = u32;
    type ChildrenType = Vec<PersonSimple>;
}
struct Person<Fields: PersonFieldsType, Shared> {
    _shared: Shared,
    name: <Fields as PersonFieldsType>::NameType,
    age: <Fields as PersonFieldsType>::AgeType,
    children: <Fields as PersonFieldsType>::ChildrenType,
}
type PersonSimple = Person<SimpleFields, SimpleShared<1>>;
impl<Fields, Shared> Default for Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    <Fields as PersonFieldsType>::NameType: Default,
    <Fields as PersonFieldsType>::AgeType: Default,
    <Fields as PersonFieldsType>::ChildrenType: Default,
    Shared: Default,
{
    fn default() -> Self {
        Self {
            _shared: Default::default(),
            name: Default::default(),
            age: Default::default(),
            children: Default::default(),
        }
    }
}

struct PersonMessageProperties;
impl MessageProperties for PersonMessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize = 0;
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
impl<Fields, Shared> Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    for<'a> FieldAndSharedRef<'a, Fields::NameType, Shared>:
        GetOptFieldMethod<'a, PersonFieldProperties<1>, Fields::ImplTag>,
{
    pub fn name_opt(
        &self,
    ) -> <FieldAndSharedRef<Fields::NameType, Shared> as GetOptFieldMethod<
        PersonFieldProperties<1>,
        Fields::ImplTag,
    >>::GetterType {
        FieldAndSharedRef::new(&self.name, &self._shared).get_opt()
    }
}
use internal::methods::GetFieldMethodImpl;
impl<Fields, Shared> Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    for<'a> FieldAndSharedRef<'a, Fields::NameType, Shared>: GetFieldMethodImpl<
        'a,
        PersonFieldProperties<1>,
        Fields::ImplTag,
        tags::Optional,
        tags::String,
    >,
{
    pub fn hoge(&self) {}
} /*
impl<Fields, Shared> Person<Fields, Shared>
where
Fields: PersonFieldsType,
for<'a> FieldAndSharedRef<'a, Fields::NameType, Shared>:
GetFieldMethod<'a, PersonFieldProperties<1>, Fields::ImplTag>,
{
pub fn hoge(&self) {}
pub fn name(
&self,
) -> <FieldAndSharedRef<Fields::NameType, Shared> as GetFieldMethod<
PersonFieldProperties<1>,
Fields::ImplTag,
>>::GetterType {
FieldAndSharedRef::new(&self.name, &self._shared).get()
}
}*/
impl<Fields, Shared> Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    for<'a> FieldAndSharedRef<'a, Fields::AgeType, Shared>:
        GetOptFieldMethod<'a, PersonFieldProperties<2>, Fields::ImplTag>,
{
    pub fn age_opt(
        &self,
    ) -> <FieldAndSharedRef<Fields::AgeType, Shared> as GetOptFieldMethod<
        PersonFieldProperties<2>,
        Fields::ImplTag,
    >>::GetterType {
        FieldAndSharedRef::new(&self.age, &self._shared).get_opt()
    }
}
impl<Fields, Shared> Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    for<'a> FieldAndSharedRef<'a, Fields::ChildrenType, Shared>:
        GetFieldMethod<'a, PersonFieldProperties<3>, Fields::ImplTag>,
{
    pub fn children(
        &self,
    ) -> <FieldAndSharedRef<Fields::ChildrenType, Shared> as GetFieldMethod<
        PersonFieldProperties<3>,
        Fields::ImplTag,
    >>::GetterType {
        FieldAndSharedRef::new(&self.children, &self._shared).get()
    }
}

macro_rules! derive_person_params {
    ($base:ty, $new_param_type:ty, $new_param_ident:ident) => {
        derive_person_params!(@typedecl $base, NameType, $new_param_type, $new_param_ident);
        derive_person_params!(@typedecl $base, AgeType, $new_param_type, $new_param_ident);
        derive_person_params!(@typedecl $base, ChildrenType, $new_param_type, $new_param_ident);
    };
    (@typedecl $base:ty, NameType, $new_param_type:ty, NameType) => {
        type NameType = $new_param_type;
    };
    (@typedecl $base:ty, $ident:ident, $new_param_type:ty, $new_param_ident:ident) => {
        type $ident = <$base as PersonFieldsType>::$ident;
    };
}

#[cfg(test2)]
fn test() {
    let person = PersonSimple::default();
    let _hoge = person.age_opt();
    assert_eq!(Some(0), person.age_opt());
    let _foo = person.name_opt();
    assert_eq!(Some(""), person.name_opt());
    let _hoga = person.children();
    //let _nama = person.name();
    //person.hoge();
    let name = String::new();
    let shared = SimpleShared::<1>::default();
    let fands = FieldAndSharedRef::new(&name, &shared);
    <FieldAndSharedRef<String, SimpleShared<1>> as GetFieldMethodImpl<
        PersonFieldProperties<1>,
        tags::SimpleImpl,
        tags::Optional,
        tags::String,
    >>::get_impl(&fands);
}

trait DefaultValue {
    type Type;
    const DEFAULT_VALUE: Self::Type;
}
struct StrFoo;
impl DefaultValue for StrFoo {
    type Type = &'static str;
    const DEFAULT_VALUE: Self::Type = "foo";
}
trait OrDefaultTrait<D: DefaultValue<Type = Self::Type>> {
    type Type;
    fn or_default(val: Option<Self::Type>) -> Self::Type;
}
struct StrOrFoo<'a>(std::marker::PhantomData<&'a ()>);
impl<'a> OrDefaultTrait<StrFoo> for StrOrFoo<'a> {
    type Type = &'a str;
    fn or_default(val: Option<Self::Type>) -> Self::Type {
        val.unwrap_or(StrFoo::DEFAULT_VALUE)
    }
}
