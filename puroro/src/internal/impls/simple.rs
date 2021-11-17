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

pub mod de;

use ::std::borrow::Borrow;
use ::std::marker::PhantomData;

pub struct BorrowedIter<B: ?Sized, I>(I, PhantomData<B>);
impl<B: ?Sized, I> BorrowedIter<B, I> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'a, B, I, T> Iterator for BorrowedIter<B, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Borrow<B>,
    B: 'a + ?Sized,
{
    type Item = &'a B;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|owned| owned.borrow())
    }
}

pub trait VecOrOptionOrBare<T> {
    fn push(&mut self, val: T);
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T;
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}
impl<T> VecOrOptionOrBare<T> for Option<T> {
    fn push(&mut self, val: T) {
        *self = Some(val);
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Option<T>>::get_or_insert_with(self, f)
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::option::Iter<'a, T>;
    fn iter(&self) -> Self::Iter<'_> {
        Option::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for Vec<T> {
    fn push(&mut self, val: T) {
        self.push(val);
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Vec<T>>::push(self, (f)());
        self.last_mut().unwrap()
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::slice::Iter<'a, T>;
    fn iter(&self) -> <Self as VecOrOptionOrBare<T>>::Iter<'_> {
        <[T]>::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for T {
    fn push(&mut self, val: T) {
        *self = val;
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        self
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::iter::Once<&'a T>;
    fn iter(&self) -> Self::Iter<'_> {
        ::std::iter::once(self)
    }
}
