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
use std::marker::Unsize;
use std::ops::CoerceUnsized;

#[derive(Debug)]
pub struct OnceList<T: ?Sized, A: Allocator = Global> {
    head: OnceCell<Box<Cons<T, T, A>, A>>,
    alloc: A,
}

#[derive(Debug)]
struct Cons<T: ?Sized, NexT: ?Sized, A: Allocator> {
    // We cannot use T here because of unsized coercion condition:
    // T can only appear in the last field of the struct.
    // Instead of T, we use NexT here.
    next: OnceCell<Box<Cons<NexT, NexT, A>, A>>,
    val: T,
}

impl<T, NexT, U, A: Allocator> CoerceUnsized<Cons<T, NexT, A>> for Cons<U, NexT, A> where
    U: Unsize<T> + CoerceUnsized<T>
{
}

impl<T: ?Sized> OnceList<T, Global> {
    pub fn new() -> Self {
        Self {
            head: OnceCell::new(),
            alloc: Global,
        }
    }
}
impl<T: ?Sized, A: Allocator> OnceList<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            head: OnceCell::new(),
            alloc,
        }
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
}

impl<T: ?Sized, A: Allocator + Clone> OnceList<T, A> {
    pub fn push_unsized<U: Sized + Unsize<T>>(&self, val: U) -> &U {
        let mut next = &self.head;
        let val_cons = Box::new_in(Cons::<U, T, A>::new(val), self.alloc.clone());
        let result_ptr: *const U = &val_cons.val;
        let mut val_cons = val_cons as Box<Cons<T, T, A>, A>;
        loop {
            match next.try_insert(val_cons) {
                Ok(_) => {
                    // This is safe because the address of the pointee of Box is stable even after the box is moved.
                    unsafe { return &*result_ptr }
                }
                Err((current, new_val_cons)) => {
                    next = &current.next;
                    val_cons = new_val_cons;
                }
            }
        }
    }

    pub fn get_or_push_unsized<P, F, U>(&self, pred_opt: P, f: F) -> &U
    where
        P: Fn(&T) -> Option<&U>,
        F: Fn() -> U,
        U: Sized + Unsize<T>,
    {
        self.get_or_try_push_unsized(pred_opt, || -> Result<_, ()> { Ok(f()) })
            .unwrap()
    }

    pub fn get_or_try_push_unsized<P, F, E, U>(&self, pred_opt: P, f: F) -> Result<&U, E>
    where
        P: Fn(&T) -> Option<&U>,
        F: Fn() -> Result<U, E>,
        U: Sized + Unsize<T>,
    {
        let mut next = &self.head;
        while let Some(c) = next.get() {
            if let Some(u) = pred_opt(&c.val) {
                return Ok(u);
            }
            next = &c.next;
        }
        let new_boxed = Box::new_in(Cons::<U, T, A>::new(f()?), self.alloc.clone());
        let result_ptr: *const U = &new_boxed.val;
        let Ok(_) = next.set(new_boxed as Box<Cons<T, T, A>, A>) else {
            unreachable!("This should not fail because we confirmed that next.get() is None.");
        };
        // This is safe because the address of the pointee of Box is stable even after the box is moved.
        unsafe {
            return Ok(&*result_ptr);
        }
    }
}

impl<T: ?Sized, A: Allocator> OnceList<T, A> {
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

impl<T, NexT: ?Sized, A: Allocator> Cons<T, NexT, A> {
    fn new(val: T) -> Self {
        Self {
            next: OnceCell::new(),
            val,
        }
    }
}
