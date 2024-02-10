// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.num (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.num
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::ops::{Deref, DerefMut};
use ::std::result::Result;

#[derive(PartialEq, Eq)]
pub(crate) struct UnfrozenMut<'a, T: ?Sized> {
    ptr: *mut T,
    phantom: PhantomData<&'a ()>,
}
pub(crate) struct FrozenMut<'a, T: ?Sized> {
    ptr: *mut T,
    #[cfg(debug_assertions)]
    child: *mut T,
    phantom: PhantomData<&'a ()>,
}
pub(crate) enum FreezeStatus<'a, T: ?Sized> {
    Unfrozen(UnfrozenMut<'a, T>),
    Frozen(FrozenMut<'a, T>, UnfrozenMut<'a, T>),
}

#[allow(unused)]
impl<'a, T: 'a + ?Sized> UnfrozenMut<'a, T> {
    pub(crate) fn new(val: &'a mut T) -> Self {
        Self {
            ptr: val,
            phantom: PhantomData,
        }
    }
    pub(crate) fn work<F: FnOnce(&'a mut T) -> Option<&'a mut T>>(
        self,
        f: F,
    ) -> FreezeStatus<'a, T> {
        self.try_work::<(), _>(move |ptr| Ok((f)(ptr))).unwrap()
    }
    pub(crate) fn try_work<E, F: FnOnce(&'a mut T) -> Result<Option<&'a mut T>, E>>(
        self,
        f: F,
    ) -> Result<FreezeStatus<'a, T>, E> {
        if let Some(child) = (f)(unsafe { transmute(self.ptr) })? {
            let frozen_self = FrozenMut {
                ptr: self.ptr,
                child,
                phantom: PhantomData,
            };
            let child_chain = UnfrozenMut::new(child);
            Ok(FreezeStatus::Frozen(frozen_self, child_chain))
        } else {
            Ok(FreezeStatus::Unfrozen(self))
        }
    }
}
impl<'a, T: ?Sized> Deref for UnfrozenMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.ptr) }
    }
}
impl<'a, T: ?Sized> DerefMut for UnfrozenMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.ptr) }
    }
}

impl<'a, T: ?Sized> FrozenMut<'a, T> {
    pub(crate) fn unfreeze(self, child: UnfrozenMut<'a, T>) -> UnfrozenMut<'a, T> {
        debug_assert_eq!(self.child, child.ptr);
        UnfrozenMut {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}
