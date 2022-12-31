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

use crate::internal::ser::{ser_bytes_shared, ser_numerical_shared, FieldData, WireType};
use crate::{tags, Message};
use crate::{ErrorKind, Result};
use ::std::io::{Result as IoResult, Write};
use ::std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct NumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);

#[derive(Default, Clone)]
pub struct BytesField(Vec<u8>);

#[derive(Default, Clone)]
pub struct StringField(String);

#[derive(Default, Clone)]
pub struct HeapMessageField<M>(Box<M>);

pub trait OneofFieldType {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_>;
    type MutGetterType<'a>
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_>;

    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<I>,
    ) -> Result<()>;
    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()>;
}

pub trait OneofFieldTypeOpt<'a> {
    type OptGetterType;
    type GetterType;
    type DefaultValueType;
    fn get_field_opt(self) -> Self::OptGetterType;
    fn get_field_or_else<D: FnOnce() -> Self::DefaultValueType>(
        self,
        default: D,
    ) -> Self::GetterType;
}

impl<RustType, ProtoType> OneofFieldType for NumericalField<RustType, ProtoType>
where
    RustType: Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.clone()
    }
    type MutGetterType<'a> = &'a mut RustType
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }

    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<I>,
    ) -> Result<()> {
        match field_data {
            FieldData::Variant(variant) => {
                self.0 = variant.get::<ProtoType>()?;
            }
            FieldData::LengthDelimited(_) => {
                Err(ErrorKind::InvalidWireType(WireType::LengthDelimited as u32))?
            }
            FieldData::Bits32(bits) => {
                self.0 = <ProtoType as tags::NumericalType>::from_bits32(bits)?;
            }
            FieldData::Bits64(bits) => {
                self.0 = <ProtoType as tags::NumericalType>::from_bits64(bits)?;
            }
        }
        Ok(())
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        ser_numerical_shared::<_, ProtoType, _>(self.0.clone(), number, out)?;
        Ok(())
    }
}

impl OneofFieldType for BytesField {
    type GetterType<'a> = &'a [u8]
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.as_ref()
    }
    type MutGetterType<'a> = &'a mut Vec<u8>
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }

    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<I>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(iter) = field_data {
            self.0 = iter.collect::<IoResult<Vec<u8>>>()?;
            Ok(())
        } else {
            Err(ErrorKind::InvalidWireType(field_data.wire_type() as u32))?
        }
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        ser_bytes_shared(self.0.as_slice(), number, out)
    }
}

impl OneofFieldType for StringField {
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.as_str()
    }
    type MutGetterType<'a> = &'a mut String
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }

    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<I>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(iter) = field_data {
            self.0 = String::from_utf8(iter.collect::<IoResult<Vec<u8>>>()?)?;
            Ok(())
        } else {
            Err(ErrorKind::InvalidWireType(field_data.wire_type() as u32))?
        }
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        ser_bytes_shared(self.0.as_bytes(), number, out)
    }
}

impl<M: Message + Default> OneofFieldType for HeapMessageField<M> {
    type GetterType<'a> = &'a M
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        &self.0
    }
    type MutGetterType<'a> = &'a mut M
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }

    fn deser_from_iter<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<I>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(iter) = field_data {
            let msg = self.0.as_mut();
            msg.merge_from_bytes_iter(Box::new(iter) as Box<dyn Iterator<Item = IoResult<u8>>>)?;
            Ok(())
        } else {
            Err(ErrorKind::InvalidWireType(field_data.wire_type() as u32))?
        }
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        let mut vec = Vec::new();
        self.0.to_bytes(&mut vec)?;
        ser_bytes_shared(vec.as_slice(), number, out)?;
        Ok(())
    }
}

pub trait OneofFieldTypeOptForNonMessageType {
    type GetterType;
    fn get_field_opt(self) -> Option<Self::GetterType>;
}

impl<'a, T: OneofFieldTypeOptForNonMessageType> OneofFieldTypeOpt<'a> for T {
    type OptGetterType = Option<<T as OneofFieldTypeOptForNonMessageType>::GetterType>;
    type GetterType = <T as OneofFieldTypeOptForNonMessageType>::GetterType;
    type DefaultValueType = <T as OneofFieldTypeOptForNonMessageType>::GetterType;
    fn get_field_opt(self) -> Self::OptGetterType {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self)
    }
    fn get_field_or_else<D: FnOnce() -> Self::DefaultValueType>(
        self,
        default: D,
    ) -> Self::GetterType {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<'a, RustType, ProtoType> OneofFieldTypeOptForNonMessageType
    for Option<&'a NumericalField<RustType, ProtoType>>
where
    RustType: Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    type GetterType = RustType;
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a> OneofFieldTypeOptForNonMessageType for Option<&'a BytesField> {
    type GetterType = &'a [u8];
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a> OneofFieldTypeOptForNonMessageType for Option<&'a StringField> {
    type GetterType = &'a str;
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a, M: Message + Default> OneofFieldTypeOpt<'a> for Option<&'a HeapMessageField<M>> {
    type OptGetterType = Option<&'a M>;
    type GetterType = Option<&'a M>;
    type DefaultValueType = ();
    fn get_field_opt(self) -> Self::OptGetterType {
        self.map(|f| f.get_field())
    }
    fn get_field_or_else<D: FnOnce() -> Self::DefaultValueType>(
        self,
        _default: D,
    ) -> Self::GetterType {
        self.get_field_opt()
    }
}
