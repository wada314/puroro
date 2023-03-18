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

use crate::internal::ScopedIter;
use crate::Result;
#[cfg(feature = "allocator_api")]
use ::std::alloc::Allocator;
use ::std::io::Result as IoResult;

pub trait MessageInternal: crate::Message {
    fn from_scoped_bytes_iter<'a, I: Iterator<Item = IoResult<u8>>>(
        scoped_iter: &mut ScopedIter<'a, I>,
    ) -> Result<Self>
    where
        Self: Default,
    {
        let mut msg = Self::default();
        msg.merge_from_scoped_bytes_iter(scoped_iter)?;
        Ok(msg)
    }

    fn merge_from_scoped_bytes_iter<'a, I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        scoped_iter: &mut ScopedIter<'a, I>,
    ) -> Result<()>;

    fn from_boxed_view(v: Box<Self::ViewType>) -> Self;
    fn into_boxed_view(self) -> Box<Self::ViewType>;
}

pub trait MessageViewInternal: crate::MessageView {
    #[cfg(feature = "allocator_api")]
    fn new_in<A: Allocator>(allocator: A) -> Box<Self, A>;
}
