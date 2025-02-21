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

use super::OnceList1;
use ::cached_pair::{Converter, Pair};
use ::std::{alloc::Allocator, cell::Cell};

pub(crate) struct MultiPair<L, R, A: Allocator, X, C> {
    pair: Pair<L, OnceList1<R, A>, MultiConverterAdapter<X, C, A>>,
    allocator: A,
}

impl<L, R, A, X, C> MultiPair<L, R, A, X, C>
where
    A: Allocator + Clone,
    X: Default + Copy,
    C: Default,
{
    pub fn from_left(left: L, allocator: A) -> Self {
        Self::from_left_conv(left, C::default(), allocator)
    }
    pub fn from_right(right: R, allocator: A) -> Self {
        Self::from_right_conv(right, C::default(), allocator)
    }
}

impl<L, R, A, X, C> MultiPair<L, R, A, X, C>
where
    A: Allocator + Clone,
    X: Default + Copy,
{
    pub fn from_left_conv(left: L, converter: C, allocator: A) -> Self {
        Self {
            pair: Pair::from_left_conv(
                left,
                MultiConverterAdapter::new(converter, X::default(), allocator.clone()),
            ),
            allocator,
        }
    }
    pub fn from_right_conv(right: R, converter: C, allocator: A) -> Self {
        Self {
            pair: Pair::from_right_conv(
                OnceList1::new_in(right, allocator.clone()),
                MultiConverterAdapter::new(converter, X::default(), allocator.clone()),
            ),
            allocator,
        }
    }
}

impl<L, R, A, X, C> MultiPair<L, R, A, X, C>
where
    A: Allocator + Clone,
    X: Copy + Default,
    C: MultiConverter<L, R, X>,
{
    pub fn try_left(&self) -> Result<&L, C::ToLeftError> {
        self.pair.try_left()
    }

    pub fn try_right<E>(&self, context: X) -> Result<&R, E>
    where
        E: From<C::ToLeftError> + From<C::ToRightError>,
    {
        if let Some(right) = self.pair.right_opt() {
            if let Some(right_item) = right
                .iter()
                .find(|item| self.converter().matches_context(item, &context))
            {
                return Ok(right_item);
            }
            right.push(
                self.converter()
                    .convert_to_right(self.try_left()?, &context)?,
            );
        }
        Ok(self.pair.try_right()?.last())
    }

    pub fn try_left_mut(&mut self) -> Result<&mut L, C::ToLeftError> {
        self.pair.try_left_mut()
    }

    pub fn try_right_mut<E>(&mut self) -> Result<&mut R, E> {
        todo!()
    }

    pub fn try_into_left(self) -> Result<L, C::ToLeftError> {
        self.pair.try_into_left()
    }

    pub fn try_into_right<E>(self) -> Result<R, E> {
        todo!()
    }

    pub fn converter(&self) -> &C {
        self.pair.converter().inner()
    }

    pub fn allocator(&self) -> &A {
        &self.allocator
    }
}

pub(crate) trait MultiConverter<L, R, X> {
    type ToLeftError;
    type ToRightError;
    fn convert_to_left(&self, right: &R) -> Result<L, Self::ToLeftError>;
    fn convert_to_right(&self, left: &L, context: &X) -> Result<R, Self::ToRightError>;
    fn matches_context(&self, right: &R, context: &X) -> bool;
    fn reduce_right<'a>(&self, first: &'a R, #[allow(unused)] second: &'a R) -> &'a R {
        first
    }
}

pub(crate) struct BoxedFnMultiConverter<L, R, X, EL, ER> {
    convert_to_left: Box<dyn Fn(&R) -> Result<L, EL>>,
    convert_to_right: Box<dyn Fn(&L, &X) -> Result<R, ER>>,
    matches_context: Box<dyn Fn(&R, &X) -> bool>,
    reduce_right: Box<dyn for<'a> Fn(&'a R, &'a R) -> &'a R>,
}

impl<L, R, X, EL, ER> MultiConverter<L, R, X> for BoxedFnMultiConverter<L, R, X, EL, ER> {
    type ToLeftError = EL;
    type ToRightError = ER;
    fn convert_to_left(&self, right: &R) -> Result<L, Self::ToLeftError> {
        (self.convert_to_left)(right)
    }
    fn convert_to_right(&self, left: &L, context: &X) -> Result<R, Self::ToRightError> {
        (self.convert_to_right)(left, context)
    }
    fn matches_context(&self, right: &R, context: &X) -> bool {
        (self.matches_context)(right, context)
    }
    fn reduce_right<'a>(&self, first: &'a R, second: &'a R) -> &'a R {
        (self.reduce_right)(first, second)
    }
}

struct MultiConverterAdapter<X, C, A> {
    converter: C,
    context: Cell<X>,
    allocator: A,
}

impl<L, R, X, A, C> Converter<L, OnceList1<R, A>> for MultiConverterAdapter<X, C, A>
where
    C: MultiConverter<L, R, X>,
    X: Copy,
    A: Allocator + Clone,
{
    type ToLeftError = C::ToLeftError;
    type ToRightError = C::ToRightError;
    fn convert_to_left(&self, right: &OnceList1<R, A>) -> Result<L, Self::ToLeftError> {
        let reduced = right.reduce(|first, second| self.converter.reduce_right(first, second));
        self.converter.convert_to_left(reduced)
    }
    fn convert_to_right(&self, left: &L) -> Result<OnceList1<R, A>, Self::ToRightError> {
        let scalar = self.converter.convert_to_right(left, &self.context.get())?;
        Ok(OnceList1::new_in(scalar, self.allocator.clone()))
    }
}
impl<X, C, A> MultiConverterAdapter<X, C, A>
where
    X: Copy,
{
    pub(crate) fn new(converter: C, context: X, allocator: A) -> Self {
        Self {
            converter,
            context: Cell::new(context),
            allocator,
        }
    }
    pub(crate) fn set_context(&self, context: X) {
        self.context.set(context);
    }
    pub(crate) fn inner(&self) -> &C {
        &self.converter
    }
}
