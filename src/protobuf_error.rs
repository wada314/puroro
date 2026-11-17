//! Conversion from [`protobuf_core::ProtobufError`] to puroro error types.

use ::protobuf_core::ProtobufError;

use crate::error::DecodeError;

impl From<ProtobufError> for DecodeError {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::UnexpectedEof | ProtobufError::IoError(_) => DecodeError::UnexpectedEof,
            ProtobufError::VarintTooLong | ProtobufError::VarintDowncastOutOfRange { .. } => {
                DecodeError::InvalidVarint
            }
            ProtobufError::InvalidWireType { .. }
            | ProtobufError::FieldNumberOutOfRange { .. }
            | ProtobufError::MismatchedEndGroup { .. } => DecodeError::InvalidTag,
            ProtobufError::SizeLimitExceeded { value, limit } => {
                DecodeError::SizeLimitExceeded { value, limit }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DecodeError, ProtobufError};
    use ::protobuf_core::{AsRefExtProtobuf, MAX_STRING_SIZE};

    #[test]
    fn size_limit_maps_value_and_limit() {
        let err = DecodeError::from(ProtobufError::SizeLimitExceeded { value: 9, limit: 8 });
        assert_eq!(err, DecodeError::SizeLimitExceeded { value: 9, limit: 8 });
        assert_eq!(err.to_string(), "size 9 exceeds the limit of 8 bytes");
    }

    #[test]
    fn oversized_len_prefix_from_the_reader_maps() {
        let len = u32::try_from(MAX_STRING_SIZE).unwrap() + 1;
        let mut bytes = ::std::vec![0x0a_u8];
        let mut n = u64::from(len);
        loop {
            let mut byte = (n & 0x7f) as u8;
            n >>= 7;
            if n != 0 {
                byte |= 0x80;
            }
            bytes.push(byte);
            if n == 0 {
                break;
            }
        }
        let err = bytes.read_protobuf_fields().next().unwrap().unwrap_err();
        assert_eq!(
            DecodeError::from(err),
            DecodeError::SizeLimitExceeded {
                value: u64::from(len),
                limit: MAX_STRING_SIZE,
            }
        );
    }
}
