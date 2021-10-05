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
use ::std::ops::Deref;

pub struct EitherRepeatedField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Either<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct EitherRepeatedLDField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedLDField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedLDField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator,
    U: RepeatedField<'msg> + IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        IndependentEitherIter<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        IndependentEitherIter(
            self.0
                .map_left(|t| t.into_iter())
                .map_right(|u| u.into_iter()),
        )
    }
}

pub struct EitherRepeatedMessageField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedMessageField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedMessageField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator,
    U: RepeatedField<'msg> + IntoIterator,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        IndependentEitherIter<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        IndependentEitherIter(
            self.0
                .map_left(|t| t.into_iter())
                .map_right(|u| u.into_iter()),
        )
    }
}

pub struct IndependentEitherIter<T, U>(
    Either<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>,
)
where
    T: IntoIterator,
    U: IntoIterator;

impl<T, U> Iterator for IndependentEitherIter<T, U>
where
    T: Iterator,
    U: Iterator,
{
    type Item = Either<<T as Iterator>::Item, <U as Iterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().either(
            |t| t.next().map(|l| Either::Left(l)),
            |u| u.next().map(|r| Either::Right(r)),
        )
    }
}
