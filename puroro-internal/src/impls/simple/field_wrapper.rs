use ::std::borrow::Cow;
use puroro::{tags, Result};
use std::ops::DerefMut;

/// A utility type-function which returns a type wrapped by `Option` or `Vec` according
/// to the label (e.g. `optional` => `Option`).
/// Can be applied for every types except length delimited types (String, Bytes, Message).
/// - `optional` => `Option<T>`
/// - `required` => `Option<T>` // Needs revisit!!
/// - (unlabeled), (oneof), (mapentry) => `T`
/// - `repeated` => `Vec<T>`
pub trait LabelWrappedType: Sized {
    type Type<T: Clone>: Clone;
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T;
    fn extend<I: Iterator<Item = Result<T>>, T: Clone>(
        wrapped: &mut Self::Type<T>,
        iter: I,
    ) -> Result<()>;
    fn default_with<F: FnOnce() -> T, T: Clone>(f: F) -> Self::Type<T>;
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> Iter<'_, T>;
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

impl<_1, _2> LabelWrappedType for tags::OptionalOrRequired<_1, _2> {
    type Type<T: Clone> = Option<T>;
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T {
        wrapped.get_or_insert_with(f)
    }
    fn extend<I: Iterator<Item = Result<T>>, T: Clone>(
        wrapped: &mut Self::Type<T>,
        iter: I,
    ) -> Result<()> {
        if let Some(x) = iter.last() {
            *wrapped = Some(x?);
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> T, T: Clone>(_: F) -> Self::Type<T> {
        None
    }
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> Iter<'_, T> {
        Iter::Option(wrapped.iter())
    }
}
impl<_1, _2, _3> LabelWrappedType for tags::UnlabeledOrOneofOrMapEntry<_1, _2, _3> {
    type Type<T: Clone> = T;
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, _: F) -> &mut T {
        wrapped
    }
    fn extend<I: Iterator<Item = Result<T>>, T: Clone>(
        wrapped: &mut Self::Type<T>,
        iter: I,
    ) -> Result<()> {
        if let Some(x) = iter.last() {
            *wrapped = x?;
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> T, T: Clone>(f: F) -> Self::Type<T> {
        (f)()
    }
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> Iter<'_, T> {
        Iter::Once(::std::iter::once(wrapped))
    }
}
impl LabelWrappedType for tags::Repeated {
    type Type<T: Clone> = Vec<T>;
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T {
        wrapped.push((f)());
        wrapped.last_mut().unwrap()
    }
    fn extend<I: Iterator<Item = Result<T>>, T: Clone>(
        wrapped: &mut Self::Type<T>,
        iter: I,
    ) -> Result<()> {
        for x in iter {
            wrapped.push(x?);
        }
        Ok(())
    }
    fn default_with<F: FnOnce() -> T, T: Clone>(_: F) -> Self::Type<T> {
        Vec::new()
    }
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> Iter<'_, T> {
        Iter::Slice(wrapped.iter())
    }
}

/// A custom version of `LabelWrappedType` for String and Bytes.
///
/// Let `B` as a Borrowed type and `T` as an owned type:
/// - proto2:
///   - `optional` => `Option<Cow<'static, B>>`
///   - `required` => `Option<Cow<'static, B>>` // Need revisit!!
///   - `repeated` => `Vec<T>`
/// - proto3:
///   - (unlabeled, oneof, mapentry) => `T`
///   - `optional` => `Option<T>`
///   - `repeated` => `Vec<T>`
pub trait LabelWrappedLdType {
    type Type<T>: Clone
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone;
    fn get_or_insert_default<T>(wrapped: &mut Self::Type<T>) -> &mut <T as ToOwned>::Owned
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone;
    fn default<T>() -> Self::Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone;
    fn iter<T>(wrapped: &Self::Type<T>) -> LdIter<'_, T>
    where
        T: 'static + ?Sized + ToOwned,
        <T as ToOwned>::Owned: Clone;
}
pub enum LdIter<'a, B: 'static + ?Sized + ToOwned> {
    OnceOwned(::std::iter::Once<&'a B::Owned>),
    OptionOwned(::std::option::Iter<'a, B::Owned>),
    OptionCow(::std::option::Iter<'a, Cow<'static, B>>),
    SliceOwned(::std::slice::Iter<'a, B::Owned>),
}
impl<'a, B: ?Sized + ToOwned> Iterator for LdIter<'a, B> {
    type Item = &'a B;
    fn next(&mut self) -> Option<Self::Item> {
        use ::std::borrow::Borrow as _;
        use ::std::ops::Deref as _;
        match self {
            LdIter::OnceOwned(iter) => iter.next().map(|x| x.borrow()),
            LdIter::OptionOwned(iter) => iter.next().map(|x| x.borrow()),
            LdIter::OptionCow(iter) => iter.next().map(|c| c.deref()),
            LdIter::SliceOwned(iter) => iter.next().map(|x| x.borrow()),
        }
    }
}

impl<_1, _2> LabelWrappedLdType for (tags::Proto2, tags::OptionalOrRequired<_1, _2>) {
    type Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    = Option<Cow<'static, T>>;
    fn get_or_insert_default<T>(wrapped: &mut Self::Type<T>) -> &mut <T as ToOwned>::Owned
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone,
    {
        wrapped
            .get_or_insert_with(|| Cow::Owned(Default::default()))
            .to_mut()
    }
    fn default<T>() -> Self::Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        None
    }
    fn iter<T>(wrapped: &Self::Type<T>) -> LdIter<'_, T>
    where
        T: 'static + ?Sized + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        LdIter::OptionCow(wrapped.iter())
    }
}
impl LabelWrappedLdType for (tags::Proto3, tags::Optional) {
    type Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    = Option<<T as ToOwned>::Owned>;
    fn get_or_insert_default<T>(wrapped: &mut Self::Type<T>) -> &mut <T as ToOwned>::Owned
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone,
    {
        wrapped.get_or_insert_with(Default::default)
    }
    fn default<T>() -> Self::Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        Default::default()
    }
    fn iter<T>(wrapped: &Self::Type<T>) -> LdIter<'_, T>
    where
        T: 'static + ?Sized + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        LdIter::OptionOwned(wrapped.iter())
    }
}
impl<X, _1, _2, _3> LabelWrappedLdType for (X, tags::UnlabeledOrOneofOrMapEntry<_1, _2, _3>) {
    type Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    = <T as ToOwned>::Owned;
    fn get_or_insert_default<T>(wrapped: &mut Self::Type<T>) -> &mut <T as ToOwned>::Owned
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone,
    {
        wrapped
    }
    fn default<T>() -> Self::Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone,
    {
        Default::default()
    }
    fn iter<T>(wrapped: &Self::Type<T>) -> LdIter<'_, T>
    where
        T: 'static + ?Sized + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        LdIter::OnceOwned(::std::iter::once(wrapped))
    }
}
impl<X> LabelWrappedLdType for (X, tags::Repeated) {
    type Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    = Vec<<T as ToOwned>::Owned>;
    fn get_or_insert_default<T>(wrapped: &mut Self::Type<T>) -> &mut <T as ToOwned>::Owned
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Default + Clone,
    {
        wrapped.push(Default::default());
        wrapped.last_mut().unwrap()
    }
    fn default<T>() -> Self::Type<T>
    where
        T: ?Sized + 'static + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        Vec::new()
    }
    fn iter<T>(wrapped: &Self::Type<T>) -> LdIter<'_, T>
    where
        T: 'static + ?Sized + ToOwned,
        <T as ToOwned>::Owned: Clone,
    {
        LdIter::SliceOwned(wrapped.iter())
    }
}

/// A custom version of `LabelWrappedType` for Message types.
///
///  - (unlabeled) => `Option<Box<T>>`
///  - `required` => `Option<Box<T>>`
///  - `optional` => `Option<Box<T>>`
///  - `repeated` => `Vec<T>`
pub trait LabelWrappedMessageType: Sized {
    type Type<T: Clone>: Clone;
    fn default<T: Clone>() -> Self::Type<T>;
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> MsgIter<'_, T>;
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T;
}
pub enum MsgIter<'a, T> {
    OptionBox(::std::option::Iter<'a, Box<T>>),
    Slice(::std::slice::Iter<'a, T>),
}
impl<'a, T> Iterator for MsgIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        use ::std::ops::Deref as _;
        match self {
            MsgIter::OptionBox(iter) => iter.next().map(|b| b.deref()),
            MsgIter::Slice(iter) => iter.next(),
        }
    }
}

impl<_1, _2> LabelWrappedMessageType for tags::NonRepeated<_1, _2> {
    type Type<T: Clone> = Option<Box<T>>;
    fn default<T: Clone>() -> Self::Type<T> {
        None
    }
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> MsgIter<'_, T> {
        MsgIter::OptionBox(wrapped.iter())
    }
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T {
        <Option<Box<T>>>::get_or_insert_with(wrapped, || Box::new((f)())).deref_mut()
    }
}
impl LabelWrappedMessageType for tags::Repeated {
    type Type<T: Clone> = Vec<T>;
    fn default<T: Clone>() -> Self::Type<T> {
        Vec::new()
    }
    fn iter<T: Clone>(wrapped: &Self::Type<T>) -> MsgIter<'_, T> {
        MsgIter::Slice(wrapped.iter())
    }
    fn get_or_insert_with<F: FnOnce() -> T, T: Clone>(wrapped: &mut Self::Type<T>, f: F) -> &mut T {
        <Vec<T>>::push(wrapped, (f)());
        wrapped.last_mut().unwrap()
    }
}
