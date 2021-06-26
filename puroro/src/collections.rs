use crate::{tags, IsMessageImplOfTag};
use std::borrow::{Borrow, Cow};

/// Used for a normal proto impl which is using a `Vec` as a repeated field
/// value container.
/// Converts a `&'a T` into:
///  * For numerical types: `T`
///  * For string type: `Cow<'a, str>`
///  * For bytes type: `Cow<'a, [u8]>`
///  * For message type: `Cow<'a, T>`.
pub trait RefTypeToGetterType<TypeTag> {
    type Item;
    fn into(self) -> Self::Item;
}

impl<'a, V> RefTypeToGetterType<(tags::wire::Variant, V)>
    for &'a <V as tags::VariantTypeTag>::NativeType
where
    V: tags::VariantTypeTag,
    <V as tags::VariantTypeTag>::NativeType: Clone,
{
    type Item = <V as tags::VariantTypeTag>::NativeType;
    fn into(self) -> Self::Item {
        self.clone()
    }
}
impl<'a, T> RefTypeToGetterType<tags::Bytes> for &'a T
where
    T: Borrow<[u8]>,
{
    type Item = Cow<'a, [u8]>;
    fn into(self) -> Self::Item {
        Cow::Borrowed(self.borrow())
    }
}
impl<'a, T> RefTypeToGetterType<tags::String> for &'a T
where
    T: Borrow<str>,
{
    type Item = Cow<'a, str>;
    fn into(self) -> Self::Item {
        Cow::Borrowed(self.borrow())
    }
}
impl<'a, T, M> RefTypeToGetterType<tags::Message<M>> for &'a T
where
    T: 'a + Borrow<T> + Clone + IsMessageImplOfTag<M>,
{
    type Item = Cow<'a, T>;
    fn into(self) -> Self::Item {
        Cow::Borrowed(self.borrow())
    }
}
impl<'a, V> RefTypeToGetterType<(tags::wire::Bits32, V)>
    for &'a <V as tags::Bits32TypeTag>::NativeType
where
    V: tags::Bits32TypeTag,
    <V as tags::Bits32TypeTag>::NativeType: Clone,
{
    type Item = <V as tags::Bits32TypeTag>::NativeType;
    fn into(self) -> Self::Item {
        self.clone()
    }
}
impl<'a, V> RefTypeToGetterType<(tags::wire::Bits64, V)>
    for &'a <V as tags::Bits64TypeTag>::NativeType
where
    V: tags::Bits64TypeTag,
    <V as tags::Bits64TypeTag>::NativeType: Clone,
{
    type Item = <V as tags::Bits64TypeTag>::NativeType;
    fn into(self) -> Self::Item {
        self.clone()
    }
}

pub trait RepeatedField<TypeTag, T> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T);
    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>>;
    type Iter<'this>: Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_>;
}

impl<'a, TypeTag, T, U> RepeatedField<TypeTag, T> for &'a Vec<U>
where
    &'a U: RefTypeToGetterType<TypeTag, Item = T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(self)
            .map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x)))
    }

    type Iter<'this> = impl Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_> {
        <[U]>::iter(self).map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x))
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'msg, 'bump, TypeTag, T, U> RepeatedField<TypeTag, T>
    for &'msg ::bumpalo::collections::Vec<'bump, U>
where
    &'msg U: RefTypeToGetterType<TypeTag, Item = T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(&self)
            .map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x)))
    }

    type Iter<'this> = impl Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_> {
        <[U]>::iter(self).map(|x| <&U as RefTypeToGetterType<TypeTag>>::into(x))
    }
}
/*
pub trait MapField<'msg, Q, R>
where
    Q: ?Sized,
{
    fn get(&'msg self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq;
}

impl<'msg, Q, R, K, V> MapField<'msg, Q, R> for &'msg HashMap<K, V>
where
    Q: ?Sized,
    K: Hash + Eq + Borrow<Q>,
    &'msg V: RefTypeToGetterType<TypeTag, Item = R>,
{
    fn get(&self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq,
    {
        <HashMap<K, V>>::get(self, key).map(|x| <&V as RefTypeToGetterType>::into(x))
    }
}
*/
