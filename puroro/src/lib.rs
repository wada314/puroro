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
use internal::Bitfield;

pub struct SimpleFields;
#[derive(Default, Clone, Debug)]
pub struct SimpleShared<const BITFIELD_U32_LEN: usize> {
    bitfield: crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
}

pub trait SharedObjects {
    type AllocatorType;
    type BitfieldType: Bitfield;
    fn alloc(&self) -> &Self::AllocatorType;
    fn bitfield(&self) -> &Self::BitfieldType;
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType;
}
impl<const BITFIELD_U32_LEN: usize> SharedObjects for SimpleShared<BITFIELD_U32_LEN> {
    type AllocatorType = ();
    type BitfieldType =
        crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>;
    fn alloc(&self) -> &Self::AllocatorType {
        &()
    }
    fn bitfield(&self) -> &Self::BitfieldType {
        &self.bitfield
    }
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType {
        &mut self.bitfield
    }
}

pub struct FieldAndSharedRef<'a, Field, Shared> {
    field: &'a Field,
    shared: &'a Shared,
}
impl<'a, Field, Shared> FieldAndSharedRef<'a, Field, Shared> {
    pub fn new(field: &'a Field, shared: &'a Shared) -> Self {
        Self { field, shared }
    }
}

pub trait FieldProperties {
    const OPTIONAL_FIELD_BIT_VEC_INDEX: usize = 0;
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
}

pub trait OptGetFieldMethod<'a, FP, LabelTag, TypeTag> {
    type OptGetterType;
    fn get_opt(&self) -> Option<Self::OptGetterType>;
}

impl<'a, _1, _2, _3, _4, _5, FP, FieldType, Shared>
    OptGetFieldMethod<'a, FP, tags::NeedOptionalBitLabel<_1, _2>, tags::NonLdType<_3, _4, _5>>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NeedOptionalBitLabel<_1, _2>,
        TypeTag = tags::NonLdType<_3, _4, _5>,
    >,
    tags::NonLdType<_3, _4, _5>: tags::NumericalTypeTag,
    <FP as FieldProperties>::TypeTag: tags::NumericalTypeTag,
    FieldType: Clone + Into<<tags::NonLdType<_3, _4, _5> as tags::NumericalTypeTag>::NativeType>,
    Shared: SharedObjects,
{
    type OptGetterType = <<FP as FieldProperties>::TypeTag as tags::NumericalTypeTag>::NativeType;
    fn get_opt(&self) -> Option<Self::OptGetterType> {
        if self
            .shared
            .bitfield()
            .get(<FP as FieldProperties>::OPTIONAL_FIELD_BIT_VEC_INDEX)
        {
            Some(self.field.clone().into())
        } else {
            None
        }
    }
}

// assume a proto like this:
// message Person {
//     optional string name = 1;
//     optional uint32 age = 2;
//     repeated Person children = 3;
// }
//
trait PersonFieldsType {
    type NameType;
    type AgeType;
    type ChildrenType;
}
impl PersonFieldsType for () {
    type NameType = ();
    type AgeType = ();
    type ChildrenType = ();
}
impl PersonFieldsType for SimpleFields {
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

struct PersonFieldProperties<const FIELD_NUMBER: i32>;
impl FieldProperties for PersonFieldProperties<2> {
    const OPTIONAL_FIELD_BIT_VEC_INDEX: usize = 1;
    type LabelTag = tags::Optional;
    type TypeTag = tags::UInt32;
}
impl<Fields, Shared> Person<Fields, Shared>
where
    Fields: PersonFieldsType,
    for<'a> FieldAndSharedRef<'a, Fields::AgeType, Shared>:
        OptGetFieldMethod<'a, PersonFieldProperties<2>, tags::Optional, tags::UInt32>,
{
    pub fn age_opt(
        &self,
    ) -> Option<
        <FieldAndSharedRef<Fields::AgeType, Shared> as OptGetFieldMethod<
            '_,
            PersonFieldProperties<2>,
            tags::Optional,
            tags::UInt32,
        >>::OptGetterType,
    > {
        FieldAndSharedRef::new(&self.age, &self._shared).get_opt()
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
struct Person_NameType<NameType>(NameType);
impl<NameType> PersonFieldsType for Person_NameType<NameType> {
    derive_person_params!((), NameType, NameType);
}

fn test() {
    let person = PersonSimple::default();
    let _hoge = person.age_opt();
    assert_eq!(Some(0), person.age_opt());
}
