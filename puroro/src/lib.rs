#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]

mod error;
pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;

pub trait DeserializableFromIter {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait Serializable: Sized {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> Result<()>;
}

pub trait Message<'bump> {
    type InternalData: InternalData<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData;
}
pub trait Enum {}

pub trait InternalData<'bump> {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &'bump ::bumpalo::Bump;
}
