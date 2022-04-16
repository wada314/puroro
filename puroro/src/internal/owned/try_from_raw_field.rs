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
use crate::internal::Bitfield;
use crate::message::AsMessageImplRef;
use crate::tags;
use crate::{ErrorKind, Result};

pub trait TryFromRawField<MD, FD, F, B>: Sized {
    fn try_from_raw_field<'f>(_field: &'f F, _: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        Err(ErrorKind::ReflectionError)?
    }
}
pub trait TryFromRawFieldImpl<MD, FD, F, B, IsRepeated, IsMessage, IsLd>: Sized {
    fn try_from_raw_field_impl<'f>(_field: &'f F, _: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<T, MD, FD, F, B, LabelTag, TypeTag> TryFromRawField<MD, FD, F, B> for T
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    T: TryFromRawFieldImpl<MD, FD, F, B, LabelTag::IsRepeated, TypeTag::IsMessage, TypeTag::IsLd>,
{
    fn try_from_raw_field<'f>(field: &'f F, bitfield: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        Self::try_from_raw_field_impl(field, bitfield)
    }
}

impl<MD, FD, F, B, T> TryFromRawFieldImpl<MD, FD, F, B, False, False, False> for Option<T>
where
    FD: StaticFieldDescriptor,
    F: Clone + TryInto<T>,
    T: Default + PartialEq,
    ErrorKind: From<<F as TryInto<T>>::Error>,
    B: Bitfield,
{
    fn try_from_raw_field_impl<'f>(field: &'f F, bitfield: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        let val: T = field
            .clone()
            .try_into()
            .map_err(|e| Into::<ErrorKind>::into(e))?;
        Ok(
            if let Some(has_field_bit_index) = FD::OWNED_HASFIELD_BITFIELD_INDEX {
                bitfield.get(has_field_bit_index).then(|| val)
            } else {
                (val != T::default()).then(|| val)
            },
        )
    }
}

impl<'a, MD, FD, F, B, T> TryFromRawFieldImpl<MD, FD, F, B, False, False, True> for Option<&'a T>
where
    FD: StaticFieldDescriptor,
    F: AsRef<T>,
    T: ?Sized,
    &'a T: Default + PartialEq,
    B: Bitfield,
{
    fn try_from_raw_field_impl<'f>(field: &'f F, bitfield: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        let s: &T = field.as_ref();
        Ok(
            if let Some(has_field_bit_index) = FD::OWNED_HASFIELD_BITFIELD_INDEX {
                bitfield.get(has_field_bit_index).then(|| s)
            } else {
                (s != <&T>::default()).then(|| s)
            },
        )
    }
}

impl<'a, MD, FD, B, M, MI> TryFromRawFieldImpl<MD, FD, Option<M>, B, False, True, True>
    for Option<&'a MI>
where
    M: AsMessageImplRef<MessageImplType = MI>,
{
    fn try_from_raw_field_impl<'f>(field: &'f Option<M>, _bitfield: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        Ok(field.as_ref().map(|m| m.as_message_impl_ref()))
    }
}

impl<'a, MD, FD, B, M, MI> TryFromRawFieldImpl<MD, FD, M, B, False, True, True> for Option<&'a MI>
where
    M: AsMessageImplRef<MessageImplType = MI>,
{
    fn try_from_raw_field_impl<'f>(field: &'f M, _bitfield: &'f B) -> Result<Self>
    where
        Self: 'f,
    {
        Ok(Some(field.as_message_impl_ref()))
    }
}
