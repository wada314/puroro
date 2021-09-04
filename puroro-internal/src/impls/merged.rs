use crate::Derefable;
use ::puroro::{Either, EitherOrBoth};
use ::std::iter::Chain;
use ::std::ops::Deref;

pub struct MergedRepeatedField<T, U>(EitherOrBoth<T, U>);
impl<T, U> MergedRepeatedField<T, U> {
    pub fn new(from: EitherOrBoth<T, U>) -> Self {
        Self(from)
    }
}
impl<T, U> IntoIterator for MergedRepeatedField<T, U>
where
    T: IntoIterator<Item = <U as IntoIterator>::Item>,
    U: IntoIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Chain<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().chain(self.1.into_iter())
    }
}

pub struct MergedRepeatedLDField<T, U>(EitherOrBoth<T, U>);
impl<T, U> MergedRepeatedLDField<T, U> {
    pub fn new(from: EitherOrBoth<T, U>) -> Self {
        Self(from)
    }
}
impl<T, U> IntoIterator for MergedRepeatedLDField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        impl Iterator<Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|v| Either::Left(v))
            .chain(self.1.into_iter().map(|v| Either::Right(v)))
    }
}

pub struct MergedRepeatedMessageField<T, U>(EitherOrBoth<T, U>);
impl<T, U> MergedRepeatedMessageField<T, U> {
    pub fn new(from: EitherOrBoth<T, U>) -> Self {
        Self(from)
    }
}
impl<T, U> IntoIterator for MergedRepeatedMessageField<T, U>
where
    T: IntoIterator,
    U: IntoIterator,
    <T as IntoIterator>::Item: Deref,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Derefable<EitherOrBoth<<T as IntoIterator>::Item, <U as IntoIterator>::Item>>;
    type IntoIter = impl Iterator<
        Item = Derefable<EitherOrBoth<<T as IntoIterator>::Item, <U as IntoIterator>::Item>>,
    >;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|v| Derefable::new(EitherOrBoth::Left(v)))
            .chain(
                self.1
                    .into_iter()
                    .map(|v| Derefable::new(EitherOrBoth::Right(v))),
            )
    }
}
