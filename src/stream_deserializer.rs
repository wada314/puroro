use ::itertools::Itertools;
use ::num_derive::FromPrimitive;
use ::num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::io::{Error as IoError, Result as IoResult};
use std::marker::PhantomData;
use std::num::TryFromIntError;

#[non_exhaustive]
#[derive(FromPrimitive)]
enum WireType {
    Variant = 0,
    Bytes64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bytes32 = 5,
}

#[non_exhaustive]
enum LengthDelimitedFieldInterpretation {
    Unknown = 0,
    String = 1,
    Bytes = 2,
    Message = 3,
    PackedRepeatedFields = 4,
}

#[derive(::thiserror::Error, Debug)]
pub(crate) enum DeserializeError {
    #[error("The input binary has terminated.")]
    ExpectedInputTermination,
    #[error("The input binary has terminated in irregular position.")]
    UnexpectedInputTermination,
    #[error("A variant integer type has too large or too small value.")]
    IntegerOverflow(#[from] std::num::TryFromIntError),
    #[error("A variant integer type is longer than 10 bytes.")]
    TooLargeVariant,
    #[error("Unexpected field type. e.g. Expected int32, but found a Message field.")]
    UnexpectedFieldType,
    #[error("Unexpected field number. In protobuf standard, the deserializer should accept this though.")]
    UnexpectedFieldId,
    #[error("The bytestream iterator returned an error: {0}")]
    IteratorError(#[from] IoError),
}
type Result<T> = std::result::Result<T, DeserializeError>;

#[derive(Debug)]
struct Variant([u8; 8]);
impl Variant {
    pub(crate) fn from_bytes<I>(bytes: &mut I) -> Result<Self>
    where
        I: Iterator<Item = IoResult<u8>>,
    {
        let mut x = 0u64;
        for i in 0..9 {
            match bytes.next() {
                Some(maybe_byte) => {
                    let byte = maybe_byte?;
                    x |= ((byte & 0x7F) as u64) << (i * 7);
                    if byte < 0x80 {
                        return Ok(Variant(x.to_ne_bytes()));
                    }
                }
                None => {
                    return Err(DeserializeError::UnexpectedInputTermination);
                }
            }
        }
        // i == 9, so now checking a last MSBit.
        match bytes.next() {
            Some(maybe_byte) => {
                let byte = maybe_byte?;
                x |= ((byte & 0x01) as u64) << 63;
                if byte & 0xFE != 0 {
                    return Err(DeserializeError::TooLargeVariant);
                } else {
                    return Ok(Variant(x.to_ne_bytes()));
                }
            }
            None => {
                return Err(DeserializeError::UnexpectedInputTermination);
            }
        }
    }

    pub(crate) fn to_u32(&self) -> Result<u32> {
        Ok(u32::try_from(u64::from_ne_bytes(self.0))?)
    }
    pub(crate) fn to_u64(&self) -> Result<u64> {
        Ok(u64::from_ne_bytes(self.0))
    }
    fn to_usize(&self) -> Result<usize> {
        Ok(usize::try_from(u64::from_ne_bytes(self.0))?)
    }
    pub(crate) fn to_i32(&self) -> Result<i32> {
        Ok(i32::try_from(i64::from_ne_bytes(self.0))?)
    }
    pub(crate) fn to_i64(&self) -> Result<i64> {
        Ok(i64::from_ne_bytes(self.0))
    }
}

struct Deserializer {}
impl Deserializer {
    fn new() -> Self {
        Self {}
    }
    fn parse_read<T, H>(&mut self, input: T, handler: H) -> Result<H::Target>
    where
        T: std::io::Read,
        H: DeserializeEventHandler,
    {
        let mut counting_iterator = IndexedIterator::new(input.bytes());
        let mut state = MessageDeserializingState::new(&mut counting_iterator);
        state.deserialize_as_message(None, handler)
    }
}

struct MessageDeserializingState<'a, I>
where
    I: 'a + Iterator<Item = IoResult<u8>>,
{
    indexed_iter: &'a mut IndexedIterator<I>,
}

impl<'a, I> MessageDeserializingState<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn new(indexed_iter: &'a mut IndexedIterator<I>) -> Self {
        Self { indexed_iter }
    }

    pub(crate) fn deserialize_as_message<H: DeserializeEventHandler>(
        &mut self,
        opt_length: Option<usize>,
        mut handler: H,
    ) -> Result<H::Target> {
        let start_pos = self.indexed_iter.count();
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
                    let variant = Variant::from_bytes(self.indexed_iter)?;
                    handler.deserialized_variant(field_number, variant)?;
                }
                WireType::LengthDelimited => {
                    let field_length = Variant::from_bytes(self.indexed_iter)?.to_usize()?;
                    handler.met_length_delimited_field(field_number, field_length, self)?;
                }
                _ => {
                    todo!()
                }
            }
        }

        handler.finish()
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

    pub(crate) fn deserialize_as_packed_variants<H: DeserializeEventHandler>(
        &mut self,
        handler: &mut H,
    ) -> Result<()> {
        todo!()
    }
}

trait DeserializeEventHandler: Sized {
    type Target;
    fn new() -> Result<Self>;
    fn finish(self) -> Result<Self::Target>;

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()>;

    fn met_length_delimited_field<'a, I>(
        &mut self,
        field_number: usize,
        length: usize,
        state: &mut MessageDeserializingState<'a, I>,
    ) -> Result<()>
    where
        I: Iterator<Item = IoResult<u8>>;
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
    use std::io::{ErrorKind, Read};

    fn into_iter<'a>(slice: &'a [u8]) -> impl Iterator<Item = IoResult<u8>> + 'a {
        slice.iter().copied().map(|x| Ok(x))
    }
    fn collect_iter<I: Iterator<Item = IoResult<u8>>>(iter: I) -> Vec<u8> {
        iter.map(|r| r.unwrap()).collect::<Vec<_>>()
    }

    #[test]
    fn test_variant_from_bytes() {
        let io_error1 = IoError::new(ErrorKind::InvalidData, "");

        fn expect_ok(input: &[u8], expected_value: u64, expected_remaining: &[u8]) {
            let mut iter = input.bytes();
            let result = Variant::from_bytes(&mut iter);
            assert!(result.is_ok());
            let variant = result.unwrap();
            assert_eq!(variant.0, expected_value.to_ne_bytes());
            assert_eq!(collect_iter(iter), expected_remaining);
        }
        fn get_err(input: Vec<IoResult<u8>>) -> DeserializeError {
            let mut iter = input.into_iter();
            let result = Variant::from_bytes(&mut iter);
            assert!(result.is_err());
            result.unwrap_err()
        }
        expect_ok(&[0x00], 0, &[]);
        expect_ok(&[0x00, 0x80, 0x81], 0, &[0x80, 0x81]);
        expect_ok(&[0x80, 0x40], 0b1000000_0000000, &[]);
        expect_ok(
            &[0x80, 0x80, 0x80, 0x40],
            0b1000000_0000000_0000000_0000000,
            &[],
        );
        assert!(matches!(
            get_err(vec![Ok(0x80)]),
            DeserializeError::UnexpectedInputTermination
        ));
        assert!(matches!(
            get_err(vec![Err(io_error1)]),
            DeserializeError::IteratorError(_)
        ));
    }

    #[test]
    fn test_variant_unsigned() {
        fn get_u32(input: &[u8]) -> Result<u32> {
            let mut iter = input.bytes();
            let v = Variant::from_bytes(&mut iter)?;
            assert_eq!(collect_iter(iter), Vec::<u8>::new());
            v.to_u32()
        }
        assert_eq!(get_u32(&[0x00]).unwrap(), 0);
        assert_eq!(get_u32(&[0x01]).unwrap(), 1);
        assert_eq!(get_u32(&[0x7F]).unwrap(), 0x7F);
        assert_eq!(get_u32(&[0x80, 0x01]).unwrap(), 0x80);
        assert_eq!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]).unwrap(),
            0xFFFFFFFF
        );
        assert!(matches!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]).unwrap_err(),
            DeserializeError::IntegerOverflow(_)
        ));
        assert!(matches!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F]).unwrap_err(),
            DeserializeError::IntegerOverflow(_)
        ));
    }

    #[derive(Default)]
    struct MockHandler<T: Default> {
        value: T,
        deserialized_variant: Option<Box<dyn Fn(&mut T, usize, Variant) -> Result<()>>>,

        met_length_delimited_field: Option<
            Box<dyn Fn(&mut T, usize, usize, &mut MessageDeserializingState<'a, I>) -> Result<()>>,
        >,
    }
    impl<T: Default> DeserializeEventHandler for MockHandler<T> {
        type Target = T;

        fn new() -> Result<Self> {
            Ok(Default::default())
        }
        fn finish(self) -> Result<Self::Target> {
            todo!()
        }
        fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
            let value_mut = &mut self.value;
            self.deserialized_variant
                .as_ref()
                .map_or(Ok(()), |f| (f)(value_mut, field_number, variant))
        }

        fn met_length_delimited_field<'a, I>(
            &mut self,
            field_number: usize,
            length: usize,
            state: &mut MessageDeserializingState<'a, I>,
        ) -> Result<()>
        where
            I: Iterator<Item = IoResult<u8>>,
        {
            todo!()
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

        let mut handler = MockHandler::<Test1>::new().unwrap();
        handler.deserialized_variant = Some(Box::new(|value, field_number, variant| {
            assert_eq!(field_number, 1);
            value.a = variant.to_i32()?;
            Ok(())
        }));

        let mut deserializer = Deserializer::new();
        let result = deserializer.parse_read(input, handler);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().a, 150);
    }
}
