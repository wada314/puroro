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
//! ### An `商集合` of the message type
//!
//! To solve the recursive definition problem, we define an `商集合` idea.
//! An `商集合` is a group of the proto message types. Technically,
//!
//! For a pair of proto message `A` and `B`,
//!   * if `A` and `B` can be a (maybe indirect) child message of each other,
//!     then we say that `A` and `B` are in the same `商集合`.
//!   * if `A` can be a child message of `B`, but `B` can not be a child message of `A`,
//!     then we say that `A` is a superset of `B`, and `B` is a subset of `A`.
//!

#[repr(transparent)]
pub struct Person<T = PersonInner>(T);
pub struct PersonInner {
    name: String,
    age: u32,
    children: Vec<PersonInner>,
}

pub trait ScalarStringField<const N: i32> {
    fn get(&self) -> &str;
}
pub trait ScalarU32Field<const N: i32> {
    fn get(&self) -> u32;
}
pub trait RepeatedMessageField<const N: i32> {
    type Message;
    fn get(&self) -> impl Iterator<Item = &Self::Message>;
}

impl ScalarStringField<1> for PersonInner {
    fn get(&self) -> &str {
        &self.name
    }
}
impl ScalarU32Field<2> for PersonInner {
    fn get(&self) -> u32 {
        self.age
    }
}
impl RepeatedMessageField<3> for PersonInner {
    type Message = PersonInner;
    fn get(&self) -> impl Iterator<Item = &Self::Message> {
        self.children.iter()
    }
}

impl<T, const N: i32> ScalarStringField<N> for &T
where
    T: ScalarStringField<N>,
{
    fn get(&self) -> &str {
        <T as ScalarStringField<N>>::get(self)
    }
}

impl<T, const N: i32> ScalarU32Field<N> for &T
where
    T: ScalarU32Field<N>,
{
    fn get(&self) -> u32 {
        <T as ScalarU32Field<N>>::get(self)
    }
}

impl<T, const N: i32> RepeatedMessageField<N> for &T
where
    T: RepeatedMessageField<N>,
{
    type Message = <T as RepeatedMessageField<N>>::Message;
    fn get(&self) -> impl Iterator<Item = &Self::Message> {
        <T as RepeatedMessageField<N>>::get(self)
    }
}

pub trait PersonTrait: ScalarStringField<1> + ScalarU32Field<2> + RepeatedMessageField<3> {
    fn name(&self) -> &str {
        <Self as ScalarStringField<1>>::get(self)
    }
    fn age(&self) -> u32 {
        <Self as ScalarU32Field<2>>::get(self)
    }
    fn children(&self) -> impl Iterator<Item = impl PersonTrait>;
}

impl PersonTrait for PersonInner {
    fn children(&self) -> impl Iterator<Item = impl PersonTrait> {
        <Self as RepeatedMessageField<3>>::get(self)
    }
}
impl<T: PersonTrait> PersonTrait for &T {
    fn children(&self) -> impl Iterator<Item = impl PersonTrait> {
        <T as PersonTrait>::children(self)
    }
}

impl<T> Person<T>
where
    T: PersonTrait,
{
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn age(&self) -> u32 {
        self.0.age()
    }
    pub fn children(&self) -> impl Iterator<Item = Person<impl PersonTrait>> {
        self.0.children().map(|x| Person(x))
    }
}

fn foo<T: PersonTrait>(p: Person<T>) {
    println!("{}", p.name());
    println!("{}", p.age());
    for child in p.children() {
        println!("{}", child.name());
        for grandchild in child.children() {
            println!("{}", grandchild.name());
        }
    }
}
