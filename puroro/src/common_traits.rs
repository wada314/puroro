use crate::desc::MessageDescriptor;
use crate::{Either, Result};
use ::std::convert::TryFrom;
use ::std::io::Write;

pub trait Message<M> {
    fn descriptor() -> &'static MessageDescriptor
    where
        M: MessageRepresentativeImpl,
    {
        M::descriptor()
    }

    fn merge_from_bytes<I>(&mut self, iter: I) -> Result<()>
    where
        Self: DeserializableMessageFromBytesIterator,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        <Self as DeserializableMessageFromBytesIterator>::deser(self, iter)
    }

    fn from_bytes<I>(iter: I) -> Result<Self>
    where
        Self: DeserializableMessageFromBytesIterator + Default,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        let mut msg = <Self as Default>::default();
        msg.merge_from_bytes(iter)?;
        Ok(msg)
    }

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
