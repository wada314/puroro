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
use crate::internal::bool::{False, True};
use crate::internal::Bitfield;
use crate::message::{MessageImpl, MessageOptFieldGetter, MessageScalarFieldGetter};
use crate::tags;
use crate::Result;
use ::std::marker::PhantomData;

pub mod try_from_raw_field;
use try_from_raw_field::TryFromRawField;

#[derive(Default)]
pub struct OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
{
    bitvec: MD::OwnedBitfield,
    fields: FS,
    _phantom: PhantomData<MD>,
}

impl<'msg, MD, FS> MessageImpl<'msg, MD> for OwnedMessageImpl<MD, FS> where
    MD: StaticMessageDescriptor
{
}

pub trait OwnedRawFieldGetter<FD> {
    type Type;
    fn get(&self) -> &Self::Type;
}

impl<'msg, MD, FD, FS, TypeTag, R> MessageOptFieldGetter<'msg, FD> for OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
    Self: MessageOptFieldGetterImpl<'msg, FD, TypeTag::IsMessage, OptReturnTypeImpl = R>,
{
    type OptReturnType = R;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::OptReturnType>> {
        self.try_get_opt_field_impl()
    }
}

trait MessageOptFieldGetterImpl<'msg, FD, IsMessage> {
    type OptReturnTypeImpl;
    fn try_get_opt_field_impl(&'msg self) -> Result<Option<Self::OptReturnTypeImpl>>;
}

// Non-message field getter. Getter type is determined by `FieldTypeTag::NonMessageScalarGetterType`
impl<'msg, MD, FD, FS, TypeTag, GetterType> MessageOptFieldGetterImpl<'msg, FD, False>
    for OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag<NonMessageScalarGetterType<'msg> = GetterType>,
    FS: OwnedRawFieldGetter<FD, Type = GetterType>,
{
    type OptReturnTypeImpl = GetterType;
    fn try_get_opt_field_impl(&'msg self) -> Result<Option<Self::OptReturnTypeImpl>> {
        let get_inner = || {
            <Option<GetterType> as TryFromRawField<FD>>::try_from_raw_field(
                self.fields.get(),
                &self.bitvec,
            )
        };
    }
}
