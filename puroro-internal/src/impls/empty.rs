use ::std::marker::PhantomData;

pub struct EmptyRepeatedField<T>(PhantomData<T>);
impl<T> EmptyRepeatedField<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'msg, T> ::puroro::RepeatedField<'msg, T> for EmptyRepeatedField<T> {}
impl<T> IntoIterator for EmptyRepeatedField<T> {
    type Item = T;
    type IntoIter = ::std::iter::Empty<T>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::empty()
    }
}
