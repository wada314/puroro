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

use super::{DeserMessageHandlerBase, DeserMessageHandlerForRead};
use crate::Result;
use ::std::marker::PhantomData;

/// An implementation for [DeserMessageHandlerForRead] type.
/// Note this is not an exclusive implementation for [DeserMessageHandlerForRead].
/// Anyone can implement this trait for their own type.

pub struct Handler<'a, R> {
    _phantom: PhantomData<R>,
    stack: Stack<&'a mut dyn Message>,
}

impl<R> DeserMessageHandlerBase for Handler<'_, R> {
    fn parse_variant(&mut self, num: i32, var: crate::variant::Variant) -> Result<()> {
        todo!()
    }

    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
        todo!()
    }

    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
        todo!()
    }

    fn is_message_field(&self, num: i32) -> bool {
        todo!()
    }

    fn start_message(&mut self, num: i32) -> Result<()> {
        self.stack
            .apply_last_mut(|msg| msg.get_or_insert_mut(num))?;
        Ok(())
    }

    fn end_message(&mut self) -> Result<()> {
        todo!()
    }
}

pub trait Message {
    fn get_or_insert_mut(&mut self, field_number: i32) -> Result<Option<&mut dyn Message>>;
}

pub struct Stack<T>(Vec<T>);
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    pub fn last(&self) -> Option<&T> {
        self.0.last()
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }
}
impl<'a, T: ?Sized> Stack<&'a mut T> {
    pub fn apply_last_mut(
        &mut self,
        f: impl FnOnce(&'a mut T) -> Result<Option<&'a mut T>>,
    ) -> Result<()> {
        if let Some(last) = self.0.last_mut() {
            let mut copied_last = ::std::mem::MaybeUninit::<&mut T>::uninit();
            unsafe {
                ::std::ptr::copy_nonoverlapping(last, copied_last.as_mut_ptr(), 1);
            }
            if let Some(new_msg) = f(unsafe { copied_last.assume_init() })? {
                self.0.push(new_msg);
            }
        }
        Ok(())
    }
}
