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

use crate::bumpalo::Bump;
use crate::internal::de::from_iter::deser_from_iter;
use crate::internal::de::DeserMessageFromBytesIter;
use crate::internal::se::SerMessageToIoWrite;
use crate::internal::NoAllocBumpBox;
use crate::Result;
use ::std::convert::TryFrom;
use ::std::io::Write;

/// A common trait for protobuf message implementation in Rust.
pub trait Message<M> {
    /// Deserialize the message from input bytes.
    /// This method does not clear the `&mut self` before deserializing.
    /// i.e. This method "merges" the input into `&mut self`.
    ///
    /// Please note that this method is not implemented for some types
    /// (e.g. Either<T, U> cannot be deserialized).
    ///
    /// ```ignore
    /// use puroro::Message;
    /// use std::io::Read;
    /// let mut my_message = MyMessage::default();
    /// let input = vec![0x80, 0x0a];
    /// my_message.merge_from_bytes(input.bytes()).unwrap();
    /// ```
    fn merge_from_bytes<I>(&mut self, iter: I) -> Result<()>
    where
        Self: DeserMessageFromBytesIter,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        deser_from_iter(self, iter)
    }

    /// A shorthand method that allocates and deserializes the message.
    fn from_bytes<I>(iter: I) -> Result<Self>
    where
        Self: DeserMessageFromBytesIter + Default,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        let mut msg = <Self as Default>::default();
        msg.merge_from_bytes(iter)?;
        Ok(msg)
    }

    /// Serializes the message into [`std::io::Write`].
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        Self: SerMessageToIoWrite,
        W: Write,
    {
        <Self as SerMessageToIoWrite>::ser(&self, out)
    }
}
impl<M> Message<M> for () where M: MessageRepresentativeImpl {}
impl<M, T, U> Message<M> for crate::Either<T, U>
where
    T: Message<M>,
    U: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<M, T, U> Message<M> for (T, U)
where
    T: Message<M>,
    U: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<M, T> Message<M> for Option<T>
where
    T: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<M, T> Message<M> for Box<T>
where
    T: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<'bump, M, T> Message<M> for NoAllocBumpBox<T>
where
    T: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<'a, M, T> Message<M> for &'a T
where
    T: Message<M>,
    M: MessageRepresentativeImpl,
{
}
impl<'a, M, T> Message<M> for &'a mut T
where
    T: Message<M>,
    M: MessageRepresentativeImpl,
{
}

pub trait MessageRepresentativeImpl {}

pub trait Enum2:
    'static + PartialEq + Clone + Default + TryFrom<i32, Error = i32> + Into<i32>
{
}
pub trait Enum3: 'static + PartialEq + Clone + Default + From<i32> + Into<i32> {}

/// Bumpalo message, initialized from bump ptr instance.
pub trait BumpaloMessage<'bump> {
    fn new_in(bump: &'bump Bump) -> Self;
}
impl<'bump, T> BumpaloMessage<'bump> for NoAllocBumpBox<T>
where
    T: BumpaloMessage<'bump>,
{
    fn new_in(bump: &'bump Bump) -> Self {
        NoAllocBumpBox::new_in(BumpaloMessage::new_in(bump), bump)
    }
}
