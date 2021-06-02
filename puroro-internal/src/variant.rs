use crate::tags;
use crate::{ErrorKind, Result};
use std::convert::TryFrom;
use std::io::Result as IoResult;
use std::io::Write;

#[derive(Debug, Default, Clone)]
pub struct Variant([u8; 8]);

impl Variant {
    fn new(bytes: [u8; 8]) -> Self {
        Variant(bytes)
    }
    pub(crate) fn decode_bytes<I>(bytes: &mut I) -> Result<Self>
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
                None => Err(ErrorKind::UnexpectedInputTermination)?,
            }
        }
        // i == 9, so now checking a last MSBit.
        match bytes.next() {
            Some(maybe_byte) => {
                let byte = maybe_byte?;
                x |= ((byte & 0x01) as u64) << 63;
                if byte & 0xFE != 0 {
                    Err(ErrorKind::TooLargeVariant)?
                } else {
                    return Ok(Variant(x.to_ne_bytes()));
                }
            }
            None => Err(ErrorKind::UnexpectedInputTermination)?,
        }
    }
    pub(crate) fn encode_bytes<W>(&self, write: &mut W) -> Result<()>
    where
        W: Write,
    {
        let mut x = u64::from_le_bytes(self.0);
        if x == 0 {
            write.write_all(&[0])?;
            return Ok(());
        }
        let mut length = 0;
        let mut buf: [u8; 10] = Default::default();
        for i in 0..10 {
            if x == 0 {
                length = i;
                break;
            }
            buf[i] = ((x as u8) & 0x7F) | 0x80;
            x = x >> 7;
        }
        buf[length - 1] = buf[length - 1] & 0x7F;
        write.write_all(buf.split_at(length).0)?;
        Ok(())
    }

    pub fn to_native<T: VariantTypeTag>(&self) -> Result<T::NativeType> {
        T::from_variant(self)
    }
    pub fn from_native<T: VariantTypeTag>(val: T::NativeType) -> Result<Variant> {
        T::to_variant(val)
    }

    fn to_sint(&self) -> Result<i64> {
        // decode zigzag encoding for sint32 and sint64.
        let x = self.to_i64()?;
        Ok((x ^ (0 - (x & 1))) >> 1)
    }
    fn from_sint(x: i64) -> Self {
        let encoded = (x >> 63) ^ (x << 1);
        Self(encoded.to_le_bytes())
    }

    pub fn zero() -> Self {
        Self([0; 8])
    }
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }
}

macro_rules! define_convert_methods {
    ($vtype:ty, $toname:ident, $fromname:ident) => {
        pub fn $toname(&self) -> Result<<$vtype as VariantTypeTag>::NativeType> {
            self.to_native::<$vtype>()
        }
        pub fn $fromname(val: <$vtype as VariantTypeTag>::NativeType) -> Result<Variant> {
            <$vtype as VariantTypeTag>::to_variant(val)
        }
    };
}
impl Variant {
    define_convert_methods!(tags::value::UInt32, to_u32, from_u32);
    define_convert_methods!(tags::value::UInt64, to_u64, from_u64);
    define_convert_methods!(tags::value::SInt32, to_si32, from_si32);
    define_convert_methods!(tags::value::SInt64, to_si64, from_si64);
    define_convert_methods!(tags::value::Int32, to_i32, from_i32);
    define_convert_methods!(tags::value::Int64, to_i64, from_i64);
    define_convert_methods!(tags::value::Bool, to_bool, from_bool);
    define_convert_methods!(RustUsize, to_usize, from_usize);
}

pub trait VariantTypeTag {
    type NativeType;
    fn from_variant(var: &Variant) -> Result<Self::NativeType>;
    fn to_variant(val: Self::NativeType) -> Result<Variant>;
}

pub struct RustUsize();

impl VariantTypeTag for tags::value::Int32 {
    type NativeType = i32;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(i32::try_from(i64::from_le_bytes(var.0))?)
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(i64::to_le_bytes(i64::from(val))))
    }
}
impl VariantTypeTag for tags::value::UInt32 {
    type NativeType = u32;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(u32::try_from(u64::from_le_bytes(var.0))?)
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(u64::to_le_bytes(u64::from(val))))
    }
}
impl VariantTypeTag for tags::value::SInt32 {
    type NativeType = i32;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(i32::try_from(var.to_sint()?)?)
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::from_sint(i64::from(val)))
    }
}

impl VariantTypeTag for tags::value::Int64 {
    type NativeType = i64;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(i64::from_le_bytes(var.0))
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(i64::to_le_bytes(val)))
    }
}
impl VariantTypeTag for tags::value::UInt64 {
    type NativeType = u64;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(u64::from_le_bytes(var.0))
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(u64::to_le_bytes(val)))
    }
}
impl VariantTypeTag for tags::value::SInt64 {
    type NativeType = i64;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(var.to_sint()?)
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::from_sint(val))
    }
}
impl VariantTypeTag for tags::value::Bool {
    type NativeType = bool;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        match u64::from_le_bytes(var.0) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ErrorKind::InvalidBooleanValue)?,
        }
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(u64::to_le_bytes(if val { 1 } else { 0 })))
    }
}
impl<T> VariantTypeTag for tags::value::Enum<T>
where
    T: TryFrom<i32, Error = i32> + Into<i32>,
{
    type NativeType = std::result::Result<T, i32>;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(T::try_from(i32::try_from(i64::from_le_bytes(var.0))?))
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        let int_val = match val {
            Ok(v) => T::into(v),
            Err(i) => i,
        };
        Ok(Variant::new(i64::to_le_bytes(i64::from(int_val))))
    }
}
impl VariantTypeTag for RustUsize {
    type NativeType = usize;
    fn from_variant(var: &Variant) -> Result<Self::NativeType> {
        Ok(usize::try_from(u64::from_le_bytes(var.0))?)
    }
    fn to_variant(val: Self::NativeType) -> Result<Variant> {
        Ok(Variant::new(u64::to_le_bytes(u64::try_from(val)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PuroroError;
    use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read};

    fn collect_iter<I: Iterator<Item = IoResult<u8>>>(iter: I) -> Vec<u8> {
        iter.map(|r| r.unwrap()).collect::<Vec<_>>()
    }

    #[test]
    fn test_variant_from_bytes() {
        let io_error1 = IoError::new(IoErrorKind::InvalidData, "");

        fn expect_ok(input: &[u8], expected_value: u64, expected_remaining: &[u8]) {
            let mut iter = input.bytes();
            let result = Variant::decode_bytes(&mut iter);
            assert!(result.is_ok());
            let variant = result.unwrap();
            assert_eq!(variant.0, expected_value.to_ne_bytes());
            assert_eq!(collect_iter(iter), expected_remaining);
        }
        fn get_err(input: Vec<IoResult<u8>>) -> PuroroError {
            let mut iter = input.into_iter();
            let result = Variant::decode_bytes(&mut iter);
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
            get_err(vec![Ok(0x80)]).kind,
            ErrorKind::UnexpectedInputTermination
        ));
        assert!(matches!(
            get_err(vec![Err(io_error1)]).kind,
            ErrorKind::IteratorError(_)
        ));
    }

    #[test]
    fn test_variant_unsigned() {
        fn get_u32(input: &[u8]) -> Result<u32> {
            let mut iter = input.bytes();
            let v = Variant::decode_bytes(&mut iter)?;
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
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]).unwrap_err().kind,
            ErrorKind::IntegerOverflow(_)
        ));
        assert!(matches!(
            get_u32(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F])
                .unwrap_err()
                .kind,
            ErrorKind::IntegerOverflow(_)
        ));
    }

    #[test]
    fn test_variant_signed_zigzag() {
        fn get_si32(input: &[u8]) -> Result<i32> {
            let mut iter = input.bytes();
            let v = Variant::decode_bytes(&mut iter)?;
            assert_eq!(collect_iter(iter), Vec::<u8>::new());
            v.to_si32()
        }
        assert_eq!(get_si32(&[0x00]).unwrap(), 0);
        assert_eq!(get_si32(&[0x01]).unwrap(), -1);
        assert_eq!(get_si32(&[0x02]).unwrap(), 1);
        assert_eq!(get_si32(&[0x7F]).unwrap(), -0x40);
        assert_eq!(get_si32(&[0x80, 0x01]).unwrap(), 0x40);
        assert_eq!(
            get_si32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]).unwrap(),
            -0x80000000
        );
        assert!(matches!(
            get_si32(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]).unwrap_err().kind,
            ErrorKind::IntegerOverflow(_)
        ));
        assert!(matches!(
            get_si32(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F])
                .unwrap_err()
                .kind,
            ErrorKind::IntegerOverflow(_)
        ));
    }
}
