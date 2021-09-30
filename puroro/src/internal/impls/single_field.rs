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
