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
pub mod repeated_field;
pub mod tags;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub use self::repeated_field::{AsRefRepeatedField, CloneThenIntoRepeatedField, RepeatedField};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

use ::std::ops::{Deref, DerefMut};

// Bumpalo wrapper
pub struct BumpaloOwned<T> {
    // The field order matters, `Drop` drops the field in decl order.
    t: T,
    bump: Box<crate::bumpalo::Bump>,
}
impl<T> BumpaloOwned<T> {
    pub fn bump(this: &BumpaloOwned<T>) -> &crate::bumpalo::Bump {
        &this.bump
    }
    pub fn inner(this: &BumpaloOwned<T>) -> &T {
        &this.t
    }
    pub fn inner_mut(this: &mut BumpaloOwned<T>) -> &mut T {
        &mut this.t
    }
}
impl<T> BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    pub fn new() -> Self {
        let bump = Box::new(crate::bumpalo::Bump::new());
        let t = crate::internal::BumpDefault::default_in(unsafe {
            ::std::mem::transmute(bump.as_ref())
        });
        Self { t, bump }
    }
}
impl<T> Default for BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Deref for BumpaloOwned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
impl<T> DerefMut for BumpaloOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<M, T> Message<M> for BumpaloOwned<T> where T: Message<M> {}
