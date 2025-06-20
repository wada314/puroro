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

pub trait MsgFieldGetter<const N: i32> {
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

impl<T: MsgFieldGetter<N>, const N: i32> MsgFieldGetter<N> for &T {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        T::get(self)
    }
}

impl<T: MsgFieldGetter<N>, const N: i32> MsgFieldGetter<N> for Option<T> {
    type Message<'a>
        = T::Message<'a>
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.as_ref().and_then(|x| x.get())
    }
}

impl MsgFieldGetter<1> for B1 {
    type Message<'a>
        = &'a C1
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.c)
    }
}
impl MsgFieldGetter<1> for C1 {
    type Message<'a>
        = &'a B1
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.b.as_deref()
    }
}
impl MsgFieldGetter<2> for C1 {
    type Message<'a>
        = &'a D1
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.d)
    }
}
impl MsgFieldGetter<1> for D1 {
    type Message<'a>
        = &'a D1
    where
        Self: 'a;
    fn get(&self) -> Option<Self::Message<'_>> {
        self.d.as_deref()
    }
}

pub trait Registry {
    // type A<'a>: AView
    // where
    //     Self: 'a;
    // type B<'a>: BView
    // where
    //     Self: 'a;
    // type C<'a>: CView
    // where
    //     Self: 'a;
    type D<'a>
    where
        Self: 'a;
}

pub struct SomeImplSet;
impl Registry for SomeImplSet {
    // type A<'a> = &'a A1;
    // type B<'a> = &'a B1;
    // type C<'a> = &'a C1;
    type D<'a> = MessageView<&'a D1>;
}

pub trait AView {
    fn b(&self) -> Option<&impl BView>;
}
pub trait BView {
    fn c(&self) -> Option<&impl CView>;
}
pub trait CView {
    fn b(&self) -> Option<&impl BView>;
    fn d(&self) -> Option<&impl DView>;
}
pub trait DView {
    type Registry: Registry;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>>;
}

// Wrapper struct for the `NView` trait blanket implementation
#[repr(transparent)]
pub struct MessageView<T>(pub T);

impl<T> DView for MessageView<T>
where
    for<'a> T: MsgFieldGetter<1, Message<'a> = <Self::Registry as Registry>::D<'a>>,
{
    type Registry = SomeImplSet;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        <T as MsgFieldGetter<1>>::get(&self.0).map(MessageView)
    }
}

#[test]
fn foo() {
    // let a_data = A1::default();
    // let b_data = B1::default();
    // let c_data = C1::default();
    let d_data = D1::default();

    // Use the user-facing types with default implementations
    // let a = AMain::new(a_data);
    // let b = BMain::new(b_data);
    // let c = CMain::new(c_data);
    let d = MessageView(d_data);

    // Test that the methods work through the user-facing types
    // Users don't need to know about View traits
    // let _ = a.b();
    // let _ = a.b().unwrap().c();
    // let _ = b.c();
    // let _ = b.c().unwrap().d();
    // let _ = c.b();
    // let _ = c.d();
    let _ = d.d();
}
