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

#[cfg(feature = "allocator_api")]
use ::std::alloc::Allocator;
use ::std::ops::Deref;

#[cfg(feature = "allocator_api")]
pub struct NoAllocVec<T>(*mut T, usize, usize);

#[cfg(not(feature = "allocator_api"))]
pub struct NoAllocVec<T>(Vec<T>);

#[cfg(feature = "allocator_api")]
pub struct VecMutRef<'a, T, A: Allocator>(&'a mut NoAllocVec<T>, Vec<T, A>);

#[cfg(feature = "allocator_api")]
impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.0, self.1) }
    }
}

#[cfg(not(feature = "allocator_api"))]
impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}
