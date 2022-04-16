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

use crate::desc::{FieldDescriptor, FieldLabelEnum, StaticFieldDescriptor};
use crate::internal::bool::{False, True};
use crate::internal::Bitfield;
use crate::message::AsMessageImplRef;
use crate::tags::{self, FieldLabelTag};
use crate::{ErrorKind, PuroroError, Result};

pub trait TryRawFieldInto<MD, FD, B> {
    type Target<'a>
    where
        Self: 'a,
        B: 'a;
    fn try_raw_field_into<'a>(&'a self, _: &'a B) -> Result<Self::Target<'a>>;
}

pub trait TryRawFieldIntoImpl<MD, FD, B, IsRepeated, IsLd, IsMessage> {
    type TargetImpl<'a>
    where
        Self: 'a,
        B: 'a;
    fn try_raw_field_into_impl<'a>(&'a self, _: &'a B) -> Result<Self::TargetImpl<'a>>;
}

impl<MD, FD, B, T, LabelTag, TypeTag> TryRawFieldInto<MD, FD, B> for T
where
    T: TryRawFieldIntoImpl<MD, FD, B, LabelTag::IsRepeated, TypeTag::IsLd, TypeTag::IsMessage>,
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
{
    type Target<'a> = T::TargetImpl<'a> where Self: 'a, B: 'a;
    fn try_raw_field_into<'a>(&'a self, bitfields: &'a B) -> Result<Self::Target<'a>> {
        self.try_raw_field_into_impl(bitfields)
    }
}

impl<MD, FD, B, T, TypeTag> TryRawFieldIntoImpl<MD, FD, B, False, False, False> for T
where
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
    B: Bitfield,
    T: Clone + TryInto<TypeTag::NonMessageScalarGetterType<'static>, Error = PuroroError>,
    TypeTag::NonMessageScalarGetterType<'static>: Default + PartialEq,
{
    type TargetImpl<'a> = Option<TypeTag::DefaultValueType>;
    fn try_raw_field_into_impl<'a>(&'a self, bitfields: &'a B) -> Result<Self::TargetImpl<'a>> {
        let v = self.clone().try_into()?;
        if let Some(hasbit_index) = FD::OWNED_HASFIELD_BITFIELD_INDEX {
            bitfields.get(hasbit_index).then(|| v)
        } else {
            (v != Default::default()).then(|| v)
        }
    }
}
