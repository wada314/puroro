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

use ::std::iter;

/// A trait for repeated field value returned from message traits getter methods.
///
/// Currently this trait is just an analogous of [`std::iter::IntoIterator`].
pub trait RepeatedField<'msg>: IntoIterator {}

impl<'msg, T> RepeatedField<'msg> for &'msg [T] {}

pub struct MappedRepeatedField<T, F> {
    inner: T,
    map: F,
}
impl<T, F> MappedRepeatedField<T, F> {
    pub fn new(inner: T, map: F) -> Self {
        Self { inner, map }
    }
}
impl<'msg, T, U, F> RepeatedField<'msg> for MappedRepeatedField<T, F>
where
    T: IntoIterator,
    F: Map<<T as IntoIterator>::Item, U>,
{
}
impl<T, U, F> IntoIterator for MappedRepeatedField<T, F>
where
    T: IntoIterator,
    F: Map<<T as IntoIterator>::Item, U>,
{
    type Item = U;
    type IntoIter = MapIter<<T as IntoIterator>::IntoIter, F>;
    fn into_iter(self) -> Self::IntoIter {
        MapIter::new(self.inner.into_iter(), self.map)
    }
}

trait Map<From, To> {
    fn map(from: From) -> To;
}
struct MapIter<I, M> {
    iter: I,
    map: M,
}
impl<I, M> MapIter<I, M> {
    fn new(iter: I, map: M) -> Self {
        Self { iter, map }
    }
}
impl<I, M, To> Iterator for MapIter<I, M>
where
    I: Iterator,
    M: Map<<I as Iterator>::Item, To>,
{
    type Item = To;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| self.map.map(x))
    }
}
