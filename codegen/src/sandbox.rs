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
use std::ops::Deref;

// The low-level, number-based getter trait.
pub trait ScalarMsgFieldGetter<const N: i32> {
    type Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>>;
}

// 1. The Registry Trait with GATs
// This is the central piece that connects all message families.
// It defines what concrete types correspond to the views.
pub trait Registry {
    type A<'a>: AView<Registry = Self>
    where
        Self: 'a;
    type B<'a>: BView<Registry = Self>
    where
        Self: 'a;
    type C<'a>: CView<Registry = Self>
    where
        Self: 'a;
    type D<'a>: DView<Registry = Self>
    where
        Self: 'a;
}

// 2. The View Traits, Generic over the Registry
// They are generic over a `Registry` type. This breaks the cycle.
// `AView` does not know about `BView` directly, only through `R::B`.
pub trait AView: Sized {
    type Registry: Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>>;
}
pub trait BView: Sized {
    type Registry: Registry;
    fn c(&self) -> Option<<Self::Registry as Registry>::C<'_>>;
}
pub trait CView: Sized {
    type Registry: Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>>;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>>;
}
pub trait DView: Sized {
    type Registry: Registry;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>>;
}

// 3. Define Concrete Message Structs
#[derive(Default, Debug, Clone)]
pub struct A1 {
    pub b: Box<B1>,
}
#[derive(Default, Debug, Clone)]
pub struct B1 {
    pub c: Box<C1>,
}
#[derive(Default, Debug, Clone)]
pub struct C1 {
    pub b: Option<Box<B1>>,
    pub d: Box<D1>,
}
#[derive(Default, Debug, Clone)]
pub struct D1 {
    pub d: Option<Box<D1>>,
}

// 4. Implement the low-level getters for concrete structs (codegen output)
impl ScalarMsgFieldGetter<1> for A1 {
    type Message<'a> = &'a B1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.b)
    }
}
impl ScalarMsgFieldGetter<1> for B1 {
    type Message<'a> = &'a C1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.c)
    }
}
impl ScalarMsgFieldGetter<1> for C1 {
    type Message<'a> = &'a B1;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.b.as_deref()
    }
}
impl ScalarMsgFieldGetter<2> for C1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.d)
    }
}
impl ScalarMsgFieldGetter<1> for D1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.d.as_deref()
    }
}

// 5. Define a Concrete Registry for this family of structs
pub enum MyFamily {}
impl Registry for MyFamily {
    type A<'a> = &'a A1;
    type B<'a> = &'a B1;
    type C<'a> = &'a C1;
    type D<'a> = &'a D1;
}

// 6. Implement the View traits by delegating to the low-level getters (codegen output)
impl AView for A1 {
    type Registry = MyFamily;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        // The cast `as _` is needed to satisfy the GAT lifetime from the Registry
        ScalarMsgFieldGetter::<1>::get(self).map(|v| v as _)
    }
}
impl BView for B1 {
    type Registry = MyFamily;
    fn c(&self) -> Option<<Self::Registry as Registry>::C<'_>> {
        ScalarMsgFieldGetter::<1>::get(self).map(|v| v as _)
    }
}
impl CView for C1 {
    type Registry = MyFamily;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        ScalarMsgFieldGetter::<1>::get(self)
            .flatten()
            .map(|v| v as _)
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        ScalarMsgFieldGetter::<2>::get(self).map(|v| v as _)
    }
}
impl DView for D1 {
    type Registry = MyFamily;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        ScalarMsgFieldGetter::<1>::get(self)
            .flatten()
            .map(|v| v as _)
    }
}

// 7. Blanket implementations for wrappers
// First, for the low-level getter
impl<'s, const N: i32, T: ?Sized + ScalarMsgFieldGetter<N>> ScalarMsgFieldGetter<N> for &'s T {
    type Message<'a>
    where
        Self: 'a,
    = T::Message<'a>;
    fn get(&self) -> Option<Self::Message<'_>> {
        T::get(self)
    }
}
impl<const N: i32, T: ScalarMsgFieldGetter<N>> ScalarMsgFieldGetter<N> for Option<T> {
    type Message<'a>
    where
        Self: 'a,
    = T::Message<'a>;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().and_then(|v| v.get())
    }
}
// Then, for the high-level views
impl<'s, T> AView for &'s T
where
    T: ?Sized + AView,
{
    type Registry = T::Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        T::b(self)
    }
}
impl<T> AView for Option<T>
where
    T: AView,
{
    type Registry = T::Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        self.as_ref().and_then(|v| v.b())
    }
}
// ...and so on for BView, CView, DView...
impl<'s, T> BView for &'s T
where
    T: ?Sized + BView,
{
    type Registry = T::Registry;
    fn c(&self) -> Option<<Self::Registry as Registry>::C<'_>> {
        T::c(self)
    }
}
impl<T> BView for Option<T>
where
    T: BView,
{
    type Registry = T::Registry;
    fn c(&self) -> Option<<Self::Registry as Registry>::C<'_>> {
        self.as_ref().and_then(|v| v.c())
    }
}
impl<'s, T> CView for &'s T
where
    T: ?Sized + CView,
{
    type Registry = T::Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        T::b(self)
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        T::d(self)
    }
}
impl<T> CView for Option<T>
where
    T: CView,
{
    type Registry = T::Registry;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        self.as_ref().and_then(|v| v.b())
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        self.as_ref().and_then(|v| v.d())
    }
}
impl<'s, T> DView for &'s T
where
    T: ?Sized + DView,
{
    type Registry = T::Registry;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        T::d(self)
    }
}
impl<T> DView for Option<T>
where
    T: DView,
{
    type Registry = T::Registry;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        self.as_ref().and_then(|v| v.d())
    }
}

// User-facing wrappers now are much simpler.
// They are generic over a concrete view type.
pub struct AMain<T: AView>(T);
pub struct BMain<T: BView>(T);
pub struct CMain<T: CView>(T);
pub struct DMain<T: DView>(T);

impl<T: AView> AMain<T> {
    pub fn b(&self) -> Option<BMain<<T::Registry as Registry>::B<'_>>> {
        self.0.b().map(BMain)
    }
}
impl<T: BView> BMain<T> {
    pub fn c(&self) -> Option<CMain<<T::Registry as Registry>::C<'_>>> {
        self.0.c().map(CMain)
    }
}
impl<T: CView> CMain<T> {
    pub fn b(&self) -> Option<BMain<<T::Registry as Registry>::B<'_>>> {
        self.0.b().map(BMain)
    }
    pub fn d(&self) -> Option<DMain<<T::Registry as Registry>::D<'_>>> {
        self.0.d().map(DMain)
    }
}
impl<T: DView> DMain<T> {
    pub fn d(&self) -> Option<DMain<<T::Registry as Registry>::D<'_>>> {
        self.0.d().map(DMain)
    }
}

#[test]
fn foo() {
    // We wrap the concrete type in the user-facing struct.
    let a = AMain(A1::default());

    // Chain the calls correctly to avoid lifetime issues with closures.
    if let Some(b_main) = a.b() {
        if let Some(c_main) = b_main.c() {
            let _ = c_main.d();
        }
    }

    // We can also test the wrappers directly.
    let a_opt = AMain(Some(A1::default()));
    if let Some(b_main) = a_opt.b() {
        if let Some(c_main) = b_main.c() {
            let _ = c_main.d();
        }
    }
}
