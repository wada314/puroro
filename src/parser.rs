use ::num_traits::{One, PrimInt, Zero};
use std::ops::BitOrAssign;

trait Unsigned: PrimInt + Zero + One + BitOrAssign {
    fn from_u8(from: u8) -> Self;
}
impl Unsigned for u32 {
    fn from_u8(from: u8) -> Self {
        from as u32
    }
}
impl Unsigned for u64 {
    fn from_u8(from: u8) -> Self {
        from as u64
    }
}

#[derive(::thiserror::Error, Debug, PartialEq)]
pub(crate) enum ParseError {
    #[error("The input binary has terminated in irregular position.")]
    UnexpectedInputTermination,
    #[error("A value of an integer type has too large or too small value.")]
    IntegerOverflow,
}

#[derive(Debug)]
struct Variant<'a>(&'a [u8]);
impl<'a> Variant<'a> {
    pub(crate) fn from_bytes(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), ParseError> {
        let mut i = 0;
        while i < bytes.len() {
            if (bytes[i] & 0x80) == 0 {
                let (variant, remaining) = bytes.split_at(i + 1);
                return Ok((Self(variant), remaining));
            }
            i += 1;
        }
        Err(ParseError::UnexpectedInputTermination)
    }
}

impl<'a> Variant<'a> {
    fn as_unsigned<T: Unsigned>(&self) -> Result<T, ParseError> {
        debug_assert!(self.0.len() >= 1);
        let mut value: T = T::zero();
        let bits: usize = std::mem::size_of::<T>() * 8;
        if self.0.len() > (bits + 6) / 7 {
            return Err(ParseError::IntegerOverflow);
        }
        for i in 0..(self.0.len() - 1) {
            value |= T::from_u8(0x7f & self.0[i]) << (i * 7);
        }
        // last byte for this int size.
        let i = self.0.len() - 1;
        let remaining_bits = bits - i * 7;
        debug_assert_eq!(remaining_bits, (bits + 6) % 7 + 1);
        debug_assert!(remaining_bits <= 7);
        let remaining_bits_mask = ((1u32 << remaining_bits) - 1) as u8;
        if (self.0[i] & !remaining_bits_mask) != 0 {
            return Err(ParseError::IntegerOverflow);
        }
        value |= T::from_u8(self.0[i]) << (i * 7);
        Ok(value)
    }
    pub(crate) fn as_u32(&self) -> Result<u32, ParseError> {
        self.as_unsigned::<u32>()
    }
    pub(crate) fn as_u64(&self) -> Result<u64, ParseError> {
        self.as_unsigned::<u64>()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

trait ParseEventHandler {
    fn found_variant_field(&mut self, field_number: usize, value: &Variant);
    fn found_binary_field(&mut self, field_number: usize, value: &[u8]);
}

#[cfg(test)]
mod tests {
    use super::*;
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
        expect_err(&[0x80], ParseError::UnexpectedInputTermination);
    }
}
