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

use crate::Result;
use ::allocator_api2::alloc::Allocator;
use ::cached_pair::Pair;
use ::once_list2::OnceList;
use ::std::iter;

#[derive(Clone)]
pub struct OnceList1<T, A: Allocator>(T, OnceList<T, A>);
impl<T, A: Allocator> OnceList1<T, A> {
    pub fn new_in(first: T, alloc: A) -> Self {
        Self(first, OnceList::new_in(alloc))
    }
    pub fn first(&self) -> &T {
        &self.0
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.0).chain(self.1.iter())
    }
    pub fn allocator(&self) -> &A {
        self.1.allocator()
    }
}
impl<T, A: Allocator + Clone> OnceList1<T, A> {
    pub fn push(&self, value: T) -> &T {
        self.1.push(value)
    }
}

impl<T: ::std::fmt::Debug, A: Allocator> ::std::fmt::Debug for OnceList1<T, A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_list()
            .entry(&self.0)
            .entries(self.1.iter())
            .finish()
    }
}

pub trait PairWithOnceList1Ext<L, T, A> {
    fn try_get_or_insert_into_right<F: Fn(&T) -> bool, G: FnOnce() -> T>(
        &self,
        pred: F,
        default: G,
        alloc: &A,
    ) -> Result<&T>;
}
impl<L, T, A: Allocator + Clone> PairWithOnceList1Ext<L, T, A> for Pair<L, OnceList1<T, A>> {
    fn try_get_or_insert_into_right<F: Fn(&T) -> bool, G: FnOnce() -> T>(
        &self,
        pred: F,
        default: G,
        alloc: &A,
    ) -> Result<&T> {
        // Search for the value in the list.
        if let Some(list) = self.right_opt() {
            if let Some(item) = list.iter().find(move |x| pred(*x)) {
                return Ok(item);
            }
        }
        // No cached value found. We need to create a new value.
        let mut value_opt = Some(default());
        let list_ref =
            self.right_with(|_| OnceList1::new_in(value_opt.take().unwrap(), A::clone(&alloc)));
        Ok(if let Some(value) = value_opt {
            list_ref.push(value)
        } else {
            list_ref.first()
        })
    }
}
