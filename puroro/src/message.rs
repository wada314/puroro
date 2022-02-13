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

use crate::{AsMessageImplRef, DefaultIn};
use ::std::marker::PhantomData;

pub struct MessageImpl<MP, ImplTag, Fields, Shared> {
    pub(crate) fields: Fields,
    pub(crate) shared: Shared,
    _phantom: PhantomData<(MP, ImplTag)>,
}

impl<MP, ImplTag, Fields, Shared> MessageImpl<MP, ImplTag, Fields, Shared> {
    pub fn from_raw_parts(fields: Fields, shared: Shared) -> Self {
        Self {
            fields,
            shared,
            _phantom: PhantomData,
        }
    }
}

impl<MP, ImplTag, Fields, Shared> Default for MessageImpl<MP, ImplTag, Fields, Shared>
where
    Fields: Default,
    Shared: Default,
{
    fn default() -> Self {
        Self {
            fields: Default::default(),
            shared: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<MP, ImplTag, Fields, Shared, Alloc> DefaultIn for MessageImpl<MP, ImplTag, Fields, Shared>
where
    Fields: DefaultIn<AllocatorType = Alloc>,
    Shared: DefaultIn<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type AllocatorType = <Shared as DefaultIn>::AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        Self {
            fields: DefaultIn::default_in(Clone::clone(&alloc)),
            shared: DefaultIn::default_in(Clone::clone(&alloc)),
            _phantom: Default::default(),
        }
    }
}

impl<MP, ImplTag, Fields, Shared> AsMessageImplRef for MessageImpl<MP, ImplTag, Fields, Shared> {
    type MessageImplType = MessageImpl<MP, ImplTag, Fields, Shared>;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType {
        self
    }
}
