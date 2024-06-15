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
use crate::variant::Variant;
use crate::Result;
use ::std::io::Read;
use ::std::marker::PhantomData;

/// An implementation for [DeserMessageHandlerForRead] type.
/// Note this is not an exclusive implementation for [DeserMessageHandlerForRead].
/// Anyone can implement this trait for their own type.

pub struct Handler<'a, R> {
    _phantom: PhantomData<R>,
    stack: Stack<&'a mut dyn Message>,
}

impl<R> DeserMessageHandlerBase for Handler<'_, R> {
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
        self.stack.assume_last_mut()?.set_variant_field(num, var)
    }
    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
        self.stack.assume_last_mut()?.set_bytes4_field(num, val)
    }
    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
        self.stack.assume_last_mut()?.set_bytes8_field(num, val)
    }
    fn is_message_field(&self, num: i32) -> bool {
        self.stack
            .last()
            .map_or(false, |msg| msg.is_message_field(num))
    }
    fn start_message(&mut self, num: i32) -> Result<()> {
        self.stack
            .apply_last_mut(|msg| msg.get_or_insert_message(num))?;
        Ok(())
    }
    fn end_message(&mut self) -> Result<()> {
        self.stack
            .pop()
            .ok_or_else(|| "deser stack underflow".to_string())?;
        Ok(())
    }
}
impl<R: Read> DeserMessageHandlerForRead<R> for Handler<'_, R> {
    fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
        self.stack
            .assume_last_mut()?
            .set_len_field_from_read(num, read)
    }
}

pub trait Message {
    fn set_variant_field(&mut self, field_number: i32, value: Variant) -> Result<()>;
    fn set_bytes4_field(&mut self, field_number: i32, value: [u8; 4]) -> Result<()>;
    fn set_bytes8_field(&mut self, field_number: i32, value: [u8; 8]) -> Result<()>;
    fn set_len_field_from_read(&mut self, field_number: i32, read: &mut dyn Read) -> Result<usize>;
    fn is_message_field(&self, field_number: i32) -> bool;
    fn get_or_insert_message(&mut self, field_number: i32) -> Result<Option<&mut dyn Message>>;
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
    pub fn assume_last(&self) -> Result<&T> {
        Ok(self
            .0
            .last()
            .ok_or_else(|| "deser stack underflow".to_string())?)
    }
    pub fn assume_last_mut(&mut self) -> Result<&mut T> {
        Ok(self
            .0
            .last_mut()
            .ok_or_else(|| "deser stack underflow".to_string())?)
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
