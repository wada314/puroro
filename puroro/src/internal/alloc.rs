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

use crate::internal::detach_alloc::{AttachAlloc, DetachAlloc};
#[cfg(feature = "allocator_api")]
use ::std::alloc::Allocator;
use ::std::mem::ManuallyDrop;
use ::std::ops::Deref;

pub struct NoAllocVec<T>(*mut T, usize, usize);
pub struct NoAllocBox<T: ?Sized>(*mut T);

impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.0, self.1) }
    }
}
impl<T> Deref for NoAllocBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
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
    unsafe fn make_nodrop_copy(&self, allocator: A) -> ManuallyDrop<Self::Attached> {
        ManuallyDrop::new(Self(self.0, self.1, self.2).attach(allocator))
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
    unsafe fn make_nodrop_copy(&self, _allocator: ()) -> ManuallyDrop<Self::Attached> {
        ManuallyDrop::new(Self(self.0, self.1, self.2).attach(()))
    }
}

#[cfg(feature = "allocator_api")]
impl<T, A: Allocator> DetachAlloc for Box<T, A> {
    type Detached = NoAllocBox<T>;
    type Allocator = A;
    fn detach(self) -> (Self::Detached, Self::Allocator) {
        let (ptr, allocator) = Box::into_raw_with_allocator(self);
        (NoAllocBox(ptr), allocator)
    }
}
#[cfg(feature = "allocator_api")]
unsafe impl<T, A: Allocator> AttachAlloc<A> for NoAllocBox<T> {
    type Attached = Box<T, A>;
    unsafe fn attach(self, allocator: A) -> Self::Attached {
        Box::from_raw_in(self.0, allocator)
    }
    unsafe fn make_nodrop_copy(&self, allocator: A) -> ManuallyDrop<Self::Attached> {
        ManuallyDrop::new(Self(self.0).attach(allocator))
    }
}

#[cfg(not(feature = "allocator_api"))]
impl<T> DetachAlloc for Box<T> {
    type Detached = NoAllocBox<T>;
    type Allocator = ();
    fn detach(self) -> (Self::Detached, Self::Allocator) {
        let ptr = Box::into_raw(self);
        (NoAllocBox(ptr), ())
    }
}
#[cfg(not(feature = "allocator_api"))]
unsafe impl<T> AttachAlloc<()> for NoAllocBox<T> {
    type Attached = Box<T>;
    unsafe fn attach(self, _allocator: ()) -> Self::Attached {
        Box::from_raw(self.0)
    }
    unsafe fn make_nodrop_copy(&self, allocator: ()) -> ManuallyDrop<Self::Attached> {
        ManuallyDrop::new(Self(self.0).attach(()))
    }
}
