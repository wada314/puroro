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

//!
//! # Generated code
//!
//! ## Packages
//!
//! The package declaration in the `.proto` files are converted into
//! `lower_snake_case` and be used as a module path in generated code.
//! For example,
//!
//! ```protobuf
//! package foo.bar.my_package
//! ```
//!
//! will generate code in `foo::bar::my_package` module.
//!
//! ## Messages and nested Messages
//!
//! The message in the `.proto` generates a `CamelCase` named struct:
//!
//! ```protobuf
//! message LaptopDevice {}
//! ```
//!
//! generates:
//!
//! ```rust
//! pub struct LaptopDevice {}
//! ```
//!
//! The struct implements [`Default`], [`Clone`], [`PartialEq`] and
//! [`Debug`](std::fmt::Debug) traits by default. It also has `new()` method to
//! initialize it.
//!
//! The generated struct always implements [`Message`](crate::Message) trait.
//! The trait has methods for serializing and deserializing the protobuf fields.
//!
//! For the nested message:
//!
//! ```protobuf
//! message LaptopDevice {
//!     message Manufacturer {}
//! }
//! ```
//!
//! a `lower_snake_case` module name is generated from the outer message name:
//!
//! ```rust
//! pub struct LaptopDevice {}
//! pub mod laptop_device {
//!     pub struct Manufacturer {}
//! }
//! ```
//!
//! ## Fields
//!
//! ### proto2 & proto3 optional numeric fields
//!
//! For either of the following inputs:
//!
//! ```protobuf
//! optional int32 foo = 1;
//! required int32 foo = 1;
//! ```
//!
//! puroro generates the methods like this:
//!
//! ```rust
//! # pub struct Message;
//! # impl Message {
//! // Returns the field value if the field is present.
//! // If the field is not present, then returns the "default" value,
//! // which is normally 0 but in proto2 you can override it with
//! // a field option like `[default = 10]`.
//! pub fn foo(&self) -> i32 {
//! #   todo!()
//!     // ...
//! }
//!
//! // Returns `Some` if the field is present, and `None` if not.
//! // Unlike the `foo()` method, the default value setting does not
//! // make any effect to this method.
//! pub fn foo_opt(&self) -> Option<i32> {
//! #   todo!()
//!     // ...
//! }
//!
//! // A shorthand of `self.foo_opt().is_some()`.
//! pub fn has_foo(&self) -> bool {
//! #   todo!()
//!     // ...
//! }
//!
//! // Returns a mutable reference. Similar to `Option::get_or_insert_with()`,
//! // it sets the field present at invocation timing (even if the returned
//! // mutable reference is not used).
//! pub fn foo_mut(&mut self) -> &mut i32 {
//! #   todo!()
//!     // ...
//! }
//!
//! // Unset the field.
//! pub fn clear_foo(&mut self) {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!
//! A list of corresponding primitive types between
//! protobuf and rust:
//!
//! |protobuf type|rust type|
//! |-------------|---------|
//! | `int32`    | `i32` |
//! | `sint32`   | `i32` |
//! | `sfixed32` | `i32` |
//! | `uint32`   | `u32` |
//! | `fixed32`  | `u32` |
//! | `int64`    | `i64` |
//! | `sint64`   | `i64` |
//! | `sfixed64` | `i64` |
//! | `uint64`   | `u64` |
//! | `fixed64`  | `u64` |
//! | `float`    | `f32` |
//! | `double`   | `f64` |
//! | `bool`     | `bool`|
//!
//! ### proto3 scalar numeric fields
//!
//! For the following input:
//!
//! ```protobuf
//! int32 foo = 1;
//! ```
//!
//! puroro generates the methods like this:
//!
//! ```rust
//! # pub struct Message;
//! # impl Message {
//! // Returns the field value.
//! pub fn foo(&self) -> i32 {
//! #   todo!()
//!     // ...
//! }
//!
//! // If the field value is non-`0`, then returns `Some` wrapped value.
//! // If the field value is `0`, then returns `None`.
//! pub fn foo_opt(&self) -> Option<i32> {
//! #   todo!()
//!     // ...
//! }
//!
//! // A shorthand of `self.foo_opt().is_some()`.
//! pub fn has_foo(&self) -> bool {
//! #   todo!()
//!     // ...
//! }
//!
//! // Returns a mutable reference.
//! pub fn foo_mut(&mut self) -> &mut i32 {
//! #   todo!()
//!     // ...
//! }
//!
//! // Set the field value to `0`.
//! pub fn clear_foo(&mut self) {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!

pub mod de;
