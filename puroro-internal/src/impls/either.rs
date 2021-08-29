use ::puroro::{Either, RepeatedField};
use ::std::borrow::Cow;

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

pub struct EitherRepeatedMessageField<T, U>(Either<T, U>);
impl<T, U> EitherRepeatedMessageField<T, U> {
    pub fn new(from: Either<T, U>) -> Self {
        Self(from)
    }
}
impl<'msg, T, U, TM, UM> IntoIterator for EitherRepeatedMessageField<T, U>
where
    T: RepeatedField<'msg> + IntoIterator<Item = Cow<'msg, TM>>,
    U: RepeatedField<'msg> + IntoIterator<Item = Cow<'msg, UM>>,
    TM: 'msg + Clone,
    UM: 'msg + Clone,
{
    type Item = Cow<'msg, Either<Cow<'msg, TM>, Cow<'msg, UM>>>;
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

impl<'msg, T, U, TM, UM> Iterator for IndependentEitherIter<T, U>
where
    T: Iterator<Item = Cow<'msg, TM>>,
    U: Iterator<Item = Cow<'msg, UM>>,
    TM: 'msg + Clone,
    UM: 'msg + Clone,
{
    type Item = Cow<'msg, Either<Cow<'msg, TM>, Cow<'msg, UM>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().either(
            |iter| iter.next().map(|item| Cow::Owned(Either::Left(item))),
            |iter| iter.next().map(|item| Cow::Owned(Either::Right(item))),
        )
    }
}
