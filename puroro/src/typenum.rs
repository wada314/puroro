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

use ::std::marker::PhantomData;
use ::typenum::{UInt, UTerm, B0, B1, U0, U1, U2, U3};

trait Func {
    type R;
}

trait Bool {
    type If<T, F>;
}
impl Bool for B0 {
    type If<T, F> = F;
}
impl Bool for B1 {
    type If<T, F> = F;
}
struct Not<B>(PhantomData<B>);
impl<B: Bool> Bool for Not<B> {
    type If<T, F> = B::If<F, T>;
}
struct And<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for And<A, B> {
    type If<T, F> = A::If<B::If<T, F>, F>;
}
struct Or<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Or<A, B> {
    type If<T, F> = A::If<T, B::If<T, F>>;
}
struct Xor<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Xor<A, B> {
    type If<T, F> = A::If<B::If<F, T>, B::If<T, F>>;
}

trait ComparableNumber {
    type IsUterm: Bool;
    type Lower: Bool;
    type Higher: ComparableNumber;
}
impl<U: ComparableNumber, B: Bool> ComparableNumber for UInt<U, B> {
    type IsUterm = B0;
    type Lower = B;
    type Higher = U;
}
impl ComparableNumber for UTerm {
    type IsUterm = B1;
    type Lower = B0;
    type Higher = UTerm;
}

struct CmpEq<L, R>(PhantomData<(L, R)>);
impl<L: ComparableNumber, R: ComparableNumber> Bool for CmpEq<L, R> {
    type If<T, F> = <Or<
        And<L::IsUterm, R::IsUterm>,
        And<
            Not<Or<L::IsUterm, R::IsUterm>>,
            And<Not<Xor<L::Lower, R::Lower>>, CmpEq<L::Higher, R::Higher>>,
        >,
    > as Bool>::If<T, F>;
}

fn hoge() {
    fn except_u32(_v: u32) {}
    fn except_f32(_v: f32) {}
    let u: <CmpEq<U0, U0> as Bool>::If<u32, f32> = 0u32;
}
