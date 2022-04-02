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

use crate::desc::StaticFieldDescriptor;
use crate::internal::bool::{False, True};
use crate::tags;
use crate::{ErrorKind, Result};

pub trait TryOptFromRawField<'f, MD, FD, F>: Sized {
    fn try_opt_from_raw_field(_field: &'f F) -> Result<Option<Self>> {
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

impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, u32, False, False> for u32 {
    fn try_opt_from_raw_field_impl(field: &u32) -> Result<Option<Self>> {
        Ok(Some(*field))
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, String, False, False> for u32 {}

impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, String, False, False> for &'f str {
    fn try_opt_from_raw_field_impl(field: &'f String) -> Result<Option<Self>> {
        Ok(Some(field))
    }
}
impl<'f, MD, FD> TryOptFromRawFieldImpl<'f, MD, FD, u32, False, False> for &'f str {}

impl<'f, MD, FD, M> TryOptFromRawFieldImpl<'f, MD, FD, Option<Box<M>>, False, True> for &'f M {
    fn try_opt_from_raw_field_impl(field: &'f Option<Box<M>>) -> Result<Option<Self>> {
        Ok(field.as_deref())
    }
}
