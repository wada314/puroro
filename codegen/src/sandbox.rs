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

//! A temporary file for testing the design of the protobuf code generator.
//!
//! Assuming the following protobuf definition:
//! ```protobuf
//! message Person {
//!     string name = 1;
//!     uint32 age = 2;
//!     repeated Person children = 3;
//! }
//! ```
//!
//! This file contains the expected output of the code generator,
//! and potential puroro library code which will be used to implement the code generator.
//!
//! # Target of the design
//!
//! * Supporting the protobuf features as much as possible.
//! * 2 kinds of the immutable traits corresponding to the
//! protobuf message types: fallible and infallible.
//!   * Let's call them a `View` trait and a `TryView` trait.
//!   * The "default" implementation should implement the `View` trait.
//!   * Though, it is true that if we support the fallible type, then there are
//!     many extensive implementations of the protobuf is possible. e.g. Lazy type,
//!     generic protobuf message type ([`::puroro::DynamicMessage`]), etc.
//! * We provide at least 2 kinds of the implementations for the `TryView` trait:
//!   * The "default" implementation, which is an implementation optimized for the
//!     given protobuf message type, and the getter methods are infallible.
//!     * Implements the both `View` and `TryView` traits.
//!   * The "generic" implementation, which uses the common implementation for the
//!     all message types. As a natural consequence, the getter methods are fallible.
//!     * Implements the `TryView` trait.
//! * It's great if we can minimize the size of the generated code, but not a first priority.
//!   * But at most O(N) length please...
//! * We do not rely on the `dyn` trait, at least for the default implementation.
//! * For the `View` and `TryView` traits, we provide the blanket implementations for:
//!   * The `&T` and `&mut T` types,
//!   * The `Option<T>` type,
//!   * The `Either<T, U>`, `EitherOrBoth<T, U>`, and `Both<T, U>` types from the `quither` crate,
//! * The `View` and `TryView` traits should be open for the library users to implement.
//! * Support for the allocators (allocator_api2 crate?), but we can do that later.
//!
//! # Design details
//!
//! ## Infallible field getters
//!
//! We define a `FieldGetter` trait with the `const i32` generic parameter for the field number,
//! and a getter result type as another generic parameter.
//!
//! * A single field MAY have multiple getter types for a single field.
//!   For example, a fixed32 protobuf field can be converted to a u32, i32 or f32.
//! * The getter result type is a Rust type, not a protobuf type.
//!   In perspective of this getter, protobuf's `int32` and `sfixed32` fields are the same.
//! * For the message type field, the return type would be (an option or an iterator of) `impl PersonView`.
//!
//! ### The `View` trait recursive definition problem
//!
//! To make the generated code as simple as possible, we want to implement the `View` trait
//! automatically for the message type which implements the `FieldGetter` trait
//! with the predefined field numbers and the result types.
//!
//! The problem is, though, a message type can have itself as a field.
//! If we, say, going to define the `PersonView` trait for the `Person` message type we defined above,
//! then it requires the 3rd field to return a type which implements the `PersonView` trait.
//! And normally the `Person` rust type itself and its 3rd field's `Person` rust type are the same.
//!
//! Thus, we need the `Person` rust type to implement the`PersonView` trait
//! to make the `Person` type to implement the `PersonView` trait!
//!
//! ### An `同値類` of the message type
//!
//! To solve the recursive definition problem, we use the idea of the `同値類`.
//! ある一群のproto message typeが与えられたとき、それらの間の同値 (operator ~)・subset・superset関係を次のように定義する：
//!
//! For a pair of proto message `A` and `B`,
//!   * if `A` and `B` can be a (maybe indirect) child message of each other,
//!     then we say that `A` and `B` are equivalent.
//!   * if `A` can be a child message of `B`, but `B` can not be a child message of `A`,
//!     then we say that `A` is a superset of `B`, and `B` is a subset of `A`.
//!
//! この同値関係を用いて、商集合 _S/~_ と、その要素である同値類 _Si_ を定義できる。
//! つまり、_Si_ は proto message typeの集合でもある。
//!
//! 例として、次のようなprotobuf messagesがあるとする。
//!
//! ```protobuf
//! message A {
//!   B b = 1;
//! }
//! mesasge B {
//!   C c = 1;
//! }
//! message C {
//!   B b = 1;
//!   D d = 2;
//! }
//! message D {
//!   D d = 1;
//! }
//! ```
//!
//!
//! このとき、BとCは互いが互いの子であるから、同値である。 つまり、BとCは同じ同値類に属する。
//! 最終的に、このケースでは3つの同値類、{A}, {B, C}, {D} が定義される。
//!
//!

pub struct A {
    pub b: Box<B>,
}
pub struct B {
    pub c: Box<C>,
}
pub struct C {
    pub b: Box<B>,
    pub d: Box<D>,
}
pub struct D {
    pub d: Box<D>,
}
pub struct A2 {
    pub b: Box<B>,
}

pub trait AView {
    fn b(&self) -> &impl BView;
}
pub trait BView {
    fn c(&self) -> &impl CView;
}
pub trait CView {
    fn b(&self) -> &impl BView;
    fn d(&self) -> &impl DView;
}
pub trait DView {
    fn d(&self) -> &impl DView;
}

pub trait MsgFieldGetter<const N: i32> {
    type Message;
    fn get(&self) -> &Self::Message;
}

impl MsgFieldGetter<1> for A {
    type Message = B;
    fn get(&self) -> &B {
        &self.b
    }
}
impl MsgFieldGetter<1> for B {
    type Message = C;
    fn get(&self) -> &C {
        &self.c
    }
}
impl MsgFieldGetter<1> for C {
    type Message = B;
    fn get(&self) -> &B {
        &self.b
    }
}
impl MsgFieldGetter<2> for C {
    type Message = D;
    fn get(&self) -> &D {
        &self.d
    }
}
impl MsgFieldGetter<1> for D {
    type Message = D;
    fn get(&self) -> &D {
        &self.d
    }
}

impl MsgFieldGetter<1> for A2 {
    type Message = B;
    fn get(&self) -> &B {
        &self.b
    }
}

fn foo(a: A, b: B, c: C, d: D, a2: A2) {
    let _ = <B as BView>::c(&b);
    let _ = <D as DView>::d(&d);
}

pub trait EquivA: EquivBC {
    type A: AView;
}
pub trait EquivBC: EquivD {
    type B: BView;
    type C: CView;
}
pub trait EquivD {
    type D: DView;
}

pub struct SomeImpl;

impl EquivA for SomeImpl {
    type A = A;
}
impl EquivBC for SomeImpl {
    type B = B;
    type C = C;
}
impl EquivD for SomeImpl {
    type D = D;
}

pub struct SomeImpl2;
impl EquivA for SomeImpl2 {
    type A = A2;
}
impl EquivBC for SomeImpl2 {
    type B = B;
    type C = C;
}
impl EquivD for SomeImpl2 {
    type D = D;
}

pub trait Equiv {
    type Equiv;
}
impl Equiv for A {
    type Equiv = SomeImpl;
}
impl Equiv for B {
    type Equiv = SomeImpl;
}
impl Equiv for C {
    type Equiv = SomeImpl;
}
impl Equiv for D {
    type Equiv = SomeImpl;
}

impl<T> AView for T
where
    T: Equiv,
    <T as Equiv>::Equiv: EquivBC,
    T: MsgFieldGetter<1, Message = <<T as Equiv>::Equiv as EquivBC>::B>,
{
    fn b(&self) -> &impl BView {
        self.get()
    }
}
impl<T> BView for T
where
    T: Equiv,
    <T as Equiv>::Equiv: EquivBC,
    T: MsgFieldGetter<1, Message = <<T as Equiv>::Equiv as EquivBC>::C>,
{
    fn c(&self) -> &impl CView {
        self.get()
    }
}
impl<T> CView for T
where
    T: Equiv,
    <T as Equiv>::Equiv: EquivBC + EquivD,
    T: MsgFieldGetter<1, Message = <<T as Equiv>::Equiv as EquivBC>::B>,
    T: MsgFieldGetter<2, Message = <<T as Equiv>::Equiv as EquivD>::D>,
{
    fn b(&self) -> &impl BView {
        <T as MsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> &impl DView {
        <T as MsgFieldGetter<2>>::get(self)
    }
}
impl<T> DView for T
where
    T: Equiv,
    <T as Equiv>::Equiv: EquivD,
    T: MsgFieldGetter<1, Message = <<T as Equiv>::Equiv as EquivD>::D>,
{
    fn d(&self) -> &impl DView {
        <T as MsgFieldGetter<1>>::get(self)
    }
}

// impl<T> AView for T
// where
//     T: MsgFieldGetter<1>,
//     <T as MsgFieldGetter<1>>::Message: BView,
// {
//     fn b(&self) -> &impl BView {
//         self.get()
//     }
// }
// impl<T> BView for T
// where
//     T: MsgFieldGetter<1>,
//     <T as MsgFieldGetter<1>>::Message: CView,
// {
//     fn c(&self) -> &impl CView {
//         self.get()
//     }
// }
// impl<T> CView for T
// where
//     T: MsgFieldGetter<1>,
//     <T as MsgFieldGetter<1>>::Message: BView,
//     T: MsgFieldGetter<2>,
//     <T as MsgFieldGetter<2>>::Message: DView,
// {
//     fn b(&self) -> &impl BView {
//         <T as MsgFieldGetter<1>>::get(self)
//     }
//     fn d(&self) -> &impl DView {
//         <T as MsgFieldGetter<2>>::get(self)
//     }
// }
// impl<T> DView for T
// where
//     T: MsgFieldGetter<1>,
//     <T as MsgFieldGetter<1>>::Message: DView,
// {
//     fn d(&self) -> &impl DView {
//         self.get()
//     }
// }
