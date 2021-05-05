#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]

mod error;
use std::ops::Deref;

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

pub trait RepeatedField<T> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T);
    fn boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &T>>;

    #[cfg(feature = "puroro-nightly")]
    type Iter<'a>: ::std::iter::Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    #[cfg(feature = "puroro-nightly")]
    fn iter(&self) -> Self::Iter<'_>;
}

impl<T> RepeatedField<T> for Vec<T> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T),
    {
        self.deref().iter().for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = &T>> {
        Box::new(self.deref().iter())
    }

    #[cfg(feature = "puroro-nightly")]
    type Iter<'a>
    where
        Self: 'a,
        T: 'a,
    = std::slice::Iter<'a, T>;

    #[cfg(feature = "puroro-nightly")]
    fn iter(&self) -> Self::Iter<'_> {
        self.deref().iter()
    }
}

impl<'bump, T> RepeatedField<T> for ::bumpalo::collections::Vec<'bump, T> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T),
    {
        self.deref().iter().for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = &T>> {
        Box::new(self.deref().iter())
    }

    #[cfg(feature = "puroro-nightly")]
    type Iter<'a>
    where
        Self: 'a,
        T: 'a,
    = std::slice::Iter<'a, T>;

    #[cfg(feature = "puroro-nightly")]
    fn iter(&self) -> Self::Iter<'_> {
        self.deref().iter()
    }
}
