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

use super::{OwnedField, Reflection};
use crate::{ErrorKind, Result};
use ::std::ops::Index;

pub trait TryIntoOwnedFieldGetter {
    fn try_into_uint32(&self) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_into_string(&self) -> Result<&str> {
        Err(ErrorKind::ReflectionError)?
    }
}
impl TryIntoOwnedFieldGetter for u32 {
    fn try_into_uint32(&self) -> Result<u32> {
        Ok(*self)
    }
}

pub struct U32ScalarOwnedField<const BITFIELD_START_INDEX: usize>(u32);

impl<const BITFIELD_START_INDEX: usize> OwnedField for U32ScalarOwnedField<BITFIELD_START_INDEX> {
    fn has_field<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<bool> {
        Ok(self.0 != Default::default())
    }
    fn get_uint32<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<u32> {
        Ok(self.0)
    }

    type StringType<'a> = &'a str
    where
        Self: 'a;
    type MessageType<'a> = ()
    where
        Self: 'a;

    const BITFIELD_START_INDEX: usize = BITFIELD_START_INDEX;
    const BITFIELD_NEXT_INDEX: usize = BITFIELD_START_INDEX + 0;
}

pub struct OptionalOwnedField<T, const BITFIELD_START_INDEX: usize>(T);

impl<T: TryIntoOwnedFieldGetter, const BITFIELD_START_INDEX: usize> OwnedField
    for OptionalOwnedField<T, BITFIELD_START_INDEX>
{
    fn has_field<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<bool> {
        Ok(bitfield[Self::BITFIELD_START_INDEX])
    }
    fn get_uint32<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<u32> {
        Ok(if bitfield[Self::BITFIELD_START_INDEX] {
            <T as TryIntoOwnedFieldGetter>::try_into_uint32(&self.0)?
        } else {
            Default::default()
        })
    }
    fn get_string<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<&str> {
        Ok(if bitfield[Self::BITFIELD_START_INDEX] {
            <T as TryIntoOwnedFieldGetter>::try_into_string(&self.0)?
        } else {
            Default::default()
        })
    }

    type StringType<'a> = &'a str
    where
        Self: 'a;
    type MessageType<'a> = ()
    where
        Self: 'a;

    const BITFIELD_START_INDEX: usize = BITFIELD_START_INDEX;
    const BITFIELD_NEXT_INDEX: usize = BITFIELD_START_INDEX + 1;
}

pub struct MessageScalarOwnedField<T, const BITFIELD_START_INDEX: usize>(Option<Box<T>>);
impl<T: Reflection, const BITFIELD_START_INDEX: usize> OwnedField
    for MessageScalarOwnedField<T, BITFIELD_START_INDEX>
{
    fn has_field<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<bool> {
        Ok(self.0.is_some())
    }
    fn get_message<B: Index<usize, Output = bool>>(
        &self,
        _bitfield: &B,
    ) -> Result<Self::MessageType<'_>> {
        todo!()
    }

    type StringType<'a> = &'a str
    where
        Self: 'a;

    type MessageType<'a> = &'a T
    where
        Self: 'a;

    const BITFIELD_START_INDEX: usize = BITFIELD_START_INDEX;
    const BITFIELD_NEXT_INDEX: usize = BITFIELD_START_INDEX + 0;
}
