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

pub struct OnceList<T, A: Allocator> {
    head: OnceCell<Box<Cons<T, A>, A>>,
    alloc: A,
}

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
}

impl<T, A: Allocator> Cons<T, A> {
    fn new(val: T) -> Self {
        Self {
            next: OnceCell::new(),
            val,
        }
    }
}
