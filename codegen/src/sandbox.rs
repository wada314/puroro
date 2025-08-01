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

//! # Internal Memo: The Journey to a Working Recursive View Trait
//!
//! This document serves as a memory of the design process for creating a flexible
//! and type-safe view system for protobuf messages, specifically addressing the
//! challenges of recursive message types.
//!
//! ## 1. The Core Problem: Recursive Message Views
//!
//! Our goal is to define a `XView` trait for each message `X` that provides
//! getter methods for its fields. For a recursive message like `D { D d = 1; }`,
//! the trait `DView` needs a method `d()` that returns something implementing `DView`.
//!
//! The initial challenge was supporting both by-reference returns (for simple
//! structs) and by-value returns (for wrapper types like `(T, U)`). A Generic
//! Associated Type (GAT) approach was chosen for this:
//!
//! ```rust,ignore
//! trait DView {
//!     type D<'a>: DView where Self: 'a;
//!     fn d(&self) -> Self::D<'_>;
//! }
//! ```
//!
//! ## 2. The Blocker: The Trait Bound Recursion Problem
//!
//! The true problem emerged when we tried to implement this trait for a concrete
//! recursive type, e.g., `impl DView for D1`. The compiler needs to prove that `D1`
//! satisfies the bounds of `DView`. The process is:
//!
//! 1.  To prove `D1: DView`, the compiler must check the return type of `D1.d()`.
//! 2.  The return type (e.g., `&D1` or `Option<&D1>`) must also implement `DView`.
//! 3.  To prove `&D1: DView` (via a blanket impl), the compiler needs to know that
//!     the inner type `D1` implements `DView`.
//! 4.  This creates a circular dependency: to prove `D1: DView`, we must first
//!     prove `D1: DView`. The compiler's trait solver cannot resolve this and
//!     results in an "overflow evaluating the requirement" error.
//!
//! We attempted several solutions, like using supertraits (`trait DView: Getter<...>`)
//! with default method implementations. These failed for a subtle but critical reason:
//! The `where` clause on the trait definition itself (`where for<'a> Message<'a>: DView`)
//! is what introduces the circular dependency at the *definition* level, which is
//! unresolvable.
//!
//! The absolute root cause was correctly identified as the limitation of Higher-Ranked
//! Trait Bounds (HRTB). The `for<'a>` in a `where` clause is universal and does not
//! inherit the contextual `Self: 'a` bound from a GAT. This forces the compiler
//! to attempt an impossible proof (that a type `T` can outlive any lifetime `'a`,
//! including `'static'`), leading to "may not live long enough" errors.
//!
//! ## 3. The Final, Working Solution: The "Registry" Pattern
//!
//! The robust and compilable solution is to break the direct trait dependency cycle
//! using a "Registry".
//!
//! -   **How it works:** Instead of `AView` knowing about `BView` directly, both
//!     traits only know about a central `Registry`. The `Registry` acts as a map,
//!     defining the associated view types (e.g., `type B<'a> = &'a B1;`). The methods
//!     in `AView` return types looked up from this registry: `Option<<Self::Registry as Registry>::B<'_>>`.
//!
//! -   **Why it works:** This transforms the problem. The compiler no longer needs to
//!     prove a recursive trait bound (`D1: DView` ?=> `&D1: DView` ?=> `D1: DView`).
//!     Instead, it only needs to resolve concrete associated types (`MyRegistry::D<'a> == &'a D1`),
//!     which is a much simpler operation that does not involve recursion.
//!
//! ## 4. Refinement: Combining Registry with `FieldGetter`
//!
//! To keep the implementation clean, we re-introduced a low-level `ScalarMsgFieldGetter` trait.
//! The final architecture is:
//!
//! -   **`Registry`:** Breaks the cycle between `View` trait definitions.
//! -   **`ScalarMsgFieldGetter<N, Marker>`:** Provides a low-level, number- and
//!     marker-based API for accessing fields. This is implemented by codegen for
//!     each concrete message struct.
//! -   **`XView` Trait `impl`:** The implementation of a high-level `View` trait
//!     (e.g., `impl AView for A1`) simply delegates its methods to the corresponding
//!     `ScalarMsgFieldGetter` implementation.
//! -   **Blanket `impl`s:** Blanket implementations for wrappers like `&T` and `Option<T>`
//!     are provided for both the low-level `Getter` traits and the high-level `View` traits.
//!     The number of combinations is small enough that writing these manually is more
//!     readable and maintainable than using complex macros.
//!
//! This layered approach is robust, compilable, and maintains a clean separation of concerns.

use puroro::{Both, Either, EitherOrBoth};
use std::ops::Deref;

// Marker types for each message
pub struct A;
pub struct B;
pub struct C;
pub struct D;

// The low-level, number-based getter trait, now generic over a message marker type.
pub trait ScalarMsgFieldGetter<const N: i32, MsgMarker> {
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
impl ScalarMsgFieldGetter<1, B> for A1 {
    type Message<'a> = &'a B1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.b)
    }
}
impl ScalarMsgFieldGetter<1, C> for B1 {
    type Message<'a> = &'a C1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.c)
    }
}
impl ScalarMsgFieldGetter<1, B> for C1 {
    type Message<'a> = Option<&'a B1>;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.b.as_deref())
    }
}
impl ScalarMsgFieldGetter<2, D> for C1 {
    type Message<'a> = &'a D1;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(&self.d)
    }
}
impl ScalarMsgFieldGetter<1, D> for D1 {
    type Message<'a> = Option<&'a D1>;
    fn get(&self) -> Option<Self::Message<'_>> {
        Some(self.d.as_deref())
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
        ScalarMsgFieldGetter::<1, B>::get(self).map(|v| v as _)
    }
}
impl BView for B1 {
    type Registry = MyFamily;
    fn c(&self) -> Option<<Self::Registry as Registry>::C<'_>> {
        ScalarMsgFieldGetter::<1, C>::get(self).map(|v| v as _)
    }
}
impl CView for C1 {
    type Registry = MyFamily;
    fn b(&self) -> Option<<Self::Registry as Registry>::B<'_>> {
        ScalarMsgFieldGetter::<1, B>::get(self)
            .flatten()
            .map(|v| v as _)
    }
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        ScalarMsgFieldGetter::<2, D>::get(self).map(|v| v as _)
    }
}
impl DView for D1 {
    type Registry = MyFamily;
    fn d(&self) -> Option<<Self::Registry as Registry>::D<'_>> {
        ScalarMsgFieldGetter::<1, D>::get(self)
            .flatten()
            .map(|v| v as _)
    }
}

// 7. Blanket implementations for wrappers
// First, for the low-level getter
impl<'s, const N: i32, MsgMarker, T: ?Sized + ScalarMsgFieldGetter<N, MsgMarker>>
    ScalarMsgFieldGetter<N, MsgMarker> for &'s T
{
    type Message<'a>
    where
        Self: 'a,
    = T::Message<'a>;
    fn get(&self) -> Option<Self::Message<'_>> {
        T::get(self)
    }
}
impl<const N: i32, MsgMarker, T: ScalarMsgFieldGetter<N, MsgMarker>>
    ScalarMsgFieldGetter<N, MsgMarker> for Option<T>
{
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
