// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(::thiserror::Error, Debug)]
pub enum PuroroError {
    #[error("The input binary has terminated in irregular position.")]
    UnexpectedInputTermination,
    #[error("A variant integer type has too large or too small value.")]
    IntegerOverflow(#[from] std::num::TryFromIntError),
    #[error("A boolean value is nither 0 or 1.")]
    InvalidBooleanValue,
    #[error("Unknown field label [optional, repeated, required]")]
    InvalidFieldLabel,
    #[error("Too large field number. The max value is 2^29 - 1.")]
    InvalidFieldNumber,
    #[error("A variant integer type is longer than 10 bytes.")]
    TooLargeVariant,
    #[error("The serialized message is too long. The upper limit is 2^31 - 1 bytes.")]
    TooLongToSerialize,
    #[error("Invalid wire type value: {0}.")]
    InvalidWireType(u32),
    #[error("Unexpected wire type. e.g. Expected int32, but found a message field.")]
    UnexpectedWireType,
    #[error("Unexpected field type. e.g. Expected int32, but found a uint64 field.")]
    UnexpectedFieldType,
    #[error("Unknown field number. This should be recoverable.")]
    UnknownFieldNumber(crate::internal::ser::FieldData<Vec<u8>>),
    #[error("An internal error while converting enum from / into integer type.")]
    EnumConvertError,
    #[error("Failed to parse a boolean value.")]
    BoolParseError,
    #[error("Found a packed repeated field, but its length was zero.")]
    ZeroLengthPackedField,
    #[error("Unknown enum variant.")]
    UnknownEnumVariant(i32),
    #[error("The bytestream iterator returned an error: {0}")]
    IteratorError(#[from] std::io::Error),
    #[error("The formatter returned an error: {0}")]
    FormatterError(#[from] std::fmt::Error),
    #[error("The length of given field is not valid (Mostly a negative number).")]
    InvalidFieldLength,
    #[error("Internal error in SliceView struct.")]
    InvalidSliceViewType,
    #[error("Invalid UTF8 string is given.")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Invalid UTF8 string is given to puroro::String.")]
    InvalidUtf8Puroro(#[from] crate::string::FromUtf8Error),
    #[error("Invalid UTF8 string is given.")]
    InvalidUtf8Bumpalo(),
    #[error("Group is not supported.")]
    GroupNotSupported,
    #[error("Internal error in oneof item treatment.")]
    InvalidOneofIndex,
    #[error("Puroro library's error. A bug.")]
    InternalError,
    #[error("Other error: {0}")]
    OtherErrors(Box<dyn std::error::Error>),
}
