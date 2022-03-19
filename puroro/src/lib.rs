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
pub mod internal;
mod sample;
pub mod tags;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

//////////////////////////////////////////////////////////////

pub trait GenericMessage {
    fn try_get_field(&self, number: i32) -> Result<&dyn GenericField>;
}

pub trait GenericField {
    fn try_get_u32(&self) -> Result<u32> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_str(&self) -> Result<&str> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
    fn try_get_message(&self) -> Result<&dyn GenericMessage> {
        Err(ErrorKind::IncorrectFieldGetter)?
    }
}

impl GenericField for u32 {
    fn try_get_u32(&self) -> Result<u32> {
        Ok(*self)
    }
}

impl<'msg> GenericField for String {
    fn try_get_str(&self) -> Result<&str> {
        Ok(self.as_str())
    }
}

pub struct MessageField<M>(M);
impl<M: AsRef<dyn GenericMessage>> GenericField for MessageField<M> {
    fn try_get_message(&self) -> Result<&dyn GenericMessage> {
        Ok(self.0.as_ref())
    }
}
