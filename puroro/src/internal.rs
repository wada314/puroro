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
pub mod owned;
pub mod types;
pub mod variant;
pub mod option;

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

pub trait MessageProperties {
    const BITFIELD_OPTIONAL_FIELD_COUNT: usize;
    type Fields<const NUMBER: i32>;
}
pub trait FieldProperties {
    type LabelTag: tags::FieldLabelTag;
    type TypeTag: tags::FieldTypeTag;
    const DEFAULT_VALUE: <Self::TypeTag as tags::FieldTypeTag>::DefaultValueType;
    const OPTIONAL_FIELD_BITFIELD_INDEX: usize = 0;
}

pub trait ImplProperties {
    type ImplTag;
    type FieldsType;
    type SharedType;
}

pub trait GetField<const NUMBER: i32> {
    type Type;
    fn get_field(&self) -> &Self::Type;
}
pub trait GetFieldMut<const NUMBER: i32>: GetField<NUMBER> {
    fn get_field_mut(&mut self) -> &mut <Self as GetField<NUMBER>>::Type;
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
