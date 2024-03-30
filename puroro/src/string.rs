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

//! A [::std::string::String] replacement which supports an allocator.

use ::std::alloc::{Allocator, Global};
use ::std::borrow::{Borrow, BorrowMut};
use ::std::ops::{Deref, DerefMut};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct String<A: Allocator = Global> {
    vec: Vec<u8, A>,
}

impl<A: Allocator> String<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            vec: Vec::new_in(alloc),
        }
    }
}
impl String<Global> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
}

impl<A: Allocator> AsRef<str> for String<A> {
    fn as_ref(&self) -> &str {
        unsafe { ::std::str::from_utf8_unchecked(&self.vec) }
    }
}
impl<A: Allocator> AsMut<str> for String<A> {
    fn as_mut(&mut self) -> &mut str {
        unsafe { ::std::str::from_utf8_unchecked_mut(&mut self.vec) }
    }
}
impl<A: Allocator> Borrow<str> for String<A> {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}
impl<A: Allocator> BorrowMut<str> for String<A> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut()
    }
}
impl<A: Allocator> Deref for String<A> {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}
impl<A: Allocator> DerefMut for String<A> {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut()
    }
}
impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}
