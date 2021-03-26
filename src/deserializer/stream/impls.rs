use ::num_traits::FromPrimitive;
use std::io::Result as IoResult;

use super::*;

pub(crate) struct DeserializerImpl<T> {
    input: T,
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
    pub(crate) fn new(input: T) -> Self {
        Self { input }
    }
}

pub(crate) struct StateImpl<I>
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

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

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
