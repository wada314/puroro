use ::itertools::Itertools;
use std::convert::TryFrom;
use std::io::{Error as IoError, Result as IoResult};
use std::marker::PhantomData;
use std::num::TryFromIntError;

#[non_exhaustive]
enum WireType {
    Variant = 0,
    Bytes64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bytes32 = 5,
}

#[derive(::thiserror::Error, Debug)]
pub(crate) enum DeserializeError {
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

struct MessageParsingState<'a, I>
where
    I: 'a + Iterator<Item = IoResult<u8>>,
{
    iter: &'a mut CountingIterator<I>,
    start_pos: usize,
    expected_length: Option<usize>,
}

impl<'a, I> MessageParsingState<'a, I>
where
    I: Iterator<Item = IoResult<u8>>,
{
    fn new(
        iter: &'a mut CountingIterator<I>,
        start_pos: usize,
        expected_length: Option<usize>,
    ) -> Self {
        Self {
            iter,
            start_pos,
            expected_length,
        }
    }

    fn deserialize_message<H: DeserializeEventHandler>(
        &mut self,
        opt_length: Option<usize>,
        handler: &mut H,
    ) -> Result<()> {
        let mut iter = CountingIterator::new(self.iter.by_ref());
        todo!()
    }

    fn deserialize_field_or_eof(&mut self) {}

    fn deserialize_as_packed_variants<T: DeserializeEventHandler>(&mut self) -> Result<()> {
        todo!()
    }
}

trait DeserializeEventHandler {
    type Target;
    fn met_variant<I>(&mut self, field_id: usize, variant: Variant) -> Result<()>
    where
        I: Iterator<Item = IoResult<u8>>;
}

struct CountingIterator<I> {
    counter: usize,
    iter: I,
}
impl<I> Iterator for CountingIterator<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        self.iter.next()
    }
}
impl<I> CountingIterator<I> {
    fn new(iter: I) -> Self {
        CountingIterator { counter: 0, iter }
    }
    fn count(&self) -> usize {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

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
            let mut iter = into_iter(input);
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
            let mut iter = into_iter(input);
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

    #[test]
    fn do_parse_test1() {
        // https://developers.google.com/protocol-buffers/docs/encoding#simple
        // message Test1 {
        //   optional int32 a = 1;
        // }
        let input: &[u8] = &[0x08, 0x96, 0x01];
        #[derive(Default)]
        struct Test1 {
            a: i32,
        }
        struct Handler {
            msg: Test1,
        }
        impl DeserializeEventHandler for Handler {
            type Target = Test1;

            fn met_variant<I>(&mut self, field_id: usize, variant: Variant) -> Result<()>
            where
                I: Iterator<Item = IoResult<u8>>,
            {
                assert_eq!(field_id, 1);
                self.msg.a = variant.to_i32()?;
                Ok(())
            }
        }

        let input = into_iter(&[0x08, 0x96, 0x01]);
    }
}
