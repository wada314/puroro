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

pub trait AView {
    type B<'a>: BView
    where
        Self: 'a;
    fn b(&self) -> Option<Self::B<'_>>;
}
pub trait BView {
    type C<'a>: CView
    where
        Self: 'a;
    fn c(&self) -> Option<Self::C<'_>>;
}
pub trait CView {
    type B<'a>: BView
    where
        Self: 'a;
    type D<'a>: DView
    where
        Self: 'a;
    fn b(&self) -> Option<Self::B<'_>>;
    fn d(&self) -> Option<Self::D<'_>>;
}
pub trait DView {
    type D<'a>: DView
    where
        Self: 'a;
    fn d(&self) -> Option<Self::D<'_>>;
}

// The code generator would produce implementations for the `ScalarMsgFieldGetter` traits first.
// This provides a low-level, generic way to access fields by number.
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
    type Message<'a> = &'a B1;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.b.as_deref()
    }
}
impl ScalarMsgFieldGetter<2> for C1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_ref())
    }
}
impl ScalarMsgFieldGetter<1> for D1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.d.as_deref()
    }
}

// Then, the generator implements the high-level, named `View` traits
// by delegating to the `ScalarMsgFieldGetter` implementations.
impl AView for A1 {
    type B<'a> = <Self as ScalarMsgFieldGetter<1>>::Message<'a>;
    fn b(&self) -> Option<Self::B<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}
impl BView for B1 {
    type C<'a> = <Self as ScalarMsgFieldGetter<1>>::Message<'a>;
    fn c(&self) -> Option<Self::C<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}
impl CView for C1 {
    type B<'a> = <Self as ScalarMsgFieldGetter<1>>::Message<'a>;
    type D<'a> = <Self as ScalarMsgFieldGetter<2>>::Message<'a>;
    fn b(&self) -> Option<Self::B<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<Self::D<'_>> {
        <Self as ScalarMsgFieldGetter<2>>::get(self)
    }
}
impl DView for D1 {
    type D<'a> = <Self as ScalarMsgFieldGetter<1>>::Message<'a>;
    fn d(&self) -> Option<Self::D<'_>> {
        <Self as ScalarMsgFieldGetter<1>>::get(self)
    }
}

// Blanket implementations for wrappers like `&T` provide flexibility.
impl<'s, T: ?Sized + AView> AView for &'s T {
    type B<'a>
        = T::B<'a>
    where
        Self: 'a;
    fn b(&self) -> Option<Self::B<'_>> {
        (*self).b()
    }
}
impl<'s, T: ?Sized + BView> BView for &'s T {
    type C<'a>
        = T::C<'a>
    where
        Self: 'a;
    fn c(&self) -> Option<Self::C<'_>> {
        (*self).c()
    }
}
impl<'s, T: ?Sized + CView> CView for &'s T {
    type B<'a>
        = T::B<'a>
    where
        Self: 'a;
    type D<'a>
        = T::D<'a>
    where
        Self: 'a;
    fn b(&self) -> Option<Self::B<'_>> {
        (*self).b()
    }
    fn d(&self) -> Option<Self::D<'_>> {
        (*self).d()
    }
}
impl<'s, T: ?Sized + DView> DView for &'s T {
    type D<'a>
        = T::D<'a>
    where
        Self: 'a;
    fn d(&self) -> Option<Self::D<'_>> {
        (*self).d()
    }
}

pub struct AMain<T: AView>(T);
pub struct BMain<T: BView>(T);
pub struct CMain<T: CView>(T);
pub struct DMain<T: DView>(T);

impl<T: AView> AMain<T> {
    pub fn b(&self) -> Option<BMain<impl BView + '_>> {
        self.0.b().map(BMain)
    }
}

impl<T: BView> BMain<T> {
    pub fn c(&self) -> Option<CMain<impl CView + '_>> {
        self.0.c().map(CMain)
    }
}

impl<T: CView> CMain<T> {
    pub fn b(&self) -> Option<BMain<impl BView + '_>> {
        self.0.b().map(BMain)
    }
    pub fn d(&self) -> Option<DMain<impl DView + '_>> {
        self.0.d().map(DMain)
    }
}

impl<T: DView> DMain<T> {
    pub fn d(&self) -> Option<DMain<impl DView + '_>> {
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
    let c_from_a = a.b().and_then(|x| x.c());
    let _ = b.c();
    let d_from_b = b.c().and_then(|x| x.d());
    let _ = c.b();
    let _ = c.d();
    let _ = d.d();
    let d_from_d = d.d().and_then(|x| x.d());
}

// The GAT-based traits are not object-safe by default.
// To use them as `dyn` traits, you'd need to use a crate like `async_trait`
// or avoid using GATs in the methods directly, which is a more advanced topic.
// For now, we remove the `bar` function that uses `dyn DView`.
