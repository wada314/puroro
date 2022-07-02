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

use crate::{Bool, Const, Func, B0, B1};
use ::std::marker::PhantomData;

#[macro_export]
macro_rules! make_list {
    () => {
        ()
    };
    ($a:ty$(, $rest:ty)* $(,)?) => {
        ($a, make_list!($($rest),*))
    };
}

pub trait List {
    type IsTerm: Bool;
    type Car;
    type Cdr: List;
}
impl List for () {
    type IsTerm = B1;
    type Car = ();
    type Cdr = ();
}
impl<T, U: List> List for (T, U) {
    type IsTerm = B0;
    type Car = T;
    type Cdr = U;
}

pub struct Map<L, F>(PhantomData<(L, F)>);
impl<L, F> List for Map<L, F>
where
    L: List,
    <L::IsTerm as Bool>::Then<Const<()>, F>: Func<L::Car>,
    <L::IsTerm as Bool>::Then<(), Map<L::Cdr, F>>: List,
{
    type IsTerm = L::IsTerm;
    type Car = <<L::IsTerm as Bool>::Then<Const<()>, F> as Func<L::Car>>::Type;
    type Cdr = <L::IsTerm as Bool>::Then<(), Map<L::Cdr, F>>;
}

pub struct IntoTupleListFunctor;
impl<L: List> Func<L> for IntoTupleListFunctor
where
    <L::IsTerm as Bool>::Then<Const<()>, IntoTupleListFunctor>: Func<L::Cdr>,
{
    type Type = <L::IsTerm as Bool>::Then<
        (),
        (
            L::Car,
            <<L::IsTerm as Bool>::Then<Const<()>, IntoTupleListFunctor> as Func<L::Cdr>>::Type,
        ),
    >;
}
pub trait IntoTupleList {
    type TupleList;
}
impl<L> IntoTupleList for L
where
    IntoTupleListFunctor: Func<L>,
{
    type TupleList = <IntoTupleListFunctor as Func<L>>::Type;
}
