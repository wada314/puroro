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
#[error(r#"PuroroError. kind = "{kind}""#)]
pub struct PuroroError {
    #[from]
    pub kind: ErrorKind,
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
    #[error("A variant integer type is longer than 10 bytes.")]
    TooLargeVariant,
    #[error("Invalid wire type value: {0}.")]
    InvalidWireType(i32),
    #[error("The io returned an error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("The formatter returned an error: {0}")]
    FormatterError(#[from] std::fmt::Error),
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
