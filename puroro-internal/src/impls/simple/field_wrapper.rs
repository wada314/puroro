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
}
impl<T> LabelWrappedLDType<tags::Required, tags::Proto2> for T
where
    T: ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<Cow<'static, T>>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
            .get_or_insert_with(|| Cow::Owned(Default::default()))
            .to_mut()
    }
}
impl<T> LabelWrappedLDType<tags::Optional, tags::Proto2> for T
where
    T: ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<Cow<'static, T>>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
            .get_or_insert_with(|| Cow::Owned(Default::default()))
            .to_mut()
    }
}
impl<T> LabelWrappedLDType<tags::Unlabeled, tags::Proto3> for T
where
    T: ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = <T as ToOwned>::Owned;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped
    }
}
impl<T> LabelWrappedLDType<tags::Optional, tags::Proto3> for T
where
    T: ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Option<<T as ToOwned>::Owned>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped.get_or_insert_with(Default::default)
    }
}
impl<T, X> LabelWrappedLDType<tags::Repeated, X> for T
where
    T: ToOwned + 'static,
    <T as ToOwned>::Owned: Default,
{
    type Type = Vec<<T as ToOwned>::Owned>;
    fn get_or_insert_default(wrapped: &mut Self::Type) -> &mut <Self as ToOwned>::Owned {
        wrapped.push(Default::default());
        wrapped.last_mut().unwrap()
    }
}
