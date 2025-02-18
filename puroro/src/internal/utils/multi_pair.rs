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
use ::std::alloc::Allocator;

pub(crate) struct MultiPair<L, R, A: Allocator, C> {
    pair: Pair<L, OnceList1<R, A>, ConverterForOnceList1<C, A>>,
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

#[derive(Clone)]
pub(crate) struct ConverterForOnceList1<C, A>(pub(crate) C, pub(crate) A);

impl<C, A> ConverterForOnceList1<C, A> {
    pub(crate) fn new_in(converter: C, alloc: A) -> Self {
        Self(converter, alloc)
    }
    pub(crate) fn inner(&self) -> &C {
        &self.0
    }
    #[allow(unused)]
    pub(crate) fn inner_mut(&mut self) -> &mut C {
        &mut self.0
    }
    #[allow(unused)]
    pub(crate) fn allocator(&self) -> &A {
        &self.1
    }
}

impl<L, R, A, C> Converter<L, OnceList1<R, A>> for ConverterForOnceList1<C, A>
where
    C: Converter<L, R>,
    A: Allocator + Clone,
{
    type ToLeftError = C::ToLeftError;
    type ToRightError = C::ToRightError;

    fn convert_to_left(
        &self,
        right: &OnceList1<R, A>,
    ) -> ::std::result::Result<L, Self::ToLeftError> {
        self.0.convert_to_left(right.first())
    }

    fn convert_to_right(
        &self,
        left: &L,
    ) -> ::std::result::Result<OnceList1<R, A>, Self::ToRightError> {
        Ok(OnceList1::new_in(
            self.0.convert_to_right(left)?,
            self.1.clone(),
        ))
    }
}
