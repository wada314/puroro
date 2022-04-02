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

use crate::desc::{StaticFieldDescriptor, StaticMessageDescriptor};
use crate::message::{MessageFieldGetter, MessageImpl};
use crate::Result;
use ::std::marker::PhantomData;

pub mod try_from_raw_field;
pub mod try_opt_from_raw_field;

pub use try_from_raw_field::TryFromRawField;
pub use try_opt_from_raw_field::TryOptFromRawField;

#[derive(Default)]
pub struct OwnedMessageImpl<MD, FS, const BITFIELD_U32_LEN: usize> {
    bitvec: ::bitvec::array::BitArray<::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
    fields: FS,
    _phantom: PhantomData<MD>,
}
impl<MD, FS, const BITFIELD_U32_LEN: usize> OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN> {
    pub fn try_get_field_as<'msg, FD, R>(&'msg self) -> Result<R>
    where
        FD: StaticFieldDescriptor,
        FS: OwnedRawFieldGetter<FD>,
        <FS as OwnedRawFieldGetter<FD>>::Type: 'msg,
        R: TryFromRawField<'msg, MD, FD, <FS as OwnedRawFieldGetter<FD>>::Type>,
    {
        let raw_field_ref = <FS as OwnedRawFieldGetter<FD>>::get(&self.fields);
        R::try_from_raw_field(raw_field_ref)
    }
}

impl<'msg, MD, FD, FS, R, const BITFIELD_U32_LEN: usize> MessageFieldGetter<'msg, FD, R>
    for OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN>
where
    MD: StaticMessageDescriptor,
    FD: StaticFieldDescriptor,
    FS: OwnedRawFieldGetter<FD>,
    <FS as OwnedRawFieldGetter<FD>>::Type: 'msg,
    R: TryFromRawField<'msg, MD, FD, <FS as OwnedRawFieldGetter<FD>>::Type>,
{
    fn try_get_field(&'msg self) -> Result<R> {
        self.try_get_field_as::<FD, R>()
    }
}

impl<'msg, MD, FS, const BITFIELD_U32_LEN: usize> MessageImpl<'msg, MD>
    for OwnedMessageImpl<MD, FS, BITFIELD_U32_LEN>
{
}

pub trait OwnedRawFieldGetter<FD> {
    type Type;
    fn get(&self) -> &Self::Type;
}
