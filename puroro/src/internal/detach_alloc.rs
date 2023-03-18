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
use ::std::alloc::Global;
use ::std::mem::ManuallyDrop;
use ::std::ops::{Deref, DerefMut};

pub trait DetachAlloc {
    type Detached: AttachAlloc<Self::Allocator, Attached = Self>;
    type Allocator;
    fn detach(self) -> (Self::Detached, Self::Allocator);
}

#[cfg(feature = "allocator_api")]
pub unsafe trait AttachAlloc<A = Global> {
    type Attached: DetachAlloc<Allocator = A, Detached = Self>;
    unsafe fn attach(self, allocator: A) -> Self::Attached;
    unsafe fn make_nodrop_copy(&self, allocator: A) -> ManuallyDrop<Self::Attached>;
    unsafe fn ref_mut_in(&mut self, allocator: A) -> RefMut<'_, Self::Attached> {
        RefMut {
            tmp: self.make_nodrop_copy(allocator),
            src: self,
        }
    }
}
#[cfg(not(feature = "allocator_api"))]
pub unsafe trait AttachAlloc<A = ()> {
    type Attached: DetachAlloc<Allocator = A, Detached = Self>;
    unsafe fn attach(self, _allocator: A) -> Self::Attached;
    unsafe fn make_nodrop_copy(&self, _allocator: A) -> ManuallyDrop<Self::Attached>;
    unsafe fn ref_mut_in(&mut self, _allocator: A) -> RefMut<'_, Self::Attached> {
        RefMut {
            tmp: self.make_nodrop_copy(_allocator),
            src: self,
        }
    }
}

pub struct RefMut<'a, T>
where
    T: DetachAlloc,
{
    tmp: ManuallyDrop<T>,
    src: &'a mut T::Detached,
}
impl<T: DetachAlloc> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.tmp
    }
}
impl<T: DetachAlloc> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tmp
    }
}
impl<T: DetachAlloc> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        *self.src = unsafe { ManuallyDrop::take(&mut self.tmp) }.detach().0;
    }
}
impl<T: DetachAlloc> RefMut<'_, T> {
    pub fn retrieve_allocator(mut this: Self) -> T::Allocator {
        let (detached, allocator) = unsafe { ManuallyDrop::take(&mut this.tmp) }.detach();
        *this.src = detached;
        allocator
    }
}
