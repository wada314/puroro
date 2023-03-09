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
}
#[cfg(not(feature = "allocator_api"))]
pub unsafe trait AttachAlloc<A = ()> {
    type Attached: DetachAlloc<Allocator = A, Detached = Self>;
    unsafe fn attach(self, allocator: A) -> Self::Attached;
    unsafe fn make_nodrop_copy(&self, allocator: A) -> ManuallyDrop<Self::Attached>;
}
