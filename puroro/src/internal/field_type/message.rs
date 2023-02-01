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

use super::{FieldType, NonRepeatedFieldType, RepeatedFieldType};
use crate::internal::bitvec::BitSlice;
use crate::internal::ser::ser_bytes_shared;
use crate::Message;
use crate::Result;
use ::std::io::{Result as IoResult, Write};

#[derive(Default, Clone)]
pub struct SingularHeapMessageField<M>(Option<Box<M>>);
#[derive(Default, Clone)]
pub struct RepeatedMessageField<M>(Vec<M>);

impl<M> FieldType for SingularHeapMessageField<M>
where
    M: Message + Default,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let msg = self.0.get_or_insert_with(Default::default).as_mut();
        Ok(msg.merge_from_bytes_iter(Box::new(iter) as Box<dyn Iterator<Item = IoResult<u8>>>)?)
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        if let Some(ref message) = &self.0 {
            let mut vec = Vec::new();
            message.to_bytes(&mut vec)?;
            ser_bytes_shared(vec.as_slice(), number, out)?;
        }
        Ok(())
    }
}

impl<M> FieldType for RepeatedMessageField<M>
where
    M: Message + Default,
{
    fn deser_from_ld_iter<I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: I,
    ) -> Result<()> {
        let msg = M::from_bytes_iter(Box::new(iter) as Box<dyn Iterator<Item = IoResult<u8>>>)?;
        self.0.push(msg);
        Ok(())
    }

    fn ser_to_write<W: Write, B: BitSlice>(
        &self,
        _bitvec: &B,
        number: i32,
        out: &mut W,
    ) -> Result<()> {
        for message in &self.0 {
            let mut vec = Vec::new();
            message.to_bytes(&mut vec)?;
            ser_bytes_shared(vec.as_slice(), number, out)?;
        }
        Ok(())
    }
}

impl<M> NonRepeatedFieldType for SingularHeapMessageField<M>
where
    M: Message + Default,
{
    type GetterOptType<'a> = Option<&'a M>
    where
        Self: 'a;
    type DefaultValueType = ();
    type GetterOrElseType<'a> = Option<&'a M>
    where
        Self: 'a;
    type GetterMutType<'a> = &'a mut M where Self: 'a;

    fn get_field_or_else<'a, B: BitSlice, D: FnOnce() -> Self::DefaultValueType>(
        &'a self,
        bitvec: &B,
        _default: D,
    ) -> Self::GetterOrElseType<'a> {
        self.get_field_opt(bitvec)
    }
    fn get_field_opt<B: BitSlice>(&self, _bitvec: &B) -> Self::GetterOptType<'_> {
        self.0.as_deref()
    }
    fn get_field_mut<'a, B: BitSlice, D: FnOnce() -> Self::DefaultValueType>(
        &'a mut self,
        _bitvec: &mut B,
        _default: D,
    ) -> Self::GetterMutType<'a> {
        self.0.get_or_insert_with(Default::default)
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0 = None;
    }
}

impl<M: Message + Default + Clone> RepeatedFieldType for RepeatedMessageField<M> {
    type ScalarType = M;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }

    type ContainerType = Vec<Self::ScalarType>;

    fn get_field_mut<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}
