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

//! Helper functions and types related to allocator-aware collections.

use ::allocator_api2::alloc::{AllocError, Allocator};
use ::allocator_api2::boxed::Box;
use ::allocator_api2::vec::Vec;

/// Constructs a `Vec<T, A>` from an iterator using the supplied allocator.
pub fn vec_from_iter_in<T, I, A>(iter: I, alloc: A) -> Vec<T, A>
where
    I: IntoIterator<Item = T>,
    A: Allocator,
{
    let mut vec = Vec::new_in(alloc);
    vec.extend(iter);
    vec
}

/// Clones a slice into an allocator-backed vector.
pub fn vec_from_slice_in<T, A>(slice: &[T], alloc: A) -> Vec<T, A>
where
    T: Clone,
    A: Allocator,
{
    let mut vec = Vec::with_capacity_in(slice.len(), alloc);
    vec.extend_from_slice(slice);
    vec
}

/// Attempts to allocate a boxed value using the provided allocator.
pub fn try_box_new_in<T, A>(value: T, alloc: A) -> Result<Box<T, A>, AllocError>
where
    A: Allocator,
{
    Box::try_new_in(value, alloc)
}

/// Allocates a boxed value using the provided allocator, panicking on failure.
pub fn box_new_in<T, A>(value: T, alloc: A) -> Box<T, A>
where
    A: Allocator,
{
    match try_box_new_in(value, alloc) {
        Ok(value) => value,
        Err(error) => panic!("failed to allocate box: {error:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn vec_from_iter_uses_allocator() {
        let values = [1, 2, 3];
        let vec = vec_from_iter_in(values, Global);
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn vec_from_slice_copies_elements() {
        let source = [4, 5, 6];
        let vec = vec_from_slice_in(&source, Global);
        assert_eq!(vec.as_slice(), &source);
    }

    #[test]
    fn try_box_new_allocates() {
        let boxed = try_box_new_in(7u32, Global).unwrap();
        assert_eq!(*boxed, 7);
    }
}
