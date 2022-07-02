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

use crate::func::Functor;
use crate::func::{Const, Pred};
use crate::list::List;
use ::std::marker::PhantomData;
pub use ::typenum::{B0, B1};

pub trait Bool {
    type Then<T, F>;
}
impl Bool for B0 {
    type Then<T, F> = F;
}
impl Bool for B1 {
    type Then<T, F> = T;
}

pub struct Not<B>(PhantomData<B>);
pub struct And<P, Q>(PhantomData<(P, Q)>);
pub struct Or<P, Q>(PhantomData<(P, Q)>);

impl<B: Bool> Bool for Not<B> {
    type Then<T, F> = <B as Bool>::Then<F, T>;
}
impl<P: Bool, Q: Bool> Bool for And<P, Q> {
    type Then<T, F> = <P as Bool>::Then<<Q as Bool>::Then<T, F>, F>;
}
impl<P: Bool, Q: Bool> Bool for Or<P, Q> {
    type Then<T, F> = <P as Bool>::Then<T, <Q as Bool>::Then<T, F>>;
}

pub struct AnyOf<L>(PhantomData<L>);
impl<L> Bool for AnyOf<L>
where
    L: List,
    <L::IsTerm as Bool>::Then<B0, L::Car>: Bool,
    <L::IsTerm as Bool>::Then<B0, AnyOf<L::Cdr>>: Bool,
{
    type Then<T, F> = <Or<
        <L::IsTerm as Bool>::Then<B0, L::Car>,
        <L::IsTerm as Bool>::Then<B0, AnyOf<L::Cdr>>,
    > as Bool>::Then<T, F>;
}

pub struct AllOf<L>(PhantomData<L>);
impl<L> Bool for AllOf<L>
where
    L: List,
    <L::IsTerm as Bool>::Then<B1, L::Car>: Bool,
    <L::IsTerm as Bool>::Then<B1, AllOf<L::Cdr>>: Bool,
{
    type Then<T, F> = <And<
        <L::IsTerm as Bool>::Then<B1, L::Car>,
        <L::IsTerm as Bool>::Then<B1, AllOf<L::Cdr>>,
    > as Bool>::Then<T, F>;
}

pub struct SwitchFunctor<PredAndValueList>(PhantomData<PredAndValueList>);
impl<T, L, P, V, Succ> Functor<T> for SwitchFunctor<L>
where
    L: List,
    L::IsTerm: Bool<Then<(Const<B1>, ()), L::Car> = (P, V)>,
    P: Pred<T>,
    <L::IsTerm as Bool>::Then<Const<()>, SwitchFunctor<L::Cdr>>: Functor<T, Type = Succ>,
{
    type Type = <<P as Pred<T>>::Type as Bool>::Then<V, Succ>;
}
