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

use ::std::alloc::Allocator;
use ::std::cell::OnceCell;

#[derive(Debug)]
pub struct OnceList<T, A: Allocator> {
    head: OnceCell<Box<Cons<T, A>, A>>,
    alloc: A,
}

#[derive(Debug)]
struct Cons<T, A: Allocator> {
    next: OnceCell<Box<Cons<T, A>, A>>,
    val: T,
}

impl<T, A: Allocator> OnceList<T, A> {
    pub fn new(alloc: A) -> Self {
        Self {
            head: OnceCell::new(),
            alloc,
        }
    }
}
impl<T, A: Allocator + Clone> OnceList<T, A> {
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
        let Ok(_) = next.set(Box::new_in(Cons::new(f()?), self.alloc.clone())) else {
            unreachable!("This should not fail because we confirmed that next.get() is None.");
        };
        let Some(last_cons) = next.get() else {
            unreachable!("This should not fail because we set the cell content at the line above.");
        };
        Ok(&last_cons.val)
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
}

impl<T, A: Allocator> Cons<T, A> {
    fn new(val: T) -> Self {
        Self {
            next: OnceCell::new(),
            val,
        }
    }
}
