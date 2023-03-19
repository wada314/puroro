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
use crate::internal::ser::{ser_bytes_shared, ScopedIter};
use crate::internal::tags;
use crate::repeated::RepeatedFieldView;
use crate::Result;
use ::std::io::{Result as IoResult, Write};
use ::std::marker::PhantomData;
use ::std::ops::{Deref, Index};
use ::std::slice;

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
    ProtoType: tags::UnsizedType<RustOwnedType = RustType>,
{
    fn new<B: BitSlice>(_bitvec: &mut B) -> Self {
        Self(Default::default(), PhantomData)
    }

    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
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
    RustType: Default,
    ProtoType: tags::UnsizedType<RustOwnedType = RustType>,
{
    fn new<B: BitSlice>(bitvec: &mut B) -> Self {
        bitvec.set(BITFIELD_INDEX, false);
        Self(Default::default(), PhantomData)
    }

    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
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
    ProtoType: tags::UnsizedType<RustOwnedType = RustType>,
{
    fn new<B: BitSlice>(_bitvec: &mut B) -> Self {
        Self(Default::default(), PhantomData)
    }

    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
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

impl<ProtoType> NonRepeatedFieldType for SingularUnsizedField<ProtoType::RustOwnedType, ProtoType>
where
    ProtoType: 'static + tags::UnsizedType,
    ProtoType::RustOwnedType: Default + PartialEq,
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
        if self.0 == ProtoType::RustOwnedType::default() {
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
        self.0 = ProtoType::RustOwnedType::default();
    }
}

impl<ProtoType, const BITFIELD_INDEX: usize> NonRepeatedFieldType
    for OptionalUnsizedField<ProtoType::RustOwnedType, ProtoType, BITFIELD_INDEX>
where
    ProtoType: 'static + tags::UnsizedType,
    ProtoType::RustOwnedType: Default + PartialEq,
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
        self.0 = ProtoType::RustOwnedType::default();
    }
}

impl<ProtoType> RepeatedFieldType for RepeatedUnsizedField<ProtoType::RustOwnedType, ProtoType>
where
    ProtoType::RustOwnedType: PartialEq + Default + Clone,
    ProtoType: tags::UnsizedType,
{
    type RepeatedFieldViewType<'a> = RepeatedFieldViewImpl<'a, ProtoType::RustOwnedType>
    where
        Self: 'a;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> Self::RepeatedFieldViewType<'_> {
        RepeatedFieldViewImpl(self.0.as_slice())
    }

    type ContainerType = Vec<ProtoType::RustOwnedType>;
    fn get_field_mut<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}

pub struct RepeatedFieldViewImpl<'a, T>(pub(crate) &'a [T]);
impl<'a, T: Deref> IntoIterator for RepeatedFieldViewImpl<'a, T> {
    type Item = &'a T::Target;
    type IntoIter = Derefed<'a, slice::Iter<'a, T>, T>;
    fn into_iter(self) -> Self::IntoIter {
        Derefed(self.0.into_iter(), PhantomData)
    }
}
impl<'a, T: Deref> Index<usize> for RepeatedFieldViewImpl<'a, T> {
    type Output = T::Target;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index).deref()
    }
}
impl<'a, T: Deref> RepeatedFieldView<'a> for RepeatedFieldViewImpl<'a, T> {
    type Item = T::Target;
    fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct Derefed<'a, I, B: ?Sized>(I, PhantomData<&'a B>);
impl<'a, I, B> Iterator for Derefed<'a, I, B>
where
    I: Iterator<Item = &'a B>,
    B: Deref,
{
    type Item = &'a B::Target;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|borrowed| B::deref(borrowed))
    }
}
