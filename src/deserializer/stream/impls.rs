use ::num_traits::FromPrimitive;
use std::io::Result as IoResult;

use super::*;

pub(crate) struct DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    indexed_iter: IndexedIterator<I>,
}
impl<I> DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            indexed_iter: IndexedIterator::new(iter),
        }
    }
}
impl<I> Deserializer for DeserializerImpl<I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn deserialize<H: Handler>(mut self, handler: H) -> Result<H::Target> {
        LengthDelimitedDeserializerImpl::new(&mut self.indexed_iter, None)
            .deserialize_as_message(handler)
    }
}

pub(crate) struct LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    indexed_iter: &'a mut IndexedIterator<I>,
    bytes_len: Option<usize>,
}
impl<'a, I> LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    pub(crate) fn new(indexed_iter: &'a mut IndexedIterator<I>, bytes_len: Option<usize>) -> Self {
        Self {
            indexed_iter,
            bytes_len,
        }
    }
    fn make_sub_deserializer<'b>(
        &'b mut self,
        new_length: usize,
    ) -> LengthDelimitedDeserializerImpl<'b, I>
    where
        'a: 'b,
    {
        LengthDelimitedDeserializerImpl {
            indexed_iter: self.indexed_iter,
            bytes_len: Some(new_length),
        }
    }

    // May expectedly fail if reached to the eof
    fn try_get_wire_type_and_field_number(&mut self) -> Result<(WireType, usize)> {
        let mut peekable = self.by_ref().peekable();
        if let None = peekable.peek() {
            // Found EOF at first byte. Successfull failure.
            return Err(DeserializeError::ExpectedInputTermination);
        }
        let key = Variant::from_bytes(&mut peekable)?.to_usize()?;
        Ok((WireType::from_usize(key & 0x07).unwrap(), (key >> 3)))
    }
}
impl<'a, I> Iterator for LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    type Item = IoResult<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_iter.next()
    }
}
impl<'a, I> LengthDelimitedDeserializer for LengthDelimitedDeserializerImpl<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn index(&self) -> usize {
        self.indexed_iter.index()
    }

    fn length(&self) -> Option<usize> {
        self.bytes_len
    }

    fn deserialize_as_message<H: Handler>(mut self, mut handler: H) -> Result<H::Target> {
        let start_pos = self.index();
        loop {
            // Check message length if possible
            if let Some(message_length) = self.length() {
                if start_pos + message_length >= self.index() {
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
                    let variant = Variant::from_bytes(&mut self)?;
                    handler.deserialized_variant(field_number, variant)?;
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::from_bytes(&mut self)?.to_usize()?;
                    let deserializer_for_inner = self.make_sub_deserializer(field_length);
                    handler.deserialize_length_delimited_field(
                        deserializer_for_inner,
                        field_number,
                        field_length,
                    )?;
                }
                _ => {
                    todo!()
                }
            }
        }

        handler.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_parse_test_variant() {
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
        impl Handler for Test1 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialized_variant(
                &mut self,
                field_number: usize,
                variant: Variant,
            ) -> Result<()> {
                assert_eq!(1, field_number);
                self.a = variant.to_i32()?;
                Ok(())
            }
        }

        let handler = Test1::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().a, 150);
    }

    #[test]
    fn do_parse_test_string() {
        // https://developers.google.com/protocol-buffers/docs/encoding#strings
        // message Test2 {
        //   optional string b = 2;
        // }
        // b = "testing"
        let input: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        #[derive(Default, PartialEq)]
        struct Test2 {
            b: String,
        }
        impl Handler for Test2 {
            type Target = Self;
            fn finish(self) -> Result<Self::Target> {
                Ok(self)
            }
            fn deserialize_length_delimited_field<D: LengthDelimitedDeserializer>(
                &mut self,
                deserializer: D,
                field_number: usize,
                _length: usize,
            ) -> Result<()> {
                assert_eq!(field_number, 2);
                deserializer.deserialize_as_string(|s| {
                    self.b = s;
                    Ok(())
                })?;
                Ok(())
            }
        }

        let handler = Test2::default();
        let deserializer = DeserializerImpl::<_>::new(input.bytes());
        let result = deserializer.deserialize(handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().b, "testing");
    }
}
