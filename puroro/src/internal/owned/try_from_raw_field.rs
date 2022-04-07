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
use crate::desc::StaticFieldDescriptor;
use crate::internal::bool::{False, True};
use crate::internal::Bitfield;
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

impl<'f, MD, FD, F, B> TryFromRawFieldImpl<'f, MD, FD, F, B, False, False> for Option<u32>
where
    FD: StaticFieldDescriptor,
    F: Clone + Default + PartialEq + TryInto<u32>,
    ErrorKind: From<<F as TryInto<u32>>::Error>,
    B: Bitfield,
{
    fn try_from_raw_field_impl(field: &'f F, bitfield: &'f B) -> Result<Self> {
        let val: u32 = field
            .clone()
            .try_into()
            .map_err(|e| Into::<ErrorKind>::into(e))?;
        Ok(
            if let Some(has_field_bit_index) = FD::OWNED_HASFIELD_BITFIELD_INDEX {
                bitfield.get(has_field_bit_index).then(|| val)
            } else {
                (val != u32::default()).then(|| val)
            },
        )
    }
}

impl<'f, MD, FD, F, B> TryFromRawFieldImpl<'f, MD, FD, F, B, False, False> for Option<&'f str>
where
    FD: StaticFieldDescriptor,
    F: AsRef<str>,
    B: Bitfield,
{
    fn try_from_raw_field_impl(field: &'f F, bitfield: &'f B) -> Result<Self> {
        let s: &str = field.as_ref();
        Ok(
            if let Some(has_field_bit_index) = FD::OWNED_HASFIELD_BITFIELD_INDEX {
                bitfield.get(has_field_bit_index).then(|| s)
            } else {
                (!s.is_empty()).then(|| s)
            },
        )
    }
}

impl<'f, MD, FD, B, M> TryFromRawFieldImpl<'f, MD, FD, Option<Box<M>>, B, False, True>
    for Option<&'f M>
{
    fn try_from_raw_field_impl(field: &'f Option<Box<M>>, _bitfield: &'f B) -> Result<Self> {
        Ok(field.as_deref())
    }
}

impl<'f, MD, FD, B, M> TryFromRawFieldImpl<'f, MD, FD, Option<M>, B, False, True>
    for Option<&'f M>
{
    fn try_from_raw_field_impl(field: &'f Option<M>, _bitfield: &'f B) -> Result<Self> {
        Ok(field.as_ref())
    }
}

impl<'f, MD, FD, B, M> TryFromRawFieldImpl<'f, MD, FD, M, B, False, True> for Option<&'f M> {
    fn try_from_raw_field_impl(field: &'f M, _bitfield: &'f B) -> Result<Self> {
        Ok(Some(field))
    }
}
