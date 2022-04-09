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
use crate::message::{AsMessageImplRef, MessageOptFieldGetter, MessageScalarFieldGetter};
use crate::tags;
use crate::{ErrorKind, PuroroError, Result};

impl<'msg, FD, InnerM, ReturnType> MessageOptFieldGetter<'msg, FD> for Option<InnerM>
where
    FD: StaticFieldDescriptor,
    InnerM: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
{
    type OptReturnType = ReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::OptReturnType>> {
        // just delegate to inner mesasge type
        Ok(self
            .as_ref()
            .map(|m| m.try_get_opt_field())
            .transpose()?
            .flatten())
    }
}

impl<'msg, FD, InnerM, TypeTag, IsMessage, ReturnType> MessageScalarFieldGetter<'msg, FD>
    for Option<InnerM>
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

pub trait MessageScalarFieldGetterImpl<'msg, FD, IsMessage>:
    MessageOptFieldGetter<'msg, FD>
{
    type GetterReturnTypeImpl;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl>;
}

impl<'msg, FD, InnerM, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, False> for Option<InnerM>
where
    FD: StaticFieldDescriptor,
    InnerM: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
    FieldDefaultValue: TryInto<ReturnType, Error = PuroroError>,
{
    type GetterReturnTypeImpl = ReturnType;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
        // Get non-message scalar field. Maybe use FD::DEFAULT_VALUE.
        Ok(self
            .as_ref()
            .map(|m| m.try_get_opt_field())
            .transpose()?
            .flatten()
            .or(FD::DEFAULT_VALUE
                .map(|default| default.try_into())
                .transpose()?)
            .ok_or(ErrorKind::ReflectionError)?)
    }
}

impl<'msg, FD, InnerM, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, True> for Option<InnerM>
where
    FD: StaticFieldDescriptor,
    InnerM: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
{
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
        todo!()
    }
}
