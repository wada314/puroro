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

use crate::internal::detach_alloc::{AttachAlloc, DetachAlloc, RefMut};
#[cfg(feature = "allocator_api")]
use ::std::alloc::Allocator;
use ::std::mem::ManuallyDrop;
use ::std::ops::Deref;

pub struct NoAllocVec<T>(*mut T, usize, usize);

impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.0, self.1) }
    }
}

#[cfg(feature = "allocator_api")]
impl<T, A: Allocator> DetachAlloc for Vec<T, A> {
    type Detached = NoAllocVec<T>;
    type Allocator = A;
    fn detach(self) -> (Self::Detached, Self::Allocator) {
        let (ptr, length, capacity, allocator) = self.into_raw_parts_with_alloc();
        (NoAllocVec(ptr, length, capacity), allocator)
    }
}
#[cfg(feature = "allocator_api")]
unsafe impl<T, A: Allocator> AttachAlloc<A> for NoAllocVec<T> {
    type Attached = Vec<T, A>;
    unsafe fn attach(self, allocator: A) -> Self::Attached {
        Vec::from_raw_parts_in(self.0, self.1, self.2, allocator)
    }
    unsafe fn ref_mut(&mut self, allocator: A) -> RefMut<Self::Attached> {
        let tmp = ManuallyDrop::new(Vec::from_raw_parts_in(self.0, self.1, self.2, allocator));
        RefMut::new(self, tmp)
    }
}

#[cfg(not(feature = "allocator_api"))]
impl<T> DetachAlloc for Vec<T> {
    type Detached = NoAllocVec<T>;
    type Allocator = ();
    fn detach(mut self) -> (Self::Detached, Self::Allocator) {
        let (ptr, length, capacity) = (self.as_mut_ptr(), self.len(), self.capacity());
        ::std::mem::forget(self);
        (NoAllocVec(ptr, length, capacity), ())
    }
}
#[cfg(not(feature = "allocator_api"))]
unsafe impl<T> AttachAlloc<()> for NoAllocVec<T> {
    type Attached = Vec<T>;
    unsafe fn attach(self, _allocator: ()) -> Self::Attached {
        Vec::from_raw_parts(self.0, self.1, self.2)
    }
    unsafe fn ref_mut(&mut self, _allocator: ()) -> RefMut<Self::Attached> {
        let tmp = ManuallyDrop::new(Vec::from_raw_parts(self.0, self.1, self.2));
        RefMut::new(self, tmp)
    }
}
