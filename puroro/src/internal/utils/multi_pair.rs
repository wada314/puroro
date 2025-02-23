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
use ::cached_pair::{Converter, EitherOrBoth, Pair};
use ::polonius_the_crab::{polonius, polonius_return, polonius_try};
use ::std::alloc::Allocator;
use ::std::cell::Cell;
use ::std::rc::Rc;

#[derive(::derive_more::Debug, Clone)]
pub(crate) struct MultiPair<L, R, A: Allocator, X: Copy, C> {
    pair: Pair<L, OnceList1<R, A>, MultiConverterAdapter<X, C, A>>,
    #[debug(skip)]
    allocator: A,
    converter: Rc<C, A>,
}

impl<L, R, A: Allocator, X: Copy, C> MultiPair<L, R, A, X, C> {
    pub fn converter(&self) -> &C {
        self.pair.converter().inner()
    }
    pub fn allocator(&self) -> &A {
        &self.allocator
    }
}

impl<L, R, A, X, C> MultiPair<L, R, A, X, C>
where
    A: Allocator + Clone,
    X: Default + Copy,
    C: Default,
{
    #[allow(unused)]
    pub fn from_left(left: L, allocator: A) -> Self {
        Self::from_left_conv(left, C::default(), allocator)
    }
    #[allow(unused)]
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
        let converter = Rc::new_in(converter, allocator.clone());
        Self {
            pair: Pair::from_left_conv(
                left,
                MultiConverterAdapter::new(converter.clone(), X::default(), allocator.clone()),
            ),
            allocator,
            converter,
        }
    }
    pub fn from_right_conv(right: R, converter: C, allocator: A) -> Self {
        let converter = Rc::new_in(converter, allocator.clone());
        Self {
            pair: Pair::from_right_conv(
                OnceList1::new_in(right, allocator.clone()),
                MultiConverterAdapter::new(converter.clone(), X::default(), allocator.clone()),
            ),
            allocator,
            converter,
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
        self.pair.converter().set_context(context);
        Ok(self.pair.try_right()?.last())
    }

    pub fn try_left_mut(&mut self) -> Result<&mut L, C::ToLeftError> {
        self.pair.try_left_mut()
    }

    pub fn try_right_mut<E>(&mut self, context: X) -> Result<&mut R, E>
    where
        E: From<C::ToLeftError> + From<C::ToRightError>,
    {
        let mut this = self;
        polonius!(|this| -> Result<&'polonius mut R, E> {
            if let Some(right) = this.pair.right_opt_mut() {
                if let Some(right_item) = right
                    .iter_mut()
                    .find(|item| this.converter.matches_context(item, &context))
                {
                    polonius_return!(Ok(right_item));
                }
            }
        });

        polonius!(|this| -> Result<&'polonius mut R, E> {
            if let Some(right) = this.pair.right_opt() {
                right.push(polonius_try!(this
                    .converter
                    .convert_to_right(polonius_try!(this.try_left()), &context)));
            }
        });

        this.pair.converter().set_context(context);
        Ok(this.pair.try_right_mut()?.last_mut())
    }

    pub fn try_into_left(self) -> Result<L, C::ToLeftError> {
        self.pair.try_into_left()
    }

    #[allow(unused)]
    pub fn try_into_right<E>(self, context: X) -> Result<R, E>
    where
        E: From<C::ToLeftError> + From<C::ToRightError>,
    {
        let either = self.pair.into();
        let converter = self.converter;
        let left = match either {
            EitherOrBoth::Left(left) => left,
            EitherOrBoth::Right(right) => {
                match right.remove(|item| converter.matches_context(item, &context)) {
                    Ok((removed, _)) => {
                        return Ok(removed);
                    }
                    Err(right) => {
                        let reduced =
                            right.reduce(|first, second| converter.reduce_right(first, second));
                        converter.convert_to_left(reduced)?
                    }
                }
            }
            EitherOrBoth::Both(left, right) => {
                match right.remove(|item| converter.matches_context(item, &context)) {
                    Ok((removed, _)) => {
                        return Ok(removed);
                    }
                    Err(_) => left,
                }
            }
        };
        Ok(converter.convert_to_right(&left, &context)?)
    }
}

impl<L, R, A, X, C> MultiPair<L, R, A, X, C>
where
    A: Allocator + Clone,
    X: Copy + Default,
    C: MultiConverter<L, R, X, ToLeftError = !>,
{
    pub fn left(&self) -> &L {
        self.try_left().into_ok()
    }
    pub fn left_mut(&mut self) -> &mut L {
        self.try_left_mut().into_ok()
    }
    pub fn into_left(self) -> L {
        self.try_into_left().into_ok()
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

#[derive(Clone, ::derive_more::Debug)]
struct MultiConverterAdapter<X: Copy, C, A: Allocator> {
    converter: Rc<C, A>,
    context: Cell<X>,
    #[debug(skip)]
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
impl<X: Copy, C, A: Allocator> MultiConverterAdapter<X, C, A> {
    pub(crate) fn new(converter: Rc<C, A>, context: X, allocator: A) -> Self {
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
