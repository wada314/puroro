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

pub trait TryFromRawField<'f, MD, FD, F, B>: Sized {
    fn try_from_raw_field(_field: &'f F, _: &'f B) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
pub trait TryFromRawFieldImpl<'f, MD, FD, F, B, IsRepeated, IsMessage>: Sized {
    fn try_from_raw_field_impl(_field: &'f F, _: &'f B) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<'f, T, MD, FD, F, B, LabelTag, TypeTag> TryFromRawField<'f, MD, FD, F, B> for T
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    T: TryFromRawFieldImpl<'f, MD, FD, F, B, LabelTag::IsRepeated, TypeTag::IsMessage>,
{
    fn try_from_raw_field(field: &'f F, bitfield: &'f B) -> Result<Self> {
        Self::try_from_raw_field_impl(field, bitfield)
    }
}

macro_rules! impl_trait_using_opt {
    ($into:ty, $is_message:ty, $default:expr) => {
        impl<'f, MD, FD, F, B> TryFromRawFieldImpl<'f, MD, FD, F, B, False, $is_message> for $into
        where
            FD: StaticFieldDescriptor,
            $into: TryOptFromRawField<'f, MD, FD, F, B>,
        {
            fn try_from_raw_field_impl(field: &'f F, bitfield: &'f B) -> Result<Self> {
                match Self::try_opt_from_raw_field(field, bitfield)? {
                    Some(val) => Ok(val),
                    None => $default,
                }
            }
        }
    };
}
impl_trait_using_opt!(
    u32,
    False,
    match FD::DEFAULT_VALUE {
        FieldDefaultValue::U32(val) => Ok(val),
        _ => Err(ErrorKind::ReflectionError)?,
    }
);
impl_trait_using_opt!(
    &'f str,
    False,
    match FD::DEFAULT_VALUE {
        FieldDefaultValue::String(val) => Ok(val),
        _ => Err(ErrorKind::ReflectionError)?,
    }
);

impl<'f, MD, FD, F, B, M> TryFromRawFieldImpl<'f, MD, FD, F, B, False, True> for &'f M
where
    FD: StaticFieldDescriptor,
    &'f M: TryOptFromRawField<'f, MD, FD, F, B>,
{
    fn try_from_raw_field_impl(field: &'f F, bitfield: &'f B) -> Result<Self> {
        match Self::try_opt_from_raw_field(field, bitfield)? {
            Some(val) => Ok(val),
            None => todo!(),
        }
    }
}
