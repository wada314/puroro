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

use crate::Result;
use ::futures::io::{AsyncRead, AsyncWrite};
use ::std::io::{BufRead, BufReader, Read, Write};
use ::std::ops::{Deref, DerefMut};

/// Protobuf message, which can be serialized, deserialized, and field accessible.
pub trait Message {
    fn merge_from_read<R: Read>(&mut self, read: R) -> Result<()> {
        self.merge_from_bufread(BufReader::new(read))
    }
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()>;
    fn deser_from_read<R: Read>(read: R) -> Result<Self>
    where
        Self: Sized + Default,
    {
        let mut msg = Self::default();
        msg.merge_from_read(read)?;
        Ok(msg)
    }
    fn deser_from_bufread<R: BufRead>(read: R) -> Result<Self>
    where
        Self: Sized + Default,
    {
        let mut msg = Self::default();
        msg.merge_from_bufread(read)?;
        Ok(msg)
    }
    fn write<W: Write>(&self, write: W) -> Result<usize>;
}

impl<T> Message for T
where
    T: Deref + DerefMut,
    <T as Deref>::Target: Message,
{
    fn merge_from_read<R: Read>(&mut self, read: R) -> Result<()> {
        DerefMut::deref_mut(self).merge_from_read(read)
    }
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        DerefMut::deref_mut(self).merge_from_bufread(read)
    }
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        Deref::deref(self).write(&mut write)
    }
}
