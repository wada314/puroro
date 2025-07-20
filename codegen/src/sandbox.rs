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
//!     Address address = 3;
//!     repeated Person children = 4;
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
//! ## The Message Type Field Getter Problem
//!
//! When implementing field getters for message type fields in protobuf, we need to define what the getter
//! methods in the `View` trait should return. For example, consider a simple field getter for a scalar
//! message field (`.address` in the above example) of `Person` message type.
//! What should the trait's getter method return type be?
//!
//! The typical approaches would be:
//!
//! - `Option<&dyn AddressView>`
//! - `Option<&impl AddressView>`
//! - `Option<Rc<dyn AddressView>>`
//!
//! Each has some pros and cons. But every these approach have a same issue that
//! it can not return a by-value type.
//! Actually this is quite a problem for certain use cases.
//! For example, a tuple type like `(T, U)` where `T` and `U` are both `AddressView` type
//! will need to return a tuple of `(T.address(), U.address())` by value, but the all examples above
//! can not do this.
//!
//! To solve this problem, we propose a Generic Associated Type (GAT) field getter approach:
//!
//! ```rust
//! pub trait PersonView {
//!     type Address<'a>: AddressView
//!     where
//!         Self: 'a;
//!     fn address(&self) -> Self::Address<'_>;
//! }
//!
//! // with the following blanket implementation to consider the `&T` type can also be a `AddressView` type
//! // if `T` implements the `AddressView` trait.
//! impl<T: AddressView> AddressView for &T { /* ... */ }
//! ```
//!
//! This design provides the flexibility to return a by-value type,
//! or a reference type (via the `&T` type's blanket implementation).
//!

use puroro::{Both, Either, EitherOrBoth};

pub trait ScalarMsgFieldGetter<const N: i32> {
    type Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>>;
}

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

// Low-level field getter implementations, produced by a code generator.
impl ScalarMsgFieldGetter<1> for A1 {
    type Message<'a> = &'a B1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.b.as_ref())
    }
}
impl ScalarMsgFieldGetter<1> for B1 {
    type Message<'a> = &'a C1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.c.as_ref())
    }
}
impl ScalarMsgFieldGetter<1> for C1 {
    type Message<'a> = Option<&'a B1>;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.b.as_deref())
    }
}
impl ScalarMsgFieldGetter<2> for C1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_ref())
    }
}
impl ScalarMsgFieldGetter<1> for D1 {
    type Message<'a> = Option<&'a D1>;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_deref())
    }
}

// Blanket implementations for low-level getters on wrapper types.
// These would be part of the puroro library.
impl<'s, const N: i32, T: ?Sized + ScalarMsgFieldGetter<N>> ScalarMsgFieldGetter<N> for &'s T {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        T::get(self)
    }
}
impl<const N: i32, T: ScalarMsgFieldGetter<N>> ScalarMsgFieldGetter<N> for Option<T> {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().and_then(|v| v.get())
    }
}

// High-level, named `View` traits.
// They require the low-level getter trait as a supertrait and use a `where`
// clause to ensure the getter's return type implements the correct child `View`.
// The method implementations are provided by default.
pub trait AView: ScalarMsgFieldGetter<1>
where
    for<'a> <Self as ScalarMsgFieldGetter<1>>::Message<'a>: BView,
{
    fn b(&self) -> Option<<Self as ScalarMsgFieldGetter<1>>::Message<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}
pub trait BView: ScalarMsgFieldGetter<1>
where
    for<'a> <Self as ScalarMsgFieldGetter<1>>::Message<'a>: CView,
{
    fn c(&self) -> Option<<Self as ScalarMsgFieldGetter<1>>::Message<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}
pub trait CView: ScalarMsgFieldGetter<1> + ScalarMsgFieldGetter<2>
where
    for<'a> <Self as ScalarMsgFieldGetter<1>>::Message<'a>: BView,
    for<'a> <Self as ScalarMsgFieldGetter<2>>::Message<'a>: DView,
{
    fn b(&self) -> Option<<Self as ScalarMsgFieldGetter<1>>::Message<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<<Self as ScalarMsgFieldGetter<2>>::Message<'_>> {
        <Self as ScalarMsgFieldGetter<2>>::get(self)
    }
}
pub trait DView: ScalarMsgFieldGetter<1>
where
    for<'a> <Self as ScalarMsgFieldGetter<1>>::Message<'a>: DView,
{
    fn d(&self) -> Option<<Self as ScalarMsgFieldGetter<1>>::Message<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

// The code generator now only needs to output these empty marker impls!
impl AView for A1 {}
impl BView for B1 {}
impl CView for C1 {}
impl DView for D1 {}

// Blanket implementations for high-level views on wrapper types are also empty.
impl<'s, T: ?Sized + AView> AView for &'s T {}
impl<T: AView> AView for Option<T> {}
impl<'s, T: ?Sized + BView> BView for &'s T {}
impl<T: BView> BView for Option<T> {}
impl<'s, T: ?Sized + CView> CView for &'s T {}
impl<T: CView> CView for Option<T> {}
impl<'s, T: ?Sized + DView> DView for &'s T {}
impl<T: DView> DView for Option<T> {}

pub struct AMain<T: AView>(T);
pub struct BMain<T: BView>(T);
pub struct CMain<T: CView>(T);
pub struct DMain<T: DView>(T);

impl<T: AView> AMain<T> {
    pub fn b(&self) -> Option<BMain<<T as ScalarMsgFieldGetter<1>>::Message<'_>>> {
        self.0.b().map(BMain)
    }
}

impl<T: BView> BMain<T> {
    pub fn c(&self) -> Option<CMain<<T as ScalarMsgFieldGetter<1>>::Message<'_>>> {
        self.0.c().map(CMain)
    }
}

impl<T: CView> CMain<T> {
    pub fn b(&self) -> Option<BMain<<T as ScalarMsgFieldGetter<1>>::Message<'_>>> {
        self.0.b().map(BMain)
    }
    pub fn d(&self) -> Option<DMain<<T as ScalarMsgFieldGetter<2>>::Message<'_>>> {
        self.0.d().map(DMain)
    }
}

impl<T: DView> DMain<T> {
    pub fn d(&self) -> Option<DMain<<T as ScalarMsgFieldGetter<1>>::Message<'_>>> {
        self.0.d().map(DMain)
    }
}

#[test]
fn foo() {
    let a = AMain(A1::default());
    let b = BMain(B1::default());
    let c = CMain(C1::default());
    let d = DMain(D1::default());

    // Test that the methods work through the user-facing types
    // Users don't need to know about View traits
    let _ = a.b();
    let c_from_a = a.b().and_then(|b_impl| b_impl.c());
    let _ = b.c();
    let d_from_b = b.c().and_then(|c_impl| c_impl.d());
    let _ = c.b();
    let _ = c.d();
    let _ = d.d();
    let d_from_d = d.d().and_then(|d_impl| d_impl.d());

    // Test blanket impl for Option<T>
    let a_opt = AMain(Some(A1::default()));
    let c_from_a_opt = a_opt.b().and_then(|b_impl| b_impl.c());
}

// The GAT-based traits are not object-safe by default so `dyn View` cannot be used.
// We've removed the `bar` function which relied on this.
