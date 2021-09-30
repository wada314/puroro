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

pub mod se;

use ::std::ops::Deref;

pub struct DerefIter<I>(I);
impl<I> DerefIter<I> {
    pub fn new(iter: I) -> Self {
        Self(iter)
    }
}
impl<'a, I, T> Iterator for DerefIter<I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Deref,
{
    type Item = &'a <T as Deref>::Target;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| <T as Deref>::deref(r))
    }
}
