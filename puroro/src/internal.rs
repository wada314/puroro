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

pub mod bool;
pub mod de;
pub mod fixed_bits;
pub mod impls;
pub mod se;
pub mod types;
pub mod variant;

use crate::desc::{FieldDescriptor, MessageDescriptor};
use crate::once_cell::sync::Lazy;
use crate::{Either, Result};
use ::std::io::Write;

pub struct MessageDescriptorInitializer {
    pub name: &'static str,
    pub lazy_fields: Lazy<&'static [FieldDescriptor]>,
}

pub fn init_message_descriptor(init: MessageDescriptorInitializer) -> MessageDescriptor {
    MessageDescriptor {
        name: init.name,
        lazy_fields: init.lazy_fields,
    }
}

pub struct FieldDescriptorInitializer {
    pub name: &'static str,
    pub number: i32,
    pub lazy_containing_type: Lazy<&'static MessageDescriptor>,
}

pub fn init_field_descriptor(init: FieldDescriptorInitializer) -> FieldDescriptor {
    FieldDescriptor {
        name: init.name,
        number: init.number,
        lazy_containing_type: init.lazy_containing_type,
    }
}

pub trait DeserializableMessageFromBytesIterator {
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
impl<'a, T> DeserializableMessageFromBytesIterator for &'a mut T
where
    T: DeserializableMessageFromBytesIterator,
{
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        <T as DeserializableMessageFromBytesIterator>::deser(*self, iter)
    }
}
impl<T> DeserializableMessageFromBytesIterator for Box<T>
where
    T: DeserializableMessageFromBytesIterator,
{
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        <T as DeserializableMessageFromBytesIterator>::deser(self.as_mut(), iter)
    }
}
impl<T> DeserializableMessageFromBytesIterator for Option<T>
where
    T: DeserializableMessageFromBytesIterator + Default,
{
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        <T as DeserializableMessageFromBytesIterator>::deser(
            self.get_or_insert_with(Default::default),
            iter,
        )
    }
}

pub trait SerializableMessageToIoWrite {
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write;
}
impl SerializableMessageToIoWrite for () {
    fn ser<W>(&self, _: &mut W) -> Result<()>
    where
        W: Write,
    {
        Ok(())
    }
}
impl<T, U> SerializableMessageToIoWrite for (T, U)
where
    T: SerializableMessageToIoWrite,
    U: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.0.ser(out)?;
        self.1.ser(out)?;
        Ok(())
    }
}
impl<T, U> SerializableMessageToIoWrite for Either<T, U>
where
    T: SerializableMessageToIoWrite,
    U: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        match self {
            Either::Left(v) => v.ser(out)?,
            Either::Right(v) => v.ser(out)?,
        }
        Ok(())
    }
}
impl<T> SerializableMessageToIoWrite for Option<T>
where
    T: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        if let Some(ref msg) = self {
            msg.ser(out)?;
        }
        Ok(())
    }
}
impl<T> SerializableMessageToIoWrite for Box<T>
where
    T: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerializableMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
impl<'a, T> SerializableMessageToIoWrite for &'a T
where
    T: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerializableMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
impl<'a, T> SerializableMessageToIoWrite for &'a mut T
where
    T: SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerializableMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
