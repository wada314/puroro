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
use ::std::error::Error;
use ::std::fmt::Display;
use ::std::ops::{Deref, DerefMut};
use ::std::str::Utf8Error;

#[derive(Clone, PartialEq)]
pub struct String {
    bytes: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromUtf8Error {
    bytes: Bytes,
    error: ::std::str::Utf8Error,
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

impl<'a> From<&'a str> for String {
    fn from(value: &'a str) -> Self {
        let mut string = Self::new();
        string.push_str(value);
        string
    }
}

impl From<::std::string::String> for String {
    fn from(value: ::std::string::String) -> Self {
        Self {
            bytes: value.into_bytes().into(),
        }
    }
}

impl From<&::std::string::String> for String {
    fn from(value: &::std::string::String) -> Self {
        value.as_str().into()
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl String {
    pub fn from_utf8(bytes: Bytes) -> Result<Self, FromUtf8Error> {
        match ::std::str::from_utf8(&bytes) {
            Ok(..) => Ok(String { bytes }),
            Err(error) => Err(FromUtf8Error { bytes, error }),
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.bytes.extend_from_slice(string.as_bytes())
    }
}

impl Display for FromUtf8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Utf8Error as Display>::fmt(&self.error, f)
    }
}

impl Error for FromUtf8Error {
    fn description(&self) -> &str {
        "invalid utf-8"
    }
}
