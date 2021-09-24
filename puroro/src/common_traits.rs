use crate::desc::MessageDescriptor;
use crate::internal::{DeserializableMessageFromBytesIterator, SerializableMessageToIoWrite};
use crate::Result;
use ::std::convert::TryFrom;
use ::std::io::Write;

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
    /// Please note that not all the message implementing this `Message` trait can be
    /// deserialized from the input bytes (e.g. Either<T, U> cannot be deserialized).
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

pub trait RepeatedField<'msg>: IntoIterator {}
impl<'msg, T> RepeatedField<'msg> for T where T: IntoIterator {}
