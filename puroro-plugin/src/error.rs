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

#[derive(Debug, ::thiserror::Error)]
#[error(r#"GeneratorError. kind="{kind}""#)]
pub struct GeneratorError {
    #[from]
    kind: ErrorKind,
    backtrace: std::backtrace::Backtrace,
}

#[derive(Debug, ::thiserror::Error)]
pub enum ErrorKind {
    #[error(r#"The type name "{name}" is not found in any other input .proto files."#)]
    UnknownTypeName { name: String },
    #[error(r#"The group feature is not yet supported. GIVE ME A DOCUMENT!!!"#)]
    GroupNotSupported,
    #[error(r#"Unknown value for proto file's syntax: "{name}"."#)]
    UnknownProtoSyntax { name: String },
    // #[error(r#"The enum type "{name}" has no values. The empty enum is not allowed."#)]
    // EmptyEnum { name: String },
    // #[error(r#"A length of some sort of array in the proto is too large."#)]
    // TooLargeLength,
    #[error(r#"An error from formatter: "{source}""#)]
    WriteError { source: std::fmt::Error },
    #[error(r#"An error from ParseIntError: "{source}""#)]
    ParseIntError { source: std::num::ParseIntError },
    #[error(r#"Ar error from std::io::Error: "{source}""#)]
    IoError { source: ::std::io::Error },
    #[error(r#"Bad format string: "{string}""#)]
    InvalidString { string: String },
    #[error(r#"An error from puroro: "{source}""#)]
    PuroroError { source: crate::puroro::PuroroError },
    #[error(r#"Expected the field descriptor's type_name field is filled, but is not"#)]
    MissingTypeName,
    #[error(r#"Utf8 error."#)]
    FromUtf8Error {
        source: ::std::string::FromUtf8Error,
    },
    #[error(r#"Something went wrong: "{detail}""#)]
    InternalError { detail: String },
}
impl From<::std::fmt::Error> for GeneratorError {
    fn from(e: ::std::fmt::Error) -> Self {
        Self {
            kind: ErrorKind::WriteError { source: e },
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
impl From<::std::io::Error> for GeneratorError {
    fn from(e: ::std::io::Error) -> Self {
        Self {
            kind: ErrorKind::IoError { source: e },
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
impl From<crate::puroro::PuroroError> for GeneratorError {
    fn from(e: crate::puroro::PuroroError) -> Self {
        Self {
            kind: ErrorKind::PuroroError { source: e },
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
impl From<::std::string::FromUtf8Error> for GeneratorError {
    fn from(e: ::std::string::FromUtf8Error) -> Self {
        Self {
            kind: ErrorKind::FromUtf8Error { source: e },
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
