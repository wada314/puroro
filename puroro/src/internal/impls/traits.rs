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

//! # Generated trait
//! For every message in input .pb files, puroro generates a Trait named
//! `<MyMessageName>Trait`.
//!
//!
//! For an input protobuf:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     int32 my_number = 1;
//!     repeated string my_name = 2;
//!     MyMessage my_child = 3;
//! }
//! ```
//!
//! Trait like this is generated:
//!
//! ```rust
//! // A readonly trait for message `MyMessage`
//! # #![feature(generic_associated_types)]
//! # use ::std::fmt::Debug;
//! # use ::std::ops::Deref;
//! pub trait MyMessageTrait {
//!     fn my_number(&self) -> i32;
//!     type Field2RepeatedType<'this>: IntoIterator<Item=&'this str> where Self: 'this;
//!     fn my_name(&self) -> Self::Field2RepeatedType<'_>;
//!     type Field3MessageType<'this>: MyMessageTrait + Clone + PartialEq + Debug where Self: 'this;
//!     fn my_child(&self) -> Option<Self::Field3MessageType<'_>>;
//! }
//! ```
//!
//! Each message field will have a single getter method which has same name
//! (lower_snake_case_nized if it's not) with the original protobuf fields.
//! If the field is a message type, then an associated type
//! `Field[number]MessageType` which implements the message type's trait
//! (something like `SomeMessageTrait`) is generated.
//!
//! And if the field is a repeated type, another associated type
//! `Field[number]RepeatedType` is generated.
//! This type implements [`puroro::RepeatedField`](crate::RepeatedField), which is the same
//! as the [`IntoIterator`].
//!
//! The list of the generated methods' return types:
//!
//! | base protobuf type | `required` / `optional` / `oneof` field | (unlabeled) | `repeated` |
//! |--------------------|-----------------------------------------|-------------|------------|
//! | `int32`            | `Option<i32>`                           | `i32`       | `impl IntoIterator<Item=i32>`|
//! | (Any numeric types)| `Option<T>`                             | `T`         | `impl IntoIterator<Item=T>`|
//! | `bytes`            | `Option<&[u8]>`                         | `&[u8]`     | `impl IntoIterator<Item=&[u8]>`|
//! | `string`           | `Option<&str>`                          | `&str`      | `impl IntoIterator<Item=&str>`|
//! | `SomeMessage`      | `Option<impl SomeMessageTrait>`         | `Option<impl SomeMessageTrait>`|`impl IntoIterator<Item=impl SomeMessageTrait>`|
//!
//! ## oneofs
//!
//! From a proto like this:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     oneof my_oneofs {
//!         int32 item1 = 1;
//!         float item2 = 2;
//!     }
//! }
//! ```
//!
//! This trait and enum are generated:
//!
//! ```rust
//! pub trait MyMessageTrait {
//!     fn my_oneofs(&self) -> Option<my_message::MyOneofs>;
//! }
//! pub mod my_message {
//!     pub enum MyOneofs {
//!         Item1(i32),
//!         Item2(f32),
//!     }
//! }
//! ```
//!
//! # trait impls
//!
//! The generated trait is implemented for the generated message structs and
//! the following types:
//!
//! ```rust
//! # trait MyMessageTrait {}
//! impl MyMessageTrait for () { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a T { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a mut T { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Box<T> { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Option<T> { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for (T, U) { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for puroro::Either<T, U> { /* ... */ }
//! ```
//!
//! ### `&'a T`, `&'a mut T`, `Box<T>`
//! Behaves as same as `T`.
//!
//! ### `()`
//! Always returns default values in every methods.
//!
//! ### `Option<T>`
//! If the value is `Some`, then behaves as same as `T`.
//! If the value is `None`, then behaves as same as `()`.
//!
//! ### `puroro::Either<T, U>`
//! Behaves as either `T` or `U`.
//!
//! ### `(T, U)`
//! Behaves as a merged message of `T` and `U`.
//! - Non-repeated, non-message field: Prioritize `U`'s value.
//! - Non-repeated, message field: Merges `T`'s and `U`'s values.
//! - Repeated field: Concatenates `T` and `U`'s repaeted values.
//!
