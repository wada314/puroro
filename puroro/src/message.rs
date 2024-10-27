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

use crate::default_in::DefaultIn;
use crate::Result;
use ::std::alloc::Allocator;
use ::std::io::{BufRead, BufReader, Read, Write};
use ::std::ops::{Deref, DerefMut};

/// Protobuf message, which can be serialized and field accessible.
pub trait Message {
    fn write<W: Write>(&self, write: W) -> Result<usize>;
}

/// Mutable protobuf message
pub trait MessageMut<A: Allocator>: Message {
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
    fn deser_from_read_in<R: Read>(read: R, alloc: A) -> Result<Self>
    where
        Self: Sized + DefaultIn<A>,
    {
        let mut msg = Self::default_in(alloc);
        msg.merge_from_read(read)?;
        Ok(msg)
    }
    fn deser_from_bufread_in<R: BufRead>(read: R, alloc: A) -> Result<Self>
    where
        Self: Sized + DefaultIn<A>,
    {
        let mut msg = Self::default_in(alloc);
        msg.merge_from_bufread(read)?;
        Ok(msg)
    }
}

impl<T: Message> Message for &T {
    fn write<W: Write>(&self, write: W) -> Result<usize> {
        T::write(self, write)
    }
}
impl<T: Message> Message for &mut T {
    fn write<W: Write>(&self, write: W) -> Result<usize> {
        T::write(self, write)
    }
}
impl<T: Message> Message for Option<T> {
    fn write<W: Write>(&self, write: W) -> Result<usize> {
        match self {
            Some(msg) => msg.write(write),
            None => Ok(0),
        }
    }
}
impl<T: Message, U: Message> Message for (T, U) {
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        total_bytes += self.0.write(&mut write)?;
        total_bytes += self.1.write(&mut write)?;
        Ok(total_bytes)
    }
}

impl<T: MessageMut<A>, A: Allocator> MessageMut<A> for &mut T {
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        T::merge_from_bufread(self, read)
    }
}
impl<T: Message, U: MessageMut<A>, A: Allocator> MessageMut<A> for (T, U) {
    fn merge_from_bufread<R: BufRead>(&mut self, mut read: R) -> Result<()> {
        self.1.merge_from_bufread(&mut read)?;
        Ok(())
    }
}
