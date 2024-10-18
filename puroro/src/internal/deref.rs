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

use ::std::ops::{Deref, DerefMut};

pub struct DerefWrapper<T, F: FnOnce(T)> {
    inner: Option<T>,
    on_drop: Option<F>,
}
impl<T, F: FnOnce(T)> DerefWrapper<T, F> {
    pub fn new(inner: T, on_drop: F) -> Self {
        Self {
            inner: Some(inner),
            on_drop: Some(on_drop),
        }
    }
}
impl<T, F: FnOnce(T)> Drop for DerefWrapper<T, F> {
    fn drop(&mut self) {
        if let (Some(inner), Some(on_drop)) = (self.inner.take(), self.on_drop.take()) {
            on_drop(inner);
        }
    }
}
impl<T, F: FnOnce(T)> Deref for DerefWrapper<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}
impl<T, F: FnOnce(T)> DerefMut for DerefWrapper<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}
