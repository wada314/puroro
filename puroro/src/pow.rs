use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::ops::Deref;

/// A custom [`std::borrow::Cow`] for puroro library.
/// Instead of restricting the owned type into `<B as ToOwned>::Owned` type,
/// any type that implements [`std::borrow::Borrow<B>`] can be used as an owned
/// type.
/// Instead, this type loses a function to copy-on-write, specifically
/// `Cow::into_owned()` method. In other words, though this type supports
/// owned => borrowed conversion using `borrow()` method, not vice versa.
///
/// We need this custom Cow type because the owned type might want to use a
/// custom allocator: e.g. [`bumpalo::collections::String<'bump>`].
pub enum Pow<'a, B, O = <B as ToOwned>::Owned>
where
    B: 'a + ?Sized + ToOwned,
    O: Borrow<B>,
{
    Borrowed(&'a B),
    Owned(O),
}

impl<'a, B, O> Deref for Pow<'a, B, O>
where
    B: 'a + ?Sized + ToOwned,
    O: Borrow<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        match *self {
            Pow::Borrowed(borrowed) => borrowed,
            Pow::Owned(ref owned) => owned.borrow(),
        }
    }
}

impl<'a, B, O> Borrow<B> for Pow<'a, B, O>
where
    B: 'a + ?Sized + ToOwned,
    O: Borrow<B>,
{
    fn borrow(&self) -> &B {
        &*self
    }
}

impl<'a, B, O> Display for Pow<'a, B, O>
where
    B: 'a + ?Sized + ToOwned + Display,
    O: Borrow<B> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pow::Borrowed(borrowed) => <B as Display>::fmt(borrowed, f),
            Pow::Owned(ref owned) => <O as Display>::fmt(owned, f),
        }
    }
}

impl<'a, B, O> Debug for Pow<'a, B, O>
where
    B: 'a + ?Sized + ToOwned + Debug,
    O: Borrow<B> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pow::Borrowed(borrowed) => <B as Debug>::fmt(borrowed, f),
            Pow::Owned(ref owned) => <O as Debug>::fmt(owned, f),
        }
    }
}

impl<'a, O> From<&'a str> for Pow<'a, str, O>
where
    O: Borrow<str>,
{
    fn from(borrowed: &'a str) -> Self {
        Pow::Borrowed(borrowed)
    }
}

impl<'a> From<String> for Pow<'a, str, String> {
    fn from(owned: String) -> Self {
        Pow::Owned(owned)
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump> From<crate::bumpalo::collections::String<'bump>>
    for Pow<'a, str, crate::bumpalo::collections::String<'bump>>
{
    fn from(owned: crate::bumpalo::collections::String<'bump>) -> Self {
        Pow::Owned(owned)
    }
}

impl<'a, T, O> From<&'a [T]> for Pow<'a, [T], O>
where
    T: Clone,
    O: Borrow<[T]>,
{
    fn from(borrowed: &'a [T]) -> Self {
        Pow::Borrowed(borrowed)
    }
}

impl<'a, T> From<Vec<T>> for Pow<'a, [T], Vec<T>>
where
    T: Clone,
{
    fn from(owned: Vec<T>) -> Self {
        Pow::Owned(owned)
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T> From<crate::bumpalo::collections::Vec<'bump, T>>
    for Pow<'a, [T], crate::bumpalo::collections::Vec<'bump, T>>
where
    T: Clone,
{
    fn from(owned: crate::bumpalo::collections::Vec<'bump, T>) -> Self {
        Pow::Owned(owned)
    }
}
