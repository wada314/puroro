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

//! A customized version of `std::string::String`
//! for use in protobuf field type `string`.
//! The `string` type in protobuf can only have max 2^32 - 1 bytes length.
//! So we can make a special optimized version for it.

use crate::bytes::Bytes;
use ::std::ops::{Deref, DerefMut};

pub struct String {
    bytes: Bytes,
}

impl String {
    pub fn new() -> Self {
        Self {
            bytes: Bytes::new(),
        }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        let bytes: &[u8] = &self.bytes;
        unsafe { ::std::str::from_utf8_unchecked(bytes) }
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let bytes: &mut [u8] = &mut self.bytes;
        unsafe { ::std::str::from_utf8_unchecked_mut(bytes) }
    }
}
