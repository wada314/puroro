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

pub struct AsRefRepeatedField<'field, T, R: ?Sized>(&'field T, PhantomData<R>);
impl<'field, T, R: ?Sized> AsRefRepeatedField<'field, T, R> {
    pub fn new(repeated_field: &'field T) -> Self {
        Self(repeated_field, PhantomData)
    }
}
impl<'field, T, U, R: ?Sized> IntoIterator for AsRefRepeatedField<'field, T, R>
where
    &'field T: IntoIterator<Item = &'field U>,
    U: 'field + AsRef<R>,
    R: 'field,
{
    type Item = &'field R;
    type IntoIter = AsRefIter<<&'field T as IntoIterator>::IntoIter, R>;
    fn into_iter(self) -> Self::IntoIter {
        AsRefIter::new(self.0.into_iter())
    }
}
impl<'field, T, U, R: ?Sized> RepeatedField<'field> for AsRefRepeatedField<'field, T, R>
where
    &'field T: IntoIterator<Item = &'field U>,
    U: 'field + AsRef<R>,
    R: 'field,
{
}

pub struct AsRefIter<I, R: ?Sized>(I, PhantomData<R>);
impl<I, R: ?Sized> AsRefIter<I, R> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'a, I, T, R> Iterator for AsRefIter<I, R>
where
    I: Iterator<Item = &'a T>,
    T: 'a + AsRef<R>,
    R: 'a + ?Sized,
{
    type Item = &'a R;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| r.as_ref())
    }
}

pub struct CloneThenIntoIter<I, T>(I, PhantomData<T>);
impl<I, T> CloneThenIntoIter<I, T> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'a, I, T, R> Iterator for CloneThenIntoIter<I, T>
where
    I: Iterator<Item = &'a R>,
    R: 'a + Clone + Into<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| Into::into(Clone::clone(v)))
    }
}
impl<I, T> Clone for CloneThenIntoIter<I, T>
where
    I: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
