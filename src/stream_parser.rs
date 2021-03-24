use ::itertools::Itertools;
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::marker::PhantomData;
use std::num::TryFromIntError;

#[derive(::thiserror::Error, Debug)]
pub(crate) enum ParseError {
    #[error("The input binary has terminated in irregular position.")]
    UnexpectedInputTermination,
    #[error("A variant integer type has too large or too small value.")]
    IntegerOverflow(#[from] std::num::TryFromIntError),
    #[error("A variant integer type is longer than 10 bytes.")]
    TooLargeVariant,
    #[error("Unexpected field type. e.g. Expected int32, but found a Message field")]
    UnexpectedFieldType,
    #[error("The bytestream iterator returned an error: {0}")]
    IteratorError(#[from] IoError),
}

#[derive(Debug)]
struct Variant(u64);
impl Variant {
    pub(crate) fn from_bytes<T>(bytes: &mut T) -> Result<Self, ParseError>
    where
        T: Iterator<Item = Result<u8, IoError>>,
    {
        let mut x = 0u64;
        let mut i = 0usize;
        loop {
            match bytes.next() {
                Some(maybe_byte) => {
                    let byte = maybe_byte?;
                    x |= ((byte & 0x7F) as u64) << (i * 7);
                    if byte < 0x80 {
                        return Ok(Variant(x));
                    }
                }
                None => {
                    return Err(ParseError::UnexpectedInputTermination);
                }
            }
            i += 1;
            if i >= 9 {
                break;
            }
        }
        // i == 9, so now checking a last MSBit.
        match bytes.next() {
            Some(maybe_byte) => {
                let byte = maybe_byte?;
                x |= ((byte & 0x01) as u64) << 63;
                if byte & 0xFE != 0 {
                    return Err(ParseError::TooLargeVariant);
                } else {
                    return Ok(Variant(x));
                }
            }
            None => {
                return Err(ParseError::UnexpectedInputTermination);
            }
        }
    }

    pub(crate) fn to_u32(&self) -> Result<u32, ParseError> {
        Ok(u32::try_from(self.0)?)
    }
    pub(crate) fn to_u64(&self) -> Result<u64, ParseError> {
        Ok(self.0)
    }
}

struct ParseState<I>
where
    I: Iterator<Item = Result<u8, IoError>>,
{
    iter: I,
}

impl<I> ParseState<I>
where
    I: Iterator<Item = Result<u8, IoError>>,
{
    fn try_parse_message<T, H: ParseEventHandler<I>>(
        &self,
        handler: &mut H,
    ) -> Result<T, ParseError> {
        todo!()
    }

    fn try_parse_as_packed_variants<T: ParseEventHandler<I>>(
        &self,
        handler: &T,
    ) -> Result<(), ParseError> {
        todo!()
    }
}

trait ParseEventHandler<I>
where
    I: Iterator<Item = Result<u8, IoError>>,
{
    fn start_parse_message(&mut self, state: ParseState<I>);

    fn met_variant_field(&mut self, field_number: usize, value: &Variant)
        -> Result<(), ParseError>;
    fn met_binary_field(
        &mut self,
        field_number: usize,
        state: ParseState<I>,
    ) -> Result<(), ParseError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn test_variant_from_bytes() {
        fn expect_ok(input: &[u8], expected_len: usize, expected_remaining: &[u8]) {
            let result = Variant::from_bytes(input);
            assert!(result.is_ok());
            let (variant, remaining) = result.unwrap();
            assert_eq!(variant.len(), expected_len);
            assert_eq!(remaining, expected_remaining);
        }
        fn expect_err(input: &[u8], expected_err: ParseError) {
            let result = Variant::from_bytes(input);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected_err);
        }
        expect_ok(&[0x00], 1, &[]);
        expect_ok(&[0x00, 0x80, 0x81], 1, &[0x80, 0x81]);
        expect_ok(&[0x80, 0x40], 2, &[]);
        expect_ok(&[0x80, 0x00, 0x00, 0x40], 4, &[]);
        expect_err(&[0x80], ParseError::UnexpectedInputTermination);
    }

    #[test]
    fn test_variant_unsigned() {
        fn get_u32(input: &[u8]) -> Result<u32> {
            let (v, _) = Variant::from_bytes(input)?;
            v.to_u32()
        }
        assert_eq!(get_u32(&[0x00]), Ok(0));
        assert_eq!(get_u32(&[0x01]), Ok(1));
        assert_eq!(get_u32(&[0x7F]), Ok(0x7F));
        assert_eq!(get_u32(&[0x80, 0x01]), Ok(0x80));
        assert_eq!(get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]), Ok(0xFFFFFFFF));
        assert_eq!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]),
            Err(ParseError::IntegerOverflow)
        );
        assert_eq!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
            Err(ParseError::IntegerOverflow)
        );
    } */
}
