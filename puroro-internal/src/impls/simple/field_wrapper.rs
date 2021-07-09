use ::std::borrow::Cow;
use puroro::{tags, Result};

/// A utility type-function which returns a type wrapped by `Option` or `Vec` according
/// to the label (e.g. `optional` => `Option`).
/// Can be applied for every types except length delimited types (String, Bytes, Message).
/// - `optional` => `Option<T>`
/// - `required` => `Option<T>` // Needs revisit!!
/// - (unlabeled) => `T`
/// - `repeated` => `Vec<T>`
pub trait LabelWrappedType<L>: Sized {
    type Type;
    fn get_or_insert_with<F: FnOnce() -> Self>(wrapped: &mut Self::Type, f: F) -> &mut Self;
    fn extend<I: Iterator<Item = Result<Self>>>(wrapped: &mut Self::Type, iter: I) -> Result<()>;
    fn default_with<F: FnOnce() -> Self>(f: F) -> Self::Type;
    fn iter(wrapped: &Self::Type) -> Iter<'_, Self>;
}
pub enum Iter<'a, T> {
    Once(::std::iter::Once<&'a T>),
    Option(::std::option::Iter<'a, T>),
    Slice(::std::slice::Iter<'a, T>),
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Once(iter) => iter.next(),
            Iter::Option(iter) => iter.next(),
            Iter::Slice(iter) => iter.next(),
        }
    }
}

impl<T> LabelWrappedType<tags::Required> for T {
    // TODO: Revisit... T or Option<T>
    type Type = Option<T>;
    fn get_or_insert_with<F: FnOnce() -> Self>(wrapped: &mut Self::Type, f: F) -> &mut Self {
        wrapped.get_or_insert_with(f)
    }
    fn extend<I: Iterator<Item = Result<Self>>>(wrapped: &mut Self::Type, iter: I) -> Result<()> {
        if let Some(x) = iter.last() {
            *wrapped = Some(x?);
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> Self>(_: F) -> Self::Type {
        None
    }
    fn iter(wrapped: &Self::Type) -> Iter<'_, Self> {
        Iter::Option(wrapped.iter())
    }
}
impl<T> LabelWrappedType<tags::Optional> for T {
    type Type = Option<T>;
    fn get_or_insert_with<F: FnOnce() -> Self>(wrapped: &mut Self::Type, f: F) -> &mut Self {
        wrapped.get_or_insert_with(f)
    }
    fn extend<I: Iterator<Item = Result<Self>>>(wrapped: &mut Self::Type, iter: I) -> Result<()> {
        if let Some(x) = iter.last() {
            *wrapped = Some(x?);
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> Self>(_: F) -> Self::Type {
        None
    }
    fn iter(wrapped: &Self::Type) -> Iter<'_, Self> {
        Iter::Option(wrapped.iter())
    }
}
impl<T> LabelWrappedType<tags::Unlabeled> for T {
    type Type = T;
    fn get_or_insert_with<F: FnOnce() -> Self>(wrapped: &mut Self::Type, _: F) -> &mut Self {
        wrapped
    }
    fn extend<I: Iterator<Item = Result<Self>>>(wrapped: &mut Self::Type, iter: I) -> Result<()> {
        if let Some(x) = iter.last() {
            *wrapped = x?;
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> Self>(f: F) -> Self::Type {
        (f)()
    }
    fn iter(wrapped: &Self::Type) -> Iter<'_, Self> {
        Iter::Once(::std::iter::once(wrapped))
    }
}
impl<T> LabelWrappedType<tags::Repeated> for T {
    type Type = Vec<T>;
    fn get_or_insert_with<F: FnOnce() -> Self>(wrapped: &mut Self::Type, f: F) -> &mut Self {
        wrapped.push((f)());
        wrapped.last_mut().unwrap()
    }
    fn extend<I: Iterator<Item = Result<Self>>>(wrapped: &mut Self::Type, iter: I) -> Result<()> {
        for x in iter {
            wrapped.push(x?);
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> Self>(_: F) -> Self::Type {
        Vec::new()
    }
    fn iter(wrapped: &Self::Type) -> Iter<'_, Self> {
        Iter::Slice(wrapped.iter())
    }
}

/// A custom version of `LabelWrappedType` for String and Bytes.
///
/// - proto2:
///   - `optional` => `Option<Cow<'static, T>>`
///   - `required` => `Option<Cow<'static, T>>` // Needs revisit!!
///   - `repeated` => `Vec<T>`
/// - proto3:
///   - (unlabeled) => `T`
///   - `optional` => `Option<T>`
///   - `repeated` => `Vec<T>`
pub trait LabelWrappedLDType<L, X>: ToOwned
where
    <Self as ToOwned>::Owned: Default,
{
    type Type;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned;
    fn default() -> Self::Type;
}
impl<T> LabelWrappedLDType<tags::Required, tags::Proto2> for T
where
    T: ?Sized + ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<Cow<'static, T>>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
            .get_or_insert_with(|| Cow::Owned(Default::default()))
            .to_mut()
    }
    fn default() -> Self::Type {
        None
    }
}
impl<T> LabelWrappedLDType<tags::Optional, tags::Proto2> for T
where
    T: ?Sized + ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<Cow<'static, T>>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
            .get_or_insert_with(|| Cow::Owned(Default::default()))
            .to_mut()
    }
    fn default() -> Self::Type {
        None
    }
}
impl<T> LabelWrappedLDType<tags::Unlabeled, tags::Proto3> for T
where
    T: ?Sized + ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = <T as ToOwned>::Owned;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
    }
    fn default() -> Self::Type {
        Default::default()
    }
}
impl<T> LabelWrappedLDType<tags::Optional, tags::Proto3> for T
where
    T: ?Sized + ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<<T as ToOwned>::Owned>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped.get_or_insert_with(Default::default)
    }
    fn default() -> Self::Type {
        Default::default()
    }
}
impl<T, X> LabelWrappedLDType<tags::Repeated, X> for T
where
    T: ?Sized + ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Vec<<T as ToOwned>::Owned>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped.push(Default::default());
        wrapped.last_mut().unwrap()
    }
    fn default() -> Self::Type {
        Vec::new()
    }
}
