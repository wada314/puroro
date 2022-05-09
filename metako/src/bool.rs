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

use crate::func::Func;
use crate::func::Pred;
pub use ::typenum::{B0, B1};

pub trait If {
    type Type<T, F>;
    type Not: If;
    type And<T: If>: If;
    type Or<T: If>: If;
}
impl If for B0 {
    type Type<T, F> = F;
    type Not = B1;
    type And<T: If> = B0;
    type Or<T: If> = T;
}
impl If for B1 {
    type Type<T, F> = T;
    type Not = B0;
    type And<T: If> = T;
    type Or<T: If> = B1;
}

pub struct AnyOf;
impl Func<()> for AnyOf {
    type Type = B0;
}
impl<T, U> Func<(T, U)> for AnyOf
where
    T: If,
    AnyOf: Pred<U>,
{
    type Type = T::Or<<AnyOf as Pred<U>>::Type>;
}

pub struct AllOf;
impl Func<()> for AllOf {
    type Type = B1;
}
impl<T, U> Func<(T, U)> for AllOf
where
    T: If,
    AllOf: Pred<U>,
{
    type Type = T::And<<AllOf as Pred<U>>::Type>;
}
