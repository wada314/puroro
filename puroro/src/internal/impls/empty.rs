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

use crate::RepeatedField;
use ::std::marker::PhantomData;

#[derive(Default)]
pub struct EmptyRepeatedField<T>(PhantomData<T>);
impl<T> EmptyRepeatedField<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'msg, T> RepeatedField<'msg> for EmptyRepeatedField<T> {}

impl<T> IntoIterator for EmptyRepeatedField<T> {
    type Item = T;
    type IntoIter = ::std::iter::Empty<T>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::empty()
    }
}
