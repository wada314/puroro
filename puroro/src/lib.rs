#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]

mod error;

pub mod fixed_bits;
pub mod tags;
pub mod types;
pub mod variant;
pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::hashbrown;

pub trait Message {}

pub trait GetImpl<NewImplTypeTag> {
    type Type;
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
