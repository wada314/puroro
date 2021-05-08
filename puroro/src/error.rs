#[derive(::thiserror::Error, Debug)]
#[error(r#"PuroroError. kind = "{kind}""#)]
pub struct PuroroError {
    #[from]
    pub kind: ErrorKind,
    #[cfg(feature = "puroro-nightly")]
    pub backtrace: std::backtrace::Backtrace,
}

#[derive(::thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("The input binary has terminated in irregular position.")]
    UnexpectedInputTermination,
    #[error("A variant integer type has too large or too small value.")]
    IntegerOverflow(#[from] std::num::TryFromIntError),
    #[error("A boolean value is nither 0 or 1.")]
    InvalidBooleanValue,
    #[error("Unknown field label [optional, repeated, required]")]
    InvalidFieldLabel,
    #[error("A variant integer type is longer than 10 bytes.")]
    TooLargeVariant,
    #[error("Invalid wire type.")]
    InvalidWireType,
    #[error("Unexpected wire type. e.g. Expected int32, but found a message field.")]
    UnexpectedWireType,
    #[error("Unexpected field type. e.g. Expected int32, but found a uint64 field.")]
    UnexpectedFieldType,
    #[error("Unexpected field number. In protobuf standard, the deserializer should accept this though.")]
    UnexpectedFieldId,
    #[error("An internal error while converting enum from / into integer type.")]
    EnumConvertError,
    #[error("Failed to parse a boolean value.")]
    BoolParseError,
    #[error("Found a packed repeated field, but its length was zero.")]
    ZeroLengthPackedField,
    #[error("The bytestream iterator returned an error: {0}")]
    IteratorError(#[from] std::io::Error),
    #[error("The formatter returned an error: {0}")]
    FormatterError(#[from] std::fmt::Error),
    #[error("The string length is not correct.")]
    InvalidFieldLength,
    #[error("Internal error in SliceView struct.")]
    InvalidSliceViewType,
    #[error("Group is not supported.")]
    GroupNotSupported,
    #[error("Other error: {0}")]
    OtherErrors(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for PuroroError {
    fn from(input: std::io::Error) -> Self {
        PuroroError::from(ErrorKind::from(input))
    }
}
impl From<std::fmt::Error> for PuroroError {
    fn from(input: std::fmt::Error) -> Self {
        PuroroError::from(ErrorKind::from(input))
    }
}
impl From<std::num::TryFromIntError> for PuroroError {
    fn from(input: std::num::TryFromIntError) -> Self {
        PuroroError::from(ErrorKind::from(input))
    }
}
