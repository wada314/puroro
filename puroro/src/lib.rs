#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

mod collections;
mod error;

pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;
pub use collections::{MapField, RepeatedField};

pub trait DeserializableFromIter {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
pub trait DeserializableFromSlice<'slice>: Sized {
    fn deser_from_slice(slice: &'slice [u8]) -> Result<Self>;
}

pub trait Serializable: Sized {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> Result<()>;
}

pub trait Message<'bump> {
    type InternalData: InternalData<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData;
    type BoxedType: AsMut<Self>;
    fn into_boxed(self) -> Self::BoxedType;
}
pub trait Enum {}

pub trait InternalData<'bump> {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &'bump ::bumpalo::Bump;
}
