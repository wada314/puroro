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
pub mod de;
pub mod fixed_bits;
pub mod impls;
pub mod methods;
pub mod se;
pub mod types;
pub mod utils;
pub mod variant;

pub use impls::simple::{SimpleFields, SimpleShared};

pub use impls::bumpalo::NoAllocBox as NoAllocBumpBox;
pub use impls::bumpalo::NoAllocString as NoAllocBumpString;
pub use impls::bumpalo::NoAllocVec as NoAllocBumpVec;
pub use impls::bumpalo::RefMutString as RefMutBumpString;
pub use impls::bumpalo::RefMutVec as RefMutBumpVec;
pub use impls::bumpalo::{AddBumpVecView, BumpDefault};

use crate::tags;
use ::bitvec::array::BitArray;
use ::bitvec::order::BitOrder;
use ::bitvec::slice::BitSlice;
use ::bitvec::view::BitViewSized;
use ::std::fmt::{self, Debug};
use ::std::ops::{Deref, DerefMut};

pub trait Bitfield {
    fn get(&self, index: usize) -> bool;
}
impl<O: BitOrder, V: BitViewSized> Bitfield for BitArray<O, V> {
    fn get(&self, index: usize) -> bool {
        <BitSlice<_, _>>::get(&**self, index).map_or(false, |b| *b)
    }
}
impl Bitfield for () {
    fn get(&self, _index: usize) -> bool {
        false
    }
}
pub struct FlipBitOn<Base, const INDEX: usize>(Base);
impl<Base: Bitfield, const INDEX: usize> Bitfield for FlipBitOn<Base, INDEX> {
    fn get(&self, index: usize) -> bool {
        index == INDEX || self.0.get(index)
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

pub struct Bare<T>(T);
impl<T> Bare<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
    pub fn inner(self) -> T {
        self.0
    }
}
impl<T> Deref for Bare<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Bare<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<T> for Bare<T> {
    fn from(from: T) -> Self {
        Self(from)
    }
}
impl<T: Default> Default for Bare<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: PartialEq> PartialEq for Bare<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Clone> Clone for Bare<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Copy> Copy for Bare<T> {}
impl<T: Debug> Debug for Bare<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as Debug>::fmt(&self.0, f)
    }
}

pub trait MessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize;
}
pub trait FieldProperties {
    type MessageProperties: self::MessageProperties;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
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

pub trait SharedObjects {
    type AllocatorType;
    type BitfieldType: Bitfield;
    fn alloc(&self) -> &Self::AllocatorType;
    fn bitfield(&self) -> &Self::BitfieldType;
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType;
}
