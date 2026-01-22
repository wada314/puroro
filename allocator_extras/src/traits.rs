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

//! Traits that mirror standard library behaviour with allocator awareness.

use ::allocator_api2::alloc::Allocator;

/// Allocator-aware counterpart of `Default`.
pub trait DefaultIn<A: Allocator>: Sized {
    /// Creates a default value using the provided allocator.
    fn default_in(alloc: A) -> Self;
}

/// Allocator-aware counterpart of `Clone`.
pub trait CloneIn<A: Allocator>: Sized {
    /// Clones `self`, allocating with the provided allocator.
    fn clone_in(&self, alloc: A) -> Self;
}

/// Allocator-aware counterpart of `ToOwned`.
pub trait ToOwnedIn<A: Allocator> {
    /// The owned type produced when cloning into an allocator.
    type Owned;

    /// Creates an owned value using the provided allocator.
    fn to_owned_in(&self, alloc: A) -> Self::Owned;
}
