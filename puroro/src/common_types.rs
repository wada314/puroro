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

use ::std::iter;
use ::std::marker::PhantomData;
use ::std::slice;

/// Cloned slice, which can be accessed / iterated over `T` instead of `&T`.
pub struct ClonedSlice<'slice, T>(&'slice [T]);
impl<'slice, T> ClonedSlice<'slice, T> {
    pub fn new(slice: &'slice [T]) -> Self {
        Self(slice)
    }
}
impl<'slice, T: Clone> IntoIterator for ClonedSlice<'slice, T> {
    type Item = T;
    type IntoIter = iter::Cloned<std::slice::Iter<'slice, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}

pub struct AsRefSlice<'slice, T, U: ?Sized>(&'slice [T], PhantomData<U>);
impl<'slice, T, U: ?Sized> AsRefSlice<'slice, T, U> {
    pub fn new(slice: &'slice [T]) -> Self {
        Self(slice, PhantomData)
    }
}
impl<'slice, T: AsRef<U>, U: 'slice + ?Sized> IntoIterator for AsRefSlice<'slice, T, U> {
    type Item = &'slice U;
    type IntoIter = AsRefIter<'slice, slice::Iter<'slice, T>, U>;
    fn into_iter(self) -> Self::IntoIter {
        AsRefIter::new(self.0.iter())
    }
}

pub struct AsRefIter<'slice, I, T: ?Sized>(I, PhantomData<&'slice T>);
impl<'slice, I, T: ?Sized> AsRefIter<'slice, I, T> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'slice, I, T, U> Iterator for AsRefIter<'slice, I, T>
where
    I: Iterator<Item = &'slice U>,
    U: 'slice + AsRef<T>,
    T: ?Sized,
{
    type Item = &'slice T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.as_ref())
    }
}
