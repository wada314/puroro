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

use crate::{ErrorKind, Result};
use ::std::ops::Deref;

pub struct ProtoPath(str);
pub struct ProtoPathBuf(String);

impl ProtoPath {
    pub fn new<S: AsRef<str> + ?Sized>(path: &S) -> &Self {
        unsafe { &*(path.as_ref() as *const str as *const ProtoPath) }
    }
    pub fn is_absolute(&self) -> bool {
        self.0.starts_with('.')
    }
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }
    pub fn parent(&self) -> Option<&Self> {
        self.0
            .rsplit_once('.')
            .map(|(parent, _)| ProtoPath::new(parent))
            .or_else(|| self.is_relative().then_some(ProtoPath::new("")))
    }
    pub fn ancestors(&self) -> impl Iterator<Item = &Self> {
        ::std::iter::successors(Some(self), |path| path.parent())
    }
    pub fn strip_prefix(&self, prefix: &Self) -> Result<&Self> {
        if prefix.0.ends_with('.') {
            if self.0.starts_with(&prefix.0) {
                Ok(ProtoPath::new(&self.0[prefix.0.len()..]))
            } else {
                Err(ErrorKind::ProtoPathStripPrefixError(
                    self.0.to_string(),
                    prefix.0.to_string(),
                ))?
            }
        } else {
            if self.0.starts_with(&prefix.0) && self.0[prefix.0.len()..].starts_with('.') {
                Ok(ProtoPath::new(&self.0[prefix.0.len() + 1..]))
            } else {
                Err(ErrorKind::ProtoPathStripPrefixError(
                    self.0.to_string(),
                    prefix.0.to_string(),
                ))?
            }
        }
    }
}

impl ProtoPathBuf {
    pub fn new() -> Self {
        Self("".to_string())
    }
}

impl AsRef<str> for ProtoPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<&ProtoPath> for String {
    fn from(v: &ProtoPath) -> String {
        v.0.to_string()
    }
}

impl From<&ProtoPath> for ProtoPathBuf {
    fn from(v: &ProtoPath) -> ProtoPathBuf {
        ProtoPathBuf(v.0.to_string())
    }
}
impl Deref for ProtoPathBuf {
    type Target = ProtoPath;
    fn deref(&self) -> &Self::Target {
        ProtoPath::new(&self.0)
    }
}

impl AsRef<str> for ProtoPathBuf {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<ProtoPathBuf> for String {
    fn from(v: ProtoPathBuf) -> String {
        v.0
    }
}
