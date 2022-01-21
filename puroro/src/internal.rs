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

pub mod bool;
pub mod fixed_bits;
pub mod impls;
pub mod methods;
pub mod types;
pub mod utils;
pub mod variant;

pub use impls::bumpalo::AddBumpVecView;
pub use impls::bumpalo::NoAllocBox as NoAllocBumpBox;
pub use impls::bumpalo::NoAllocString as NoAllocBumpString;
pub use impls::bumpalo::NoAllocVec as NoAllocBumpVec;
pub use impls::bumpalo::RefMutString as RefMutBumpString;
pub use impls::bumpalo::RefMutVec as RefMutBumpVec;
pub use impls::simple::{SimpleFields, SimpleShared};

use crate::tags;
use ::bitvec::array::BitArray;
use ::bitvec::order::BitOrder;
use ::bitvec::slice::BitSlice;
use ::bitvec::view::BitViewSized;

pub trait Bitfield {
    fn get(&self, index: usize) -> bool;
    fn set(&mut self, index: usize, val: bool);
}
impl<O: BitOrder, V: BitViewSized> Bitfield for BitArray<O, V> {
    fn get(&self, index: usize) -> bool {
        <BitSlice<_, _>>::get(&**self, index).map_or(false, |b| *b)
    }
    fn set(&mut self, index: usize, val: bool) {
        <BitSlice<_, _>>::set(&mut **self, index, val);
    }
}

impl Bitfield for () {
    fn get(&self, _: usize) -> bool {
        false
    }
    fn set(&mut self, _: usize, _: bool) {
        unimplemented!()
    }
}

pub struct FlipBitOn<Base, const INDEX: usize>(Base);
impl<Base: Bitfield, const INDEX: usize> Bitfield for FlipBitOn<Base, INDEX> {
    fn get(&self, index: usize) -> bool {
        index == INDEX || self.0.get(index)
    }
    fn set(&mut self, _: usize, _: bool) {
        unimplemented!()
    }
}

pub trait IsDefault {
    fn is_default(&self) -> bool;
}
impl<T: Default + PartialEq> IsDefault for T {
    fn is_default(&self) -> bool {
        *self == Default::default()
    }
}
impl IsDefault for NoAllocBumpString {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
impl<T> IsDefault for NoAllocBumpVec<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

pub trait MessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize;
    type Fields<const NUMBER: i32>;
}
pub trait FieldProperties {
    type MessageProperties: self::MessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType;
}

pub trait FieldsContainer {}

pub trait HasField<const NUMBER: i32>: FieldsContainer {
    type Type;
    fn get(&self) -> &Self::Type;
    fn get_mut(&mut self) -> &mut Self::Type;
}
#[macro_export]
macro_rules! impl_has_field {
    ($container:ty, $number:expr, $ty:ty, $name:ident) => {
        impl crate::internal::HasField<$number> for $container {
            type Type = $ty;
            fn get(&self) -> &Self::Type {
                &self.$name
            }
            fn get_mut(&mut self) -> &mut Self::Type {
                &mut self.$name
            }
        }
    };
}
#[macro_export]
macro_rules! impl_scalar_getters {
    ($mp:ty, $number:expr, $get:ident, $get_opt:ident) => {
        impl<ImplTag, FieldsType, SharedType> Message<$mp, ImplTag, FieldsType, SharedType>
        where
            Self: crate::internal::methods::GetOptFieldMethod<$number>,
        {
            pub fn $get_opt(
                &self,
            ) -> <Self as crate::internal::methods::GetOptFieldMethod<$number>>::GetterType<'_>
            {
                <Self as crate::internal::methods::GetOptFieldMethod<$number>>::get_opt(self)
            }
        }
    };
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
pub struct FieldAndSharedMut<'a, Field, Shared> {
    field: &'a mut Field,
    shared: &'a mut Shared,
}
impl<'a, Field, Shared> FieldAndSharedMut<'a, Field, Shared> {
    pub fn new(field: &'a mut Field, shared: &'a mut Shared) -> Self {
        Self { field, shared }
    }
}

pub trait SharedBitfield {
    type BitfieldType: Bitfield;
    fn bitfield(&self) -> &Self::BitfieldType;
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType;
}
pub trait SharedAllocator {
    type AllocatorType;
    fn alloc(&self) -> &Self::AllocatorType;
}
