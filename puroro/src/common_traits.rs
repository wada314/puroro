use crate::desc::MessageDescriptor;
use crate::{Either, Result};
use ::std::convert::TryFrom;

pub trait Message<M>
where
    M: MessageRepresentativeImpl,
{
    fn descriptor() -> &'static MessageDescriptor {
        M::descriptor()
    }
}
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
impl<M> Message<M> for () where M: MessageRepresentativeImpl {}

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

pub trait DeserFromBytesIter {
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait SerToIoWrite {
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: ::std::io::Write;
}
impl SerToIoWrite for () {
    fn ser<W>(&self, _: &mut W) -> Result<()>
    where
        W: ::std::io::Write,
    {
        Ok(())
    }
}
impl<T, U> SerToIoWrite for (T, U)
where
    T: SerToIoWrite,
    U: SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: ::std::io::Write,
    {
        self.0.ser(out)?;
        self.1.ser(out)?;
        ::std::result::Result::Ok(())
    }
}

impl<T, U> SerToIoWrite for Either<T, U>
where
    T: SerToIoWrite,
    U: SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: ::std::io::Write,
    {
        match self {
            Either::Left(v) => v.ser(out)?,
            Either::Right(v) => v.ser(out)?,
        }
        ::std::result::Result::Ok(())
    }
}
