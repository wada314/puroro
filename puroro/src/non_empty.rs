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

use ::std::alloc::Allocator;

use ::derive_more::{Debug, Deref, DerefMut, Display};

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}
macro_rules! impl_is_empty_for_primitives {
    ($t:ty, $v:expr) => {
        impl IsEmpty for $t {
            fn is_empty(&self) -> bool {
                *self == $v
            }
        }
    };
}
impl_is_empty_for_primitives!(i32, 0);
impl_is_empty_for_primitives!(u32, 0);
impl_is_empty_for_primitives!(i64, 0);
impl_is_empty_for_primitives!(u64, 0);
impl_is_empty_for_primitives!(f32, 0.0);
impl_is_empty_for_primitives!(f64, 0.0);
impl_is_empty_for_primitives!(bool, false);
impl IsEmpty for ::std::string::String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<A: Allocator> IsEmpty for crate::string::String<A> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl<A: Allocator> IsEmpty for ::std::vec::Vec<u8, A> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[repr(transparent)]
#[derive(Debug, Display, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmpty<T>(T);

impl<T: IsEmpty> NonEmpty<T> {
    pub fn new(value: T) -> Option<Self> {
        (!value.is_empty()).then(|| NonEmpty(value))
    }
}
