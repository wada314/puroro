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

use crate::desc::StaticFieldDescriptor;
use crate::message::{AsMessageImplRef, MessageOptFieldGetter};
use crate::Result;

pub struct OptionMessageImpl<MIR>(Option<MIR>);
impl<MIR> From<Option<MIR>> for OptionMessageImpl<MIR> {
    fn from(v: Option<MIR>) -> Self {
        OptionMessageImpl(v)
    }
}
impl<MIR> AsMessageImplRef for OptionMessageImpl<MIR> {
    type MessageImplType = Self;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        self
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
