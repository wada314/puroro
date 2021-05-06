use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

pub trait RepeatedField<T: ?Sized> {
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T);
    fn boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &T>>;

    #[cfg(feature = "puroro-nightly")]
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    #[cfg(feature = "puroro-nightly")]
    fn iter<'a>(&'a self) -> Self::Iter<'a>
    where
        T: 'a;
}

impl<T, U> RepeatedField<T> for Vec<U>
where
    U: Borrow<T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T),
    {
        self.deref().iter().map(|x| x.borrow()).for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = &T>> {
        Box::new(self.deref().iter().map(|x| x.borrow()))
    }

    #[cfg(feature = "puroro-nightly")]
    #[rustfmt::skip]
    type Iter<'a> where Self: 'a, T: 'a = impl Iterator<Item = &'a T>;

    #[cfg(feature = "puroro-nightly")]
    fn iter<'a>(&'a self) -> Self::Iter<'a>
    where
        T: 'a,
    {
        self.deref().iter().map(|x| <U as Borrow<T>>::borrow(x))
    }
}

impl<'bump, T, U> RepeatedField<T> for ::bumpalo::collections::Vec<'bump, U>
where
    U: Borrow<T>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T),
    {
        self.deref().iter().map(|x| x.borrow()).for_each(f)
    }

    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = &T>> {
        Box::new(self.deref().iter().map(|x| x.borrow()))
    }

    #[cfg(feature = "puroro-nightly")]
    #[rustfmt::skip]
    type Iter<'a> where Self: 'a, T: 'a = impl Iterator<Item = &'a T>;

    #[cfg(feature = "puroro-nightly")]
    fn iter<'a>(&'a self) -> Self::Iter<'a>
    where
        T: 'a,
    {
        self.deref().iter().map(|x| <U as Borrow<T>>::borrow(x))
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
