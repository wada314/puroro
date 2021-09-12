#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod bool;
pub mod desc;
mod error;
pub mod fixed_bits;
pub mod internal;
pub mod tags;
pub mod types;
pub mod variant;

use ::std::convert::TryFrom;

pub use self::error::{ErrorKind, PuroroError};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;
pub use ::hashbrown;
pub use ::once_cell;

pub trait Message<M>
where
    M: MessageRepresentativeImpl,
{
    fn descriptor() -> &'static desc::MessageDescriptor {
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
impl<M> Message<M> for () where M: MessageRepresentativeImpl {}

pub trait MessageRepresentativeImpl {
    fn descriptor() -> &'static desc::MessageDescriptor;
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
