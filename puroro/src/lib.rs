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
#![feature(assert_matches)]
#![feature(once_cell_try)]
#![feature(once_cell_try_insert)]
#![feature(never_type)]
#![feature(unwrap_infallible)]
#![feature(downcast_unchecked)]
#![feature(vec_into_raw_parts)]

pub mod default_in;
pub mod dynamic;
pub mod editions;
pub mod either_ext;
pub mod google;
pub mod internal;
pub mod message;
pub mod non_empty;
pub mod repeated;
pub mod string;
pub mod variant;

pub use self::non_empty::{IsEmpty, NonEmpty};
pub use ::culpa::throws;
pub use ::quither::{Both, Either, EitherOrBoth};

use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("infallible error")]
    Infallible(#[from] ::std::convert::Infallible),
    #[error("io error: {0}")]
    IoError(#[from] ::std::io::Error),
    #[error("std::num::TryFromIntError: {0}")]
    StdTryFromIntError(#[from] ::std::num::TryFromIntError),
    #[error("std::str::Utf8Error: {0}")]
    Utf8Error(#[from] ::std::str::Utf8Error),
    #[error("puroro error: {0}")]
    PuroroError(String),
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
    #[error("Error when matching the DynamicMessage type's field type.")]
    DynamicMessageFieldTypeError,
    #[error("Error when converting an int32 to an (closed) enum value: {0}")]
    TryFromIntIntoEnumError(i32),
    #[error("No left value available for generating default value")]
    NoLeftValueForDefault,
}
impl From<!> for ErrorKind {
    fn from(value: !) -> Self {
        match value {}
    }
}
impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        ErrorKind::PuroroError(s)
    }
}
impl From<&'static str> for ErrorKind {
    fn from(s: &'static str) -> Self {
        ErrorKind::PuroroError(s.to_string())
    }
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub struct Person<T = PersonInner>(T);
pub struct PersonInner {
    name: String,
}
pub trait PersonView {
    fn name(&self) -> &str;
}
pub trait PersonTryView {
    fn try_name(&self) -> Result<&str>;
}

impl<T: PersonView> Person<T> {
    pub fn name(&self) -> &str {
        self.0.name()
    }
}

impl<T: PersonTryView> Person<T> {
    pub fn try_name(&self) -> Result<&str> {
        self.0.try_name()
    }
}

impl PersonView for PersonInner {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PersonTryView for self::dynamic::DynamicMessage {
    fn try_name(&self) -> Result<&str> {
        let name = self
            .field(1)
            .map(|f| f.as_scalar_string(dynamic::FieldReducingErrorStrategy::Skip))
            .transpose()?
            .flatten()
            .unwrap_or_default();
        Ok(name)
    }
}
