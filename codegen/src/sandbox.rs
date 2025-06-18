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
//! Assuming the following message ownership graph (parent ──► child):
//!
//! ```ascii
//! [A] ────► [B] ────► [C] ────► [D] ────┐
//!            ▲         │         ▲      │
//!            │         │         │      │
//!            └─────────┘         └──────┘
//!
//! [E] ────► [F] ────► [G] ────┐
//!  ▲                          │
//!  └──────────────────────────┘
//! ```
//!
//! The registry traits are organized based on the message type dependencies. This organization
//! is determined by the following rules:
//!
//! 1. **Message Type Dependencies**
//!    - Due to our implementation design, when implementing a message type, we need to know the
//!      concrete types of all message types that are reachable from it through any chain of references.
//!    - Let's call this list of the message concrete types as a registry.
//!    - Each message type can have only one implementation in a single registry.
//!    - For example:
//!      * To implement `A`, we need to know the concrete types of `B`, `C`, and `D`
//!        (because `A → B → C → D`)
//!      * To implement `B`, we need to know the concrete types of `B`, `C`, and `D`
//!        (because `B → C → D`)
//!      * To implement `D`, we only need to know the concrete type of `D`
//!        (because it only references itself)
//!
//!    - This dependency affects how we can organize registries into subgroups:
//!      * Case 1: We can share registry subgroups
//!        ```
//!        Complete Registry Type: {A, B, C, D}
//!        Registry Instance 1: {A1, B1, C1, D1}
//!        Registry Instance 2: {A2, B1, C1, D1}
//!        ```
//!        Here, we can share the registry subgroup instances because they have the same implementation types
//!        for all message types in their respective registry types. For example, we can cut out `B`, `C`, and `D`
//!        as a separate subgroup instance `{B1, C1, D1}` because they all have the same implementation types.
//!        This `BCD` subgroup can be shared between registry instances since they share the same implementations.
//!
//!      * Case 2: We cannot share registry subgroups
//!        ```
//!        Complete Registry Type: {A, B, C, D}
//!        Registry Instance 1: {A1, B1, C1, D1}
//!        Registry Instance 2: {A2, B1, C1, D2}
//!        ```
//!        Here, we cannot share the registry subgroup instances because they have different implementation types
//!        for some message types in their respective registry types. For example, we cannot cut out `B`, `C`, and `D`
//!        as a separate subgroup instance because they have different `D` implementations (`D1` vs `D2`).
//!        We also cannot cut out just `B` and `C` as a separate subgroup because `C` references `D`, so we need to know
//!        which specific `D` implementation (`D1` or `D2`) is being used.
//!
//!    - To optimize this, we can create registry subgroups that contain only the necessary message types:
//!      * A registry subgroup is a subset of a complete registry that contains only the message types needed
//!        for a specific group of messages
//!      * For example:
//!        ```
//!        Complete Registry Type: {A, B, C, D, E, F, G}
//!        BCD Subgroup Type: {B, C, D}
//!        EFG Subgroup Type: {E, F, G}
//!        D Subgroup Type: {D}
//!
//!        Complete Registry Instance: {A1, B1, C1, D1, E1, F1, G1}
//!        BCD Subgroup Instance: {B1, C1, D1}
//!        EFG Subgroup Instance: {E1, F1, G1}
//!        D Subgroup Instance: {D1}
//!        ```
//!      * These subgroups can be shared between different complete registries.
//!      * This allows for more efficient registry organization while maintaining type safety.
//!
//! 2. **Formalization through Equivalence Classes**
//!    - The equivalence classes define the granularity of registry subgroups:
//!      * Two message types belong to the same equivalence class if they must be in the same registry subgroup
//!      * This is determined by:
//!        - Direct references between message types
//!        - Cyclic references between message types
//!        - Reachability through reference chains
//!    - Each equivalence class becomes a registry subgroup in the trait hierarchy
//!    - The ordering relationship between registry subgroups is determined by the reference relationships:
//!      * If a message type in subgroup S1 can reach a message type in subgroup S2 through references,
//!        then S1 must be a superset of S2
//!      * For example, in our message graph:
//!        - `BC` subgroup must be a superset of `D` subgroup because `B` and `C` can reach `D`
//!        - The full registry must be a superset of `BC` subgroup because `A` can reach `B` and `C`
//!    - This ordering relationship forms the registry trait hierarchy through trait inheritance
//!
//! 3. **Implementation through Trait Inheritance**
//!    - The registry hierarchy is implemented using trait inheritance (supertraits)
//!    - Each registry trait defines the view implementations for its message types
//!    - A registry trait inherits all view implementations from its supertraits
//!    - This ensures type safety across all message type relationships
//!
//! This grouping mechanism ensures that:
//! * Related message types are grouped together based on their reference relationships
//! * Each message type belongs to exactly one equivalence class
//! * Type consistency is enforced through the registry trait hierarchy
//! * Type safety is maintained across cyclic references
//! * The hierarchy is determined by the natural structure of message references

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
pub trait NonMsgFieldGetter<const N: i32> {
    type Value<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::Value<'_>;
}

impl<T: MsgFieldGetter<N>, const N: i32> MsgFieldGetter<N> for &T {
    type Message = T::Message;
    fn get(&self) -> Option<&Self::Message> {
        T::get(self)
    }
}

impl<T: MsgFieldGetter<N>, const N: i32> MsgFieldGetter<N> for Option<T> {
    type Message = T::Message;
    fn get(&self) -> Option<&Self::Message> {
        self.as_ref().and_then(|x| x.get())
    }
}

impl MsgFieldGetter<1> for A1 {
    type Message = B1;
    fn get(&self) -> Option<&B1> {
        Some(&self.b)
    }
}
impl MsgFieldGetter<1> for B1 {
    type Message = C1;
    fn get(&self) -> Option<&C1> {
        Some(&self.c)
    }
}
impl MsgFieldGetter<1> for C1 {
    type Message = B1;
    fn get(&self) -> Option<&B1> {
        self.b.as_deref()
    }
}
impl MsgFieldGetter<2> for C1 {
    type Message = D1;
    fn get(&self) -> Option<&D1> {
        Some(&self.d)
    }
}
impl MsgFieldGetter<1> for D1 {
    type Message = D1;
    fn get(&self) -> Option<&D1> {
        self.d.as_deref()
    }
}

impl MsgFieldGetter<1> for A2 {
    type Message = B1;
    fn get(&self) -> Option<&B1> {
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
    let a_data = A1::default();
    let b_data = B1::default();
    let c_data = C1::default();
    let d_data = D1::default();

    // Use the user-facing types with default implementations
    let a = A::new(a_data);
    let b = B::new(b_data);
    let c = C::new(c_data);
    let d = D::new(d_data);

    // Test that the methods work through the user-facing types
    // Users don't need to know about View traits
    let _ = a.b();
    let _ = a.b().unwrap().c();
    let _ = b.c();
    let _ = b.c().unwrap().d();
    let _ = c.b();
    let _ = c.d();
    let _ = d.d();
}

pub trait DViewRegistry {
    type D: Message;
}

pub trait BCDViewRegistry: DViewRegistry {
    type B: Message;
    type C: Message;
}

pub trait MessageViewRegistry: BCDViewRegistry {
    type A: Message;
}

pub struct SomeImplSet;

impl DViewRegistry for SomeImplSet {
    type D = D1;
}

impl BCDViewRegistry for SomeImplSet {
    type B = B1;
    type C = C1;
}

impl MessageViewRegistry for SomeImplSet {
    type A = A1;
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

impl Message for A1 {
    type Registry = SomeImplSet;
}

impl Message for B1 {
    type Registry = SomeImplSet;
}

impl Message for C1 {
    type Registry = SomeImplSet;
}

impl Message for D1 {
    type Registry = SomeImplSet;
}

impl Message for D2 {
    type Registry = SomeImplSet2;
}

impl<T: Message> Message for &T {
    type Registry = T::Registry;
}
impl<T: Message> Message for Option<T> {
    type Registry = T::Registry;
}

// Wrapper struct for the `NView` trait blanket implementation
#[repr(transparent)]
pub struct MessageView<T>(pub T);

impl<A, B> AView for MessageView<A>
where
    A: Message,
    B: Message,
    A: MsgFieldGetter<1, Message = B>,
    <A as Message>::Registry: BCDViewRegistry<B = B>,
    // <B as Message>::Registry: BCDViewRegistry,
    // BCDRegistryEq<<A as Message>::Registry>:
    //     RegistryEq<Combined = <BCDRegistryEq<<B as Message>::Registry> as RegistryEq>::Combined>,
{
    fn b(&self) -> Option<&impl BView> {
        self.0
            .get()
            .map(|x| unsafe { ::std::mem::transmute::<_, &MessageView<B1>>(x) })
    }
}

impl<B, C> BView for MessageView<B>
where
    B: Message,
    C: Message,
    B: MsgFieldGetter<1, Message = C>,
    <B as Message>::Registry: BCDViewRegistry<C = C>,
    // <C as Message>::Registry: BCDViewRegistry,
    // BCDRegistryEq<<B as Message>::Registry>:
    //     RegistryEq<Combined = <BCDRegistryEq<<C as Message>::Registry> as RegistryEq>::Combined>,
{
    fn c(&self) -> Option<&impl CView> {
        self.0
            .get()
            .map(|x| unsafe { ::std::mem::transmute::<_, &MessageView<C1>>(x) })
    }
}

impl<C, B, D> CView for MessageView<C>
where
    C: Message,
    B: Message,
    D: Message,
    C: MsgFieldGetter<1, Message = B>,
    C: MsgFieldGetter<2, Message = D>,
    <C as Message>::Registry: BCDViewRegistry<B = B> + DViewRegistry<D = D>,
    // <B as Message>::Registry: BCDViewRegistry,
    // <D as Message>::Registry: DViewRegistry,
    // BCDRegistryEq<<C as Message>::Registry>:
    //     RegistryEq<Combined = <BCDRegistryEq<<B as Message>::Registry> as RegistryEq>::Combined>,
    // DRegistryEq<<C as Message>::Registry>:
    //     RegistryEq<Combined = <DRegistryEq<<D as Message>::Registry> as RegistryEq>::Combined>,
{
    fn b(&self) -> Option<&impl BView> {
        <C as MsgFieldGetter<1>>::get(&self.0)
            .map(|x| unsafe { ::std::mem::transmute::<_, &MessageView<B1>>(x) })
    }
    fn d(&self) -> Option<&impl DView> {
        <C as MsgFieldGetter<2>>::get(&self.0)
            .map(|x| unsafe { ::std::mem::transmute::<_, &MessageView<D1>>(x) })
    }
}

impl<D> DView for MessageView<D>
where
    D: Message,
    D: MsgFieldGetter<1, Message = D>,
    <D as Message>::Registry: DViewRegistry<D = D>,
    // DRegistryEq<<D as Message>::Registry>:
    //     RegistryEq<Combined = <DRegistryEq<<D as Message>::Registry> as RegistryEq>::Combined>,
{
    fn d(&self) -> Option<&impl DView> {
        <D as MsgFieldGetter<1>>::get(&self.0)
            .map(|x| unsafe { ::std::mem::transmute::<_, &MessageView<D1>>(x) })
    }
}

// User-facing message types with default implementations
// These are the main types that users will interact with
pub struct A<T = MessageView<A1>>(pub T);
pub struct B<T = MessageView<B1>>(pub T);
pub struct C<T = MessageView<C1>>(pub T);
pub struct D<T = MessageView<D1>>(pub T);

// Implement methods directly without relying on View traits
// Users should not need to know about View traits
impl<T> A<T>
where
    T: AView,
{
    pub fn b(&self) -> Option<&impl BView> {
        self.0.b()
    }
}

impl<T> B<T>
where
    T: BView,
{
    pub fn c(&self) -> Option<&impl CView> {
        self.0.c()
    }
}

impl<T> C<T>
where
    T: CView,
{
    pub fn b(&self) -> Option<&impl BView> {
        self.0.b()
    }
    pub fn d(&self) -> Option<&impl DView> {
        self.0.d()
    }
}

impl<T> D<T>
where
    T: DView,
{
    pub fn d(&self) -> Option<&impl DView> {
        self.0.d()
    }
}

// Convenience constructors for the default implementations
impl A {
    pub fn new(inner: A1) -> Self {
        A(MessageView(inner))
    }
}

impl B {
    pub fn new(inner: B1) -> Self {
        B(MessageView(inner))
    }
}

impl C {
    pub fn new(inner: C1) -> Self {
        C(MessageView(inner))
    }
}

impl D {
    pub fn new(inner: D1) -> Self {
        D(MessageView(inner))
    }
}
