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
use crate::internal::ser::{FieldData, WireType};
use crate::internal::variant::Variant;
use crate::tags;
use crate::Message;
use crate::{ErrorKind, Result};
use ::std::io::Result as IoResult;

pub trait FieldType {
    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        field_data: FieldData<I>,
    ) -> Result<()> {
        match field_data {
            FieldData::Variant(v) => self.deser_from_variant(bitvec, v),
            FieldData::LengthDelimited(iter) => self.deser_from_ld_iter(bitvec, iter),
            FieldData::Bits32(bits) => self.deser_from_bits32(bitvec, bits),
            FieldData::Bits64(bits) => self.deser_from_bits64(bitvec, bits),
        }
    }
    fn deser_from_variant<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] variant: Variant,
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::Variant as i32))?
    }
    fn deser_from_bits32<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 4],
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::Bits32 as i32))?
    }
    fn deser_from_bits64<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 8],
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::Bits64 as i32))?
    }
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] iter: I,
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::LengthDelimited as i32))?
    }
}

impl<RustType, ProtoType> FieldType for SingularVariantField<RustType, ProtoType>
where
    RustType: PartialEq + Default,
    ProtoType: tags::VariantType + tags::NumericalType<RustType = RustType>,
{
    fn deser_from_variant<B: BitSlice>(&mut self, _bitvec: &mut B, variant: Variant) -> Result<()> {
        let v = variant.get::<ProtoType>()?;
        if v != RustType::default() {
            self.0 = v;
        }
        Ok(())
    }
}

impl<RustType, ProtoType, const BITFIELD_INDEX: usize> FieldType
    for OptionalVariantField<RustType, ProtoType, BITFIELD_INDEX>
where
    ProtoType: tags::VariantType + tags::NumericalType<RustType = RustType>,
{
    fn deser_from_variant<B: BitSlice>(&mut self, bitvec: &mut B, variant: Variant) -> Result<()> {
        self.0 = variant.get::<ProtoType>()?;
        bitvec.set::<BITFIELD_INDEX>(true);
        Ok(())
    }
}

impl<RustType, ProtoType> FieldType for RepeatedVariantField<RustType, ProtoType>
where
    ProtoType: tags::VariantType + tags::NumericalType<RustType = RustType>,
{
    fn deser_from_variant<B: BitSlice>(&mut self, _bitvec: &mut B, variant: Variant) -> Result<()> {
        self.0.push(variant.get::<ProtoType>()?);
        Ok(())
    }

    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        mut iter: I,
    ) -> Result<()> {
        while let Some(var) = Variant::decode_bytes(iter.by_ref())? {
            self.0.push(var.get::<ProtoType>()?)
        }
        Ok(())
    }
}

impl<RustType, ProtoType> FieldType for SingularFixedLengthField<RustType, ProtoType> {}

impl FieldType for SingularStringField {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let vec = iter.collect::<IoResult<Vec<u8>>>()?;
        let s = String::from_utf8(vec)?;
        if !s.is_empty() {
            self.0 = s
        }
        Ok(())
    }
}

impl<const BITFIELD_INDEX: usize> FieldType for OptionalStringField<BITFIELD_INDEX> {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let vec = iter.collect::<IoResult<Vec<u8>>>()?;
        self.0 = String::from_utf8(vec)?;
        bitvec.set::<BITFIELD_INDEX>(true);
        Ok(())
    }
}

impl<M> FieldType for SingularHeapMessageField<M>
where
    M: Message + Default,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let msg = self.0.get_or_insert_with(Default::default).as_mut();
        Ok(msg.merge_from_bytes_iter(Box::new(iter) as Box<dyn Iterator<Item = IoResult<u8>>>)?)
    }
}
