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

use ::std::mem::ManuallyDrop;
use ::std::ops::{Deref, DerefMut};

pub trait DetachAlloc {
    type Detached: AttachAlloc<Self::Allocator, Attached = Self>;
    type Allocator;
    fn detach(self) -> (Self::Detached, Self::Allocator);
}

pub trait AttachAlloc<A> {
    type Attached: DetachAlloc<Allocator = A, Detached = Self>;
    fn attach(self, allocator: A) -> Self::Attached;
    fn ref_mut(&mut self, allocator: A) -> RefMut<Self::Attached>;
}

pub struct RefMut<'a, D: DetachAlloc> {
    tmp: ManuallyDrop<D>,
    src: &'a mut D::Detached,
}

impl<'a, D: DetachAlloc> Deref for RefMut<'a, D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.tmp
    }
}

impl<'a, D: DetachAlloc> DerefMut for RefMut<'a, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tmp
    }
}

impl<'a, D: DetachAlloc> Drop for RefMut<'a, D> {
    fn drop(&mut self) {
        unsafe {
            *self.src = ManuallyDrop::take(&mut self.tmp).detach().0;
        }
    }
}
