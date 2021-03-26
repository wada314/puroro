use crate::types::*;
use ::num_traits::FromPrimitive;
use std::io::Result as IoResult;

pub mod error;
pub mod variant;

pub use error::{DeserializeError, Result};
pub use variant::Variant;

struct DeserializerImpl<T> {
    input: T,
}
pub trait Deserializer {
    type State: State;
    fn parse_read<H>(self, handler: H) -> Result<H::Target>
    where
        H: Handler<<Self as Deserializer>::State>;
}
impl<T: std::io::Read> Deserializer for DeserializerImpl<T> {
    type State = StateImpl<std::io::Bytes<T>>;

    fn parse_read<H>(self, handler: H) -> Result<H::Target>
    where
        H: Handler<<Self as Deserializer>::State>,
    {
        let mut state = StateImpl::new(self.input.bytes());
        state.deserialize_as_message(None, handler)
    }
}
impl<T: std::io::Read> DeserializerImpl<T> {
    fn new(input: T) -> Self {
        Self { input }
    }
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

struct StateImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    indexed_iter: IndexedIterator<I>,
}

impl<I> StateImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn new(iter: I) -> Self {
        Self {
            indexed_iter: IndexedIterator::new(iter),
        }
    }

    // May expectedly fail if reached to the eof
    fn try_get_wire_type_and_field_number(&mut self) -> Result<(WireType, usize)> {
        let mut peekable = self.indexed_iter.by_ref().peekable();
        if let None = peekable.peek() {
            // Found EOF at first byte. Successfull failure.
            return Err(DeserializeError::ExpectedInputTermination);
        }
        let key = Variant::from_bytes(&mut peekable)?.to_usize()?;
        Ok((WireType::from_usize(key & 0x07).unwrap(), (key >> 3)))
    }
}
impl<I> State for StateImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn deserialize_as_message<H: Handler<Self>>(
        &mut self,
        opt_length: Option<usize>,
        mut handler: H,
    ) -> Result<H::Target> {
        let start_pos = self.indexed_iter.index();
        loop {
            // Check message length if possible
            if let Some(message_length) = opt_length {
                if start_pos + message_length >= self.indexed_iter.index() {
                    break;
                }
            }

            // get field number, wire type
            let (wire_type, field_number) = match self.try_get_wire_type_and_field_number() {
                Ok(x) => x,
                Err(DeserializeError::ExpectedInputTermination) => {
                    break;
                } // This is ok. finish This message deserialization.
                Err(e) => {
                    return Err(e);
                }
            };

            match wire_type {
                WireType::Variant => {
                    let variant = Variant::from_bytes(&mut self.indexed_iter)?;
                    handler.deserialized_variant(field_number, variant)?;
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::from_bytes(&mut self.indexed_iter)?.to_usize()?;
                    handler.deserialize_length_delimited_field(field_number, field_length, self)?;
                }
                _ => {
                    todo!()
                }
            }
        }

        handler.finish()
    }

    fn deserialize_as_string(&mut self) -> Result<()> {
        todo!()
    }

    fn deserialize_as_packed_variants(&mut self) -> Result<()> {
        todo!()
    }
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
    struct MockHandler<T: Default, S: State> {
        value: T,
        deserialized_variant: Option<Box<dyn Fn(&mut T, usize, Variant) -> Result<()>>>,

        deserialize_length_delimited_field:
            Option<Box<dyn Fn(&mut T, usize, usize, &mut S) -> Result<()>>>,
    }

    impl<T: Default, S: State> MockHandler<T, S> {
        fn new() -> Self {
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

    #[test]
    fn do_parse_test1() {
        // https://developers.google.com/protocol-buffers/docs/encoding#simple
        // message Test1 {
        //   optional int32 a = 1;
        // }
        // a = 150
        let input: &[u8] = &[0x08, 0x96, 0x01];
        #[derive(Default, PartialEq)]
        struct Test1 {
            a: i32,
        }

        let mut handler = MockHandler::<Test1, _>::new();
        handler.deserialized_variant = Some(Box::new(|value, field_number, variant| {
            assert_eq!(field_number, 1);
            value.a = variant.to_i32()?;
            Ok(())
        }));

        let deserializer = DeserializerImpl::<_>::new(input);
        let result = deserializer.parse_read(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().a, 150);
    }
}
