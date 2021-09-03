use ::puroro::RepeatedField;
use ::std::iter::Chain;

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
