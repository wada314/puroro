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

use ::std::marker::PhantomData;

/// A trait for repeated field value returned from message traits getter methods.
///
/// Currently this trait is just an analogous of [`std::iter::IntoIterator`].
pub trait RepeatedField<'msg>: IntoIterator {}

impl<'msg, T> RepeatedField<'msg> for &'msg [T] {}
impl<'msg, T> RepeatedField<'msg> for &'msg crate::internal::NoAllocBumpVec<T> {}

pub struct MappedRepeatedField<T, F> {
    inner: T,
    map: F,
}
impl<T, F> MappedRepeatedField<T, F> {
    pub fn new_with_map(inner: T, map: F) -> Self {
        Self { inner, map }
    }
}
impl<'msg, T, F> RepeatedField<'msg> for MappedRepeatedField<T, F>
where
    T: IntoIterator,
    F: Map<From = <T as IntoIterator>::Item>,
{
}
impl<T, F> IntoIterator for MappedRepeatedField<T, F>
where
    T: IntoIterator,
    F: Map<From = <T as IntoIterator>::Item>,
{
    type Item = <F as Map>::To;
    type IntoIter = MapIter<<T as IntoIterator>::IntoIter, F>;
    fn into_iter(self) -> Self::IntoIter {
        MapIter::new(self.inner.into_iter(), self.map)
    }
}

pub trait Map {
    type From;
    type To;
    fn map(&self, from: Self::From) -> Self::To;
}
pub struct MapIter<I, M> {
    iter: I,
    map: M,
}
impl<I, M> MapIter<I, M> {
    fn new(iter: I, map: M) -> Self {
        Self { iter, map }
    }
}
impl<I, M> Iterator for MapIter<I, M>
where
    I: Iterator,
    M: Map<From = <I as Iterator>::Item>,
{
    type Item = <M as Map>::To;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| self.map.map(x))
    }
}

pub struct CloneThenInto<'a, From, To>(PhantomData<&'a (From, To)>);
impl<'a, From, To> Map for CloneThenInto<'a, From, To>
where
    From: Clone + Into<To>,
{
    type From = &'a From;
    type To = To;

    fn map(&self, from: Self::From) -> Self::To {
        <From as Into<To>>::into(<From as Clone>::clone(from))
    }
}
pub type CloneThenIntoRepeatedField<'a, InnerRepeated, InnerItem, TargetItem> =
    MappedRepeatedField<&'a InnerRepeated, CloneThenInto<'a, InnerItem, TargetItem>>;
impl<'a, InnerRepeated, InnerItem, TargetItem>
    CloneThenIntoRepeatedField<'a, InnerRepeated, InnerItem, TargetItem>
{
    pub fn new(inner: &'a InnerRepeated) -> Self {
        Self::new_with_map(inner, CloneThenInto(PhantomData))
    }
}

pub struct AsRefMap<'a, From, To: ?Sized>(PhantomData<&'a (From, To)>);
impl<'a, From, To> Map for AsRefMap<'a, From, To>
where
    From: AsRef<To>,
    To: ?Sized,
{
    type From = &'a From;
    type To = &'a To;
    fn map(&self, from: Self::From) -> Self::To {
        AsRef::as_ref(from)
    }
}
pub type AsRefRepeatedField<'a, InnerRepeated, InnerItem, TargetItem> =
    MappedRepeatedField<&'a InnerRepeated, AsRefMap<'a, InnerItem, TargetItem>>;
impl<'a, InnerRepeated, InnerItem, TargetItem: ?Sized>
    AsRefRepeatedField<'a, InnerRepeated, InnerItem, TargetItem>
{
    pub fn new(inner: &'a InnerRepeated) -> Self {
        Self::new_with_map(inner, AsRefMap(PhantomData))
    }
}
