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

use std::alloc::{Allocator, Global};
use std::cell::OnceCell;

#[derive(Debug)]
pub struct OnceList<T, A: Allocator = Global> {
    head: OnceCell<Box<Cons<T, A>, A>>,
    alloc: A,
}

#[derive(Debug)]
struct Cons<T, A: Allocator> {
    next: OnceCell<Box<Cons<T, A>, A>>,
    val: T,
}

impl<T> OnceList<T, Global> {
    pub fn new() -> Self {
        Self {
            head: OnceCell::new(),
            alloc: Global,
        }
    }
}
impl<T, A: Allocator> OnceList<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            head: OnceCell::new(),
            alloc,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut next = &self.head;
        ::std::iter::from_fn(move || match next.get() {
            Some(c) => {
                next = &c.next;
                Some(&c.val)
            }
            None => None,
        })
    }

    pub fn clear(&mut self) {
        self.head = OnceCell::new();
    }

    pub fn alloc(&self) -> &A {
        &self.alloc
    }
}

impl<T: Sized, A: Allocator + Clone> OnceList<T, A> {
    pub fn push(&self, val: T) -> &T {
        let mut next = &self.head;
        let mut val_cons = Box::new_in(Cons::new(val), self.alloc.clone());
        loop {
            match next.try_insert(val_cons) {
                Ok(c) => return &c.val,
                Err((inner, new_val_cons)) => {
                    next = &inner.next;
                    val_cons = new_val_cons;
                }
            }
        }
    }

    pub fn get_or_push<P, F>(&self, pred: P, f: F) -> &T
    where
        P: Fn(&T) -> bool,
        F: Fn() -> T,
    {
        self.get_or_try_push(pred, || -> Result<_, ()> { Ok(f()) })
            .unwrap()
    }

    pub fn get_or_try_push<P, F, E>(&self, pred: P, f: F) -> Result<&T, E>
    where
        P: Fn(&T) -> bool,
        F: Fn() -> Result<T, E>,
    {
        let mut next = &self.head;
        while let Some(c) = next.get() {
            if pred(&c.val) {
                return Ok(&c.val);
            }
            next = &c.next;
        }
        let Ok(last_cons) = next.try_insert(Box::new_in(Cons::new(f()?), self.alloc.clone()))
        else {
            unreachable!("This should not fail because we confirmed that next.get() is None.");
        };
        Ok(&last_cons.val)
    }

    pub fn take<P>(&mut self, pred: P) -> Option<T>
    where
        P: Fn(&T) -> bool,
    {
        let mut next = &mut self.head;
        while let Some(mut taken_next) = next.take() {
            if pred(&taken_next.val) {
                // reconnect the list
                if let Some(next_next) = taken_next.next.take() {
                    next.set(next_next);
                }
                return Some(taken_next.val);
            }

            next.set(taken_next);
            next = &mut unsafe { next.get_mut().unwrap_unchecked() }.next;
        }
        None
    }

    pub fn take_map<F, U>(&mut self, f: F) -> Option<U>
    where
        F: Fn(T) -> Result<U, T>,
    {
        let mut next = &mut self.head;
        while let Some(mut taken_next) = next.take() {
            match f(taken_next.val) {
                Ok(u) => {
                    // reconnect the list
                    if let Some(next_next) = taken_next.next.take() {
                        next.set(next_next);
                    }
                    return Some(u);
                }
                Err(t) => {
                    taken_next.val = t;
                    next.set(taken_next);
                    next = &mut unsafe { next.get_mut().unwrap_unchecked() }.next;
                }
            }
        }
        None
    }
}

impl<T, A: Allocator> Cons<T, A> {
    fn new(val: T) -> Self {
        Self {
            next: OnceCell::new(),
            val,
        }
    }
}
