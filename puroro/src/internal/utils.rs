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

#[derive(Clone, Debug)]
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
    pub fn from_left(left: T) -> Self {
        TransmutableEitherOrBoth::Left(left, OnceCell::new())
    }
    pub fn from_right(right: U) -> Self {
        TransmutableEitherOrBoth::Right(right, OnceCell::new())
    }
    pub fn from_both(left: T, right: U) -> Self {
        TransmutableEitherOrBoth::Both(left, right)
    }

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

    pub fn try_into_left(self) -> Result<T, (E, U)> {
        Ok(match self {
            TransmutableEitherOrBoth::Left(left, _) => left,
            TransmutableEitherOrBoth::Right(right, mut left_cell) => match left_cell.take() {
                Some(left) => left,
                None => <&U>::try_into(&right).map_err(|e| (e, right))?,
            },
            TransmutableEitherOrBoth::Both(left, _) => left,
        })
    }
    pub fn try_into_right(self) -> Result<U, (E, T)> {
        Ok(match self {
            TransmutableEitherOrBoth::Left(left, mut right_cell) => match right_cell.take() {
                Some(right) => right,
                None => <&T>::try_into(&left).map_err(|e| (e, left))?,
            },
            TransmutableEitherOrBoth::Right(right, _) => right,
            TransmutableEitherOrBoth::Both(_, right) => right,
        })
    }

    pub fn try_left_mut(&mut self) -> Result<&mut T, E> {
        unsafe {
            let taken_self = ::std::ptr::read(self);
            let (new_self, result) = match taken_self.try_into_left() {
                Ok(left) => (
                    TransmutableEitherOrBoth::Left(left, OnceCell::new()),
                    Ok(()),
                ),
                Err((e, right)) => (
                    TransmutableEitherOrBoth::Right(right, OnceCell::new()),
                    Err(e),
                ),
            };
            ::std::ptr::write(self, new_self);
            result
        }?;
        let TransmutableEitherOrBoth::Left(left, _) = self else {
            unreachable!();
        };
        Ok(left)
    }
    pub fn try_right_mut(&mut self) -> Result<&mut U, E> {
        unsafe {
            let taken_self = ::std::ptr::read(self);
            let (new_self, result) = match taken_self.try_into_right() {
                Ok(right) => (
                    TransmutableEitherOrBoth::Right(right, OnceCell::new()),
                    Ok(()),
                ),
                Err((e, left)) => (
                    TransmutableEitherOrBoth::Left(left, OnceCell::new()),
                    Err(e),
                ),
            };
            ::std::ptr::write(self, new_self);
            result
        }?;
        let TransmutableEitherOrBoth::Right(right, _) = self else {
            unreachable!();
        };
        Ok(right)
    }
}
