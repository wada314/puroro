pub mod de;
pub mod se;

use ::puroro::RepeatedField;
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::marker::PhantomData;

pub struct VecWrapper<'msg, T>(&'msg Vec<T>);

impl<'msg, T> VecWrapper<'msg, T> {
    pub fn new(vec: &'msg Vec<T>) -> Self {
        Self(vec)
    }
}

impl<'msg, T: Clone> IntoIterator for VecWrapper<'msg, T> {
    type Item = T;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'msg, T>>;

    fn into_iter(self) -> Self::IntoIter {
        <[T]>::iter(&self.0).cloned()
    }
}
impl<'msg, T: Clone> RepeatedField<'msg> for VecWrapper<'msg, T> {}

pub struct VecCowWrapper<'msg, B: ?Sized + ToOwned>(&'msg Vec<B::Owned>);

impl<'msg, B: ?Sized + ToOwned> VecCowWrapper<'msg, B> {
    pub fn new(vec: &'msg Vec<B::Owned>) -> Self {
        Self(vec)
    }
}
impl<'msg, B: 'msg + ?Sized + ToOwned> IntoIterator for VecCowWrapper<'msg, B> {
    type Item = Cow<'msg, B>;
    type IntoIter = CowIter<'msg, B, std::slice::Iter<'msg, B::Owned>>;

    fn into_iter(self) -> Self::IntoIter {
        CowIter(<[B::Owned]>::iter(self.0), PhantomData)
    }
}
pub struct CowIter<'msg, B, Iter>(Iter, PhantomData<B>)
where
    B: 'msg + ?Sized + ToOwned,
    Iter: Iterator<Item = &'msg B::Owned>;
impl<'msg, B, Iter> Iterator for CowIter<'msg, B, Iter>
where
    B: 'msg + ?Sized + ToOwned,
    Iter: Iterator<Item = &'msg B::Owned>,
{
    type Item = Cow<'msg, B>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| Cow::Borrowed(x.borrow()))
    }
}
impl<'msg, B> RepeatedField<'msg> for VecCowWrapper<'msg, B> where B: 'msg + ?Sized + ToOwned {}

pub trait VecOrOptionOrBare<T> {
    fn push(&mut self, val: T);
    type Iter<'a>: Iterator<Item = &'a T>
    where
        T: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}
impl<T> VecOrOptionOrBare<T> for Option<T> {
    fn push(&mut self, val: T) {
        *self = Some(val);
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::option::Iter<'a, T>;
    fn iter(&self) -> Self::Iter<'_> {
        Option::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for Vec<T> {
    fn push(&mut self, val: T) {
        self.push(val);
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::slice::Iter<'a, T>;
    fn iter(&self) -> <Self as VecOrOptionOrBare<T>>::Iter<'_> {
        <[T]>::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for T {
    fn push(&mut self, val: T) {
        *self = val;
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::iter::Once<&'a T>;
    fn iter(&self) -> Self::Iter<'_> {
        ::std::iter::once(self)
    }
}
