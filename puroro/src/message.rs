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
use crate::{ErrorKind, PuroroError, Result};

mod as_ref;
pub use as_ref::{AsMessageImplRef, AsMessageRef};

pub trait MessageImpl<'msg, MD> {
    fn try_get_u32<FD: StaticFieldDescriptor>(&'msg self) -> Result<u32>
    where
        Self: MessageScalarFieldGetter<'msg, FD, ReturnType = u32>,
    {
        self.try_get_field()
    }
    fn try_get_str<FD: StaticFieldDescriptor>(&'msg self) -> Result<&str>
    where
        Self: MessageScalarFieldGetter<'msg, FD, ReturnType = &'msg str>,
    {
        self.try_get_field()
    }
    fn try_get_msg<FD: StaticFieldDescriptor, M>(&'msg self) -> Result<M>
    where
        Self: MessageScalarFieldGetter<'msg, FD, ReturnType = M>,
    {
        self.try_get_field()
    }
}

pub trait MessageOptFieldGetter<'msg, FD> {
    type ReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::ReturnType>>;
}

pub trait MessageScalarFieldGetter<'msg, FD>: MessageOptFieldGetter<'msg, FD> {
    fn try_get_field(&'msg self) -> Result<Self::ReturnType>;

    // A pseudo private default impl for `try_get_field`.
    // Because the scalar message field getter cannot fulfill the `TryInto` bound
    // I cannot implement this as a common default implementation.
    fn _try_get_field(&'msg self) -> Result<Self::ReturnType>
    where
        FD: StaticFieldDescriptor,
        FieldDefaultValue: TryInto<Self::ReturnType, Error = PuroroError>,
    {
        Ok(if let Some(v) = self.try_get_opt_field()? {
            v
        } else {
            FD::DEFAULT_VALUE
                .map(|d| d.try_into())
                .transpose()?
                .ok_or(ErrorKind::ReflectionError)?
        })
    }
}
