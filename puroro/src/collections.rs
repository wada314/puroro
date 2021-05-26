use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::Hash;

pub trait VecItemIntoRepeatedFieldItem {
    type Item;
    fn into(self) -> Self::Item;
}

macro_rules! define_trivial_v2f {
    ($ty:ty) => {
        impl VecItemIntoRepeatedFieldItem for &'_ $ty {
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
impl<T: Clone> VecItemIntoRepeatedFieldItem for &'_ std::result::Result<T, i32> {
    type Item = std::result::Result<T, i32>;
    fn into(self) -> std::result::Result<T, i32> {
        self.clone()
    }
}

impl<'a> VecItemIntoRepeatedFieldItem for &'a String {
    type Item = Cow<'a, str>;
    fn into(self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump> VecItemIntoRepeatedFieldItem for &'a ::bumpalo::collections::String<'bump> {
    type Item = Cow<'a, str>;
    fn into(self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}
impl<'a> VecItemIntoRepeatedFieldItem for &'a Vec<u8> {
    type Item = Cow<'a, [u8]>;
    fn into(self) -> Cow<'a, [u8]> {
        Cow::Borrowed(self.as_slice())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump> VecItemIntoRepeatedFieldItem for &'a ::bumpalo::collections::Vec<'bump, u8> {
    type Item = Cow<'a, [u8]>;
    fn into(self) -> Cow<'a, [u8]> {
        Cow::Borrowed(self.as_slice())
    }
}
impl<'a, 'bump, T> VecItemIntoRepeatedFieldItem for &'a T
where
    T: crate::Message<'bump> + ToOwned,
{
    type Item = Cow<'a, T>;
    fn into(self) -> Cow<'a, T> {
        Cow::Borrowed(self)
    }
}

pub trait RepeatedField<'a, T> {
    fn for_each<F>(&'a self, f: F)
    where
        F: FnMut(T);
    fn boxed_iter(&'a self) -> Box<dyn 'a + Iterator<Item = T>>;

    #[cfg(feature = "puroro-nightly")]
    type Iter: Iterator<Item = T>;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, T, U> RepeatedField<'a, T> for &'a Vec<U>
where
    &'a U: VecItemIntoRepeatedFieldItem<Item = T>,
{
    fn for_each<F>(&'a self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(self)
            .map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&'a self) -> Box<dyn 'a + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x)))
    }

    type Iter = impl Iterator<Item = T>;
    fn iter(&'a self) -> Self::Iter {
        <[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T, U> RepeatedField<'a, T> for &'a ::bumpalo::collections::Vec<'bump, U>
where
    &'a U: VecItemIntoRepeatedFieldItem<Item = T>,
{
    fn for_each<F>(&'a self, f: F)
    where
        F: FnMut(T),
    {
        <[U]>::iter(self)
            .map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&'a self) -> Box<dyn 'a + Iterator<Item = T>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x)))
    }

    type Iter = impl Iterator<Item = T>;
    fn iter(&'a self) -> Self::Iter {
        <[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
    }
}

pub trait MapField<'a, Q, R>
where
    Q: ?Sized,
    R: 'a,
{
    fn get(&'a self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq;
}

impl<'a, Q, R, K, V> MapField<'a, Q, R> for &'a HashMap<K, V>
where
    Q: ?Sized,
    K: Hash + Eq + Borrow<Q>,
    R: 'a,
    &'a V: VecItemIntoRepeatedFieldItem<Item = R>,
{
    fn get(&self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq,
    {
        <HashMap<K, V>>::get(self, key).map(|x| <&V as VecItemIntoRepeatedFieldItem>::into(x))
    }
}
