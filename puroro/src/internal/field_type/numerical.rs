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
use crate::internal::ser::{ser_numerical_shared, ser_wire_and_number, ScopedIter, WireType};
use crate::internal::tags;
use crate::internal::variant::Variant;
use crate::repeated::RepeatedFieldView;
use crate::Result;
use ::std::io::{Result as IoResult, Write};
use ::std::marker::PhantomData;
use ::std::ops::Index;
use ::std::slice;

#[derive(Default, Clone)]
pub struct SingularNumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);
#[derive(Default, Clone)]
pub struct OptionalNumericalField<RustType, ProtoType, const BITFIELD_INDEX: usize>(
    RustType,
    PhantomData<ProtoType>,
);
#[derive(Default, Clone)]
pub struct RepeatedNumericalField<RustType, ProtoType>(Vec<RustType>, PhantomData<ProtoType>);

impl<RustType, ProtoType> FieldType for SingularNumericalField<RustType, ProtoType>
where
    RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    fn new<B: BitSlice>(_bitvec: &mut B) -> Self {
        Self(Default::default(), PhantomData)
    }

    fn deser_from_variant<B: BitSlice>(&mut self, _bitvec: &mut B, variant: Variant) -> Result<()> {
        let v = variant.get::<ProtoType>()?;
        if v != RustType::default() {
            self.0 = v;
        }
        Ok(())
    }
    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        while let Some(var) = Variant::decode_bytes(iter.by_ref())? {
            let v = var.get::<ProtoType>()?;
            if v != RustType::default() {
                self.0 = v
            }
        }
        Ok(())
    }
    fn deser_from_bits32<B: BitSlice>(&mut self, _bitvec: &mut B, bits: [u8; 4]) -> Result<()> {
        let x = <ProtoType as tags::NumericalType>::from_bits32(bits)?;
        if x != RustType::default() {
            self.0 = x;
        }
        Ok(())
    }
    fn deser_from_bits64<B: BitSlice>(&mut self, _bitvec: &mut B, bits: [u8; 8]) -> Result<()> {
        let x = <ProtoType as tags::NumericalType>::from_bits64(bits)?;
        if x != RustType::default() {
            self.0 = x;
        }
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if self.0 == Default::default() {
            return Ok(());
        }
        ser_numerical_shared::<_, ProtoType, _>(self.0.clone(), number, out)?;
        Ok(())
    }
}

impl<RustType, ProtoType, const BITFIELD_INDEX: usize> FieldType
    for OptionalNumericalField<RustType, ProtoType, BITFIELD_INDEX>
where
    RustType: Clone + Default,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    fn new<B: BitSlice>(bitvec: &mut B) -> Self {
        bitvec.set(BITFIELD_INDEX, false);
        Self(Default::default(), PhantomData)
    }

    fn deser_from_variant<B: BitSlice>(&mut self, bitvec: &mut B, variant: Variant) -> Result<()> {
        self.0 = variant.get::<ProtoType>()?;
        bitvec.set(BITFIELD_INDEX, true);
        Ok(())
    }
    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        while let Some(var) = Variant::decode_bytes(iter.by_ref())? {
            self.0 = var.get::<ProtoType>()?;
        }
        Ok(())
    }
    fn deser_from_bits32<B: BitSlice>(&mut self, bitvec: &mut B, bits: [u8; 4]) -> Result<()> {
        self.0 = <ProtoType as tags::NumericalType>::from_bits32(bits)?;
        bitvec.set(BITFIELD_INDEX, true);
        Ok(())
    }
    fn deser_from_bits64<B: BitSlice>(&mut self, bitvec: &mut B, bits: [u8; 8]) -> Result<()> {
        self.0 = <ProtoType as tags::NumericalType>::from_bits64(bits)?;
        bitvec.set(BITFIELD_INDEX, true);
        Ok(())
    }
    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if !bitvec.get(BITFIELD_INDEX) {
            return Ok(());
        }
        ser_numerical_shared::<_, ProtoType, _>(self.0.clone(), number, out)?;
        Ok(())
    }
}

impl<RustType, ProtoType> FieldType for RepeatedNumericalField<RustType, ProtoType>
where
    RustType: Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    fn new<B: BitSlice>(_bitvec: &mut B) -> Self {
        Self(Default::default(), PhantomData)
    }

    fn deser_from_variant<B: BitSlice>(&mut self, _bitvec: &mut B, variant: Variant) -> Result<()> {
        self.0.push(variant.get::<ProtoType>()?);
        Ok(())
    }

    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        while let Some(var) = Variant::decode_bytes(iter.by_ref())? {
            self.0.push(var.get::<ProtoType>()?)
        }
        Ok(())
    }
    fn deser_from_bits32<B: BitSlice>(&mut self, _bitvec: &mut B, bits: [u8; 4]) -> Result<()> {
        self.0
            .push(<ProtoType as tags::NumericalType>::from_bits32(bits)?);
        Ok(())
    }
    fn deser_from_bits64<B: BitSlice>(&mut self, _bitvec: &mut B, bits: [u8; 8]) -> Result<()> {
        self.0
            .push(<ProtoType as tags::NumericalType>::from_bits64(bits)?);
        Ok(())
    }
    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if let Some(first) = self.0.first() {
            match ProtoType::to_wire_type(first.clone())? {
                tags::NumericalWireType::Variant(_) => {
                    // write as length delimited
                    ser_wire_and_number(WireType::LengthDelimited, number, out)?;

                    let mut tempvec = Vec::new();
                    for item in &self.0 {
                        if let tags::NumericalWireType::Variant(bits) =
                            <ProtoType as tags::NumericalType>::to_wire_type(item.clone())?
                        {
                            let var = Variant::new(bits);
                            var.encode_bytes(&mut tempvec)?;
                        } else {
                            unreachable!()
                        }
                    }
                    let length_var = Variant::from_i32(tempvec.len().try_into()?);
                    length_var.encode_bytes(out)?;
                    out.write_all(&tempvec)?;
                }
                tags::NumericalWireType::Bits32(_) | tags::NumericalWireType::Bits64(_) => {
                    for item in &self.0 {
                        ser_numerical_shared::<_, ProtoType, _>(item.clone(), number, out)?;
                    }
                }
            };
        }
        Ok(())
    }
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
    ProtoType::RustType: Clone + Default,
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

impl<ProtoType> RepeatedFieldType for RepeatedNumericalField<ProtoType::RustType, ProtoType>
where
    ProtoType::RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType,
{
    type RepeatedFieldViewType<'a> = RepeatedFieldViewImpl<'a, ProtoType::RustType>
    where
        Self: 'a;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> Self::RepeatedFieldViewType<'_> {
        RepeatedFieldViewImpl(self.0.as_slice())
    }

    type ContainerType = Vec<ProtoType::RustType>;
    fn get_field_mut<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}

trait CheckNumType: Sized {
    fn maybe_i32(self) -> Option<i32> {
        None
    }
}
impl CheckNumType for i32 {
    fn maybe_i32(self) -> Option<i32> {
        Some(self)
    }
}
impl CheckNumType for i64 {}
impl CheckNumType for u32 {}
impl CheckNumType for u64 {}
impl CheckNumType for f32 {}
impl CheckNumType for f64 {}
impl CheckNumType for bool {}

pub struct RepeatedFieldViewImpl<'a, T>(&'a [T]);
impl<'a, T> IntoIterator for RepeatedFieldViewImpl<'a, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, T> Index<usize> for RepeatedFieldViewImpl<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<'a, T> RepeatedFieldView<'a> for RepeatedFieldViewImpl<'a, T> {
    type Item = T;
    fn len(&self) -> usize {
        self.0.len()
    }
}
