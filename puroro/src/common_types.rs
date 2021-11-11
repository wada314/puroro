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

use crate::bumpalo::Bump;
use crate::internal::impls::bumpalo::BumpaloDefault;
use ::std::fmt::Debug;
use ::std::ops::{Deref, DerefMut};

#[derive(PartialEq, Debug)]
pub struct Merged<T, U>(pub(crate) T, pub(crate) U);
pub fn merge<T, U>(t: T, u: U) -> Merged<T, U> {
    Merged(t, u)
}
impl<T: Clone, U: Clone> Clone for Merged<T, U> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}
impl<T> Merged<T, Box<Bump>>
where
    T: BumpaloDefault<'static>,
{
    pub fn new() -> Self {
        let boxed_bump = Box::new(Bump::new());
        merge(
            T::default_in(unsafe { ::std::mem::transmute(boxed_bump.as_ref()) }),
            boxed_bump,
        )
    }
}

pub struct EmptyMessageWrapper<T>(T);
impl<T: Clone> Clone for EmptyMessageWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: PartialEq> PartialEq for EmptyMessageWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Debug> Debug for EmptyMessageWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EmptyMessageWrapper").field(&self.0).finish()
    }
}
impl<T> Deref for EmptyMessageWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for EmptyMessageWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
