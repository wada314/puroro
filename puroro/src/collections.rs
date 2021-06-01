use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::Hash;

/// Used for a normal proto impl which is using a `Vec` as a repeated field
/// value container.
/// Converts a `&'a T` into:
///  * For numerical types: `T`
///  * For string type: `Cow<'a, str>`
///  * For bytes type: `Cow<'a, [u8]>`
///  * For message type: `Cow<'a, T>`.
pub trait RefTypeToGetterType {
    type Item;
    fn into(self) -> Self::Item;
}

macro_rules! define_trivial_v2f {
    ($ty:ty) => {
        impl RefTypeToGetterType for &'_ $ty {
            type Item = $ty;
            fn into(self) -> $ty {
                self.clone()
            }
        }
    };
}
define_trivial_v2f!(i32);
define_trivial_v2f!(i64);
define_trivial_v2f!(u32);
define_trivial_v2f!(u64);
define_trivial_v2f!(f32);
define_trivial_v2f!(f64);
define_trivial_v2f!(bool);
impl<T: Clone> RefTypeToGetterType for &'_ std::result::Result<T, i32> {
    type Item = std::result::Result<T, i32>;
    fn into(self) -> std::result::Result<T, i32> {
        self.clone()
    }
}

impl<'a> RefTypeToGetterType for &'a String {
    type Item = Cow<'a, str>;
    fn into(self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump> RefTypeToGetterType for &'a ::bumpalo::collections::String<'bump> {
    type Item = Cow<'a, str>;
    fn into(self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}
impl<'a> RefTypeToGetterType for &'a Vec<u8> {
    type Item = Cow<'a, [u8]>;
    fn into(self) -> Cow<'a, [u8]> {
        Cow::Borrowed(self.as_slice())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump> RefTypeToGetterType for &'a ::bumpalo::collections::Vec<'bump, u8> {
    type Item = Cow<'a, [u8]>;
    fn into(self) -> Cow<'a, [u8]> {
        Cow::Borrowed(self.as_slice())
    }
}
impl<'a, 'bump, T> RefTypeToGetterType for &'a T
where
    T: crate::Message<'bump> + ToOwned,
{
    type Item = Cow<'a, T>;
    fn into(self) -> Cow<'a, T> {
        Cow::Borrowed(self)
    }
}

pub trait RepeatedField<'a, T> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T);
    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>>;
    type Iter<'this>: Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_>;
}

impl<'a, T, U> RepeatedField<'a, T> for &'a Vec<U>
where
    &'a U: RefTypeToGetterType<Item = T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(self)
            .map(|x| <&U as RefTypeToGetterType>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as RefTypeToGetterType>::into(x)))
    }

    type Iter<'this> = impl Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_> {
        <[U]>::iter(self).map(|x| <&U as RefTypeToGetterType>::into(x))
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'msg, 'bump, T, U> RepeatedField<'msg, T> for &'msg ::bumpalo::collections::Vec<'bump, U>
where
    &'msg U: RefTypeToGetterType<Item = T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(&self)
            .map(|x| <&U as RefTypeToGetterType>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as RefTypeToGetterType>::into(x)))
    }

    type Iter<'this> = impl Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_> {
        <[U]>::iter(self).map(|x| <&U as RefTypeToGetterType>::into(x))
    }
}

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
    &'msg V: RefTypeToGetterType<Item = R>,
{
    fn get(&self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq,
    {
        <HashMap<K, V>>::get(self, key).map(|x| <&V as RefTypeToGetterType>::into(x))
    }
}
