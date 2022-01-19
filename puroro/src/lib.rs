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
use internal::{FieldProperties, MessageProperties};

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

pub trait GetFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get(&self) -> Self::GetterType {
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_impl(self)
    }
}

pub trait GetOptFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetOptFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get_opt(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetOptFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get_opt(&self) -> Self::GetterType {
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_opt_impl(
            &self,
        )
    }
}

// for [optional|required] numeric types
impl<'a, _1, _2, _3, _4, _5, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::NonLdType<_3, _4, _5>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NeedOptionalBitLabel<_1, _2>,
        TypeTag = tags::NonLdType<_3, _4, _5>,
    >,
    tags::NonLdType<_3, _4, _5>: tags::NumericalTypeTag,
    FP::TypeTag: tags::NumericalTypeTag,
    FieldType: Clone + Into<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX
            + FP::MessageProperties::OPTIONAL_FIELD_BITFIELD_START_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(self.field.clone().into())
        } else {
            None
        }
    }
}

// for [optional|required] string type
impl<'a, _1, _2, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::String,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1, _2>, TypeTag = tags::String>,
    FieldType: AsRef<str>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<&'a str>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX
            + FP::MessageProperties::OPTIONAL_FIELD_BITFIELD_START_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(AsRef::as_ref(self.field))
        } else {
            None
        }
    }
}

// for repeated message type
impl<'a, FP, MP, MessageType, Shared>
    GetFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Repeated, tags::Message<MP>>
    for FieldAndSharedRef<'a, Vec<MessageType>, Shared>
where
    FP: FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::Message<MP>>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = &'a [MessageType];
    fn get_impl(&self) -> Self::GetterTypeImpl {
        self.field
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
    const OPTIONAL_FIELD_BITFIELD_START_INDEX: usize = 0;
}
struct PersonFieldProperties<const FIELD_NUMBER: i32>;
impl FieldProperties for PersonFieldProperties<1> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Optional;
    type TypeTag = tags::String;
}
impl FieldProperties for PersonFieldProperties<2> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 1;
    type LabelTag = tags::Optional;
    type TypeTag = tags::UInt32;
}
impl FieldProperties for PersonFieldProperties<3> {
    type MessageProperties = PersonMessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag = tags::Repeated;
    type TypeTag = tags::Message<PersonMessageProperties>;
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

fn test() {
    let person = PersonSimple::default();
    let _hoge = person.age_opt();
    assert_eq!(Some(0), person.age_opt());
    let _foo = person.name_opt();
    assert_eq!(Some(""), person.name_opt());
    let _hoga = person.children();
}
