use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use crate::Message;

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

pub trait RepeatedField<'a, T>
where
    T: ?Sized,
{
    type Item: Sized;
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Self::Item);
    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = Self::Item>>;

    type Iter: Iterator<Item = Self::Item>;
    fn iter(&self) -> Self::Iter;
}

impl<'a, T, U> RepeatedField<'a, T> for &'a Vec<U>
where
    &'a U: VecItemIntoRepeatedFieldItem,
{
    type Item = <&'a U as VecItemIntoRepeatedFieldItem>::Item;

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Self::Item),
    {
        <[U]>::iter(self)
            .map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = Self::Item>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x)))
    }

    type Iter = impl Iterator<Item = Self::Item>;
    fn iter(&self) -> Self::Iter {
        <[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, T, U> RepeatedField<'a, T> for &'a ::bumpalo::collections::Vec<'bump, U>
where
    &'a U: VecItemIntoRepeatedFieldItem,
{
    type Item = <&'a U as VecItemIntoRepeatedFieldItem>::Item;

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Self::Item),
    {
        <[U]>::iter(self)
            .map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
            .for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = Self::Item>> {
        Box::new(<[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x)))
    }

    type Iter = impl Iterator<Item = Self::Item>;
    fn iter(&self) -> Self::Iter {
        <[U]>::iter(self).map(|x| <&U as VecItemIntoRepeatedFieldItem>::into(x))
    }
}

pub trait MapField<K: ?Sized, V: ?Sized> {
    fn get(&self, key: &K) -> Option<&V>
    where
        K: Hash + Eq;
}

impl<K, V, L, W> MapField<K, V> for HashMap<L, W>
where
    K: ?Sized,
    V: ?Sized,
    L: Hash + Eq + Borrow<K>,
    W: Borrow<V>,
{
    fn get(&self, key: &K) -> Option<&V>
    where
        K: Hash + Eq,
    {
        <HashMap<L, W>>::get(self, key).map(|x| x.borrow())
    }
}
