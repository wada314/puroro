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
use crate::Result;
use ::itertools::Either;
use ::ref_cast::RefCast;
use ::std::alloc::Allocator;

trait GenericMessageExt {
    type Alloc: Allocator;
    fn try_as_scalar_message<'a, T>(&'a self, number: i32) -> Result<Option<&T>>
    where
        T: 'a + RefCast<From = GenericMessage<Self::Alloc>>;
    fn try_as_repeated_message<'a, T>(
        &'a self,
        number: i32,
    ) -> Result<impl Iterator<Item = Result<&T>>>
    where
        T: 'a + RefCast<From = GenericMessage<Self::Alloc>>;
}
impl<A: Allocator + Clone> GenericMessageExt for GenericMessage<A> {
    type Alloc = A;
    fn try_as_scalar_message<'a, T>(&'a self, number: i32) -> Result<Option<&T>>
    where
        T: 'a + RefCast<From = GenericMessage<Self::Alloc>>,
    {
        let Some(field) = self.field(number) else {
            return Ok(None);
        };
        field
            .try_as_scalar_message()
            .map(|o| o.map(RefCast::ref_cast))
    }
    fn try_as_repeated_message<'a, T>(
        &'a self,
        number: i32,
    ) -> Result<impl Iterator<Item = Result<&T>>>
    where
        T: 'a + RefCast<From = GenericMessage<Self::Alloc>>,
    {
        let Some(field) = self.field(number) else {
            return Ok(Either::Left(::std::iter::empty()));
        };
        Ok(Either::Right(
            field
                .try_as_repeated_message()?
                .map(|r| r.map(RefCast::ref_cast)),
        ))
    }
}
