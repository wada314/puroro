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

use crate::{ErrorKind, Result};

pub trait MessageImpl<'msg, MD> {
    fn try_get_u32<FD>(&'msg self) -> Result<u32>
    where
        Self: MessageFieldGetter<'msg, FD, u32>,
    {
        self.try_get_field()
    }
    fn try_get_str<FD>(&'msg self) -> Result<&str>
    where
        Self: MessageFieldGetter<'msg, FD, &'msg str>,
    {
        self.try_get_field()
    }
}
pub trait MessageFieldGetter<'msg, FD, R> {
    fn try_get_field(&'msg self) -> Result<R> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait AsMessageRef {
    type MessageType;
    fn as_message_ref(&self) -> &Self::MessageType;
}
pub trait AsMessageImplRef {
    type MessageImplType;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType;
}

impl<T: AsMessageRef> AsMessageRef for &T {
    type MessageType = T::MessageType;
    fn as_message_ref(&self) -> &Self::MessageType {
        T::as_message_ref(self)
    }
}
impl<T: AsMessageImplRef> AsMessageImplRef for &T {
    type MessageImplType = T::MessageImplType;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        T::as_message_impl_ref(&self)
    }
}
