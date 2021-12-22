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
use ::std::marker::PhantomData;
use ::std::option;

pub struct EitherLeftRepeatedField<T, Right>(T, PhantomData<Right>);
impl<T, Right> EitherLeftRepeatedField<T, Right> {
    pub fn new(inner_repeated_field: T) -> Self {
        Self(inner_repeated_field, PhantomData)
    }
}
impl<T, Right> IntoIterator for EitherLeftRepeatedField<T, Right>
where
    T: IntoIterator,
{
    type Item = crate::Either<<T as IntoIterator>::Item, Right>;
    type IntoIter = EitherLeftIter<<T as IntoIterator>::IntoIter, Right>;

    fn into_iter(self) -> Self::IntoIter {
        EitherLeftIter::new(self.0.into_iter())
    }
}
impl<'msg, T, Right> RepeatedField<'msg> for EitherLeftRepeatedField<T, Right> where
    T: RepeatedField<'msg>
{
}

pub struct EitherLeftIter<Iter, Right>(Iter, PhantomData<Right>);
impl<Iter, Right> EitherLeftIter<Iter, Right> {
    pub fn new(iter: Iter) -> Self {
        Self(iter, PhantomData)
    }
}
impl<Iter, Right> Iterator for EitherLeftIter<Iter, Right>
where
    Iter: Iterator,
{
    type Item = crate::Either<<Iter as Iterator>::Item, Right>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|val| crate::Either::Left(val))
    }
}
