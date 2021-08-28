use puroro::{Either, RepeatedField};

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
impl<'msg, T, U> RepeatedField<'msg> for EitherRepeatedField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
}

pub struct EitherRepeatedMessageField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedMessageField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U> IntoIterator for EitherRepeatedMessageField<T, U>
where
    T: RepeatedField<'msg>,
    U: RepeatedField<'msg>,
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
impl<'msg, T, U> RepeatedField<'msg> for EitherRepeatedMessageField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = <U as IntoIterator>::Item>,
    U: RepeatedField<'msg>,
{
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
    type Item = Either<<T as IntoIterator>::Item, <U as IntoIterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().either(
            |iter| iter.next().map(|item| Either::Left(item)),
            |iter| iter.next().map(|item| Either::Right(item)),
        )
    }
}
