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

use super::*;
use crate::bitvec::BitSlice;
use crate::tags;
use crate::Message;

pub trait NonRepeatedFieldType: FieldType {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_field<'a, B: BitSlice>(
        &'a self,
        bitvec: &B,
        default: Self::GetterType<'a>,
    ) -> Self::GetterType<'a>;
    type OptGetterType<'a>
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::OptGetterType<'_>;
}

pub trait NonRepeatedNonMessageFieldType: FieldType {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Option<Self::GetterType<'_>>;
}

impl<T: NonRepeatedNonMessageFieldType> NonRepeatedFieldType for T {
    type GetterType<'a> = T::GetterType<'a>
    where
        Self: 'a;
    fn get_field<'a, B: BitSlice>(
        &'a self,
        bitvec: &B,
        default: Self::GetterType<'a>,
    ) -> Self::GetterType<'a> {
        self.get_field_opt(bitvec).unwrap_or(default)
    }
    type OptGetterType<'a> = Option<T::GetterType<'a>>
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::OptGetterType<'_> {
        self.get_field_opt(bitvec)
    }
}

impl<RustType, ProtoType> NonRepeatedNonMessageFieldType
    for SingularNumericalField<RustType, ProtoType>
where
    RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Option<Self::GetterType<'_>> {
        if self.0 == RustType::default() {
            None
        } else {
            Some(self.0.clone())
        }
    }
}

impl<RustType, ProtoType, const BITFIELD_INDEX: usize> NonRepeatedNonMessageFieldType
    for OptionalNumericalField<RustType, ProtoType, BITFIELD_INDEX>
where
    RustType: Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Option<Self::GetterType<'_>> {
        bitvec.get::<BITFIELD_INDEX>().then_some(self.0.clone())
    }
}

impl NonRepeatedNonMessageFieldType for SingularStringField {
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Option<Self::GetterType<'_>> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.as_ref())
        }
    }
}

impl<const BITFIELD_INDEX: usize> NonRepeatedNonMessageFieldType
    for OptionalStringField<BITFIELD_INDEX>
{
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Option<Self::GetterType<'_>> {
        bitvec.get::<BITFIELD_INDEX>().then_some(&self.0)
    }
}

impl<M> NonRepeatedFieldType for SingularHeapMessageField<M>
where
    M: Message + Default,
{
    type GetterType<'a> = Option<&'a M>
    where
        Self: 'a;
    fn get_field<'a, B: BitSlice>(
        &'a self,
        bitvec: &B,
        _default: Self::GetterType<'a>,
    ) -> Self::GetterType<'a> {
        self.get_field_opt(bitvec)
    }

    type OptGetterType<'a> = Option<&'a M>
    where
        Self: 'a;

    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Self::OptGetterType<'_> {
        self.0.as_deref()
    }
}