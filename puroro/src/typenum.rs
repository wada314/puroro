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

//! An extention to typenum crate.

use crate::internal::FieldType;
use ::std::marker::PhantomData;
use ::typenum::{UInt, UTerm, B0, B1};

pub trait Bool {
    type If<T, F>;
    type IfF<T: FieldType, F: FieldType>: FieldType;
}
impl Bool for B0 {
    type If<T, F> = F;
    type IfF<T: FieldType, F: FieldType> = F;
}
impl Bool for B1 {
    type If<T, F> = T;
    type IfF<T: FieldType, F: FieldType> = T;
}
pub struct Not<B>(PhantomData<B>);
impl<B: Bool> Bool for Not<B> {
    type If<T, F> = B::If<F, T>;
    type IfF<T: FieldType, F: FieldType> = B::IfF<F, T>;
}
pub struct And<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for And<A, B> {
    type If<T, F> = A::If<B::If<T, F>, F>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<B::IfF<T, F>, F>;
}
pub struct And3<A, B, C>(PhantomData<(A, B, C)>);
impl<A: Bool, B: Bool, C: Bool> Bool for And3<A, B, C> {
    type If<T, F> = <And<A, And<B, C>> as Bool>::If<T, F>;
    type IfF<T: FieldType, F: FieldType> = <And<A, And<B, C>> as Bool>::IfF<T, F>;
}
pub struct Or<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Or<A, B> {
    type If<T, F> = A::If<T, B::If<T, F>>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<T, B::IfF<T, F>>;
}
pub struct Xor<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Xor<A, B> {
    type If<T, F> = A::If<B::If<F, T>, B::If<T, F>>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<B::IfF<F, T>, B::IfF<T, F>>;
}

pub trait Comparable {
    type Eq<RHS: Comparable>: Bool;
    type IsTerm: Bool;
    type Lo: Bool;
    type Hi: Comparable;
}
impl Comparable for UTerm {
    type Eq<RHS: Comparable> = RHS::IsTerm;
    type IsTerm = B1;
    type Lo = B0;
    type Hi = UTerm;
}
impl<L: Bool, H: Comparable> Comparable for UInt<H, L> {
    type Eq<RHS: Comparable> = And3<Not<RHS::IsTerm>, Not<Xor<L, RHS::Lo>>, H::Eq<RHS::Hi>>;
    type IsTerm = B0;
    type Lo = L;
    type Hi = H;
}
pub struct Cmp<A, B>(PhantomData<(A, B)>);
impl<A: Comparable, B: Comparable> Bool for Cmp<A, B> {
    type If<T, F> = <A::Eq<B> as Bool>::If<T, F>;
    type IfF<T: FieldType, F: FieldType> = <A::Eq<B> as Bool>::IfF<T, F>;
}

#[test]
fn hoge() {
    let _: u32 = <Cmp<U0, U0> as Bool>::If::<u32, f32>::default();
    let _: u32 = <Cmp<U1, U1> as Bool>::If::<u32, f32>::default();
    let _: u32 = <Cmp<U2, U2> as Bool>::If::<u32, f32>::default();
    let _: f32 = <Cmp<U0, U1> as Bool>::If::<u32, f32>::default();
    let _: f32 = <Cmp<U1, U2> as Bool>::If::<u32, f32>::default();
    let _: f32 = <Cmp<U2, U0> as Bool>::If::<u32, f32>::default();
}
