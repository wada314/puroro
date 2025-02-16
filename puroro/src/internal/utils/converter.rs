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

use cached_pair::Converter;
use std::alloc::Allocator;
use std::fmt::Debug;
use std::rc::Rc;

use super::OnceList1;

pub(crate) struct BoxedFnConverterWithContext<C, L, R, EL, ER> {
    to_left: Rc<dyn Fn(&R, &C) -> ::std::result::Result<L, EL>>,
    to_right: Rc<dyn Fn(&L, &C) -> ::std::result::Result<R, ER>>,
    context: C,
}

pub(crate) fn boxed_fn_converter_with_context<C, L, R, EL, ER>(
    context: C,
    to_left: impl Fn(&R, &C) -> ::std::result::Result<L, EL> + 'static,
    to_right: impl Fn(&L, &C) -> ::std::result::Result<R, ER> + 'static,
) -> BoxedFnConverterWithContext<C, L, R, EL, ER> {
    BoxedFnConverterWithContext {
        to_left: Rc::new(to_left),
        to_right: Rc::new(to_right),
        context,
    }
}

impl<C, L, R, EL, ER> Converter<L, R> for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    type ToLeftError = EL;
    type ToRightError = ER;

    fn convert_to_left(&self, right: &R) -> ::std::result::Result<L, Self::ToLeftError> {
        (self.to_left)(right, &self.context)
    }

    fn convert_to_right(&self, left: &L) -> ::std::result::Result<R, Self::ToRightError> {
        (self.to_right)(left, &self.context)
    }
}

impl<C, L, R, EL, ER> BoxedFnConverterWithContext<C, L, R, EL, ER> {
    pub(crate) fn context(&self) -> &C {
        &self.context
    }
    #[allow(unused)]
    pub(crate) fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

impl<C: Clone, L, R, EL, ER> Clone for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    fn clone(&self) -> Self {
        Self {
            to_left: self.to_left.clone(),
            to_right: self.to_right.clone(),
            context: self.context.clone(),
        }
    }
}

impl<C: Debug, L, R, EL, ER> Debug for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("BoxedFnConverterWithContext")
            .field("to_left", &"<closure>")
            .field("to_right", &"<closure>")
            .field("context", &self.context)
            .finish()
    }
}
