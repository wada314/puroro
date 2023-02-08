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

mod error;
pub mod internal;
pub mod message;
pub mod protobuf;

pub use self::error::PuroroError;
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use crate::message::Message;

#[cfg(feature = "dev-for-protobuf-use-stable-puroro")]
mod puroro_for_protobuf {
    pub use ::stable_puroro::*;
}
#[cfg(not(feature = "dev-for-protobuf-use-stable-puroro"))]
mod puroro_for_protobuf {
    pub use super::*;
}

use ref_cast::RefCast;

pub trait PersonTrait {
    fn partner_opt(&self) -> Option<&dyn PersonTrait>;
    fn children(&self) -> &dyn RepeatedField<Item = dyn PersonTrait>;
    fn head_opt(&self) -> Option<&dyn HeadTrait>;
}

pub trait HeadTrait {
    fn mouth_opt(&self) -> Option<&dyn MouthTrait>;
}
pub trait MouthTrait {
    fn tooth(&self) -> &dyn RepeatedField<Item = dyn TeethTrait>;
}
pub trait TeethTrait {}

pub struct PersonTuple<T, U>(T, U);
impl<T: PersonTrait, U: PersonTrait> PersonTrait for PersonTuple<T, U> {
    fn partner_opt(&self) -> Option<&dyn PersonTrait> {
        todo!()
    }

    fn children(&self) -> &dyn RepeatedField<Item = dyn PersonTrait> {
        todo!()
    }

    fn head_opt(&self) -> Option<&dyn HeadTrait> {
        match (self.0.head_opt(), self.1.head_opt()) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(_), Some(_)) => Some(PersonField::<_, 3>::ref_cast(self)),
        }
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct PersonField<T, const NUMBER: usize>(T);

#[repr(transparent)]
#[derive(RefCast)]
pub struct HeadField<T, const NUMBER: usize>(T);

#[repr(transparent)]
#[derive(RefCast)]
pub struct MouthField<T, const NUMBER: usize>(T);

impl<T: PersonTrait> HeadTrait for PersonField<T, 3> {
    fn mouth_opt(&self) -> Option<&dyn MouthTrait> {
        Some(HeadField::<_, 1>::ref_cast(self))
    }
}
impl<T: HeadTrait> MouthTrait for HeadField<T, 1> {
    fn tooth(&self) -> &dyn RepeatedField<Item = dyn TeethTrait> {
        MouthField::<_, 1>::ref_cast(self)
    }
}
impl<T: MouthTrait> RepeatedField for MouthField<T, 1> {
    type Item = dyn TeethTrait;
    fn get(&self, index: usize) -> &Self::Item {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

pub trait RepeatedField {
    type Item: ?Sized;
    fn get(&self, index: usize) -> &Self::Item;
    fn len(&self) -> usize;
}
impl<'a, T> IntoIterator for &'a dyn RepeatedField<Item = T> {
    type Item = &'a T;
    type IntoIter = RepeatedFieldIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        RepeatedFieldIter {
            repeated: self,
            len: self.len(),
            cur: 0,
        }
    }
}

pub struct RepeatedFieldIter<'a, T> {
    repeated: &'a dyn RepeatedField<Item = T>,
    len: usize,
    cur: usize,
}
impl<'a, T> Iterator for RepeatedFieldIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len >= self.cur {
            None
        } else {
            let ret = self.repeated.get(self.cur);
            self.cur += 1;
            Some(ret)
        }
    }
}
