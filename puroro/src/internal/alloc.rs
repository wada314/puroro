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

use ::std::mem::forget;
use ::std::ops::{Deref, DerefMut};

pub(crate) struct NoAllocVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}
impl<T> From<Vec<T>> for NoAllocVec<T> {
    fn from(mut value: Vec<T>) -> Self {
        let ret = NoAllocVec {
            ptr: value.as_mut_ptr(),
            length: value.len(),
            capacity: value.capacity(),
        };
        forget(value);
        ret
    }
}
impl<T> From<NoAllocVec<T>> for Vec<T> {
    fn from(value: NoAllocVec<T>) -> Self {
        unsafe { Vec::from_raw_parts(value.ptr, value.length, value.capacity) }
    }
}
impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.length) }
    }
}
impl<'a, T> IntoIterator for &'a NoAllocVec<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.deref().into_iter()
    }
}

pub struct RefMutVec<'a, T> {
    tmp_vec: Vec<T>,
    src_vec: &'a mut NoAllocVec<T>,
}
impl<'a, T> Deref for RefMutVec<'a, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.tmp_vec
    }
}
impl<'a, T> DerefMut for RefMutVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tmp_vec
    }
}
