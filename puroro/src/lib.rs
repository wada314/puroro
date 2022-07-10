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
#![feature(backtrace)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
// Allow using GAT in document sample code.
#![doc(test(attr(feature(generic_associated_types))))]

mod common_traits;
mod error;
mod generic_message;
pub mod internal;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub type Result<T> = ::std::result::Result<T, PuroroError>;
pub use generic_message::GenericMessage;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

pub trait FieldDescriptorType {}
pub trait DescriptorType {}
