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

//! Reference generated-style code for the `Person` message.
//!
//! This file is checked into the repository as a concrete example of the intended generated API
//! (traits + standard implementation + lazy implementation), and as test input for the code generator.
//!
//! Source schema: `sandbox/protos/person.proto`.
//!
//! NOTE: This module is intentionally split into submodules to keep each concern readable and
//! generator-friendly (traits / enums / standard impl / lazy impl / tests).
//!
//! IMPORTANT: This file/module split is for the **design stage** only. When we implement the real
//! code generator, we may switch back to a simpler "one module = one file" output (or another
//! layout). Do not treat the current file structure as the final generated format.

mod impl_;
mod lazy;
mod status;
mod traits;

#[cfg(test)]
mod tests;

// Import Address-related types from the separate module
pub use super::address::{Address, AddressImpl, AddressMut, AddressTry};

pub use self::impl_::PersonImpl;
pub use self::lazy::PersonLazyImpl;
pub use self::status::Status;
pub use self::traits::{Person, PersonMut, PersonTry};
