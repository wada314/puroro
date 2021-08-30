use ::puroro::{Either, RepeatedField};
use ::std::ops::Deref;

pub struct EitherRepeatedField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Either<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct EitherRepeatedLDField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedLDField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedLDField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator,
    U: RepeatedField<'msg> + IntoIterator,
    <T as IntoIterator>::Item: Deref<Target = <<U as IntoIterator>::Item as Deref>::Target>,
    <U as IntoIterator>::Item: Deref,
{
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    type IntoIter =
        IndependentEitherIter<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        IndependentEitherIter(
            self.0
                .map_left(|t| t.into_iter())
                .map_right(|u| u.into_iter()),
        )
    }
}

pub struct IndependentEitherIter<T, U>(
    Either<<T as IntoIterator>::IntoIter, <U as IntoIterator>::IntoIter>,
)
where
    T: IntoIterator,
    U: IntoIterator;

impl<T, U> Iterator for IndependentEitherIter<T, U>
where
    T: Iterator,
    U: Iterator,
{
    type Item = Either<<T as Iterator>::Item, <U as Iterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().either(
            |t| t.next().map(|l| Either::Left(l)),
            |u| u.next().map(|r| Either::Right(r)),
        )
    }
}
