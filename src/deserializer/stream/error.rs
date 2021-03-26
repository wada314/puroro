use std::io::Error as IoError;
#[derive(::thiserror::Error, Debug)]
pub enum DeserializeError {
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
pub type Result<T> = std::result::Result<T, DeserializeError>;
