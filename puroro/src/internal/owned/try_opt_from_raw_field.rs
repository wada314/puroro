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

pub trait TryOptFromRawField<'f, MD, FD, F, B>: Sized {
    fn try_opt_from_raw_field(_field: &'f F, _bitfield: &'f B) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait TryOptFromRawFieldImpl<'f, MD, FD, F, B, IsRepeated, IsMessage>: Sized {
    fn try_opt_from_raw_field_impl(_field: &'f F, _bitfield: &'f B) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<'f, T, MD, FD, F, B, LabelTag, TypeTag> TryOptFromRawField<'f, MD, FD, F, B> for T
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    T: TryOptFromRawFieldImpl<'f, MD, FD, F, B, LabelTag::IsRepeated, TypeTag::IsMessage>,
{
    fn try_opt_from_raw_field(field: &'f F, bitfield: &'f B) -> Result<Option<Self>> {
        Self::try_opt_from_raw_field_impl(field, bitfield)
    }
}

macro_rules! impl_trait {
    ($into:ty, $from:ty, $is_repeated:ty, $is_message:ty
        $(, |$field_name:ident, $bitfield_name:ident| $expr:expr)?
    ) => {
        impl<'f, MD, FD, B> TryOptFromRawFieldImpl<'f, MD, FD, $from, B, $is_repeated, $is_message> for $into {
            $(fn try_opt_from_raw_field_impl(
                $field_name: &'f $from,
                $bitfield_name: &'f B
            ) -> Result<Option<Self>> {
                $expr
            })?
        }
    };
}
impl_trait!(u32, u32, False, False, |f, _b| Ok(Some(*f)));
impl_trait!(u32, &'f str, False, False);
impl_trait!(&'f str, String, False, False, |f, _b| Ok(Some(f)));
impl_trait!(&'f str, u32, False, False);

impl<'f, MD, FD, M, B> TryOptFromRawFieldImpl<'f, MD, FD, Option<Box<M>>, B, False, True> for &'f M {
    fn try_opt_from_raw_field_impl(field: &'f Option<Box<M>>, _: &B) -> Result<Option<Self>> {
        Ok(field.as_deref())
    }
}
