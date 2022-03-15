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

use crate::bumpalo::boxed::Box as BBox;
use crate::bumpalo::collections::{String as BString, Vec as BVec};
use crate::bumpalo::Bump;
use ::std::convert::TryFrom;

pub trait Enum2:
    'static + PartialEq + Clone + Default + TryFrom<i32, Error = i32> + Into<i32>
{
}
pub trait Enum3: 'static + PartialEq + Clone + Default + From<i32> + Into<i32> {}

pub trait DefaultIn {
    type AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self;
}
impl<T> DefaultIn for Box<T>
where
    T: DefaultIn,
{
    type AllocatorType = T::AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        Box::new(T::default_in(alloc))
    }
}
impl<T> DefaultIn for Vec<T> {
    type AllocatorType = ();
    fn default_in(_: Self::AllocatorType) -> Self {
        Vec::new()
    }
}
impl DefaultIn for String {
    type AllocatorType = ();
    fn default_in(_: Self::AllocatorType) -> Self {
        String::new()
    }
}
impl<'bump, T> DefaultIn for BVec<'bump, T> {
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BVec::new_in(alloc)
    }
}
impl<'bump> DefaultIn for BString<'bump> {
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BString::new_in(alloc)
    }
}
impl<'bump, T> DefaultIn for BBox<'bump, T>
where
    T: DefaultIn<AllocatorType = &'bump Bump>,
{
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BBox::new_in(DefaultIn::default_in(alloc), alloc)
    }
}
