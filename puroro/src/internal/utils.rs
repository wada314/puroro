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

use ::std::cell::OnceCell;

pub enum TransmutableEitherOrBoth<T, U> {
    Left(T, OnceCell<U>),
    Right(U, OnceCell<T>),
    Both(T, U),
}

impl<T, U, E> TransmutableEitherOrBoth<T, U>
where
    T: for<'a> TryFrom<&'a U, Error = E>,
    U: for<'a> TryFrom<&'a T, Error = E>,
{
    pub fn try_left(&self) -> Result<&T, E> {
        match self {
            TransmutableEitherOrBoth::Left(left, _) => Ok(left),
            TransmutableEitherOrBoth::Right(right, left_cell) => {
                left_cell.get_or_try_init(|| <&U>::try_into(right))
            }
            TransmutableEitherOrBoth::Both(left, _) => Ok(left),
        }
    }
    pub fn try_right(&self) -> Result<&U, E> {
        match self {
            TransmutableEitherOrBoth::Left(left, right_cell) => {
                right_cell.get_or_try_init(|| <&T>::try_into(left))
            }
            TransmutableEitherOrBoth::Right(right, _) => Ok(right),
            TransmutableEitherOrBoth::Both(_, right) => Ok(right),
        }
    }

    pub fn try_left_mut(&mut self) -> Result<&mut T, E> {
        match *self {
            TransmutableEitherOrBoth::Left(left, _) => {
                *self = TransmutableEitherOrBoth::Left(left, OnceCell::new());
            }
            TransmutableEitherOrBoth::Right(right, _) => {
                *self = TransmutableEitherOrBoth::Left(<&U>::try_into(&right)?, OnceCell::new());
            }
            TransmutableEitherOrBoth::Both(left, _) => {
                *self = TransmutableEitherOrBoth::Left(left, OnceCell::new());
            }
        };
        let TransmutableEitherOrBoth::Left(left, _) = self else {
            unreachable!();
        };
        Ok(left)
    }
}
