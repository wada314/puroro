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
//! ## Registry Type Consistency
//!
//! To ensure type safety across the registry hierarchy, we introduce the `RegistryEq` trait
//! and its implementations. This mechanism allows us to verify that related message types use
//! consistent registry implementations at compile time.
//!
//! The `RegistryEq` trait hierarchy:
//! * `DRegistryEq`: Compares registry types for `D` message types
//! * `BCDRegistryEq`: Compares registry types for `B`, `C`, and `D` message types
//! * `MessageRegistryEq`: Compares registry types for all message types
//!
//! This comparison mechanism ensures that:
//! * Parent and child message types use the same registry implementation
//! * Inconsistent registry types are detected at compile time
//! * Type safety is maintained across the entire message hierarchy
//!
//! For example, if a message type `A` uses `SomeImplSet` as its registry, all its child message types
//! (`B`, `C`, `D`) must also use `SomeImplSet`. The `RegistryEq` trait implementations
//! enforce this constraint at compile time.
//!
//! ## Message Type Grouping
//!
//! The registry traits are organized based on the cyclic reference patterns in the message types.
//! This grouping is determined by the following rules:
//!
//! 1. **Equivalence Class Definition**
//!    - Two message types belong to the same equivalence class if:
//!      * They are part of the same cyclic reference (e.g., `B` ↔ `C` or `E` → `F` → `G` → `E`)
//!      * A self-referential type (like `D`) can join another cyclic reference group if it's part of that cycle
//!    - Message types that are not part of any cyclic reference form their own single-item equivalence classes
//!
//! 2. **Quotient Set Construction**
//!    - The set of message types is partitioned into equivalence classes
//!    - Each equivalence class corresponds to a registry trait
//!    - Examples:
//!      * `{A}` → A single registry (not part of any cyclic reference)
//!      * `{B, C}` → `BCDViewRegistry` (cyclic reference group)
//!      * `{D}` → `DViewRegistry` (self-referential, not part of other cycles)
//!      * `{E, F, G}` → A new registry (cyclic reference group)
//!      * `{H}` → A single registry (not part of any cyclic reference)
//!
//! This grouping mechanism ensures that:
//! * Related message types are grouped together
//! * Each message type belongs to exactly one equivalence class
//! * The registry hierarchy reflects the natural structure of the message types
//! * Type safety is maintained across cyclic references

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
    fn d(&self) -> Option<&impl DView>;
}

pub trait MsgFieldGetter<const N: i32> {
    type Message;
    fn get(&self) -> Option<&Self::Message>;
}

impl MsgFieldGetter<1> for A {
    type Message = B;
    fn get(&self) -> Option<&B> {
        Some(&self.b)
    }
}
impl MsgFieldGetter<1> for B {
    type Message = C;
    fn get(&self) -> Option<&C> {
        Some(&self.c)
    }
}
impl MsgFieldGetter<1> for C {
    type Message = B;
    fn get(&self) -> Option<&B> {
        self.b.as_deref()
    }
}
impl MsgFieldGetter<2> for C {
    type Message = D;
    fn get(&self) -> Option<&D> {
        Some(&self.d)
    }
}
impl MsgFieldGetter<1> for D {
    type Message = D;
    fn get(&self) -> Option<&D> {
        self.d.as_deref()
    }
}

impl MsgFieldGetter<1> for A2 {
    type Message = B;
    fn get(&self) -> Option<&B> {
        Some(&self.b)
    }
}

impl MsgFieldGetter<1> for D2 {
    type Message = D2;
    fn get(&self) -> Option<&D2> {
        self.d.as_deref()
    }
}

#[test]
fn foo() {
    let a = A::default();
    let b = B::default();
    let c = C::default();
    let d = D::default();

    let _ = <B as BView>::c(&b);
}

pub trait DViewRegistry {
    type D: Message + DView;
}

pub trait BCDViewRegistry: DViewRegistry {
    type B: Message + BView;
    type C: Message + CView;
}

pub trait MessageViewRegistry: BCDViewRegistry {
    type A: Message + AView;
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

pub struct SomeImplSet2;

impl DViewRegistry for SomeImplSet2 {
    type D = D2;
}

trait RegistryEq {
    type Combined;
}
pub struct DRegistryEq<R: DViewRegistry>(::std::marker::PhantomData<R>);
impl<R: DViewRegistry> RegistryEq for DRegistryEq<R> {
    type Combined = (R::D,);
}
pub struct BCDRegistryEq<R: BCDViewRegistry>(::std::marker::PhantomData<R>);
impl<R: BCDViewRegistry> RegistryEq for BCDRegistryEq<R> {
    type Combined = (R::B, R::C, <DRegistryEq<R> as RegistryEq>::Combined);
}
pub struct MessageRegistryEq<R: MessageViewRegistry>(::std::marker::PhantomData<R>);
impl<R: MessageViewRegistry> RegistryEq for MessageRegistryEq<R> {
    type Combined = (R::A, <BCDRegistryEq<R> as RegistryEq>::Combined);
}

pub trait Message {
    type Registry;
}

impl Message for A {
    type Registry = SomeImplSet;
}

impl Message for B {
    type Registry = SomeImplSet;
}

impl Message for C {
    type Registry = SomeImplSet;
}

impl Message for D {
    type Registry = SomeImplSet;
}

impl Message for D2 {
    type Registry = SomeImplSet2;
}

impl<T> AView for T
where
    T: Message,
    T: MsgFieldGetter<1, Message = B>,
    <T as Message>::Registry: BCDViewRegistry<B = B>,
    <B as Message>::Registry: BCDViewRegistry,
    BCDRegistryEq<<T as Message>::Registry>:
        RegistryEq<Combined = <BCDRegistryEq<<B as Message>::Registry> as RegistryEq>::Combined>,
{
    fn b(&self) -> Option<&impl BView> {
        self.get()
    }
}

impl<T> BView for T
where
    T: Message,
    T: MsgFieldGetter<1, Message = C>,
    <T as Message>::Registry: BCDViewRegistry<C = C>,
    <C as Message>::Registry: BCDViewRegistry,
    BCDRegistryEq<<T as Message>::Registry>:
        RegistryEq<Combined = <BCDRegistryEq<<C as Message>::Registry> as RegistryEq>::Combined>,
{
    fn c(&self) -> Option<&impl CView> {
        self.get()
    }
}

impl<T> CView for T
where
    T: Message,
    T: MsgFieldGetter<1, Message = B>,
    T: MsgFieldGetter<2, Message = D>,
    <T as Message>::Registry: BCDViewRegistry<B = B> + DViewRegistry<D = D>,
    <B as Message>::Registry: BCDViewRegistry,
    <D as Message>::Registry: DViewRegistry,
    BCDRegistryEq<<T as Message>::Registry>:
        RegistryEq<Combined = <BCDRegistryEq<<B as Message>::Registry> as RegistryEq>::Combined>,
    DRegistryEq<<T as Message>::Registry>:
        RegistryEq<Combined = <DRegistryEq<<D as Message>::Registry> as RegistryEq>::Combined>,
{
    fn b(&self) -> Option<&impl BView> {
        <T as MsgFieldGetter<1>>::get(self)
    }
    fn d(&self) -> Option<&impl DView> {
        <T as MsgFieldGetter<2>>::get(self)
    }
}

impl<T> DView for T
where
    T: Message,
    T: MsgFieldGetter<1, Message = T>,
    <T as Message>::Registry: DViewRegistry<D = T>,
{
    fn d(&self) -> Option<&impl DView> {
        <T as MsgFieldGetter<1>>::get(self)
    }
}
