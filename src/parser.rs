#[derive(::thiserror::Error, Debug)]
enum ParseError {
    #[error("The input binary has terminated in irregular position.")]
    InputTerminatedUnexpectedly,
    #[error("A value of an integer type has too large or too small value.")]
    IntegerOverflow,
}
struct Variant<'a>(&'a [u8]);

impl<'a> Variant<'a> {
    fn as_u64(&self) -> Result<u64, ParseError> {
        let mut value: u64 = 0;
        for i in 0..((64 - 1) / 7) {
            if i >= self.0.len() {
                return Err(ParseError::InputTerminatedUnexpectedly);
            }
            value |= (0x7F & self.0[i] as u64) << (i * 7);
            if (self.0[i] & 0x80) != 0 {
                return Ok(value);
            }
        }
        // last possible byte for this int size.
        let i = (64 - 1) / 7;
        if i >= self.0.len() {
            return Err(ParseError::InputTerminatedUnexpectedly);
        }
        if (self.0[i] & 0x80) != 0 {
            return Err(ParseError::IntegerOverflow);
        }
        let remaining_bits = 64 - i * 7;
        let remaining_bits_mask = ((1u32 << remaining_bits) - 1) as u8;
        if (self.0[i] & !remaining_bits_mask) != 0 {
            return Err(ParseError::IntegerOverflow);
        }
        value |= (self.0[i] as u64) << (i * 7);
        Ok(value)
    }
}

trait ParseEventHandler {
    fn found_variant_field(&mut self, field_number: usize, value: &Variant);
    fn found_binary_field(&mut self, field_number: usize, value: &[u8]);
}
