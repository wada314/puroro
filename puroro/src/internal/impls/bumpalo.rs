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

use crate::bumpalo::boxed::Box;
use crate::bumpalo::collections::{String, Vec};
use crate::bumpalo::Bump;
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
impl<'bump, T> VecOrOptionOrBare<T> for Vec<'bump, T> {
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
        Self: 'a,
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
impl<'bump, T> BumpaloDefault<'bump> for Box<'bump, T>
where
    T: BumpaloDefault<'bump>,
{
    fn default_in(bump: &'bump Bump) -> Self {
        Box::new_in(BumpaloDefault::default_in(bump), bump)
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

pub trait BumpaloClone<'bump> {
    fn clone_in(&self, bump: &'bump Bump) -> Self;
}
impl<'bump> BumpaloClone<'bump> for String<'bump> {
    fn clone_in(&self, _: &'bump Bump) -> Self {
        self.clone()
    }
}
impl<'bump, T> BumpaloClone<'bump> for Vec<'bump, T>
where
    T: BumpaloClone<'bump>,
{
    fn clone_in(&self, bump: &'bump Bump) -> Self {
        Vec::from_iter_in(<[T]>::iter(self).map(|t| t.clone_in(bump)), bump)
    }
}
impl<'bump, T> BumpaloClone<'bump> for Box<'bump, T>
where
    T: BumpaloClone<'bump>,
{
    fn clone_in(&self, bump: &'bump Bump) -> Self {
        Box::new_in(self.as_ref().clone_in(bump), bump)
    }
}
impl<'bump, T> BumpaloClone<'bump> for Option<T>
where
    T: BumpaloClone<'bump>,
{
    fn clone_in(&self, bump: &'bump Bump) -> Self {
        self.as_ref().map(|v| BumpaloClone::clone_in(v, bump))
    }
}
macro_rules! impl_bumpalo_clone {
    ($ty:ty) => {
        impl<'bump> BumpaloClone<'bump> for $ty {
            fn clone_in(&self, _: &'bump Bump) -> Self {
                Clone::clone(self)
            }
        }
    };
}
impl_bumpalo_clone!(i32);
impl_bumpalo_clone!(u32);
impl_bumpalo_clone!(f32);
impl_bumpalo_clone!(i64);
impl_bumpalo_clone!(u64);
impl_bumpalo_clone!(f64);
impl_bumpalo_clone!(bool);
impl_bumpalo_clone!(u8);
