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

use crate::{ErrorKind, Result};
use ::std::mem;
use ::std::rc::{Rc, Weak};

pub trait SliceExt<T> {
    fn split_while<F>(&self, pred: F) -> (&[T], &[T])
    where
        F: FnMut(&T) -> bool;

    fn group_by_key<F, K>(&self, pred: F) -> GroupByKey<'_, T, F, K>
    where
        F: Fn(&T) -> K,
        K: PartialEq;
}

impl<T> SliceExt<T> for [T] {
    fn split_while<F>(&self, mut pred: F) -> (&[T], &[T])
    where
        F: FnMut(&T) -> bool,
    {
        let mut i = 0;
        while i < self.len() {
            if !pred(&self[i]) {
                break;
            }
            i = i + 1;
        }
        self.split_at(i)
    }

    fn group_by_key<F, K>(&self, pred: F) -> GroupByKey<'_, T, F, K>
    where
        F: Fn(&T) -> K,
        K: PartialEq,
    {
        GroupByKey::new(self, pred)
    }
}

pub struct GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    v: &'a [T],
    pred: F,
    finished: bool,
}

impl<'a, T, F, K> GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    pub fn new(slice: &'a [T], pred: F) -> Self {
        Self {
            v: slice,
            pred,
            finished: false,
        }
    }
}

impl<'a, T, F, K> Iterator for GroupByKey<'a, T, F, K>
where
    F: Fn(&T) -> K,
    K: PartialEq,
{
    type Item = (K, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Some((first, rest)) = self.v.split_first() {
            let key = (self.pred)(first);
            if let Some(pos) = rest.iter().position(|x| (self.pred)(x) != key) {
                let ret = Some((key, &self.v[..pos + 1]));
                self.v = &self.v[pos + 1..];
                ret
            } else {
                self.finished = true;
                Some((key, self.v))
            }
        } else {
            self.finished = true;
            None
        }
    }
}

pub trait RcExt<T> {
    fn try_new_cyclic<F, E>(data_fn: F) -> ::std::result::Result<Rc<T>, E>
    where
        T: Default,
        F: FnOnce(&Weak<T>) -> ::std::result::Result<T, E>,
    {
        Self::try_new_cyclic_with_default(data_fn, T::default)
    }
    fn try_new_cyclic_with_default<F, D, E>(
        data_fn: F,
        default: D,
    ) -> ::std::result::Result<Rc<T>, E>
    where
        F: FnOnce(&Weak<T>) -> ::std::result::Result<T, E>,
        D: FnOnce() -> T;
}

impl<T> RcExt<T> for Rc<T> {
    fn try_new_cyclic_with_default<F, D, E>(
        data_fn: F,
        default: D,
    ) -> ::std::result::Result<Rc<T>, E>
    where
        F: FnOnce(&Weak<T>) -> ::std::result::Result<T, E>,
        D: FnOnce() -> T,
    {
        let mut maybe_error = None;
        let rc = Rc::new_cyclic(|weak| match (data_fn)(weak) {
            Ok(data) => data,
            Err(err) => {
                maybe_error = Some(err);
                (default)()
            }
        });
        match maybe_error {
            Some(err) => Err(err),
            None => Ok(rc),
        }
    }
}

pub trait RcBoxExt<T: ?Sized> {
    fn try_new_boxed_cyclic<F, E>(data_fn: F) -> ::std::result::Result<Rc<Box<T>>, E>
    where
        F: FnOnce(&Weak<Box<T>>) -> ::std::result::Result<Box<T>, E>;
}
impl<T: ?Sized> RcBoxExt<T> for Rc<Box<T>> {
    fn try_new_boxed_cyclic<F, E>(data_fn: F) -> ::std::result::Result<Rc<Box<T>>, E>
    where
        F: FnOnce(&Weak<Box<T>>) -> ::std::result::Result<Box<T>, E>,
    {
        let mut maybe_error = None;
        let rc_opt = Rc::new_cyclic(|weak_opt| {
            let weak: Weak<Box<T>> = unsafe {
                Weak::from_raw(mem::transmute::<*const Option<Box<T>>, *const Box<T>>(
                    Weak::clone(weak_opt).into_raw(),
                ))
            };
            match (data_fn)(&weak) {
                Ok(data) => Some(data),
                Err(err) => {
                    maybe_error = Some(err);
                    None
                }
            }
        });
        match maybe_error {
            Some(err) => Err(err),
            None => Ok(unsafe {
                Rc::from_raw(mem::transmute::<*const Option<Box<T>>, *const Box<T>>(
                    Rc::into_raw(rc_opt),
                ))
            }),
        }
    }
}

pub trait WeakExt<T> {
    fn try_upgrade(&self) -> Result<Rc<T>>;
}
impl<T> WeakExt<T> for Weak<T> {
    fn try_upgrade(&self) -> Result<Rc<T>> {
        Ok(Weak::upgrade(self).ok_or(ErrorKind::InternalError {
            detail: "Weak ptr upgrade failed".to_string(),
        })?)
    }
}
