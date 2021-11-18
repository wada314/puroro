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

//! Data structures using bumpalo. Tentative name.
//! The data body and the reference to the `Bump` object must be separatable.

use bumpalo::Bump;
use core::mem;
use core::ptr::NonNull;
use std::ops::{Deref, DerefMut};

pub struct Box<T: ?Sized>(NonNull<T>);
impl<T> Box<T> {
    pub fn new_in<B: Deref<Target = Bump>>(val: T, bump: B) -> Self {
        Self(unsafe { NonNull::new_unchecked(bump.alloc(val)) })
    }
}
impl<T: ?Sized> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.0.as_mut());
        }
    }
}

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}
impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(mem::align_of::<T>() as *mut T) },
            cap: if mem::size_of::<T>() == 0 { 0 } else { !0 },
            len: 0,
        }
    }
}
