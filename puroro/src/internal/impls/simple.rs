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

pub mod de;
