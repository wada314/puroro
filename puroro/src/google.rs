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

pub mod protobuf;

use crate::generic_message::GenericMessage;
use ::ref_cast::RefCast;
use ::std::alloc::Allocator;

trait GenericMessageExt {
    type Alloc: Allocator;
    fn as_scalar_message<T>(&self, number: i32) -> Option<&T>
    where
        T: RefCast<From = GenericMessage<Self::Alloc>>;
    fn as_repeated_message<'a, T: 'a>(&'a self, number: i32) -> impl Iterator<Item = &'a T>
    where
        T: RefCast<From = GenericMessage<Self::Alloc>>;
}
impl<A: Allocator + Clone> GenericMessageExt for GenericMessage<A> {
    type Alloc = A;
    fn as_scalar_message<T>(&self, number: i32) -> Option<&T>
    where
        T: RefCast<From = GenericMessage<Self::Alloc>>,
    {
        self.field(number)?
            .as_scalar_message()
            .map(RefCast::ref_cast)
    }
    fn as_repeated_message<'a, T>(&'a self, number: i32) -> impl Iterator<Item = &'a T>
    where
        T: 'a + RefCast<From = GenericMessage<Self::Alloc>>,
    {
        self.field(number)
            .into_iter()
            .flat_map(|f| f.as_repeated_message().map(RefCast::ref_cast))
    }
}
