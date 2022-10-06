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

use super::oneof_field_type::{
    BytesField, HeapMessageField, NumericalField, OneofFieldType, StringField,
};
use crate::bitvec::BitSlice;

pub trait OneofUnion {
    type CaseRef<'a>: OneofCaseRef<'a, Union = Self>
    where
        Self: 'a;
    fn case_ref<B: BitSlice>(&self, bits: &B) -> Option<Self::CaseRef<'_>>;
    fn clear<B: BitSlice>(&mut self, bits: &mut B);
    fn clone<B: BitSlice>(&self, bits: &B) -> Self;
    fn get_view<B: BitSlice, INDEX: u32>(&self, bits: &B) -> ????
}

pub trait OneofCase: Sized {
    const BITFIELD_BEGIN: usize;
    const BITFIELD_END: usize;
    fn from_u32(x: u32) -> Option<Self>;
    fn from_bitslice<B: BitSlice>(b: &B) -> Option<Self> {
        Self::from_u32(b.get_range(Self::BITFIELD_BEGIN..Self::BITFIELD_END))
    }
    fn into_u32(self) -> u32;
}

pub trait OneofCaseRef<'a>: Sized {
    type Case: OneofCase;
    type Union;
    fn from_union_and_case(u: &'a Self::Union, case: Self::Case) -> Self;
}

pub trait OneofUnionView<const INDEX: u32> {
    type FieldType: OneofFieldType;
    fn get_field_opt(&self) -> Option<<Self::FieldType as OneofFieldType>::GetterType<'_>>;
    type GetterType<'a>
    where
        Self: 'a;
    type DefaultValueType<'a>
    where
        Self: 'a;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_>;
}

impl<RustType, ProtoType, const INDEX: u32> OneofUnionView<INDEX>
    for Option<&NumericalField<RustType, ProtoType>>
where
    RustType: Clone,
{
    type FieldType = NumericalField<RustType, ProtoType>;
    fn get_field_opt(&self) -> Option<<Self::FieldType as OneofFieldType>::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
    type GetterType<'a> = RustType
    where
        Self: 'a;
    type DefaultValueType<'a> = RustType
    where
        Self: 'a;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_> {
        <Self as OneofUnionView<INDEX>>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<const INDEX: u32> OneofUnionView<INDEX> for Option<&BytesField> {
    type FieldType = BytesField;
    fn get_field_opt(&self) -> Option<<Self::FieldType as OneofFieldType>::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
    type GetterType<'a> = &'a [u8]
    where
        Self: 'a;
    type DefaultValueType<'a> = &'a [u8]
    where
        Self: 'a;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_> {
        <Self as OneofUnionView<INDEX>>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<const INDEX: u32> OneofUnionView<INDEX> for Option<&StringField> {
    type FieldType = StringField;
    fn get_field_opt(&self) -> Option<<Self::FieldType as OneofFieldType>::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    type DefaultValueType<'a> = &'a str
    where
        Self: 'a;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_> {
        <Self as OneofUnionView<INDEX>>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<M: Default, const INDEX: u32> OneofUnionView<INDEX> for Option<&HeapMessageField<M>> {
    type FieldType = HeapMessageField<M>;
    fn get_field_opt(&self) -> Option<<Self::FieldType as OneofFieldType>::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
    type GetterType<'a> = Option<&'a M>
    where
        Self: 'a;
    type DefaultValueType<'a> = ()
    where
        Self: 'a;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        _default: D,
    ) -> Self::GetterType<'_> {
        <Self as OneofUnionView<INDEX>>::get_field_opt(self)
    }
}
