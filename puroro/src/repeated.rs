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

use ::std::ops::DerefMut;

pub trait RepeatedView<'a> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait RepeatedViewMut<'a>:
    RepeatedView<'a> + Extend<<Self as RepeatedViewMut<'a>>::Item>
{
    type Item;
    fn iter_mut(&self) -> impl Iterator<Item = impl DerefMut<Target = Self::Item>>;

    fn push(&mut self, value: <Self as RepeatedViewMut<'a>>::Item);
    fn clear(&mut self);
}

pub trait RepeatedViewApp<'a>:
    RepeatedView<'a> + Extend<<Self as RepeatedViewApp<'a>>::Item>
{
    type Item;
    fn push(&mut self, value: <Self as RepeatedViewApp<'a>>::Item);
}
