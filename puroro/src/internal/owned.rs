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
// See the License for the specific language governing perFS::MessageImplTypessions and
// liFS::MessageImplTypetations under the License.

use crate::desc::{StaticFieldDescriptor, StaticMessageDescriptor};
use crate::internal::bool::{False, True};
use crate::internal::Bitfield;
use crate::message::{AsMessageImplRef, MessageImpl, MessageOptFieldGetter};
use crate::tags;
use crate::Result;
use ::std::marker::PhantomData;

pub mod try_from_raw_field;
use try_from_raw_field::TryRawFieldInto;

#[derive(Default)]
pub struct OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
{
    bitvec: MD::OwnedBitfield,
    fields: FS,
    _phantom: PhantomData<MD>,
}

impl<MD, FS> MessageImpl<MD> for OwnedMessageImpl<MD, FS> where MD: StaticMessageDescriptor {}
impl<MD, FS> AsMessageImplRef for OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
{
    type MessageImplType = Self;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        self
    }
}

pub trait OwnedRawField<FD> {
    type Type;
    fn get(&self) -> &Self::Type;
}
pub trait OwnedRawMessageField<FD>: OwnedRawField<FD> {
    type MessageImplType;
}

// branch opt field getter impl by IsMessage.
impl<MD, FD, FS, TypeTag> MessageOptFieldGetter<FD> for OwnedMessageImpl<MD, FS>
where
    Self: MessageOptFieldGetterImpl<FD, TypeTag::IsMessage>,
    MD: StaticMessageDescriptor,
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type OptReturnType<'msg> =
        <Self as MessageOptFieldGetterImpl<FD, TypeTag::IsMessage>>::OptReturnTypeImpl<'msg>
    where Self: 'msg;
    fn try_get_opt_field<'a>(&'a self) -> Result<Option<Self::OptReturnType<'a>>> {
        self.try_get_opt_field_impl()
    }
}

trait MessageOptFieldGetterImpl<FD, IsMessage> {
    type OptReturnTypeImpl<'msg>
    where
        Self: 'msg;
    fn try_get_opt_field_impl<'a>(&'a self) -> Result<Option<Self::OptReturnTypeImpl<'a>>>;
}

// Non-message field optional getter.
// Getter type is deterFS::MessageImplTypened by `FieldTypeTag::NonMessageScalarGetterType`
impl<MD, FD, FS, TypeTag> MessageOptFieldGetterImpl<FD, False> for OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
    FD: 'static + StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
    FS: OwnedRawField<FD>,
    FS::Type: TryRawFieldInto<MD, FD, MD::OwnedBitfield>,
{
    type OptReturnTypeImpl<'msg> = <FS::Type as TryRawFieldInto<MD, FD, MD::OwnedBitfield>>::Target<'msg> where Self:'msg;
    fn try_get_opt_field_impl<'a>(&'a self) -> Result<Option<Self::OptReturnTypeImpl<'a>>> {
        self.fields.get().try_raw_field_into(&self.bitvec)
    }
}

// Message field optional getter.
impl<'msg, MD, FD, FS> MessageOptFieldGetterImpl<FD, True> for OwnedMessageImpl<MD, FS>
where
    MD: StaticMessageDescriptor,
    FD: 'static,
    FS: OwnedRawField<FD> + OwnedRawMessageField<FD>,
    FS::MessageImplType: 'msg,
    FS::Type: TryRawFieldInto<MD, FD, MD::OwnedBitfield>,
{
    type OptReturnTypeImpl<'msg2> = &'msg2 FS::MessageImplType where Self: 'msg2;
    fn try_get_opt_field_impl<'a>(&'a self) -> Result<Option<Self::OptReturnTypeImpl<'a>>> {
        self.fields.get().try_raw_field_into(&self.bitvec)
    }
}
