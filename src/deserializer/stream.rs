use crate::types::*;
use std::io::Read;

pub mod error;
mod impls;
pub mod variant;

pub use error::{DeserializeError, Result};
pub use variant::Variant;

pub trait Deserializer {
    type State: State;
    fn parse_read<H>(self, handler: H) -> Result<H::Target>
    where
        H: Handler<<Self as Deserializer>::State>;
}
pub fn deserializer_from_read<R: Read>(read: R) -> impl Deserializer {
    impls::DeserializerImpl::<R>::new(read)
}

pub trait State: Sized {
    fn deserialize_as_message<H: Handler<Self>>(
        &mut self,
        opt_length: Option<usize>,
        handler: H,
    ) -> Result<H::Target>;

    fn deserialize_as_string(&mut self) -> Result<()>;
    fn deserialize_as_packed_variants(&mut self) -> Result<()>;
}

pub trait Handler<S: State> {
    type Target;
    fn finish(self) -> Result<Self::Target>;

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()>;

    fn deserialize_length_delimited_field(
        &mut self,
        field_number: usize,
        length: usize,
        state: &mut S,
    ) -> Result<()>;
}

struct IndexedIterator<I> {
    index: usize,
    iter: I,
}
impl<I> Iterator for IndexedIterator<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iter.next()
    }
}
impl<I> IndexedIterator<I> {
    fn new(iter: I) -> Self {
        IndexedIterator { index: 0, iter }
    }
    fn index(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub(crate) struct MockHandler<T: Default, S: State> {
        pub(crate) value: T,
        pub(crate) deserialized_variant: Option<Box<dyn Fn(&mut T, usize, Variant) -> Result<()>>>,

        pub(crate) deserialize_length_delimited_field:
            Option<Box<dyn Fn(&mut T, usize, usize, &mut S) -> Result<()>>>,
    }

    impl<T: Default, S: State> MockHandler<T, S> {
        pub(crate) fn new() -> Self {
            Self {
                value: Default::default(),
                deserialized_variant: None,
                deserialize_length_delimited_field: None,
            }
        }
    }
    impl<T: Default, S: State> Handler<S> for MockHandler<T, S> {
        type Target = T;

        fn finish(self) -> Result<Self::Target> {
            Ok(self.value)
        }
        fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
            let value_mut = &mut self.value;
            self.deserialized_variant
                .as_ref()
                .map_or(Ok(()), |f| (f)(value_mut, field_number, variant))
        }

        fn deserialize_length_delimited_field(
            &mut self,
            field_number: usize,
            length: usize,
            state: &mut S,
        ) -> Result<()> {
            let value_mut = &mut self.value;
            self.deserialize_length_delimited_field
                .as_ref()
                .map_or(Ok(()), |f| (f)(value_mut, field_number, length, state))
        }
    }
}
