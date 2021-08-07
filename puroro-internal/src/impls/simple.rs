use ::puroro::RepeatedField;
use ::std::borrow::Cow;
use std::borrow::Borrow;
use std::marker::PhantomData;

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
        self.0.iter().cloned()
    }
}
impl<'msg, T: Clone> RepeatedField<'msg, T> for VecWrapper<'msg, T> {}

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
        CowIter(self.0.iter(), PhantomData)
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
impl<'msg, B> RepeatedField<'msg, Cow<'msg, B>> for VecCowWrapper<'msg, B> where
    B: 'msg + ?Sized + ToOwned
{
}
