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

/// Extension trait for `Either` type.
pub trait EitherExt {}
impl<T, U> EitherExt for Either<T, U> {}

/// Extension trait providing additional functionality for `EitherOrBoth` type.
/// This trait adds methods for transforming and combining values in an `EitherOrBoth` context.
pub trait EitherOrBothExt {
    type T;
    type U;

    /// Converts self into an `EitherOrBoth` value.
    /// This method is used by the extension's method default implementations.
    fn into_either_or_both(self) -> EitherOrBoth<Self::T, Self::U>;

    /// Returns the left value if present, otherwise the right value.
    /// Only available when both sides have exactly the same type.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(1, 1);
    /// assert_eq!(both.prefer_left(), 1);
    ///
    /// let left = EitherOrBoth::Left(1);
    /// assert_eq!(left.prefer_left(), 1);
    ///
    /// let right = EitherOrBoth::Right(1);
    /// assert_eq!(right.prefer_left(), 1);
    /// ```
    fn prefer_left<A>(self) -> A
    where
        Self: Sized + EitherOrBothExt<T = A, U = A>,
    {
        match self.into_either_or_both() {
            EitherOrBoth::Both(l, _) => l,
            EitherOrBoth::Left(l) => l,
            EitherOrBoth::Right(r) => r,
        }
    }

    /// Returns the right value if present, otherwise the left value.
    /// Only available when both sides have exactly the same type.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(1, 1);
    /// assert_eq!(both.prefer_right(), 1);
    ///
    /// let left = EitherOrBoth::Left(1);
    /// assert_eq!(left.prefer_right(), 1);
    ///
    /// let right = EitherOrBoth::Right(1);
    /// assert_eq!(right.prefer_right(), 1);
    /// ```
    fn prefer_right<A>(self) -> A
    where
        Self: Sized + EitherOrBothExt<T = A, U = A>,
    {
        match self.into_either_or_both() {
            EitherOrBoth::Both(_, r) => r,
            EitherOrBoth::Left(l) => l,
            EitherOrBoth::Right(r) => r,
        }
    }

    /// Factors out a `Result` from an `EitherOrBoth` containing `Result`s.
    ///
    /// If any of the contained `Result`s is an `Err`, returns that error.
    /// Otherwise, returns an `EitherOrBoth` containing the `Ok` values.
    ///
    /// # Example
    /// ```ignore
    /// let both: EitherOrBoth<Result<i32, Error>, Result<String, Error>> = EitherOrBoth::Both(Ok(1), Ok("hello".to_string()));
    /// let result: Result<EitherOrBoth<i32, String>, Error> = both.factor_err();
    /// ```
    fn factor_err<T2, U2, E>(self) -> Result<EitherOrBoth<T2, U2>, E>
    where
        Self: Sized + EitherOrBothExt<T = Result<T2, E>, U = Result<U2, E>>,
    {
        match self.into_either_or_both() {
            EitherOrBoth::Both(t, u) => Ok(EitherOrBoth::Both(t?, u?)),
            EitherOrBoth::Left(t) => Ok(EitherOrBoth::Left(t?)),
            EitherOrBoth::Right(u) => Ok(EitherOrBoth::Right(u?)),
        }
    }

    /// Converts an `EitherOrBoth` of iterables into a single iterator.
    ///
    /// The resulting iterator will yield all items from both sides in sequence,
    /// first from the left side, then from the right side.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(vec![1, 2], vec![3, 4]);
    /// let items: Vec<i32> = both.into_iter().collect(); // [1, 2, 3, 4]
    /// ```
    fn into_iter(self) -> impl Iterator<Item = <Self::T as IntoIterator>::Item>
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

    /// Converts an `EitherOrBoth` of iterables into an iterator of `Either`s.
    ///
    /// The resulting iterator will yield `Either::Left` for items from the left side
    /// and `Either::Right` for items from the right side.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(vec![1, 2], vec![3, 4]);
    /// let items: Vec<Either<i32, i32>> = both.factor_into_iter().collect();
    /// // [Left(1), Left(2), Right(3), Right(4)]
    /// ```
    fn factor_into_iter(
        self,
    ) -> impl Iterator<Item = Either<<Self::T as IntoIterator>::Item, <Self::U as IntoIterator>::Item>>
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

    /// Flattens an `EitherOrBoth` of `Option`s into an `Option` of `EitherOrBoth`.
    ///
    /// Returns `None` if both sides are `None`, otherwise combines the values
    /// according to which sides contain `Some` values.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(Some(1), Some("hello"));
    /// let result = both.flatten_opt(); // Some(EitherOrBoth::Both(1, "hello"))
    ///
    /// let left = EitherOrBoth::Left(Some(1));
    /// let result = left.flatten_opt(); // Some(EitherOrBoth::Left(1))
    /// ```
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

    /// Selects between left and right values based on emptiness.
    ///
    /// If both values are present, first tries the left value. If it's empty,
    /// then uses the right value. This is useful for implementing fallback behavior.
    ///
    /// # Example
    /// ```ignore
    /// let both = EitherOrBoth::Both(vec![], vec![1, 2]);
    /// let result = both.non_empty_right_or_left(
    ///     |left| left,
    ///     |right| right
    /// ); // Returns vec![1, 2] since left is empty
    /// ```
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

/// Extension trait for tuple types (T, U), providing methods for working with pairs of values.
pub trait BothExt {
    type T;
    type U;

    /// Converts self into a tuple of (T, U).
    fn into_tuple(self) -> (Self::T, Self::U);

    /// Returns the left value of the tuple.
    /// Only available when both sides have exactly the same type.
    ///
    /// # Example
    /// ```ignore
    /// let tuple = (1, 1);
    /// assert_eq!(tuple.prefer_left(), 1);
    /// ```
    fn prefer_left<A>(self) -> A
    where
        Self: Sized + BothExt<T = A, U = A>,
    {
        let (l, _) = self.into_tuple();
        l
    }

    /// Returns the right value of the tuple.
    /// Only available when both sides have exactly the same type.
    ///
    /// # Example
    /// ```ignore
    /// let tuple = (1, 1);
    /// assert_eq!(tuple.prefer_right(), 1);
    /// ```
    fn prefer_right<A>(self) -> A
    where
        Self: Sized + BothExt<T = A, U = A>,
    {
        let (_, r) = self.into_tuple();
        r
    }

    /// Converts a tuple of `Option`s into an `Option` of `EitherOrBoth`.
    ///
    /// Similar to `EitherOrBothExt::flatten_opt`, but works with tuples instead.
    ///
    /// # Example
    /// ```ignore
    /// let tuple = (Some(1), Some("hello"));
    /// let result = tuple.into_either_or_both_opt(); // Some(EitherOrBoth::Both(1, "hello"))
    /// ```
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

    /// Converts a tuple of iterables into a single iterator.
    ///
    /// The resulting iterator will yield all items from both sides in sequence.
    ///
    /// # Example
    /// ```ignore
    /// let tuple = (vec![1, 2], vec![3, 4]);
    /// let items: Vec<i32> = tuple.into_iter().collect(); // [1, 2, 3, 4]
    /// ```
    fn into_iter(self) -> impl Iterator<Item = <Self::T as IntoIterator>::Item>
    where
        Self: Sized,
        Self::T: IntoIterator,
        Self::U: IntoIterator<Item = <Self::T as IntoIterator>::Item>,
    {
        let (t, u) = self.into_tuple();
        t.into_iter().chain(u.into_iter())
    }

    /// Converts a tuple of iterables into an iterator of `Either`s.
    ///
    /// Similar to `EitherOrBothExt::factor_into_iter`, but works with tuples.
    ///
    /// # Example
    /// ```ignore
    /// let tuple = (vec![1, 2], vec![3, 4]);
    /// let items: Vec<Either<i32, i32>> = tuple.factor_into_iter().collect();
    /// // [Left(1), Left(2), Right(3), Right(4)]
    /// ```
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

    /// Factors out a `Result` from a tuple of `Result`s.
    ///
    /// If either `Result` is an `Err`, returns that error.
    /// Otherwise, returns a tuple of the `Ok` values.
    ///
    /// # Example
    /// ```ignore
    /// let tuple: (Result<i32, Error>, Result<String, Error>) = (Ok(1), Ok("hello".to_string()));
    /// let result: Result<(i32, String), Error> = tuple.factor_result();
    /// ```
    fn factor_result<T2, U2, E>(self) -> Result<(T2, U2), E>
    where
        Self: Sized + BothExt<T = Result<T2, E>, U = Result<U2, E>>,
    {
        let (t, u) = self.into_tuple();
        Ok((t?, u?))
    }
}

impl<T, U> BothExt for (T, U) {
    type T = T;
    type U = U;
    fn into_tuple(self) -> (Self::T, Self::U) {
        self
    }
}
