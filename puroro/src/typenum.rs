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

use crate::internal as int;
use crate::internal::FieldType;
use ::std::marker::PhantomData;
use ::typenum::{UInt, UTerm, B0, B1, U0, U1, U2};

trait Bool {
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
struct Not<B>(PhantomData<B>);
impl<B: Bool> Bool for Not<B> {
    type If<T, F> = B::If<F, T>;
    type IfF<T: FieldType, F: FieldType> = B::IfF<F, T>;
}
struct And<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for And<A, B> {
    type If<T, F> = A::If<B::If<T, F>, F>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<B::IfF<T, F>, F>;
}
struct And3<A, B, C>(PhantomData<(A, B, C)>);
impl<A: Bool, B: Bool, C: Bool> Bool for And3<A, B, C> {
    type If<T, F> = <And<A, And<B, C>> as Bool>::If<T, F>;
    type IfF<T: FieldType, F: FieldType> = <And<A, And<B, C>> as Bool>::IfF<T, F>;
}
struct Or<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Or<A, B> {
    type If<T, F> = A::If<T, B::If<T, F>>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<T, B::IfF<T, F>>;
}
struct Xor<A, B>(PhantomData<(A, B)>);
impl<A: Bool, B: Bool> Bool for Xor<A, B> {
    type If<T, F> = A::If<B::If<F, T>, B::If<T, F>>;
    type IfF<T: FieldType, F: FieldType> = A::IfF<B::IfF<F, T>, B::IfF<T, F>>;
}

trait Comparable {
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
struct Cmp<A, B>(PhantomData<(A, B)>);
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

trait GenericMessage {
    type FieldType<'a, N: Comparable>: FieldType
    where
        Self: 'a;
}
trait GenericField {}

impl GenericMessage for () {
    type FieldType<'a, N: Comparable> = () where Self: 'a;
}
impl GenericField for () {}

#[derive(Default)]
struct PersonMessage {
    fields: PersonMessageField,
    bitfield: int::BitArray<1>,
    unknown_fields: int::UnknownFieldsImpl,
}
#[derive(Default)]
struct PersonMessageField {
    partner: int::SingularHeapMessageField<PersonMessage>,
}
impl crate::Message for PersonMessage {
    fn from_bytes_iter<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> crate::Result<Self> {
        todo!()
    }
    fn merge_from_bytes_iter<I: Iterator<Item = std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> crate::Result<()> {
        todo!()
    }
    fn to_bytes<W: std::io::Write>(&self, out: &mut W) -> crate::Result<()> {
        todo!()
    }
}
struct FieldRef<'a, F, B, U>(&'a F, &'a B, &'a U);
impl<'a, F, B, U> GenericField for FieldRef<'a, F, B, U>
where
    F: int::FieldType,
    B: int::BitSlice,
    U: int::UnknownFields,
{
}

impl GenericMessage for PersonMessage {
    type FieldType<'a, N: Comparable> =
        FieldRef<'a, <Cmp<U1, N> as Bool>::IfF<int::SingularHeapMessageField<PersonMessage>, ()>,
        int::BitArray<1>, int::UnknownFieldsImpl> where Self: 'a;
}
