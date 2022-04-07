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
use crate::message::{AsMessageImplRef, MessageScalarFieldGetter};
use crate::tags;
use crate::{PuroroError, Result};

impl<'msg, FD, InnerM, ReturnType> MessageScalarFieldGetter<'msg, FD> for Option<InnerM>
where
    FD: StaticFieldDescriptor,
    InnerM: MessageScalarFieldGetter<'msg, FD, ReturnType = ReturnType>,
    FieldDefaultValue: TryInto<Self::ReturnType, Error = PuroroError>,
{
    type ReturnType = ReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::ReturnType>> {
        Ok(self.map(|m| m.try_get_field()).transpose()?)
    }
}
