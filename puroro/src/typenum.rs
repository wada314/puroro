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
use ::typenum::{UInt, UTerm, B0, B1};

pub trait Bool {
    type If<T, F>;
    type IfF<T: FieldType, F: FieldType>: FieldType;
    fn r#if<T, F>(t: T, f: F) -> Self::If<T, F>;
    fn if_f<'a, T: FieldType, F: FieldType>(t: &'a T, f: &'a F) -> &'a Self::IfF<T, F>;

    type Not: Bool;
    type And<T: Bool>: Bool;
    type And3<T: Bool, U: Bool>: Bool;
    type Or<T: Bool>: Bool;
    type Xor<T: Bool>: Bool;
}
impl Bool for B0 {
    type If<T, F> = F;
    type IfF<T: FieldType, F: FieldType> = F;
    fn r#if<T, F>(_t: T, f: F) -> Self::If<T, F> {
        f
    }
    fn if_f<'a, T: FieldType, F: FieldType>(t: &'a T, f: &'a F) -> &'a Self::IfF<T, F> {
        f
    }

    type Not = B1;
    type And<T: Bool> = B0;
    type And3<T: Bool, U: Bool> = B0;
    type Or<T: Bool> = T;
    type Xor<T: Bool> = T;
}
impl Bool for B1 {
    type If<T, F> = T;
    type IfF<T: FieldType, F: FieldType> = T;
    fn r#if<T, F>(t: T, _f: F) -> Self::If<T, F> {
        t
    }
    fn if_f<'a, T: FieldType, F: FieldType>(t: &'a T, f: &'a F) -> &'a Self::IfF<T, F> {
        t
    }

    type Not = B0;
    type And<T: Bool> = T;
    type And3<T: Bool, U: Bool> = T::And<U>;
    type Or<T: Bool> = B1;
    type Xor<T: Bool> = T::Not;
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
    type Eq<RHS: Comparable> =
        <H::Eq<RHS::Hi> as Bool>::And3<<RHS::IsTerm as Bool>::Not, <L::Xor<RHS::Lo> as Bool>::Not>;
    type IsTerm = B0;
    type Lo = L;
    type Hi = H;
}

#[test]
fn hoge() {
    let _: u32 = <U0::Eq<U0> as Bool>::If::<u32, f32>::default();
    let _: u32 = <U1::Eq<U1> as Bool>::If::<u32, f32>::default();
    let _: u32 = <U2::Eq<U2> as Bool>::If::<u32, f32>::default();
    let _: f32 = <U0::Eq<U1> as Bool>::If::<u32, f32>::default();
    let _: f32 = <U1::Eq<U2> as Bool>::If::<u32, f32>::default();
    let _: f32 = <U2::Eq<U0> as Bool>::If::<u32, f32>::default();
}
