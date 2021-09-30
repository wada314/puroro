use crate::desc::MessageDescriptor;
use crate::internal::{DeserializableMessageFromBytesIterator, SerializableMessageToIoWrite};
use crate::Result;
use ::std::convert::TryFrom;
use ::std::io::Write;

/// A common trait for protobuf message implementation in Rust.
pub trait Message<M> {
    /// Returns a descriptor of message.
    /// Though currently the descriptor has almost no items.
    /// More details may come in future versions.
    fn descriptor() -> &'static MessageDescriptor
    where
        M: MessageRepresentativeImpl,
    {
        M::descriptor()
    }

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
        Self: DeserializableMessageFromBytesIterator,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        <Self as DeserializableMessageFromBytesIterator>::deser(self, iter)
    }

    /// A shorthand method that allocates and deserializes the message.
    fn from_bytes<I>(iter: I) -> Result<Self>
    where
        Self: DeserializableMessageFromBytesIterator + Default,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        let mut msg = <Self as Default>::default();
        msg.merge_from_bytes(iter)?;
        Ok(msg)
    }

    /// Serializes the message into [`std::io::Write`].
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        Self: SerializableMessageToIoWrite,
        W: Write,
    {
        <Self as SerializableMessageToIoWrite>::ser(&self, out)
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

pub trait MessageRepresentativeImpl {
    fn descriptor() -> &'static MessageDescriptor;
}

pub trait Enum2:
    'static + PartialEq + Clone + Default + TryFrom<i32, Error = i32> + Into<i32>
{
}
pub trait Enum3: 'static + PartialEq + Clone + Default + From<i32> + Into<i32> {}

/// A trait for repaeted field value returned from message traits getter methods.
///
/// Currently this trait is just an analogous of [`std::iter::IntoIterator`].
pub trait RepeatedField<'msg>: IntoIterator {}
impl<'msg, T> RepeatedField<'msg> for T where T: IntoIterator {}
