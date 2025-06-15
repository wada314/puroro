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
//! ## Solution: `MessageViewRegistry`
//!
//! To solve the recursive trait implementation problem, we introduce a hierarchy of registry traits.
//! These traits act as type-level registries that map message types to their corresponding
//! implementations of the `View` trait. This allows us to break the circular dependency by:
//!
//! 1. Having each message type implement `DescendantViewRegistry` to specify which registry it belongs to.
//!    The `DescendantViewRegistry` trait only needs to specify the registry type, without any bounds.
//!    The actual bounds for the registry type are specified at the point of use in each `View` trait
//!    implementation. This allows for more granular control over which registry traits are required
//!    for each message type.
//!
//! 2. Using a hierarchy of registry traits to minimize the knowledge each message type needs:
//!    * `DViewRegistry`: Contains only the `D` type's view implementation
//!    * `BCDViewRegistry`: Contains `B`, `C`, and `D` type's view implementations, inheriting from `DViewRegistry`
//!    * `MessageViewRegistry`: Contains all view implementations, inheriting from `BCDViewRegistry`
//!
//! 3. Implementing the `View` traits using blanket implementations that reference the appropriate
//!    registry trait based on the message type's needs. For example:
//!    * `DView` implementation only requires `DViewRegistry`
//!    * `CView` implementation requires both `BCDViewRegistry` and `DViewRegistry`
//!    * `BView` and `AView` implementations require `BCDViewRegistry`
//!
//! This approach has several benefits:
//! * It allows us to have multiple implementations of the same message type (e.g., `A` and `A2`)
//! * It makes the trait implementations more modular and easier to maintain
//! * It avoids the need for explicit trait bounds in the generated code
//! * It provides a clear way to handle recursive message types
//! * It minimizes the knowledge each message type needs about other message types in the system
//! * It allows for more precise control over which registry traits are required for each message type
//!
//! The registry trait hierarchy is particularly useful for protobuf code generation because:
//! * It allows us to generate code that works with both concrete and generic message types
//! * It provides a way to handle message inheritance and extension fields
//! * It makes it easier to implement features like lazy loading and dynamic message types
//! * It enables more efficient code generation by only including necessary view implementations
//!

#[derive(Default, Debug)]
pub struct A {
    pub b: Box<B>,
}
#[derive(Default, Debug)]
pub struct B {
    pub c: Box<C>,
}
#[derive(Default, Debug)]
pub struct C {
    pub b: Option<Box<B>>,
    pub d: Box<D>,
}
#[derive(Default, Debug)]
pub struct D {
    pub d: Option<Box<D>>,
}
#[derive(Default, Debug)]
pub struct A2 {
    pub b: Box<B>,
}
#[derive(Default, Debug)]
pub struct D2 {
    pub d: Option<Box<D2>>,
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
        self.b.as_ref().unwrap()
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
        self.d.as_ref().unwrap()
    }
}

impl MsgFieldGetter<1> for A2 {
    type Message = B;
    fn get(&self) -> &B {
        &self.b
    }
}

impl MsgFieldGetter<1> for D2 {
    type Message = D2;
    fn get(&self) -> &D2 {
        self.d.as_ref().unwrap()
    }
}

#[test]
fn foo() {
    let a = A::default();
    let b = B::default();
    let c = C::default();
    let d = D::default();
    let a2 = A2::default();
    let d2 = D2::default();

    let _ = <B as BView>::c(&b);
    let b2 = <A2 as AView>::b(&a2);
    let c2 = BView::c(b2);
    let d2_from_a2 = CView::d(c2);
    use ::std::any::type_name_of_val;
    assert_ne!(type_name_of_val(&d2_from_a2), type_name_of_val(&d2));
}

pub trait DViewRegistry {
    type D: DView;
}

pub trait BCDViewRegistry: DViewRegistry {
    type B: BView;
    type C: CView;
}

pub trait MessageViewRegistry: BCDViewRegistry {
    type A: AView;
}

pub struct SomeImplSet;

impl DViewRegistry for SomeImplSet {
    type D = D;
}

impl BCDViewRegistry for SomeImplSet {
    type B = B;
    type C = C;
}

impl MessageViewRegistry for SomeImplSet {
    type A = A;
}

pub struct SomeImpl2;

impl DViewRegistry for SomeImpl2 {
    type D = D2;
}

impl BCDViewRegistry for SomeImpl2 {
    type B = B;
    type C = C;
}

impl MessageViewRegistry for SomeImpl2 {
    type A = A2;
}

pub trait DescendantViewRegistry {
    type Set;
}

impl DescendantViewRegistry for A {
    type Set = SomeImplSet;
}

impl DescendantViewRegistry for B {
    type Set = SomeImplSet;
}

impl DescendantViewRegistry for C {
    type Set = SomeImplSet;
}

impl DescendantViewRegistry for D {
    type Set = SomeImplSet;
}

impl DescendantViewRegistry for A2 {
    type Set = SomeImpl2;
}

impl DescendantViewRegistry for D2 {
    type Set = SomeImpl2;
}

impl<T> AView for T
where
    T: DescendantViewRegistry,
    T: MsgFieldGetter<1, Message = <<T as DescendantViewRegistry>::Set as BCDViewRegistry>::B>,
    <T as DescendantViewRegistry>::Set: BCDViewRegistry,
{
    fn b(&self) -> &impl BView {
        self.get()
    }
}

impl<T> BView for T
where
    T: DescendantViewRegistry,
    T: MsgFieldGetter<1, Message = <<T as DescendantViewRegistry>::Set as BCDViewRegistry>::C>,
    <T as DescendantViewRegistry>::Set: BCDViewRegistry,
{
    fn c(&self) -> &impl CView {
        self.get()
    }
}

impl<T> CView for T
where
    T: DescendantViewRegistry,
    T: MsgFieldGetter<1, Message = <<T as DescendantViewRegistry>::Set as BCDViewRegistry>::B>,
    T: MsgFieldGetter<2, Message = <<T as DescendantViewRegistry>::Set as DViewRegistry>::D>,
    <T as DescendantViewRegistry>::Set: BCDViewRegistry + DViewRegistry,
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
    T: DescendantViewRegistry,
    T: MsgFieldGetter<1, Message = <<T as DescendantViewRegistry>::Set as DViewRegistry>::D>,
    <T as DescendantViewRegistry>::Set: DViewRegistry,
{
    fn d(&self) -> &impl DView {
        <T as MsgFieldGetter<1>>::get(self)
    }
}
