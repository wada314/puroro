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

use crate::{Either, RepeatedField};
use ::std::iter::Chain;
use ::std::ops::Deref;

pub struct MergedRepeatedField<T, U>(T, U);
impl<T, U> MergedRepeatedField<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self(t, u)
    }
}

impl<'msg, T, U> RepeatedField<'msg> for MergedRepeatedField<T, U>
where
    T: IntoIterator<Item = <U as IntoIterator>::Item>,
    U: IntoIterator,
{
}
impl<T, U> IntoIterator for MergedRepeatedField<T, U>
where
    T: IntoIterator<Item = <U as IntoIterator>::Item>,
    U: IntoIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Chain<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().chain(self.1.into_iter())
    }
}

pub struct MergedRepeatedLDField<T, U>(T, U);
impl<T, U> MergedRepeatedLDField<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self(t, u)
    }
}
impl<'msg, T, U> RepeatedField<'msg> for MergedRepeatedLDField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
}
impl<T, U> IntoIterator for MergedRepeatedLDField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        impl Iterator<Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|v| Either::Left(v))
            .chain(self.1.into_iter().map(|v| Either::Right(v)))
    }
}

pub struct MergedRepeatedMessageField<T, U>(T, U);
impl<T, U> MergedRepeatedMessageField<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self(t, u)
    }
}
impl<'msg, T, U> RepeatedField<'msg> for MergedRepeatedMessageField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
{
}
impl<T, U> IntoIterator for MergedRepeatedMessageField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        impl Iterator<Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|v| Either::Left(v))
            .chain(self.1.into_iter().map(|v| Either::Right(v)))
    }
}
