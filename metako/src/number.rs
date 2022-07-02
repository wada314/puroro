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

use crate::{And, Bool, Func, Not, B0, B1};
use ::std::marker::PhantomData;
use ::typenum::{UInt, UTerm};

// We need this trait so that we can compare the
// type equality with ANY M:Number type.
// typenum's Cmp does not fulfill this condition...
pub trait Number {
    type Lsb: Bool;
    type Remains: Number;
    type IsUTerm: Bool;
    type Eq<M: Number>: Bool;
}
impl<U: Number> Number for UInt<U, B0> {
    type Lsb = B0;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: Number> = And<Not<M::Lsb>, <Self::Remains as Number>::Eq<M::Remains>>;
}
impl<U: Number> Number for UInt<U, B1> {
    type Lsb = B1;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: Number> = And<M::Lsb, <Self::Remains as Number>::Eq<M::Remains>>;
}
impl Number for UTerm {
    type Lsb = B0;
    type Remains = UTerm;
    type IsUTerm = B1;
    type Eq<M: Number> = M::IsUTerm;
}

pub struct IsNumberEqual<N>(PhantomData<N>);
impl<N, M> Func<M> for IsNumberEqual<N>
where
    M: Number,
    N: Number,
{
    type Type = M::Eq<N>;
}
