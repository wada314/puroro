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
use crate::internal::bitvec::BitSlice;
use crate::internal::tags;
use crate::Message;

pub trait NonRepeatedFieldType: FieldType {
    /// An optional getter type, which is used by the message struct's
    /// getter methods.
    /// int32 => Option<i32>
    /// String => Option<&'a str>
    /// Message => Option<&'a Message>
    type GetterOptType<'a>
    where
        Self: 'a;

    /// A default field type which can be defined in proto2.
    /// int32 => i32
    /// String => &'static str
    /// Message => unreachable!()
    type DefaultValueType;

    /// A getter type, which overrides `Self::GetterOptType`'s `None` case
    /// by the `Self::DefaultValueType`. Exceptionally, message type cannot get
    /// this benefit so it's still an optional type.
    /// int32 => i32
    /// String => &'a str
    /// Message => Option<&'a Message>
    type GetterOrElseType<'a>
    where
        Self: 'a;

    /// A mutable getter type.
    /// int32 => &'a mut i32
    /// String => &'a mut String
    /// Message => &'a mut Message
    type GetterMutType<'a>
    where
        Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a>;
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::GetterOptType<'_>;
    fn get_field_mut<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        bitvec: &mut B,
        default: F,
    ) -> Self::GetterMutType<'a>;
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B);
}

impl<ProtoType> NonRepeatedFieldType for SingularNumericalField<ProtoType::RustType, ProtoType>
where
    ProtoType::RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType,
{
    type GetterOptType<'a> = Option<ProtoType::RustType>
    where
        Self: 'a;
    type DefaultValueType = ProtoType::RustType;
    type GetterOrElseType<'a> = ProtoType::RustType
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut ProtoType::RustType where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec).unwrap_or_else(default)
    }
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Self::GetterOptType<'_> {
        if self.0 == ProtoType::RustType::default() {
            None
        } else {
            Some(self.0.clone())
        }
    }
    fn get_field_mut<'a, B: BitSlice, D: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        _bitvec: &mut B,
        _default: D,
    ) -> Self::GetterMutType<'a> {
        &mut self.0
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0 = ProtoType::RustType::default();
    }
}

impl<ProtoType, const BITFIELD_INDEX: usize> NonRepeatedFieldType
    for OptionalNumericalField<ProtoType::RustType, ProtoType, BITFIELD_INDEX>
where
    ProtoType::RustType: Clone,
    ProtoType: tags::NumericalType,
{
    type GetterOptType<'a> = Option<ProtoType::RustType>
    where
        Self: 'a;
    type DefaultValueType = ProtoType::RustType;
    type GetterOrElseType<'a> = ProtoType::RustType
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut ProtoType::RustType where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec).unwrap_or_else(default)
    }
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::GetterOptType<'_> {
        bitvec.get(BITFIELD_INDEX).then_some(self.0.clone())
    }
    fn get_field_mut<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        bitvec: &mut B,
        default: F,
    ) -> Self::GetterMutType<'a> {
        if !bitvec.get(BITFIELD_INDEX) {
            self.0 = default();
            bitvec.set(BITFIELD_INDEX, true);
        }
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B) {
        bitvec.set(BITFIELD_INDEX, false);
    }
}

impl<ProtoType> NonRepeatedFieldType for SingularUnsizedField<ProtoType::RustType, ProtoType>
where
    ProtoType: 'static + tags::UnsizedType,
    ProtoType::RustType: Default + PartialEq,
{
    type GetterOptType<'a> = Option<ProtoType::RustRefType<'a>>
    where
        Self: 'a;
    type DefaultValueType = ProtoType::DefaultValueType;
    type GetterOrElseType<'a> = ProtoType::RustRefType<'a>
    where
        Self: 'a;
    type GetterMutType<'a> = ProtoType::RustMutType<'a> where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec)
            .unwrap_or_else(|| ProtoType::default_to_ref(default()))
    }
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Self::GetterOptType<'_> {
        if self.0 == ProtoType::RustType::default() {
            None
        } else {
            Some(ProtoType::as_ref(&self.0))
        }
    }
    fn get_field_mut<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        _bitvec: &mut B,
        _default: F,
    ) -> Self::GetterMutType<'a> {
        ProtoType::as_mut(&mut self.0)
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0 = ProtoType::RustType::default();
    }
}

impl<ProtoType, const BITFIELD_INDEX: usize> NonRepeatedFieldType
    for OptionalUnsizedField<ProtoType::RustType, ProtoType, BITFIELD_INDEX>
where
    ProtoType: 'static + tags::UnsizedType,
    ProtoType::RustType: Default + PartialEq,
{
    type GetterOptType<'a> = Option<ProtoType::RustRefType<'a>>
    where
        Self: 'a;
    type DefaultValueType = ProtoType::DefaultValueType;
    type GetterOrElseType<'a> = ProtoType::RustRefType<'a>
    where
        Self: 'a;
    type GetterMutType<'a> = ProtoType::RustMutType<'a> where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        default: F,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec)
            .unwrap_or_else(|| ProtoType::default_to_ref(default()))
    }
    fn get_field_opt<B: BitSlice>(&self, bitvec: &B) -> Self::GetterOptType<'_> {
        bitvec
            .get(BITFIELD_INDEX)
            .then_some(ProtoType::as_ref(&self.0))
    }
    fn get_field_mut<'a, B: BitSlice, F: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        bitvec: &mut B,
        default: F,
    ) -> Self::GetterMutType<'a> {
        if !bitvec.get(BITFIELD_INDEX) {
            self.0 = ProtoType::default_to_value(default());
            bitvec.set(BITFIELD_INDEX, true);
        }
        ProtoType::as_mut(&mut self.0)
    }
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B) {
        bitvec.set(BITFIELD_INDEX, false);
        self.0 = ProtoType::RustType::default();
    }
}

impl<M> NonRepeatedFieldType for SingularHeapMessageField<M>
where
    M: Message + Default,
{
    type GetterOptType<'a> = Option<&'a M>
    where
        Self: 'a;
    type DefaultValueType = ();
    type GetterOrElseType<'a> = Option<&'a M>
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut M where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, D: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        _default: D,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec)
    }
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Self::GetterOptType<'_> {
        self.0.as_deref()
    }
    fn get_field_mut<'a, B: BitSlice, D: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        _bitvec: &mut B,
        _default: D,
    ) -> Self::GetterMutType<'a> {
        self.0.get_or_insert_with(Default::default)
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0 = None;
    }
}
