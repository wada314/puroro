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

use super::{FieldType, NonRepeatedFieldType, RepeatedFieldType};
use crate::internal::bitvec::BitSlice;
use crate::internal::ser::ser_bytes_shared;
use crate::internal::tags;
use crate::Result;
use ::std::io::{Result as IoResult, Write};
use ::std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct SingularUnsizedField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);
#[derive(Default, Clone)]
pub struct OptionalUnsizedField<RustType, ProtoType, const BITFIELD_INDEX: usize>(
    RustType,
    PhantomData<ProtoType>,
);
#[derive(Default, Clone)]
pub struct RepeatedUnsizedField<RustType, ProtoType>(Vec<RustType>, PhantomData<ProtoType>);

impl<RustType, ProtoType> FieldType for SingularUnsizedField<RustType, ProtoType>
where
    RustType: Default + PartialEq,
    ProtoType: tags::UnsizedType<RustType = RustType>,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let val = ProtoType::from_bytes_iter(iter)?;
        if val != RustType::default() {
            self.0 = val
        }
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if self.0 != RustType::default() {
            ser_bytes_shared(ProtoType::to_bytes_slice(&self.0)?, number, out)?;
        }
        Ok(())
    }
}

impl<RustType, ProtoType, const BITFIELD_INDEX: usize> FieldType
    for OptionalUnsizedField<RustType, ProtoType, BITFIELD_INDEX>
where
    ProtoType: tags::UnsizedType<RustType = RustType>,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        self.0 = ProtoType::from_bytes_iter(iter)?;
        bitvec.set(BITFIELD_INDEX, true);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if bitvec.get(BITFIELD_INDEX) {
            ser_bytes_shared(ProtoType::to_bytes_slice(&self.0)?, number, out)?;
        }
        Ok(())
    }
}

impl<RustType, ProtoType> FieldType for RepeatedUnsizedField<RustType, ProtoType>
where
    ProtoType: tags::UnsizedType<RustType = RustType>,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        self.0.push(ProtoType::from_bytes_iter(iter)?);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        for val in &self.0 {
            ser_bytes_shared(ProtoType::to_bytes_slice(val)?, number, out)?;
        }
        Ok(())
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

impl<ProtoType> RepeatedFieldType for RepeatedUnsizedField<ProtoType::RustType, ProtoType>
where
    ProtoType::RustType: PartialEq + Default + Clone,
    ProtoType: tags::UnsizedType,
{
    type ScalarType = ProtoType::RustType;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
    type ContainerType = Vec<Self::ScalarType>;
    fn get_field_mut<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}
