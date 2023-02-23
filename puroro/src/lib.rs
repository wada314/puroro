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

#![doc = include_str!("lib.md")]

pub mod doc_samples;
mod error;
pub mod internal;
pub mod message;
pub mod protobuf;
pub mod repeated;

pub use self::error::PuroroError;
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use crate::message::Message;
pub use crate::repeated::RepeatedFieldView;
