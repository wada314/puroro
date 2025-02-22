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

use once_list2::OnceList;
use std::alloc::Allocator;
use std::fmt::Debug;
use std::iter;

#[derive(Clone)]
pub struct OnceList1<T, A: Allocator>(pub(crate) T, pub(crate) OnceList<T, A>);

impl<T, A: Allocator> OnceList1<T, A> {
    pub fn new_in(first: T, alloc: A) -> Self {
        Self(first, OnceList::new_in(alloc))
    }
    pub fn first(&self) -> &T {
        &self.0
    }
    pub fn last(&self) -> &T {
        match self.1.last() {
            Some(last) => last,
            None => self.first(),
        }
    }
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn last_mut(&mut self) -> &mut T {
        match self.1.last_mut() {
            Some(last) => last,
            None => &mut self.0,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.0).chain(self.1.iter())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        iter::once(&mut self.0).chain(self.1.iter_mut())
    }
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        iter::once(self.0).chain(self.1.into_iter())
    }
    pub fn reduce<'a, F: FnMut(&'a T, &'a T) -> &'a T>(&'a self, mut f: F) -> &'a T {
        let mut result = &self.0;
        for item in self.1.iter() {
            result = f(result, item);
        }
        result
    }
    pub fn allocator(&self) -> &A {
        self.1.allocator()
    }
}

impl<T, A: Allocator + Clone> OnceList1<T, A> {
    pub fn extend(&self, other: impl IntoIterator<Item = T>) {
        self.1.extend(other);
    }

    /// Removes an element that matches the given predicate.
    /// If an element matches, returns Ok with the removed element and the remaining elements as OnceList.
    /// If no elements match, returns Err with the original list.
    pub fn remove<F: Fn(&T) -> bool>(self, f: F) -> Result<(T, OnceList<T, A>), OnceList1<T, A>> {
        if f(&self.0) {
            let OnceList1(first, rest) = self;
            Ok((first, rest))
        } else {
            // Try to remove from rest of list
            let OnceList1(first, mut rest) = self;
            match rest.remove(&f) {
                Some(removed) => {
                    let new_rest = OnceList::new_in(rest.allocator().clone());
                    new_rest.push(first);
                    new_rest.extend(rest.into_iter());
                    Ok((removed, new_rest))
                }
                None => Err(OnceList1(first, rest)),
            }
        }
    }
}

impl<T, A: Allocator + Clone> OnceList1<T, A> {
    pub fn push(&self, value: T) -> &T {
        self.1.push(value)
    }
    pub fn take_some(mut self, pred: impl Fn(&T) -> bool) -> Option<T> {
        if pred(&self.0) {
            Some(self.0)
        } else {
            self.1.remove(pred)
        }
    }
}

impl<T: Debug, A: Allocator> Debug for OnceList1<T, A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_list()
            .entry(&self.0)
            .entries(self.1.iter())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Global;

    #[test]
    fn test_once_list1_basic() {
        let list = OnceList1::new_in(1, Global);
        assert_eq!(*list.first(), 1);

        let items: Vec<_> = list.iter().copied().collect();
        assert_eq!(items, vec![1]);
    }

    #[test]
    fn test_once_list1_push() {
        let list = OnceList1::new_in(1, Global);
        list.push(2);
        list.push(3);

        let items: Vec<_> = list.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_once_list1_debug() {
        let list = OnceList1::new_in(1, Global);
        list.push(2);

        assert_eq!(format!("{:?}", list), "[1, 2]");
    }
}
