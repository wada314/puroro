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

use crate::desc::{FieldDefaultValue, StaticFieldDescriptor};
use crate::internal::bool::{False, True};
use crate::internal::option::OptionMessageImpl;
use crate::tags;
use crate::{ErrorKind, PuroroError, Result};

mod as_ref;
pub use as_ref::{AsMessageImplRef, AsMessageRef};

pub trait MessageImpl<MD> {
    fn try_get_u32<'msg, FD: StaticFieldDescriptor>(&'msg self) -> Result<u32>
    where
        Self: MessageScalarFieldGetter<FD, GetterReturnType<'msg> = u32>,
    {
        self.try_get_field()
    }
    fn try_get_str<'msg, FD: StaticFieldDescriptor>(&'msg self) -> Result<&str>
    where
        Self: MessageScalarFieldGetter<FD, GetterReturnType<'msg> = &'msg str>,
    {
        self.try_get_field()
    }
    fn try_get_msg<'msg, FD: StaticFieldDescriptor, M>(&'msg self) -> Result<M>
    where
        Self: MessageScalarFieldGetter<FD, GetterReturnType<'msg> = M>,
    {
        self.try_get_field()
    }
}

pub trait MessageOptFieldGetter<FD> {
    type OptReturnType<'msg>
    where
        Self: 'msg;
    fn try_get_opt_field<'a>(&'a self) -> Result<Option<Self::OptReturnType<'a>>>;
}

pub trait MessageScalarFieldGetter<FD>: MessageOptFieldGetter<FD> {
    type GetterReturnType<'msg>
    where
        Self: 'msg;
    fn try_get_field<'a>(&'a self) -> Result<Self::GetterReturnType<'a>>;
}

// Blanket impl for MessageScalarFieldGetter.
pub trait MessageScalarFieldGetterImpl<FD, IsMessage>: MessageOptFieldGetter<FD> {
    type GetterReturnTypeImpl<'msg>
    where
        Self: 'msg;
    fn try_get_field_impl<'a>(&'a self) -> Result<Self::GetterReturnTypeImpl<'a>>;
}

// Switch impl for message type and non-message types
impl<FD, T, TypeTag, IsMessage> MessageScalarFieldGetter<FD> for T
where
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag<IsMessage = IsMessage>,
    Self: MessageScalarFieldGetterImpl<FD, IsMessage>,
{
    type GetterReturnType<'msg> =
        <Self as MessageScalarFieldGetterImpl<FD, IsMessage>>::GetterReturnTypeImpl<'msg> where Self: 'msg;
    fn try_get_field<'a>(&'a self) -> Result<Self::GetterReturnType<'a>> {
        self.try_get_field_impl()
    }
}

// Non message types. Get `Some` value or use `FD::DEFAULT_VALUE`.
impl<'msg, FD, T> MessageScalarFieldGetterImpl<FD, False> for T
where
    Self: 'msg + MessageOptFieldGetter<FD>,
    FD: StaticFieldDescriptor,
    FieldDefaultValue: TryInto<Self::OptReturnType<'msg>, Error = PuroroError>,
{
    type GetterReturnTypeImpl<'a> = Self::OptReturnType<'a> where Self: 'a;
    fn try_get_field_impl<'a>(&'a self) -> Result<Self::GetterReturnTypeImpl<'a>> {
        // Get non-message scalar field. Maybe use FD::DEFAULT_VALUE.
        Ok(self
            .try_get_opt_field()?
            .or(FD::DEFAULT_VALUE
                .map(|default| default.try_into())
                .transpose()?)
            .ok_or(ErrorKind::ReflectionError)?)
    }
}

// Message field type. Return `OptionMessageImpl`
impl<FD, T> MessageScalarFieldGetterImpl<FD, True> for T
where
    FD: StaticFieldDescriptor,
    Self: MessageOptFieldGetter<FD>,
{
    type GetterReturnTypeImpl<'msg> = Self::OptReturnType<'msg> where Self: 'msg;
    fn try_get_field_impl<'a>(&'a self) -> Result<Self::GetterReturnTypeImpl<'a>> {
        Ok(self.try_get_opt_field()?.into())
    }
}
