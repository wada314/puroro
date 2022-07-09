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
use crate::reflection::r#static::desc::UsizeValue;
use crate::{ErrorKind, Result};
use ::std::ops::Index;
use std::marker::PhantomData;

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
impl TryIntoOwnedFieldGetter for String {
    fn try_into_string(&self) -> Result<&str> {
        Ok(&self)
    }
}

pub struct OptionalOwnedField<T, BitfieldIndex>(T, PhantomData<BitfieldIndex>);

impl<T: TryIntoOwnedFieldGetter, BitfieldIndex: UsizeValue> OwnedField
    for OptionalOwnedField<T, BitfieldIndex>
{
    fn has_field<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<bool> {
        Ok(bitfield[BitfieldIndex::VALUE])
    }
    fn get_uint32<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<u32> {
        Ok(if bitfield[BitfieldIndex::VALUE] {
            <T as TryIntoOwnedFieldGetter>::try_into_uint32(&self.0)?
        } else {
            Default::default()
        })
    }
    fn get_string<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<&str> {
        Ok(if bitfield[BitfieldIndex::VALUE] {
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
}
impl<T: Default, BitfieldIndex> Default for OptionalOwnedField<T, BitfieldIndex> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

pub struct ScalarMessageOwnedField<T, BitfieldIndex>(Option<Box<T>>, PhantomData<BitfieldIndex>);
impl<T, BitfieldIndex> OwnedField for ScalarMessageOwnedField<T, BitfieldIndex> {
    fn has_field<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<bool> {
        Ok(self.0.is_some())
    }
    fn get_message<B: Index<usize, Output = bool>>(
        &self,
        _bitfield: &B,
    ) -> Result<Self::MessageType<'_>> {
        Ok(self.0.as_deref())
    }

    type StringType<'a> = &'a str
    where
        Self: 'a;
    type MessageType<'a> = Option<&'a T>
    where
        Self: 'a;
}
impl<T, BitfieldIndex> Default for ScalarMessageOwnedField<T, BitfieldIndex> {
    fn default() -> Self {
        Self(None, PhantomData)
    }
}
