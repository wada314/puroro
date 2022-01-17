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
use internal::BitVec;

pub trait SharedObjects {
    type AllocatorType;
    type BitvecType;
    fn alloc(&self) -> &Self::AllocatorType;
    fn bitvec(&self) -> &Self::BitvecType;
    fn bitvec_mut(&mut self) -> &mut Self::BitvecType;
}
impl<A, B> SharedObjects for (A, B) {
    type AllocatorType = A;
    type BitvecType = B;
    fn alloc(&self) -> &Self::AllocatorType {
        &self.0
    }
    fn bitvec(&self) -> &Self::BitvecType {
        &self.1
    }
    fn bitvec_mut(&mut self) -> &mut Self::BitvecType {
        &mut self.1
    }
}

pub trait MessageProperties {
    const OPTIONAL_FIELD_BITVEC_START_INDEX: usize;
}
pub trait FieldProperties {
    type MessageProperties: self::MessageProperties;
    const OPTIONAL_FIELD_BITVEC_INDEX: usize = 0;
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
}

pub trait GetFieldDataSet<FP> {
    type GetterType<'a>
    where
        Self: 'a;
    fn as_getter(&self) -> Self::GetterType<'_>;
}
pub trait OptGetFieldDataSet<FP> {
    type OptGetterType<'a>
    where
        Self: 'a;
    fn as_opt_getter(&self) -> Option<Self::OptGetterType<'_>>;
}

impl<_1, _2, _3, FP, Shared> OptGetFieldDataSet<FP>
    for (
        &<<FP as FieldProperties>::TypeTag as tags::NumericalTypeTag>::NativeType,
        &Shared,
    )
where
    FP: FieldProperties<LabelTag = tags::LabelNonRepeated<_1, _2, _3>>,
    <FP as FieldProperties>::TypeTag: tags::NumericalTypeTag,
    Shared: SharedObjects,
    <Shared as SharedObjects>::BitvecType: BitVec,
{
    type OptGetterType<'a>
    where
        Self: 'a,
    = <<FP as FieldProperties>::TypeTag as tags::NumericalTypeTag>::NativeType;
    fn as_opt_getter(&self) -> Option<Self::OptGetterType<'_>> {
        todo!()
    }
}

trait PersonParamsTuple {
    type NameType;
    type AgeType;
    type ChildrenType;
}
impl PersonParamsTuple for () {
    type NameType = ();
    type AgeType = ();
    type ChildrenType = ();
}
impl<NameType, AgeType, ChildrenType> PersonParamsTuple for (NameType, AgeType, ChildrenType) {
    type NameType = NameType;
    type AgeType = AgeType;
    type ChildrenType = ChildrenType;
}
struct Person<T: PersonParamsTuple> {
    name: <T as PersonParamsTuple>::NameType,
    age: <T as PersonParamsTuple>::AgeType,
    children: <T as PersonParamsTuple>::ChildrenType,
}
impl<T: PersonParamsTuple<AgeType = u32>> Person<T> {
    fn age(&self) -> u32 {
        self.age
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
            type $ident = <$base as PersonParamsTuple>::$ident;
        };
    }
struct Person_NameType<NameType>(NameType);
impl<NameType> PersonParamsTuple for Person_NameType<NameType> {
    derive_person_params!((), NameType, NameType);
}

#[test]
fn test() {
    let pn_str = Person_NameType::<&str>("hoge");
    assert_eq!(16, std::mem::size_of_val(&pn_str));
    let pn_string = Person_NameType::<String>("hoge".to_string());
    assert_eq!(24, std::mem::size_of_val(&pn_str));
}
