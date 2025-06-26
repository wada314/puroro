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

use ::puroro::{Both, Either, EitherOrBoth};

#[derive(Default, Debug)]
pub struct A1 {
    pub b: Box<B1>,
}
#[derive(Default, Debug)]
pub struct B1 {
    pub c: Box<C1>,
}
#[derive(Default, Debug)]
pub struct C1 {
    pub b: Option<Box<B1>>,
    pub d: Box<D1>,
}
#[derive(Default, Debug)]
pub struct D1 {
    pub d: Option<Box<D1>>,
}
#[derive(Default, Debug)]
pub struct A2 {
    pub b: Box<B1>,
}
#[derive(Default, Debug)]
pub struct D2 {
    pub d: Option<Box<D2>>,
}

pub trait ScalarMsgFieldGetter<const N: i32> {
    type Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>>;
}
pub trait NonMsgFieldGetter<const N: i32> {
    type Value<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::Value<'_>;
}

impl<T: ScalarMsgFieldGetter<N>, const N: i32> ScalarMsgFieldGetter<N> for &T {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        T::get(self)
    }
}

impl<T: ScalarMsgFieldGetter<N>, const N: i32> ScalarMsgFieldGetter<N> for Option<T> {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().and_then(|x| x.get())
    }
}

impl<T: ScalarMsgFieldGetter<N>, U: ScalarMsgFieldGetter<N>, const N: i32> ScalarMsgFieldGetter<N>
    for Either<T, U>
{
    type Message<'a>
        = Either<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        match self {
            Either::Left(left) => left.get().map(Either::Left),
            Either::Right(right) => right.get().map(Either::Right),
        }
    }
}

impl<T: ScalarMsgFieldGetter<N>, U: ScalarMsgFieldGetter<N>, const N: i32> ScalarMsgFieldGetter<N>
    for EitherOrBoth<T, U>
{
    type Message<'a>
        = EitherOrBoth<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().map2(T::get, U::get).factor_none()
    }
}

impl<T: ScalarMsgFieldGetter<N>, U: ScalarMsgFieldGetter<N>, const N: i32> ScalarMsgFieldGetter<N>
    for Both<T, U>
{
    type Message<'a>
        = EitherOrBoth<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().map2(T::get, U::get).factor_none()
    }
}

impl ScalarMsgFieldGetter<1> for A1 {
    type Message<'a>
        = &'a dyn BView
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.b.as_ref() as &dyn BView)
    }
}

impl ScalarMsgFieldGetter<1> for B1 {
    type Message<'a>
        = &'a dyn CView
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.c.as_ref() as &dyn CView)
    }
}
impl ScalarMsgFieldGetter<1> for C1 {
    type Message<'a>
        = &'a dyn BView
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.b.as_deref().map(|x| x as &dyn BView)
    }
}
impl ScalarMsgFieldGetter<2> for C1 {
    type Message<'a>
        = &'a dyn DView
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_ref() as &dyn DView)
    }
}
impl ScalarMsgFieldGetter<1> for D1 {
    type Message<'a>
        = &'a dyn DView
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.d.as_deref().map(|x| x as &dyn DView)
    }
}

pub trait AView {
    fn b(&self) -> Option<&dyn BView>;
}
pub trait BView {
    fn c(&self) -> Option<&dyn CView>;
}
pub trait CView {
    fn b(&self) -> Option<&dyn BView>;
    fn d(&self) -> Option<&dyn DView>;
}
pub trait DView {
    fn d(&self) -> Option<&dyn DView>;
}

impl<T> AView for T
where
    for<'a> T: 'a + ScalarMsgFieldGetter<1, Message<'a> = &'a dyn BView>,
{
    fn b(&self) -> Option<&dyn BView> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<T> BView for T
where
    for<'a> T: 'a + ScalarMsgFieldGetter<1, Message<'a> = &'a dyn CView>,
{
    fn c(&self) -> Option<&dyn CView> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<T> CView for T
where
    for<'a> T: 'a + ScalarMsgFieldGetter<1, Message<'a> = &'a dyn BView>,
    for<'a> T: 'a + ScalarMsgFieldGetter<2, Message<'a> = &'a dyn DView>,
{
    fn b(&self) -> Option<&dyn BView> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<&dyn DView> {
        <Self as ScalarMsgFieldGetter<2>>::get(self)
    }
}

impl<T> DView for T
where
    for<'a> T: 'a + ScalarMsgFieldGetter<1, Message<'a> = &'a dyn DView>,
{
    fn d(&self) -> Option<&dyn DView> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

pub struct AMain<'a>(&'a dyn AView);
pub struct BMain<'a>(&'a dyn BView);
pub struct CMain<'a>(&'a dyn CView);
pub struct DMain<'a>(&'a dyn DView);

impl<'a> AMain<'a> {
    pub fn b(&self) -> Option<BMain<'a>> {
        self.0.b().map(BMain)
    }
}

impl<'a> BMain<'a> {
    pub fn c(&self) -> Option<CMain<'a>> {
        self.0.c().map(CMain)
    }
}

impl<'a> CMain<'a> {
    pub fn b(&self) -> Option<BMain<'a>> {
        self.0.b().map(BMain)
    }
    pub fn d(&self) -> Option<DMain<'a>> {
        self.0.d().map(DMain)
    }
}

impl<'a> DMain<'a> {
    pub fn d(&self) -> Option<DMain<'a>> {
        self.0.d().map(DMain)
    }
}

#[test]
fn foo() {
    let a = AMain(&A1::default());
    let b = BMain(&B1::default());
    let c = CMain(&C1::default());
    let d = DMain(&D1::default());

    // Test that the methods work through the user-facing types
    // Users don't need to know about View traits
    let _ = a.b();
    let _ = a.b().map(|x| x.c());
    let _ = b.c();
    let _ = b.c().map(|x| x.d());
    let _ = c.b();
    let _ = c.d();
    let _ = d.d();
    let _ = d.d().map(|x| x.d());
}

fn bar(d: &dyn DView) {
    let _ = d.d();
    let _ = d.d().unwrap().d();
}
