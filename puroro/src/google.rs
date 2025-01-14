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

use crate::dynamic::{DynamicMessage, FieldReducingErrorStrategy};
use crate::variant::variant_types;
use ::ref_cast::RefCast;
use ::std::alloc::Allocator;

trait DynamicMessageExt {
    type Alloc: Allocator;
    fn as_scalar_int32(&self, number: i32) -> Option<i32>;
    fn as_repeated_int32(&self, number: i32) -> impl Iterator<Item = i32>;
    fn as_scalar_enum<E>(&self, number: i32) -> Option<E>
    where
        E: 'static + TryFrom<i32> + Default,
        i32: From<E>;
    #[allow(unused)]
    fn as_repeated_enum<E>(&self, number: i32) -> impl Iterator<Item = E>
    where
        E: 'static + TryFrom<i32> + Default,
        i32: From<E>;
    fn as_scalar_string(&self, number: i32) -> Option<&str>;
    fn as_repeated_string(&self, number: i32) -> impl Iterator<Item = &str>;
    fn as_scalar_message<T>(&self, number: i32) -> Option<&T>
    where
        T: RefCast<From = DynamicMessage<Self::Alloc>>;
    fn as_repeated_message<'a, T: 'a>(&'a self, number: i32) -> impl Iterator<Item = &'a T>
    where
        T: RefCast<From = DynamicMessage<Self::Alloc>>;
}
impl<A: Allocator + Clone> DynamicMessageExt for DynamicMessage<A> {
    type Alloc = A;

    fn as_scalar_int32(&self, number: i32) -> Option<i32> {
        self.field(number).and_then(|f| {
            f.as_scalar_variant::<variant_types::Int32>(false, FieldReducingErrorStrategy::Skip)
                .unwrap()
        })
    }
    fn as_repeated_int32(&self, number: i32) -> impl Iterator<Item = i32> {
        self.field(number)
            .into_iter()
            .flat_map(|f| {
                f.as_repeated_variant::<variant_types::Int32>(false)
                    .ok()
                    .into_iter()
            })
            .flatten()
            .filter_map(|v| v.ok())
    }
    fn as_scalar_enum<E>(&self, number: i32) -> Option<E>
    where
        E: 'static + TryFrom<i32> + Default,
        i32: From<E>,
    {
        self.field(number).and_then(|f| {
            f.as_scalar_variant::<variant_types::Enum<E>>(false, FieldReducingErrorStrategy::Skip)
                .unwrap()
        })
    }
    fn as_repeated_enum<E>(&self, number: i32) -> impl Iterator<Item = E>
    where
        E: 'static + TryFrom<i32> + Default,
        i32: From<E>,
    {
        self.field(number)
            .into_iter()
            .flat_map(|f| {
                f.as_repeated_variant::<variant_types::Enum<E>>(false)
                    .ok()
                    .into_iter()
            })
            .flatten()
            .filter_map(|v| v.ok())
    }
    fn as_scalar_string(&self, number: i32) -> Option<&str> {
        self.field(number).and_then(|f| {
            f.as_scalar_string(FieldReducingErrorStrategy::Skip)
                .unwrap()
        })
    }
    fn as_repeated_string(&self, number: i32) -> impl Iterator<Item = &str> {
        self.field(number)
            .into_iter()
            .flat_map(|f| f.as_repeated_string().ok().into_iter())
            .flatten()
            .filter_map(|v| v.ok())
    }

    fn as_scalar_message<T>(&self, number: i32) -> Option<&T>
    where
        T: RefCast<From = DynamicMessage<Self::Alloc>>,
    {
        self.field(number)?
            .as_scalar_message()
            .ok()
            .flatten()
            .map(RefCast::ref_cast)
    }
    fn as_repeated_message<'a, T>(&'a self, number: i32) -> impl Iterator<Item = &'a T>
    where
        T: 'a + RefCast<From = DynamicMessage<Self::Alloc>>,
    {
        self.field(number).into_iter().flat_map(|f| {
            f.as_repeated_message()
                .ok()
                .into_iter()
                .flatten()
                .filter_map(|v| v.ok().map(RefCast::ref_cast))
        })
    }
}
