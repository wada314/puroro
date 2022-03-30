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

use crate::desc::FieldDefaultValue;
use crate::desc::StaticFieldDescriptor;
use crate::{ErrorKind, Result};
use ::std::marker::PhantomData;

struct OwnedMessageImpl<MD, FS, const BITFIELD_U32_LEN: usize> {
    bitvec: ::bitvec::array::BitArray<::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
    fields: FS,
    _phantom: PhantomData<MD>,
}
impl<MD, FS, const BITFIELD_U32_LEN: usize> OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN> {
    pub fn try_get_field_as<'a, FD, R, const NUMBER: i32>(&'a self) -> Result<R>
    where
        FD: StaticFieldDescriptor,
        FS: OwnedRawFields + OwnedRawFieldGetter<{ NUMBER }>,
        R: TryFromRawField<'a, MD, FD, <FS as OwnedRawFieldGetter<{ NUMBER }>>::Type>,
    {
        let raw_field_ref = <FS as OwnedRawFieldGetter<NUMBER>>::get(&self.fields);
        R::try_from_raw_field(raw_field_ref)
    }
}

pub trait OwnedRawFields {
    fn get<const NUMBER: i32>(&self) -> &<Self as OwnedRawFieldGetter<NUMBER>>::Type
    where
        Self: OwnedRawFieldGetter<NUMBER>,
    {
        <Self as OwnedRawFieldGetter<NUMBER>>::get(&self)
    }
}
pub trait OwnedRawFieldGetter<const NUMBER: i32> {
    type Type;
    fn get(&self) -> &Self::Type;
}

trait TryFromRawField<'f, MD, FD, F>: Sized {
    fn try_from_raw_field(_field: &'f F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
trait TryFromRawFieldOpt<'f, MD, FD, F>: Sized {
    fn try_from_raw_field_opt(_field: &'f F) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}

impl<'f, MD, FD, F> TryFromRawField<'f, MD, FD, F> for u32
where
    FD: StaticFieldDescriptor,
    u32: TryFromRawFieldOpt<'f, MD, FD, F>,
{
    fn try_from_raw_field(field: &'f F) -> Result<Self> {
        match Self::try_from_raw_field_opt(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::U32(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}
impl<'f, MD, FD> TryFromRawFieldOpt<'f, MD, FD, u32> for u32 {
    fn try_from_raw_field_opt(field: &u32) -> Result<Option<Self>> {
        Ok(Some(*field))
    }
}

impl<'f, MD, FD, F> TryFromRawField<'f, MD, FD, F> for &'f str
where
    FD: StaticFieldDescriptor,
    &'f str: TryFromRawFieldOpt<'f, MD, FD, F>,
{
    fn try_from_raw_field(field: &'f F) -> Result<Self> {
        match Self::try_from_raw_field_opt(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::String(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}
impl<'f, MD, FD> TryFromRawFieldOpt<'f, MD, FD, String> for &'f str {
    fn try_from_raw_field_opt(field: &'f String) -> Result<Option<Self>> {
        Ok(Some(field))
    }
}

impl<'f, MD, FD, M> TryFromRawField<'f, MD, FD, Option<Box<M>>> for &'f M
where
    FD: StaticFieldDescriptor,
    &'f M: TryFromRawFieldOpt<'f, MD, FD, Option<Box<M>>>,
{
    fn try_from_raw_field(field: &'f Option<Box<M>>) -> Result<Self> {
        match Self::try_from_raw_field_opt(field)? {
            Some(val) => Ok(val),
            None => todo!(),
        }
    }
}
impl<'f, MD, FD, M> TryFromRawFieldOpt<'f, MD, FD, Option<Box<M>>> for &'f M {
    fn try_from_raw_field_opt(field: &'f Option<Box<M>>) -> Result<Option<Self>> {
        Ok(field.as_deref())
    }
}
