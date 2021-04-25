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
