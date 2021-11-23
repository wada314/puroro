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
use ::std::borrow::Borrow;
use ::std::mem;
use ::std::mem::ManuallyDrop;
use ::std::ops::{Deref, DerefMut};
use ::std::ptr;
use ::std::ptr::NonNull;
use ::std::str::Utf8Error;

pub trait BumpDefault<'bump> {
    fn default_in(bump: &'bump Bump) -> Self;
}
impl<'bump> BumpDefault<'bump> for NoAllocString {
    fn default_in(bump: &'bump Bump) -> Self {
        NoAllocString::new_in(bump)
    }
}
impl<'bump, T> BumpDefault<'bump> for NoAllocVec<T> {
    fn default_in(bump: &'bump Bump) -> Self {
        NoAllocVec::new_in(bump)
    }
}
impl<'bump, T> BumpDefault<'bump> for Option<T> {
    fn default_in(_: &'bump Bump) -> Self {
        ::std::default::Default::default()
    }
}
macro_rules! impl_bumpalo_default {
    ($ty:ty) => {
        impl<'bump> BumpDefault<'bump> for $ty {
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

pub struct NoAllocBox<T>(NonNull<T>);
impl<T> NoAllocBox<T> {
    pub fn new_in(x: T, bump: &Bump) -> Self {
        Self(unsafe { NonNull::new_unchecked(bump.alloc(x)) })
    }
}
impl<T> Deref for NoAllocBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T> DerefMut for NoAllocBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T> Borrow<T> for NoAllocBox<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}
impl<T> AsRef<T> for NoAllocBox<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}
impl<T> Drop for NoAllocBox<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.0.as_ptr()) }
    }
}

pub struct NoAllocVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}
impl<T> NoAllocVec<T> {
    pub fn new_in(bump: &Bump) -> Self {
        let vec = Vec::new_in(bump);
        unsafe { Self::from_vec(vec) }
    }
    pub unsafe fn from_vec<'bump>(mut vec: Vec<'bump, T>) -> Self {
        let result = Self {
            ptr: vec.as_mut_ptr(),
            length: vec.len(),
            capacity: vec.capacity(),
        };
        mem::forget(vec);
        result
    }

    /// Construct an immutable [`Vec`](bumpalo::collections::Vec) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_vec_in<'bump>(&self, bump: &'bump Bump) -> Vec<'bump, T> {
        Vec::from_raw_parts_in(self.ptr, self.length, self.capacity, bump)
    }

    /// Construct a mutable [`Vec`](bumpalo::collections::Vec) wrapped by [`MutRefVec`].
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_vec_mut_in<'bump>(&mut self, bump: &'bump Bump) -> MutRefVec<'bump, '_, T> {
        MutRefVec {
            temp_vec: ManuallyDrop::new(self.as_vec_in(bump)),
            ref_vec: self,
        }
    }

    /// Drop. This type needs to know about itself's allocating bump ptr.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn drop_in(self, bump: &Bump) {
        // Construct a Vec and let that handle the drop functions.
        Vec::from_raw_parts_in(self.ptr, self.length, self.capacity, bump);
    }
}
impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.ptr, self.length) }
    }
}
impl<T> DerefMut for NoAllocVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { ::std::slice::from_raw_parts_mut(self.ptr, self.length) }
    }
}
impl<T> Borrow<[T]> for NoAllocVec<T> {
    fn borrow(&self) -> &[T] {
        self.deref()
    }
}
impl<T> AsRef<[T]> for NoAllocVec<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}
impl<T> Drop for NoAllocVec<T> {
    fn drop(&mut self) {
        unreachable!("This type must be manually dropped!! Call drop_in() instead.")
    }
}

pub struct MutRefVec<'bump, 'vec, T> {
    temp_vec: ManuallyDrop<Vec<'bump, T>>,
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
        unsafe {
            *self.ref_vec = NoAllocVec::from_vec(ManuallyDrop::take(&mut self.temp_vec));
        }
    }
}

pub struct NoAllocString {
    vec: NoAllocVec<u8>,
}
impl NoAllocString {
    pub fn new_in(bump: &Bump) -> Self {
        Self {
            vec: NoAllocVec::new_in(bump),
        }
    }

    pub fn from_utf8(vec: NoAllocVec<u8>) -> ::std::result::Result<Self, Utf8Error> {
        if let Err(error) = ::std::str::from_utf8(&vec) {
            Err(error)
        } else {
            Ok(Self { vec })
        }
    }

    pub fn from_utf8_unchecked(vec: NoAllocVec<u8>) -> Self {
        Self { vec }
    }

    /// Construct an immutable [`String`](bumpalo::collections::String) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_string_in<'bump>(&self, bump: &'bump Bump) -> String<'bump> {
        String::from_utf8_unchecked(self.vec.as_vec_in(bump))
    }

    /// Construct a mutable [`String`](bumpalo::collections::String) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_string_mut_in<'bump, 'string>(
        &'string mut self,
        bump: &'bump Bump,
    ) -> MutRefString<'bump, 'string> {
        MutRefString {
            temp_string: ManuallyDrop::new(self.as_string_in(bump)),
            ref_string: self,
        }
    }

    /// Drop. This type needs to know about itself's allocating bump ptr.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn drop_in(self, bump: &Bump) {
        self.vec.drop_in(bump);
    }
}

impl Deref for NoAllocString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::str::from_utf8_unchecked(&self.vec) }
    }
}
impl Borrow<str> for NoAllocString {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
impl AsRef<str> for NoAllocString {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

pub struct MutRefString<'bump, 'string> {
    temp_string: ManuallyDrop<String<'bump>>,
    ref_string: &'string mut NoAllocString,
}

impl<'bump, 'string> Deref for MutRefString<'bump, 'string> {
    type Target = String<'bump>;
    fn deref(&self) -> &Self::Target {
        &self.temp_string
    }
}
impl<'bump, 'string> DerefMut for MutRefString<'bump, 'string> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.temp_string
    }
}
impl<'bump, 'string> Drop for MutRefString<'bump, 'string> {
    fn drop(&mut self) {
        unsafe {
            let vec = ManuallyDrop::take(&mut self.temp_string).into_bytes();
            self.ref_string.vec = NoAllocVec::from_vec(vec);
        }
    }
}
