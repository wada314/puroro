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

//! # Bumpalo message structs
//!
//! **The implementation is highly experimental and the interface will change
//! in very soon. I'm planning to make the struct fields private and make a mutable
//! trait interface in future.**
//!

pub mod de;

use crate::bumpalo::collections::{String, Vec};
use crate::bumpalo::Bump;
use ::std::mem;
use ::std::ops::{Deref, DerefMut};

pub trait BumpaloDefault<'bump> {
    fn default_in(bump: &'bump Bump) -> Self;
}
impl<'bump> BumpaloDefault<'bump> for String<'bump> {
    fn default_in(bump: &'bump Bump) -> Self {
        String::new_in(bump)
    }
}
impl<'bump, T> BumpaloDefault<'bump> for Vec<'bump, T> {
    fn default_in(bump: &'bump Bump) -> Self {
        Vec::new_in(bump)
    }
}
impl<'bump, T> BumpaloDefault<'bump> for Option<T> {
    fn default_in(_: &'bump Bump) -> Self {
        ::std::default::Default::default()
    }
}
macro_rules! impl_bumpalo_default {
    ($ty:ty) => {
        impl<'bump> BumpaloDefault<'bump> for $ty {
            fn default_in(_: &'bump Bump) -> Self {
                Default::default()
            }
        }
    };
}
impl_bumpalo_default!(i32);
impl_bumpalo_default!(u32);
impl_bumpalo_default!(f32);
impl_bumpalo_default!(i64);
impl_bumpalo_default!(u64);
impl_bumpalo_default!(f64);
impl_bumpalo_default!(bool);

pub struct NoAllocVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}
impl<T> NoAllocVec<T> {
    pub fn new_in(bump: &Bump) -> Self {
        let mut vec = Vec::new_in(bump);
        let result = Self {
            ptr: vec.as_mut_ptr(),
            length: vec.len(),
            capacity: vec.capacity(),
        };
        mem::forget(vec);
        result
    }

    /// Construct an immutable `Vec` by adding bump info.
    /// This function must take a same bump ref with the one given in [`new_in`] method.
    pub unsafe fn as_vec_in<'bump>(&self, bump: &'bump Bump) -> Vec<'bump, T> {
        Vec::from_raw_parts_in(self.ptr, self.length, self.capacity, bump)
    }

    /// Construct a mutable `Vec` wrapped by [`MutRef`].
    /// This function must take a same bump ref with the one given in [`new_in`] method.
    pub unsafe fn as_vec_mut_in<'bump>(&mut self, bump: &'bump Bump) -> MutRefVec<'bump, '_, T> {
        MutRefVec {
            temp_vec: Vec::from_raw_parts_in(self.ptr, self.length, self.capacity, bump),
            ref_vec: self,
        }
    }
}

pub struct MutRefVec<'bump, 'vec, T> {
    temp_vec: Vec<'bump, T>,
    ref_vec: &'vec mut NoAllocVec<T>,
}

impl<'bump, 'vec, T: 'bump> Deref for MutRefVec<'bump, 'vec, T> {
    type Target = Vec<'bump, T>;
    fn deref(&self) -> &Self::Target {
        &self.temp_vec
    }
}
impl<'bump, 'vec, T: 'bump> DerefMut for MutRefVec<'bump, 'vec, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.temp_vec
    }
}
impl<'bump, 'vec, T> Drop for MutRefVec<'bump, 'vec, T> {
    fn drop(&mut self) {
        self.ref_vec.ptr = self.temp_vec.as_mut_ptr();
        self.ref_vec.length = self.temp_vec.len();
        self.ref_vec.capacity = self.temp_vec.capacity();
    }
}
