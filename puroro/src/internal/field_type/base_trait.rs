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
use crate::internal::ser::{
    ser_bytes_shared, ser_numerical_shared, ser_wire_and_number, FieldData, WireType,
};
use crate::internal::variant::Variant;
use crate::tags;
use crate::Message;
use crate::{ErrorKind, Result};
use ::std::io::{Result as IoResult, Write};

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
        Err(ErrorKind::InvalidWireType(WireType::Variant as u32))?
    }
    fn deser_from_bits32<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 4],
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::Bits32 as u32))?
    }
    fn deser_from_bits64<B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] bits: [u8; 8],
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::Bits64 as u32))?
    }
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        #[allow(unused)] iter: I,
    ) -> Result<()> {
        Err(ErrorKind::InvalidWireType(WireType::LengthDelimited as u32))?
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        #[allow(unused)] bitvec: &B,
        #[allow(unused)] number: i32,
        #[allow(unused)] out: &mut W,
    ) -> Result<()>;
}

impl<RustType, ProtoType> FieldType for SingularNumericalField<RustType, ProtoType>
where
    RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    fn deser_from_variant<B: BitSlice>(&mut self, _bitvec: &mut B, variant: Variant) -> Result<()> {
        let v = variant.get::<ProtoType>()?;
        if v != RustType::default() {
            self.0 = v;
        }
        Ok(())
    }
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        mut iter: I,
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
}

impl<RustType, ProtoType, const BITFIELD_INDEX: usize> FieldType
    for OptionalNumericalField<RustType, ProtoType, BITFIELD_INDEX>
where
    RustType: Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    fn deser_from_variant<B: BitSlice>(&mut self, bitvec: &mut B, variant: Variant) -> Result<()> {
        self.0 = variant.get::<ProtoType>()?;
        bitvec.set(BITFIELD_INDEX, true);
        Ok(())
    }
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        #[allow(unused)] bitvec: &mut B,
        mut iter: I,
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

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if !self.0.is_empty() {
            ser_bytes_shared(self.0.as_bytes(), number, out)?;
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
            ser_bytes_shared(self.0.as_bytes(), number, out)?;
        }
        Ok(())
    }
}

impl FieldType for RepeatedStringField {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let vec = iter.collect::<IoResult<Vec<u8>>>()?;
        self.0.push(String::from_utf8(vec)?);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        for s in &self.0 {
            ser_bytes_shared(s.as_bytes(), number, out)?;
        }
        Ok(())
    }
}

impl FieldType for SingularBytesField {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let vec = iter.collect::<IoResult<Vec<u8>>>()?;
        if !vec.is_empty() {
            self.0 = vec
        }
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if !self.0.is_empty() {
            ser_bytes_shared(self.0.as_slice(), number, out)?;
        }
        Ok(())
    }
}

impl<const BITFIELD_INDEX: usize> FieldType for OptionalBytesField<BITFIELD_INDEX> {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        self.0 = iter.collect::<IoResult<Vec<u8>>>()?;
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
            ser_bytes_shared(self.0.as_slice(), number, out)?;
        }
        Ok(())
    }
}

impl FieldType for RepeatedBytesField {
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let vec = iter.collect::<IoResult<Vec<u8>>>()?;
        self.0.push(vec);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        for v in &self.0 {
            ser_bytes_shared(v.as_slice(), number, out)?;
        }
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

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if let Some(ref message) = &self.0 {
            let mut vec = Vec::new();
            message.to_bytes(&mut vec)?;
            ser_bytes_shared(vec.as_slice(), number, out)?;
        }
        Ok(())
    }
}

impl<M> FieldType for RepeatedMessageField<M>
where
    M: Message + Default,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let msg = M::from_bytes_iter(Box::new(iter) as Box<dyn Iterator<Item = IoResult<u8>>>)?;
        self.0.push(msg);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        for message in &self.0 {
            let mut vec = Vec::new();
            message.to_bytes(&mut vec)?;
            ser_bytes_shared(vec.as_slice(), number, out)?;
        }
        Ok(())
    }
}
