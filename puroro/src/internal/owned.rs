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

use crate::desc::{FieldDefaultValue, StaticFieldDescriptor, StaticMessageDescriptor};
use crate::internal::bool::{False, True};
use crate::message::{MessageFieldGetter, MessageImpl};
use crate::tags::{self};
use crate::{ErrorKind, Result};
use ::std::marker::PhantomData;

#[derive(Default)]
pub struct OwnedMessageImpl<MD, FS, const BITFIELD_U32_LEN: usize> {
    bitvec: ::bitvec::array::BitArray<::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
    fields: FS,
    _phantom: PhantomData<MD>,
}
impl<MD, FS, const BITFIELD_U32_LEN: usize> OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN> {
    pub fn try_get_field_as<'msg, FD, R>(&'msg self) -> Result<R>
    where
        FD: StaticFieldDescriptor,
        FS: OwnedRawFieldGetter<FD>,
        <FS as OwnedRawFieldGetter<FD>>::Type: 'msg,
        R: TryFromRawField<'msg, MD, FD, <FS as OwnedRawFieldGetter<FD>>::Type>,
    {
        let raw_field_ref = <FS as OwnedRawFieldGetter<FD>>::get(&self.fields);
        R::try_from_raw_field(raw_field_ref)
    }
}
impl<'msg, MD, FD, FS, const BITFIELD_U32_LEN: usize> MessageFieldGetter<'msg, FD>
    for OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN>
where
    MD: StaticMessageDescriptor,
    FD: StaticFieldDescriptor,
    FS: OwnedRawFieldGetter<FD>,
    <FS as OwnedRawFieldGetter<FD>>::Type: 'msg,
    u32: TryFromRawField<'msg, MD, FD, <FS as OwnedRawFieldGetter<FD>>::Type>,
    &'msg str: TryFromRawField<'msg, MD, FD, <FS as OwnedRawFieldGetter<FD>>::Type>,
{
    fn try_get_u32(&'msg self) -> Result<u32> {
        self.try_get_field_as::<FD, u32>()
    }
    fn try_get_str(&'msg self) -> Result<&'msg str> {
        self.try_get_field_as::<FD, &str>()
    }
}
impl<'msg, MD, FS, const BITFIELD_U32_LEN: usize> MessageImpl<'msg, MD>
    for OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN>
{
}

pub trait OwnedRawFieldGetter<FD> {
    type Type;
    fn get(&self) -> &Self::Type;
}

pub trait TryFromRawField<'f, MD, FD, F>: Sized {
    fn try_from_raw_field(_field: &'f F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
pub trait TryOptFromRawField<'f, MD, FD, F>: Sized {
    fn try_opt_from_raw_field(_field: &'f F) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait TryFromRawFieldImpl<'f, MD, FD, F, IsRepeated, IsMessage>: Sized {
    fn try_from_raw_field_impl(_field: &'f F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
pub trait TryOptFromRawFieldImpl<'f, MD, FD, F, IsRepeated, IsMessage>: Sized {
    fn try_opt_from_raw_field_impl(_field: &'f F) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}
impl<'f, T, MD, FD, F, LabelTag, TypeTag> TryOptFromRawField<'f, MD, FD, F> for T
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    T: TryOptFromRawFieldImpl<'f, MD, FD, F, LabelTag::IsRepeated, TypeTag::IsMessage>,
{
    fn try_opt_from_raw_field(field: &'f F) -> Result<Option<Self>> {
        Self::try_opt_from_raw_field_impl(field)
    }
}

impl<'f, MD, FD, F> TryFromRawField<'f, MD, FD, F> for u32
where
    FD: StaticFieldDescriptor,
    u32: TryOptFromRawField<'f, MD, FD, F>,
{
    fn try_from_raw_field(field: &'f F) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::U32(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, u32, False, False> for u32 {
    fn try_opt_from_raw_field_impl(field: &u32) -> Result<Option<Self>> {
        Ok(Some(*field))
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, String, False, False> for u32 {}

impl<'f, MD, FD, F> TryFromRawField<'f, MD, FD, F> for &'f str
where
    FD: StaticFieldDescriptor,
    &'f str: TryOptFromRawField<'f, MD, FD, F>,
{
    fn try_from_raw_field(field: &'f F) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::String(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, String, False, False> for &'f str {
    fn try_opt_from_raw_field_impl(field: &'f String) -> Result<Option<Self>> {
        Ok(Some(field))
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, u32, False, False> for &'f str {}

impl<'f, MD, FD, M> TryFromRawField<'f, MD, FD, Option<Box<M>>> for &'f M
where
    FD: StaticFieldDescriptor,
    &'f M: TryOptFromRawField<'f, MD, FD, Option<Box<M>>>,
{
    fn try_from_raw_field(field: &'f Option<Box<M>>) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => todo!(),
        }
    }
}
impl<'f, MD, FD, M> TryOptFromRawFieldImpl<'f, MD, FD, Option<Box<M>>, False, True> for &'f M {
    fn try_opt_from_raw_field_impl(field: &'f Option<Box<M>>) -> Result<Option<Self>> {
        Ok(field.as_deref())
    }
}
