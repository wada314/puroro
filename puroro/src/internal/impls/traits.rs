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
//! The trait only implements immutable interfaces, not mutable ones.
//!
//! Please also check the [Generated struct](super::simple) page for
//! related informations like generated `enum`.
//!
//! ## Fields
//!
//! ### Singular non-message fields
//!
//! For any of the following inputs,
//!
//! ```protobuf
//! optional int32 foo = 1;
//! repeated int32 foo = 1;
//!
//! // proto3 only
//! int32 foo = 1;
//! ```
//!
//! The following trait methods are generated:
//!
//! ```rust
//! # trait MyMessageTrait {
//! // Returns the field value if the field is present.
//! // Please note that for proto3 bare singluar field, 0-value fields
//! // (or "" for string, b"" for bytes) are also considered as not present.
//! // If the field is not present, then returns the default value,
//! // which is normally 0 but in proto2 you can override it with
//! // a field option like `[default = 10]`.
//! fn foo(&self) -> i32;
//!
//! // Returns `Some` if the field is present, and `None` if not.
//! // Unlike the `foo()` method, the default value setting does not
//! // make any effect to this method.
//! fn foo_opt(&self) -> Option<i32>;
//!
//! // A shorthand of `self.foo_opt().is_some()`.
//! fn has_opt(&self) -> bool;
//! # }
//! ```
//!
//! The field return type are corresponding rust primitive type for
//! numerical types, `&str` for `string`, `&[u8]` for `bytes`, and the
//! generated `enum` (by value) for `enum` field.
//!
//! ### Singluar message field
//!
//! Assuming we have another message type `message Bar`,
//! for any of the following inputs:
//!
//! ```protobuf
//! optional Bar foo = 1;
//! repeated Bar foo = 1;
//!
//! // proto3 only
//! Bar foo = 1;
//! ```
//!
//! The following trait methods and typedefs are generated:
//!
//! ```rust
//! # trait BarTrait {}
//! # trait MyMessageTrait {
//! #
//! type FooMessageType<'this>: BarTrait;
//!
//! // Returns the field value if the field is present.
//! // Please note that for proto3 bare singluar field, 0-value fields
//! // (or "" for string, b"" for bytes) are also considered as not present.
//! // If the field is not present, then returns the default value,
//! // which is normally 0 but in proto2 you can override it with
//! // a field option like `[default = 10]`.
//! fn foo(&self) -> i32;
//!
//! // Returns `Some` if the field is present, and `None` if not.
//! // Unlike the `foo()` method, the default value setting does not
//! // make any effect to this method.
//! fn foo_opt(&self) -> Option<i32>;
//!
//! // A shorthand of `self.foo_opt().is_some()`.
//! fn has_opt(&self) -> bool;
//! # }
//! ```
//!
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
//! # use ::std::ops::Deref;
//! pub trait MyMessageTrait {
//!     fn my_number(&self) -> i32;
//!     fn my_number_opt(&self) -> Option<i32>;
//!     fn has_my_number(&self) -> bool;
//!
//!     type Field2RepeatedType<'this>: IntoIterator<Item=&'this str> where Self: 'this;
//!     fn my_name(&self) -> Self::Field2RepeatedType<'_>;
//!     fn has_my_name(&self) -> bool;
//!
//!     type Field3MessageType<'this>: MyMessageTrait where Self: 'this;
//!     fn my_child(&self) -> Option<Self::Field3MessageType<'_>>;
//!     fn my_name_opt(&self) -> Option<Self::Field3MessageType<'_>>;
//!     fn has_my_child(&self) -> bool;
//! }
//! ```
//!
//! Each message field will generate a getter method which has same name
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
//! The list of the generated getter methods' return types:
//!
//! | base protobuf type | non-`repeated` fields | `repeated`                     |
//! |--------------------|-----------------------|--------------------------------|
//! | `int32`            | `i32`                 | `impl IntoIterator<Item=i32>`  |
//! | (Any numeric types)| `T`                   | `impl IntoIterator<Item=T>`    |
//! | `bytes`            | `&[u8]`               | `impl IntoIterator<Item=&[u8]>`|
//! | `string`           | `&str`                | `impl IntoIterator<Item=&str>` |
//! | `SomeMessage`      | `Option<impl SomeMessageTrait>`|`impl IntoIterator<Item=impl SomeMessageTrait>`|
//!
//! It also generates a getter method with postfix `_opt`, which always returns `Option` type.
//! If the field value is not set (`optional` or `required` fields) or the field value is
//! default (unlabeled fields) then this method returns `None`, otherwise `Some`.
//!
//! And a method with prefix `has_` is generated, which is a shortcut of `self.foo_opt().is_some()`.
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
//!
//!     fn item1(&self) -> i32;
//!     fn item1_opt(&self) -> Option<i32>;
//!     fn has_item1(&self) -> bool;
//!     fn item2(&self) -> f32;
//!     fn item2_opt(&self) -> Option<f32>;
//!     fn has_item2(&self) -> bool;
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
//! # Default values / has_ bits
//!
//! ## proto2
//!
//! In proto2, you can set a field's default value:
//! ```protobuf
//! syntax = "proto2";
//! message MyMessage {
//!     int32 my_number = 1 [default = 42];
//! }
//! ```
//!
//! The generated trait's getter method returns the default value
//! when the value is not set (when `has_xxx()` returns false).
//!
//! ```rust
//! # trait MyMessageTrait {
//! #     fn my_number(&self) -> i32;
//! #     fn has_my_number(&self) -> bool;
//! # }
//! # impl MyMessageTrait for () {
//! #     fn my_number(&self) -> i32 { 42 }
//! #     fn has_my_number(&self) -> bool { false }
//! # }
//! assert_eq!(42, ().my_number());
//! assert_eq!(false, ().has_my_number());
//! ```
//!
//! ## proto3
//!
//! In proto3 unlabeled fields, 0 value (or empty string / bytes) fields
//! are considered as empty and not set.
//!
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     int32 my_number = 1;
//! }
//! ```
//!
//! ```rust
//! # #[derive(Default)]
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! # }
//! # trait MyMessageTrait {
//! #     fn my_number(&self) -> i32;
//! #     fn has_my_number(&self) -> bool;
//! # }
//! # impl MyMessageTrait for MyMessage {
//! #     fn my_number(&self) -> i32 { self.my_number }
//! #     fn has_my_number(&self) -> bool { self.my_number != 0 }
//! # }
//! assert_eq!(0, MyMessage::default().my_number());
//! assert_eq!(false, MyMessage::default().has_my_number());
//! ```
//!
