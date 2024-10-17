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
use ::std::fmt::{Debug, Display, Formatter, Result as FmtResult};
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
    pub fn allocator(&self) -> &A {
        self.vec.allocator()
    }
    pub fn clear(&mut self) {
        self.vec.clear();
    }
    pub fn push_str(&mut self, s: &str) {
        self.vec.extend_from_slice(s.as_bytes());
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
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
impl<A: Allocator> Display for String<A> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self.as_ref(), f)
    }
}
impl<A: Allocator> Debug for String<A> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(self.as_ref(), f)
    }
}

impl From<::std::string::String> for String<Global> {
    fn from(s: ::std::string::String) -> Self {
        Self {
            vec: s.into_bytes(),
        }
    }
}
impl From<String<Global>> for ::std::string::String {
    fn from(s: String<Global>) -> Self {
        unsafe { ::std::string::String::from_utf8_unchecked(s.vec) }
    }
}

pub trait StrExt {
    fn to_string_in<A: Allocator>(&self, alloc: A) -> String<A>;
}

impl StrExt for str {
    fn to_string_in<A: Allocator>(&self, alloc: A) -> String<A> {
        let vec = self.as_bytes().to_vec_in(alloc);
        String { vec }
    }
}
