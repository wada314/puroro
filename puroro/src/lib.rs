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

#![feature(allocator_api)]
#![feature(slice_as_chunks)]
#![feature(assert_matches)]
#![feature(lazy_cell)]
#![feature(once_cell_try)]

pub mod editions;
pub mod google;
pub mod internal;
pub mod message;
pub mod string;
pub mod untyped_message;
pub mod variant;

use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("io error")]
    IoError(#[from] ::std::io::Error),
    #[error("std::num::TryFromIntError")]
    StdTryFromIntError(#[from] ::std::num::TryFromIntError),
    #[error("integer to boolean conversion error")]
    IntegerToBoolError,
    #[error("Deserializing invalid variant (too long).")]
    TooLongEncodedVariant,
    #[error("Generic deserializing error.")]
    DeserError,
    #[error("The decoded variant value is not convertible to .proto specified int type")]
    VariantValueTooLarge,
    #[error("unknown wire type")]
    UnknownWireType,
    #[error("Unexpected EOF detected while deserializing")]
    DeserUnexpectedEof,
    #[error("Error while validating the input descriptor protos. %s")]
    DescriptorProtoValidationError(String),
    #[error("Error while constructing a descriptor tree structure. %s")]
    DescriptorStructureError(String),
    #[error("Unknown Edition.")]
    UnknownEdition,
    #[error("Error when matching the GenericMessage type's field type.")]
    GenericMessageFieldTypeError,
    #[error("Error when converting an int32 to an (closed) enum value.")]
    TryFromIntIntoEnumError(i32),
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;
