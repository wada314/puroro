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

pub struct OptionMessageImpl<MIR>(Option<MIR>);
impl<MIR> From<Option<MIR>> for OptionMessageImpl<MIR> {
    fn from(v: Option<MIR>) -> Self {
        OptionMessageImpl(v)
    }
}

impl<'msg, FD, MI, MIR, ReturnType> MessageOptFieldGetter<'msg, FD> for OptionMessageImpl<MIR>
where
    FD: StaticFieldDescriptor,
    MIR: AsMessageImplRef<MessageImplType = MI>,
    MI: 'msg + MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
{
    type OptReturnType = ReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::OptReturnType>> {
        // just delegate to inner mesasge type
        Ok(self
            .0
            .as_ref()
            .map(|m| m.as_message_impl_ref().try_get_opt_field())
            .transpose()?
            .flatten())
    }
}

// Switch impl for message type and non-message types
impl<'msg, FD, MIR, TypeTag, IsMessage, ReturnType> MessageScalarFieldGetter<'msg, FD>
    for OptionMessageImpl<MIR>
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

// Non message types. Get `Some` value or use `FD::DEFAULT_VALUE`.
impl<'msg, FD, MI, MIR, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, False>
    for OptionMessageImpl<MIR>
where
    FD: StaticFieldDescriptor,
    MIR: AsMessageImplRef<MessageImplType = MI>,
    MI: 'msg + MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
    FieldDefaultValue: TryInto<ReturnType, Error = PuroroError>,
{
    type GetterReturnTypeImpl = ReturnType;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
        // Get non-message scalar field. Maybe use FD::DEFAULT_VALUE.
        Ok(self
            .0
            .as_ref()
            .map(|m| m.as_message_impl_ref().try_get_opt_field())
            .transpose()?
            .flatten()
            .or(FD::DEFAULT_VALUE
                .map(|default| default.try_into())
                .transpose()?)
            .ok_or(ErrorKind::ReflectionError)?)
    }
}

// Message field type. Return `OptionMessageImpl`
impl<'msg, FD, MIR, ReturnType> MessageScalarFieldGetterImpl<'msg, FD, True>
    for OptionMessageImpl<MIR>
where
    FD: StaticFieldDescriptor,
    Self: MessageOptFieldGetter<'msg, FD, OptReturnType = ReturnType>,
{
    type GetterReturnTypeImpl = OptionMessageImpl<ReturnType>;
    fn try_get_field_impl(&'msg self) -> Result<Self::GetterReturnTypeImpl> {
        Ok(self.try_get_opt_field()?.into())
    }
}
