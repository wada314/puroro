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

pub struct String<A: Allocator> {
    vec: Vec<u8, A>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FromUtf8Error<A: Allocator> {
    bytes: Vec<u8, A>,
    error: ::std::str::Utf8Error,
}

impl<A: Allocator> String<A> {
    #[inline]
    #[must_use]
    pub const fn new_in(alloc: A) -> String<A> {
        String {
            vec: Vec::new_in(alloc),
        }
    }

    #[inline]
    pub fn from_utf8(vec: Vec<u8, A>) -> Result<String<A>, FromUtf8Error<A>> {
        match ::std::str::from_utf8(&vec) {
            Ok(..) => Ok(String { vec }),
            Err(e) => Err(FromUtf8Error {
                bytes: vec,
                error: e,
            }),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self
    }
}

impl<A: Allocator> ::std::ops::Deref for String<A> {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        unsafe { ::std::str::from_utf8_unchecked(&self.vec) }
    }
}
