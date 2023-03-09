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

use super::r#unsized::RepeatedFieldViewImpl;
use super::{FieldType, NonRepeatedFieldType, RepeatedFieldType};
use crate::internal::alloc::NoAllocBox;
use crate::internal::bitvec::BitSlice;
use crate::internal::detach_alloc::{AttachAlloc, DetachAlloc, RefMut};
use crate::internal::message_internal::MessageInternal;
use crate::internal::ser::{ser_bytes_shared, ScopedIter};
use crate::message::MessageView;
use crate::Result;
use ::std::io::{Result as IoResult, Write};

#[derive(Default)]
pub struct SingularMessageField<MV>(Option<NoAllocBox<MV>>);
#[derive(Default, Clone)]
pub struct RepeatedMessageField<MV>(Vec<MV>);

impl<MV, M> FieldType for SingularMessageField<MV>
where
    MV: MessageView + Default + ToOwned<Owned = M>,
{
    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        let no_alloc_box = self.0.get_or_insert_with(|| Box::<MV>::default().detach());
        let msg = 
        Ok(msg.merge_from_scoped_bytes_iter(iter)?)
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

impl<MV> FieldType for RepeatedMessageField<MV>
where
    MV: MessageView + Default,
{
    fn deser_from_ld_scoped_iter<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        _bitvec: &mut B,
        iter: &mut ScopedIter<'a, I>,
    ) -> Result<()> {
        let msg = <<MV as AttachAlloc>::Attached as MessageInternal>::from_scoped_bytes_iter(iter)?;
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

impl<MV> NonRepeatedFieldType for SingularMessageField<MV>
where
    MV: MessageView + Default + ToOwned,
{
    type GetterOptType<'a> = Option<&'a MV>
    where
        Self: 'a;
    type DefaultValueType = ();
    type GetterOrElseType<'a> = Option<&'a MV>
    where
        Self: 'a;
    type GetterMutType<'a> = RefMut<'a, <MV as ToOwned>::Owned> where Self: 'a;

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
        unsafe {
            self.0
                .get_or_insert_with(|| <Box<MV> as Default>::default().detach().0)
                .ref_mut(Default::default())
        }
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0 = None;
    }
}

impl<MV> RepeatedFieldType for RepeatedMessageField<MV>
where
    MV: MessageView + Default,
{
    type ContainerType = Vec<MV>;
    fn get_field_mut<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }

    type RepeatedFieldViewType<'a> = RepeatedFieldViewImpl<'a, MV>
    where
        Self: 'a;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> Self::RepeatedFieldViewType<'_> {
        RepeatedFieldViewImpl(self.0.as_slice())
    }
}

#[cfg(not(feature = "allocator_api"))]
impl<MV: Clone> Clone for SingularMessageField<MV> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
