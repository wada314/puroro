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

use crate::internal::message_internal::MessageInternal;
use crate::internal::ser::{
    ser_bytes_shared, ser_numerical_shared, FieldData, ScopedIter, WireType,
};
use crate::internal::tags;
use crate::{PuroroError, Result};
use ::std::io::{Result as IoResult, Write};
use ::std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct NumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);

#[derive(Default, Clone)]
pub struct UnsizedField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);

#[derive(Default, Clone)]
pub struct HeapMessageField<M>(Box<M>);

pub trait OneofFieldType: Default + Clone {
    /// A non-optional getter type, which is mainly used in the case enum's
    /// value type.
    /// int32 => i32
    /// String => &'a str
    /// Message => &'a Message
    type GetterType<'a>
    where
        Self: 'a;

    /// An optional getter type, which is mainly used in the getter methods
    /// of the field in the message struct.
    /// int32 => Option<i32>
    /// String => Option<&'a str>
    /// Message => Option<&'a Message>
    type GetterOptType<'a>
    where
        Self: 'a;

    /// A default field type which can be defined in proto2.
    /// int32 => i32
    /// String => &'a str
    /// Message => unreachable!()
    type DefaultValueType: Default;

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

    fn get_field(&self) -> Self::GetterType<'_>;
    fn get_field_opt(self_opt: Option<&Self>) -> Self::GetterOptType<'_>;
    fn get_field_or_else<'a, F: FnOnce() -> Self::DefaultValueType>(
        self_opt: Option<&'a Self>,
        f: F,
    ) -> Self::GetterOrElseType<'a>;
    fn get_field_mut(&mut self) -> Self::GetterMutType<'_>;

    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<ScopedIter<'a, I>>,
    ) -> Result<()>;
    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()>;
}

impl<RustType, ProtoType> OneofFieldType for NumericalField<RustType, ProtoType>
where
    RustType: Clone + Default,
    ProtoType: tags::NumericalType<RustType = RustType> + Default + Clone,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    type GetterOptType<'a> = Option<RustType>
    where
        Self: 'a;
    type DefaultValueType = RustType;
    type GetterOrElseType<'a> = RustType
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut RustType
    where
        Self: 'a;

    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.clone()
    }
    fn get_field_opt(self_opt: Option<&Self>) -> Self::GetterOptType<'_> {
        self_opt.map(|f| f.get_field())
    }
    fn get_field_or_else<'a, F: FnOnce() -> Self::DefaultValueType>(
        self_opt: Option<&'a Self>,
        f: F,
    ) -> Self::GetterOrElseType<'a> {
        Self::get_field_opt(self_opt).unwrap_or_else(f)
    }
    fn get_field_mut(&mut self) -> Self::GetterMutType<'_> {
        &mut self.0
    }
    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<ScopedIter<'a, I>>,
    ) -> Result<()> {
        match field_data {
            FieldData::Variant(variant) => {
                self.0 = variant.get::<ProtoType>()?;
            }
            FieldData::LengthDelimited(_) => Err(PuroroError::InvalidWireType(
                WireType::LengthDelimited as u32,
            ))?,
            FieldData::Bits32(bits) => {
                self.0 = <ProtoType as tags::NumericalType>::from_bits32(bits.clone())?;
            }
            FieldData::Bits64(bits) => {
                self.0 = <ProtoType as tags::NumericalType>::from_bits64(bits.clone())?;
            }
        }
        Ok(())
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        ser_numerical_shared::<_, ProtoType, _>(self.0.clone(), number, out)?;
        Ok(())
    }
}

impl<RustType, ProtoType> OneofFieldType for UnsizedField<RustType, ProtoType>
where
    RustType: Clone + Default,
    ProtoType: 'static + tags::UnsizedType<RustType = RustType> + Default + Clone,
{
    type GetterType<'a> = ProtoType::RustRefType<'a>
    where
        Self: 'a;
    type GetterOptType<'a> = Option<ProtoType::RustRefType<'a>>
    where
        Self: 'a;
    type DefaultValueType = ProtoType::DefaultValueType;
    type GetterOrElseType<'a> = ProtoType::RustRefType<'a>
    where
        Self: 'a;
    type GetterMutType<'a> = ProtoType::RustMutType<'a>
    where
        Self: 'a;

    fn get_field(&self) -> Self::GetterType<'_> {
        ProtoType::as_ref(&self.0)
    }
    fn get_field_opt(self_opt: Option<&Self>) -> Self::GetterOptType<'_> {
        self_opt.map(|f| f.get_field())
    }
    fn get_field_or_else<'a, F: FnOnce() -> Self::DefaultValueType>(
        self_opt: Option<&'a Self>,
        f: F,
    ) -> Self::GetterOrElseType<'a> {
        Self::get_field_opt(self_opt).unwrap_or_else(|| ProtoType::default_to_ref(f()))
    }
    fn get_field_mut(&mut self) -> Self::GetterMutType<'_> {
        ProtoType::as_mut(&mut self.0)
    }

    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<ScopedIter<'a, I>>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(iter) = field_data {
            self.0 = ProtoType::from_bytes_iter(iter)?;
            Ok(())
        } else {
            Err(PuroroError::InvalidWireType(field_data.wire_type() as u32))?
        }
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        ser_bytes_shared(ProtoType::to_bytes_slice(&self.0)?, number, out)
    }
}

impl<M: MessageInternal + Default> OneofFieldType for HeapMessageField<M>
where
    M: Default + Clone,
{
    type GetterType<'a> = &'a M
    where
        Self: 'a;
    type GetterOptType<'a> = Option<&'a M>
    where
        Self: 'a;
    type DefaultValueType = ();
    type GetterOrElseType<'a> = Option<&'a M>
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut M
    where
        Self: 'a;

    fn get_field(&self) -> Self::GetterType<'_> {
        &self.0
    }
    fn get_field_opt(self_opt: Option<&Self>) -> Self::GetterOptType<'_> {
        self_opt.map(|f| f.get_field())
    }
    fn get_field_or_else<'a, F: FnOnce() -> Self::DefaultValueType>(
        self_opt: Option<&'a Self>,
        _: F,
    ) -> Self::GetterOrElseType<'a> {
        Self::get_field_opt(self_opt)
    }
    fn get_field_mut(&mut self) -> Self::GetterMutType<'_> {
        &mut self.0
    }
    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        field_data: FieldData<ScopedIter<'a, I>>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(iter) = field_data {
            let msg = self.0.as_mut();
            msg.merge_from_scoped_bytes_iter(iter)?;
            Ok(())
        } else {
            Err(PuroroError::InvalidWireType(field_data.wire_type() as u32))?
        }
    }

    fn ser_to_write<W: Write>(&self, number: i32, out: &mut W) -> Result<()> {
        let mut vec = Vec::new();
        self.0.to_bytes(&mut vec)?;
        ser_bytes_shared(vec.as_slice(), number, out)?;
        Ok(())
    }
}
