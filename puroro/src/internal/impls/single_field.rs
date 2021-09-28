pub mod se;

use ::std::ops::Deref;

pub struct DerefIter<I>(I);
impl<I> DerefIter<I> {
    pub fn new(iter: I) -> Self {
        Self(iter)
    }
}
impl<'a, I, T> Iterator for DerefIter<I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Deref,
{
    type Item = &'a <T as Deref>::Target;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| <T as Deref>::deref(r))
    }
}

pub trait VecOrOptionOrBare<T> {
    type Iter<'a>: Iterator<Item = &'a T>
    where
        T: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}
impl<T, U> VecOrOptionOrBare<T> for U
where
    for<'a> &'a U: IntoIterator<Item = &'a T>,
{
    type Iter<'a>
    where
        T: 'a,
    = <&'a U as IntoIterator>::IntoIter;
    fn iter(&self) -> <Self as VecOrOptionOrBare<T>>::Iter<'_> {
        IntoIterator::into_iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for T {
    type Iter<'a>
    where
        T: 'a,
    = ::std::iter::Once<&'a T>;
    fn iter(&self) -> Self::Iter<'_> {
        ::std::iter::once(self)
    }
}
