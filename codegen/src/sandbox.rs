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

use puroro::{Both, Either, EitherOrBoth};

#[derive(Default, Debug)]
pub struct MsgStruct<T>(T);

#[derive(Default, Debug)]
pub struct A1 {
    pub b: Box<MsgStruct<B1>>,
}
#[derive(Default, Debug)]
pub struct B1 {
    pub c: Box<MsgStruct<C1>>,
}
#[derive(Default, Debug)]
pub struct C1 {
    pub b: Option<Box<MsgStruct<B1>>>,
    pub d: Box<MsgStruct<D1>>,
}
#[derive(Default, Debug)]
pub struct D1 {
    pub d: Option<Box<MsgStruct<D1>>>,
}
#[derive(Default, Debug)]
pub struct A2 {
    pub b: Box<MsgStruct<B1>>,
}
#[derive(Default, Debug)]
pub struct D2 {
    pub d: Option<Box<MsgStruct<D2>>>,
}

pub trait ScalarMsgFieldGetter<const N: i32> {
    type Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>>;
}
pub trait RepeatedMsgFieldGetter<const N: i32> {
    type Message<'a>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>>;
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

impl<T: RepeatedMsgFieldGetter<N>, const N: i32> RepeatedMsgFieldGetter<N> for &T {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>> {
        T::get(self)
    }
}

impl<T: RepeatedMsgFieldGetter<N>, const N: i32> RepeatedMsgFieldGetter<N> for Option<T> {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>> {
        self.as_ref().map(T::get).into_iter().flatten()
    }
}

impl<T: RepeatedMsgFieldGetter<N>, U: RepeatedMsgFieldGetter<N>, const N: i32>
    RepeatedMsgFieldGetter<N> for Either<T, U>
{
    type Message<'a>
        = Either<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>> {
        self.as_ref().map2(T::get, U::get).into_iter_either()
    }
}

impl<T: RepeatedMsgFieldGetter<N>, U: RepeatedMsgFieldGetter<N>, const N: i32>
    RepeatedMsgFieldGetter<N> for EitherOrBoth<T, U>
{
    type Message<'a>
        = Either<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>> {
        self.as_ref().map2(T::get, U::get).into_iter_either()
    }
}

impl<T: RepeatedMsgFieldGetter<N>, U: RepeatedMsgFieldGetter<N>, const N: i32>
    RepeatedMsgFieldGetter<N> for Both<T, U>
{
    type Message<'a>
        = Either<T::Message<'a>, U::Message<'a>>
    where
        Self: 'a;
    fn get(&self) -> impl Iterator<Item = Self::Message<'_>> {
        self.as_ref().map2(T::get, U::get).into_iter_either()
    }
}

impl ScalarMsgFieldGetter<1> for MsgStruct<A1> {
    type Message<'a>
        = &'a dyn BView<Registry = RegistryImpl>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.b.as_ref() as &dyn BView)
    }
}

impl ScalarMsgFieldGetter<1> for MsgStruct<B1> {
    type Message<'a>
        = &'a dyn CView<Registry = RegistryImpl>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.c.as_ref() as &dyn CView)
    }
}
impl ScalarMsgFieldGetter<1> for MsgStruct<C1> {
    type Message<'a>
        = &'a dyn BView<Registry = RegistryImpl>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.b.as_deref().map(|x| x as &dyn BView)
    }
}
impl ScalarMsgFieldGetter<2> for MsgStruct<C1> {
    type Message<'a>
        = &'a dyn DView<Registry = RegistryImpl>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_ref() as &dyn DView)
    }
}
impl ScalarMsgFieldGetter<1> for MsgStruct<D1> {
    type Message<'a>
        = &'a dyn DView<Registry = RegistryImpl>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.d.as_deref().map(|x| x as &dyn DView)
    }
}

pub trait Registry {
    type AView<'a>: AView
    where
        Self: 'a;
    type BView<'a>: BView
    where
        Self: 'a;
    type CView<'a>: CView
    where
        Self: 'a;
    type DView<'a>: DView
    where
        Self: 'a;
}
pub struct RegistryImpl;
impl Registry for RegistryImpl {
    type AView<'a> = &'a dyn AView<Registry = RegistryImpl>;
    type BView<'a> = &'a dyn BView<Registry = RegistryImpl>;
    type CView<'a> = &'a dyn CView<Registry = RegistryImpl>;
    type DView<'a> = &'a dyn DView<Registry = RegistryImpl>;
}

pub trait AView {
    type Registry: Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>>;
}
pub trait BView {
    type Registry: Registry;
    fn c(&self) -> Option<<Self::Registry as Registry>::CView<'_>>;
}
pub trait CView {
    type Registry: Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>>;
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>>;
}
pub trait DView {
    type Registry: Registry;
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>>;
}

impl<'a, T: 'a> AView for MsgStruct<T>
where
    Self: ScalarMsgFieldGetter<1, Message<'a> = &'a dyn BView<Registry = RegistryImpl>>,
{
    type Registry = RegistryImpl;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<T: ?Sized + AView> AView for &T {
    type Registry = RegistryImpl;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<'a, T: 'a> BView for MsgStruct<T>
where
    Self: ScalarMsgFieldGetter<1, Message<'a> = &'a dyn CView<Registry = RegistryImpl>>,
{
    type Registry = RegistryImpl;
    fn c(&self) -> Option<<Self::Registry as Registry>::CView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<T: ?Sized + BView> BView for &T {
    type Registry = RegistryImpl;
    fn c(&self) -> Option<<Self::Registry as Registry>::CView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<'a, T: 'a> CView for MsgStruct<T>
where
    Self: ScalarMsgFieldGetter<1, Message<'a> = &'a dyn BView<Registry = RegistryImpl>>,
    Self: ScalarMsgFieldGetter<2, Message<'a> = &'a dyn DView<Registry = RegistryImpl>>,
{
    type Registry = RegistryImpl;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>> {
        <Self as ScalarMsgFieldGetter<2>>::get(self)
    }
}

impl<T: ?Sized + CView> CView for &T {
    type Registry = RegistryImpl;
    fn b(&self) -> Option<<Self::Registry as Registry>::BView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>> {
        <Self as ScalarMsgFieldGetter<2>>::get(self)
    }
}

impl<'a, T: 'a> DView for MsgStruct<T>
where
    Self: ScalarMsgFieldGetter<1, Message<'a> = &'a dyn DView<Registry = RegistryImpl>>,
{
    type Registry = RegistryImpl;
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

impl<T: ?Sized + DView> DView for &T {
    type Registry = RegistryImpl;
    fn d(&self) -> Option<<Self::Registry as Registry>::DView<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

pub struct AMain<T: AView>(T);
pub struct BMain<T: BView>(T);
pub struct CMain<T: CView>(T);
pub struct DMain<T: DView>(T);

impl<T: AView> AMain<T> {
    pub fn b(&self) -> Option<BMain<impl BView>> {
        self.0.b().map(BMain)
    }
}

impl<T: BView> BMain<T> {
    pub fn c(&self) -> Option<CMain<impl CView>> {
        self.0.c().map(CMain)
    }
}

impl<T: CView> CMain<T> {
    pub fn b(&self) -> Option<BMain<impl BView>> {
        self.0.b().map(BMain)
    }
    pub fn d(&self) -> Option<DMain<impl DView>> {
        self.0.d().map(DMain)
    }
}

impl<T: DView> DMain<T> {
    pub fn d(&self) -> Option<DMain<impl DView>> {
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

fn bar(d: &dyn DView<Registry = RegistryImpl>) {
    let _ = d.d();
    let _ = d.d().unwrap().d();
}
