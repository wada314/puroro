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

pub trait MessageImpl<'msg, MD> {
    fn try_get_u32<FD: StaticFieldDescriptor>(&'msg self) -> Result<u32>
    where
        Self: MessageScalarFieldGetter<'msg, FD, GetterReturnType = u32>,
    {
        self.try_get_field()
    }
    fn try_get_str<FD: StaticFieldDescriptor>(&'msg self) -> Result<&str>
    where
        Self: MessageScalarFieldGetter<'msg, FD, GetterReturnType = &'msg str>,
    {
        self.try_get_field()
    }
    fn try_get_msg<FD: StaticFieldDescriptor, M>(&'msg self) -> Result<M>
    where
        Self: MessageScalarFieldGetter<'msg, FD, GetterReturnType = M>,
    {
        self.try_get_field()
    }
}

pub trait MessageOptFieldGetter<'msg, FD> {
    type OptReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::OptReturnType>>;
}

pub trait MessageScalarFieldGetter<'msg, FD>: MessageOptFieldGetter<'msg, FD> {
    type GetterReturnType;
    fn try_get_field(&'msg self) -> Result<Self::GetterReturnType>;
}

// Blanket impl for MessageScalarFieldGetter.
pub trait MessageScalarFieldGetterImpl<'msg, FD, IsMessage>:
    MessageOptFieldGetter<'msg, FD>
{
    type GetterReturnTypeImpl;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl>;
}

// Switch impl for message type and non-message types
impl<'msg, FD, T, TypeTag, IsMessage, ReturnType> MessageScalarFieldGetter<'msg, FD> for T
where
    FD: StaticFieldDescriptor<FieldTypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag<IsMessage = IsMessage>,
    Self: MessageScalarFieldGetterImpl<'msg, FD, IsMessage, GetterReturnTypeImpl = ReturnType>,
{
    type GetterReturnType = ReturnType;
    fn try_get_field(&'msg self) -> Result<Self::GetterReturnType> {
        self.try_get_field_impl()
    }
}

// Non message types. Get `Some` value or use `FD::DEFAULT_VALUE`.
impl<'msg, FD, T, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, False> for T
where
    FD: StaticFieldDescriptor,
    Self: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
    FieldDefaultValue: TryInto<ReturnType, Error = PuroroError>,
{
    type GetterReturnTypeImpl = ReturnType;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
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
impl<'msg, FD, T, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, True> for T
where
    FD: StaticFieldDescriptor,
    Self: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
{
    type GetterReturnTypeImpl = OptionMessageImpl<ReturnType>;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
        Ok(self.try_get_opt_field()?.into())
    }
}
