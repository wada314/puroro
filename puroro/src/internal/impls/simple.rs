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
//! will generate rust modules:
//!
//! ```rust
//! pub mod foo {
//!     pub mod bar {
//!         pub mod my_package {
//!             // The generated items
//!         }
//!     }
//! }
//! ```
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
//! // If the field is not present, then returns the default value,
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
//! // it sets the field as present at invocation timing (even if the returned
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
//! ### proto3 singular numeric fields
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
//! ### proto2 & proto3 optional string / bytes fields
//!
//! For the both of the following example:
//!
//! ```protobuf
//! optional string foo = 1;
//! required string foo = 1;
//! ```
//!
//! puroro generates methods like this:
//!
//! ```rust
//! # pub struct Message;
//! # impl Message {
//! // Returns the field value if the field is present.
//! // If the field is not present, then returns the default value,
//! // which is normally "" but in proto2 you can override it with
//! // a field option like `[default = "bar"]`.
//! pub fn foo(&self) -> &str {
//! #   todo!()
//!     // ...
//! }
//!
//! // Returns `Some` if the field is present, and `None` if not.
//! // Unlike the `foo()` method, the default value setting does not
//! // make any effect to this method.
//! pub fn foo_opt(&self) -> Option<&str> {
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
//! // it sets the field as present at invocation timing (even if the returned
//! // mutable reference is not used).
//! pub fn foo_mut(&mut self) -> &mut String {
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
//! The `bytes` fields behave almost as same as the string fields,
//! just replacing `str` by `[u8]`, and `String` by `Vec<u8>`.
//!
//! ### proto2 enum field
//!
//! The generated methods are the same with the numeric fields.
//! i.e. `foo(&self)`, `foo_opt(&self)`, `has_foo(&self)`,
//! `foo_mut(&mut self)` and `clear_foo(&mut self)`.
//! The return type is a generated `enum` type instead of the numeric types,
//! described below:
//!
//! For the enum like this:
//!
//! ```protobuf
//! enum MyEnum {
//!     ZEROTH = 0;
//!     FIRST = 1;
//!     TENTH = 10;
//! }
//! ```
//!
//! puroro generates an `enum` like this, and implements some traits to
//! convert from / into `i32`.
//!
//! ```rust
//! pub enum MyEnum {
//!     Zeroth,
//!     First,
//!     Tenth,
//! }
//!
//! impl TryFrom<i32> for MyEnum {
//!     type Error = i32;
//!     fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
//! #       todo!()
//!         // ...
//!     }
//! }
//! impl From<MyEnum> for i32 {
//!     fn from(value: MyEnum) -> i32 {
//! #       todo!()
//!         // ...
//!     }
//! }
//! ```
//!
//! The `enum` also implements the standard traits:
//! `Default`, `Clone`, `Copy`, `PartialEq`, and `Debug`.
//!
//! ### proto3 enum field
//!
//! It's similar to proto2 `enum`s, but it has an additional item `_Unknown(i32)`.
//! Because of existance of this item, the conversion from `i32` is now
//! a plain [`From`] instead of [`TryFrom`].
//!
//! ```rust
//! pub enum MyEnum {
//!     Zeroth,
//!     First,
//!     Tenth,
//!     _Unknown(i32),
//! }
//!
//! impl From<i32> for MyEnum {
//!     fn from(value: i32) -> Self {
//! #       todo!()
//!         // ...
//!     }
//! }
//! impl From<MyEnum> for i32 {
//!     fn from(value: MyEnum) -> i32 {
//! #       todo!()
//!         // ...
//!     }
//! }
//! ```
//!
//! ### Message singular field
//!
//! The message field has slightly different getters compared to other
//! field types.
//!
//! Assuming we have another message type `Bar`, for any of the
//! following field definitions:
//!
//! ```protobuf
//! // proto2
//! optional Bar foo = 1;
//! required Bar foo = 1;
//!
//! // proto3
//! Bar foo = 1;
//! optional Bar foo = 1;
//! ```
//!
//! puroro generates an interface like this:
//!
//! ```rust
//! # pub struct Message;
//! # pub struct Bar;
//! # impl Message {
//! // Returns the field value if the field is present.
//! // Please note that, unlike other field types, the getter returns
//! // `Option`-wrapped value.
//! pub fn foo(&self) -> Option<&Bar> {
//! #   todo!()
//!     // ...
//! }
//!
//! // Exactly the same as `foo()` method.
//! pub fn foo_opt(&self) -> Option<&Bar> {
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
//! // it sets the field as present at invocation timing (even if the returned
//! // mutable reference is not used).
//! pub fn foo_mut(&mut self) -> &mut Bar {
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
//! ### Repeated numeric / enum field
//!
//! For the following input:
//!
//! ```protobuf
//! repeated int32 foo = 1;
//! ```
//!
//! puroro generates the interface like this:
//!
//! ```rust
//! # pub struct Message;
//! # impl Message {
//! pub fn foo(&self) -> &[i32] {
//! #   todo!()
//!     // ...
//! }
//!
//! pub fn foo_mut(&mut self) -> &mut Vec<i32> {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!
//! Same for all other numeric types and enum types.
//!
//! ### Repeated string / bytes field
//!
//! For the following input:
//!
//! ```protobuf
//! repeated string foo = 1;
//! ```
//!
//! puroro generates the interface like this:
//!
//! ```rust
//! # pub struct Message;
//! # impl Message {
//! pub fn foo(&self) -> &[&str] {
//! #   todo!()
//!     // ...
//! }
//!
//! pub fn foo_mut(&mut self) -> &mut Vec<String> {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!
//! For `bytes` fields, just replace `str` by `[u8]`, and `String` by `Vec<u8>`.
//!
//! ### Repeated message field
//!
//! Assuming we have another message type `Bar`, for the following input:
//!
//! ```protobuf
//! repeated Bar foo = 1;
//! ```
//!
//! the following methods are generated:
//!
//! ```rust
//! # pub struct Message;
//! # pub struct Bar;
//! # impl Message {
//! pub fn foo(&self) -> &[Bar] {
//! #   todo!()
//!     // ...
//! }
//!
//! pub fn foo_mut(&mut self) -> &mut Vec<Bar> {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!
//! ### oneof fields
//!
//! Basically, `oneof` items have same interfaces with `optional` fields.
//! In addition to those, a getter method returning an `enum` to indicate
//! which `oneof` item is available is generated.
//!
//! For example, for the input like this:
//!
//! ```protobuf
//! oneof FooBar {
//!     int32 foo = 1;
//!     string bar = 2;
//!     // Note: you cannot specify "optional", "required" or "repeated"
//!     // for oneof items in both proto2 and proto3.
//! }
//! ```
//!
//! The following rust code is generated:
//!
//! ```rust
//! pub enum FooBar<'a> {
//!     Foo(i32),
//!     Bar(&'a str),
//! }
//!
//! # pub struct Message;
//! # impl Message {
//! pub fn foo_bar(&self) -> Option<FooBar> {
//! #   todo!()
//!     // ...
//! }
//!
//! pub fn clear_foo_bar(&mut self) {
//! #   todo!()
//!     // ...
//! }
//! # }
//! ```
//!

pub mod de;
