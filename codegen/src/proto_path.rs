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

use ::std::ops::Deref;

pub struct ProtoPath(str);
pub struct ProtoPathBuf(String);

impl ProtoPath {
    pub fn new<S: AsRef<str> + ?Sized>(path: &S) -> &Self {
        unsafe { &*(path.as_ref() as *const str as *const ProtoPath) }
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
