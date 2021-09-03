use ::puroro::{Either, RepeatedField};
use ::std::iter::Chain;
use ::std::ops::Deref;

pub struct MergedRepeatedField<T, U>(T, U);
impl<T, U> MergedRepeatedField<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self(t, u)
    }
}
impl<'msg, T, U> IntoIterator for MergedRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Chain<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().chain(self.1.into_iter())
    }
}

pub struct MergedRepeatedLDField<T, U>(T, U);
impl<T, U> MergedRepeatedLDField<T, U> {
    pub fn new(t: T, u: U) -> Self {
        Self(t, u)
    }
}
impl<'msg, T, U> IntoIterator for MergedRepeatedLDField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator,
    U: RepeatedField<'msg> + IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter = Chain<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>; // TODO!!
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
