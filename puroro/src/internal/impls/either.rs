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

pub mod getter;
pub mod getter_opt;
pub mod getter_slice;

use crate::internal::{EmptyFields, ImplProperties};
use crate::{tags, Either, MessageImpl};
use ::std::marker::PhantomData;

pub struct EitherShared<T, U> {
    either: Either<T, U>,
}
impl<T, U> From<Either<T, U>> for EitherShared<T, U> {
    fn from(either: Either<T, U>) -> Self {
        Self { either }
    }
}
impl<MP, T, U> From<Either<T, U>>
    for MessageImpl<MP, tags::EitherImpl, EmptyFields, EitherShared<T, U>>
{
    fn from(val: Either<T, U>) -> Self {
        MessageImpl::from_raw_parts(Default::default(), Into::into(val))
    }
}

pub trait IntoEitherMessage<MP> {
    type EitherMessage;
    fn into_message(self) -> Self::EitherMessage;
}

pub struct EitherImplProperties<T, U>(PhantomData<(T, U)>);
impl<T, U> ImplProperties for EitherImplProperties<T, U> {
    type ImplTag = tags::EitherImpl;
    type FieldsType = EmptyFields;
    type SharedType = EitherShared<T, U>;
}

pub struct EitherRepeatedField<T, U>(pub(crate) Either<T, U>);

impl<T, U> IntoIterator for EitherRepeatedField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter = EitherIter<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        EitherIter(
            self.0
                .map_left(|t| t.into_iter())
                .map_right(|u| u.into_iter()),
        )
    }
}

pub struct EitherIter<T, U>(Either<T, U>);

impl<T, U> Iterator for EitherIter<T, U>
where
    T: Iterator,
    U: Iterator,
{
    type Item = Either<<T as Iterator>::Item, <U as Iterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().either(
            |t| t.next().map(|v| Either::Left(v)),
            |u| u.next().map(|v| Either::Right(v)),
        )
    }
}
