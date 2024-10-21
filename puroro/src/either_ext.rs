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

use crate::IsEmpty;
use ::itertools::{Either, EitherOrBoth};

pub trait EitherExt {}
impl<T, U> EitherExt for Either<T, U> {}

pub trait EitherOrBothExt {
    type T;
    type U;
    fn into_either_or_both(self) -> EitherOrBoth<Self::T, Self::U>;
    fn into_iter(self) -> impl IntoIterator<Item = <Self::T as IntoIterator>::Item>
    where
        Self: Sized,
        Self::T: IntoIterator,
        Self::U: IntoIterator<Item = <Self::T as IntoIterator>::Item>,
    {
        let (t_opt, u_opt) = self.into_either_or_both().left_and_right();
        t_opt
            .into_iter()
            .flatten()
            .chain(u_opt.into_iter().flatten())
    }
    fn factor_into_iter(
        self,
    ) -> impl IntoIterator<Item = Either<<Self::T as IntoIterator>::Item, <Self::U as IntoIterator>::Item>>
    where
        Self: Sized,
        Self::T: IntoIterator,
        Self::U: IntoIterator,
    {
        let (t_opt, u_opt) = self.into_either_or_both().left_and_right();
        t_opt
            .into_iter()
            .flatten()
            .map(Either::Left)
            .chain(u_opt.into_iter().flatten().map(Either::Right))
    }
    fn flatten_opt<T2, U2>(self) -> Option<EitherOrBoth<T2, U2>>
    where
        Self: Sized + EitherOrBothExt<T = Option<T2>, U = Option<U2>>,
    {
        let (t, u) = self.into_either_or_both().left_and_right();
        match (t.flatten(), u.flatten()) {
            (Some(t), Some(u)) => Some(EitherOrBoth::Both(t, u)),
            (Some(t), None) => Some(EitherOrBoth::Left(t)),
            (None, Some(u)) => Some(EitherOrBoth::Right(u)),
            (None, None) => None,
        }
    }
    fn non_empty_right_or_left<F, G, M>(self, f: F, g: G) -> M
    where
        Self: Sized,
        F: FnOnce(Self::T) -> M,
        G: FnOnce(Self::U) -> M,
        M: IsEmpty,
    {
        match self.into_either_or_both() {
            EitherOrBoth::Both(t, u) => {
                let m = f(t);
                if m.is_empty() {
                    g(u)
                } else {
                    m
                }
            }
            EitherOrBoth::Left(t) => f(t),
            EitherOrBoth::Right(u) => g(u),
        }
    }
}
impl<T, U> EitherOrBothExt for EitherOrBoth<T, U> {
    type T = T;
    type U = U;
    fn into_either_or_both(self) -> EitherOrBoth<Self::T, Self::U> {
        self
    }
}

pub trait BothExt {
    type T;
    type U;
    fn into_tuple(self) -> (Self::T, Self::U);

    fn into_either_or_both_opt<T2, U2>(self) -> Option<EitherOrBoth<T2, U2>>
    where
        Self: Sized + BothExt<T = Option<T2>, U = Option<U2>>,
    {
        match self.into_tuple() {
            (Some(t), Some(u)) => Some(EitherOrBoth::Both(t, u)),
            (Some(t), None) => Some(EitherOrBoth::Left(t)),
            (None, Some(u)) => Some(EitherOrBoth::Right(u)),
            (None, None) => None,
        }
    }

    fn into_iter(self) -> impl Iterator<Item = <Self::T as IntoIterator>::Item>
    where
        Self: Sized,
        Self::T: IntoIterator,
        Self::U: IntoIterator<Item = <Self::T as IntoIterator>::Item>,
    {
        let (t, u) = self.into_tuple();
        t.into_iter().chain(u.into_iter())
    }

    fn factor_into_iter(
        self,
    ) -> impl Iterator<Item = Either<<Self::T as IntoIterator>::Item, <Self::U as IntoIterator>::Item>>
    where
        Self: Sized,
        Self::T: IntoIterator,
        Self::U: IntoIterator,
    {
        let (t, u) = self.into_tuple();
        t.into_iter()
            .map(Either::Left)
            .chain(u.into_iter().map(Either::Right))
    }
}
impl<T, U> BothExt for (T, U) {
    type T = T;
    type U = U;
    fn into_tuple(self) -> (Self::T, Self::U) {
        self
    }
}
