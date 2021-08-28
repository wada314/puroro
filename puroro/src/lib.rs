#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

pub mod bool;
mod error;
pub mod fixed_bits;
pub mod tags;
pub mod types;
pub mod variant;

use ::std::convert::TryFrom;

pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;
pub use ::hashbrown;

pub trait Message {}
impl<T, U> Message for ::either::Either<T, U>
where
    T: Message,
    U: Message,
{
}

pub trait Enum2:
    'static + PartialEq + Clone + Default + TryFrom<i32, Error = i32> + Into<i32>
{
}
pub trait Enum3: 'static + PartialEq + Clone + Default + From<i32> + Into<i32> {}

pub trait RepeatedField<'msg>: IntoIterator {}
pub struct EitherRepeatedField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Either<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'msg, T, U> RepeatedField<'msg> for EitherRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
}

pub trait DeserFromBytesIter: Message {
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait SerToIoWrite: Message {
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: ::std::io::Write;
}
