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
use ::std::iter;
use ::std::option;

pub struct OptionRepeatedField<R>(Option<R>);
impl<R> OptionRepeatedField<R> {
    pub fn new(maybe_repeated_field: Option<R>) -> Self {
        Self(maybe_repeated_field)
    }
}
impl<'msg, R> RepeatedField<'msg> for OptionRepeatedField<R> where R: RepeatedField<'msg> {}
impl<R> IntoIterator for OptionRepeatedField<R>
where
    R: IntoIterator,
{
    type Item = <R as IntoIterator>::Item;
    type IntoIter = iter::Flatten<option::IntoIter<R>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flatten()
    }
}
