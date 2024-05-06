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

use crate::untyped_message::UntypedMessage;
use crate::variant::{Int32, VariantIntegerType};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;

/// Some utility impls for `UntypedMessage`.
impl UntypedMessage<'_> {
    fn scalar_variant_field<T>(&self, number: i32) -> Result<Option<T::RustType>>
    where
        T: VariantIntegerType,
    {
        self.field(number)
            .as_scalar_variant(false)?
            .map(|v| T::try_from_variant(v))
            .transpose()
    }
    fn repeated_variant_field<'a, T>(
        &'a self,
        number: i32,
    ) -> impl 'a + Iterator<Item = Result<T::RustType>>
    where
        T: VariantIntegerType,
    {
        self.field(number)
            .as_repeated_variant(false)
            .into_iter()
            .map(|var| T::try_from_variant(var?))
    }
    fn scalar_enum2_field<T>(&self, number: i32) -> Result<Option<T>>
    where
        T: TryFrom<i32, Error = ErrorKind>,
    {
        self.scalar_variant_field::<Int32>(number)?
            .map(|i| i.try_into())
            .transpose()
    }
    fn scalar_message_field<'a, T, F: Fn(UntypedMessage<'a>) -> T>(
        &'a self,
        number: i32,
        constructor: F,
    ) -> Result<Option<T>> {
        Ok(self.field(number).as_scalar_message()?.map(constructor))
    }
    fn repeated_message_field<'a, T, F: 'a + Fn(UntypedMessage<'a>) -> T>(
        &'a self,
        number: i32,
        constructor: F,
    ) -> impl 'a + Iterator<Item = Result<T>> {
        self.field(number)
            .as_repeated_message()
            .into_iter()
            .map_ok(constructor)
    }
}
