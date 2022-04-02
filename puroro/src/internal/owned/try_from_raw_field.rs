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

use super::TryOptFromRawField;
use crate::desc::{FieldDefaultValue, StaticFieldDescriptor};
use crate::internal::bool::{False, True};
use crate::tags;
use crate::{ErrorKind, Result};

pub trait TryFromRawField<'f, MD, FD, F>: Sized {
    fn try_from_raw_field(_field: &'f F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
pub trait TryFromRawFieldImpl<'f, MD, FD, F, IsRepeated, IsMessage>: Sized {
    fn try_from_raw_field_impl(_field: &'f F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<'f, T, MD, FD, F, LabelTag, TypeTag> TryFromRawField<'f, MD, FD, F> for T
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    T: TryFromRawFieldImpl<'f, MD, FD, F, LabelTag::IsRepeated, TypeTag::IsMessage>,
{
    fn try_from_raw_field(field: &'f F) -> Result<Self> {
        Self::try_from_raw_field_impl(field)
    }
}

impl<'f, MD, FD, F> TryFromRawFieldImpl<'f, MD, FD, F, False, False> for u32
where
    FD: StaticFieldDescriptor,
    u32: TryOptFromRawField<'f, MD, FD, F>,
{
    fn try_from_raw_field_impl(field: &'f F) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::U32(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}

impl<'f, MD, FD, F> TryFromRawFieldImpl<'f, MD, FD, F, False, False> for &'f str
where
    FD: StaticFieldDescriptor,
    &'f str: TryOptFromRawField<'f, MD, FD, F>,
{
    fn try_from_raw_field_impl(field: &'f F) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::String(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}

impl<'f, MD, FD, M> TryFromRawFieldImpl<'f, MD, FD, Option<Box<M>>, False, True> for &'f M
where
    FD: StaticFieldDescriptor,
    &'f M: TryOptFromRawField<'f, MD, FD, Option<Box<M>>>,
{
    fn try_from_raw_field_impl(field: &'f Option<Box<M>>) -> Result<Self> {
        match Self::try_opt_from_raw_field(field)? {
            Some(val) => Ok(val),
            None => todo!(),
        }
    }
}
