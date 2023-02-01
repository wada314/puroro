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

use ::std::backtrace::Backtrace;

#[derive(Debug, ::thiserror::Error)]
#[error(r#"GeneratorError. kind="{kind}""#)]
pub enum GeneratorError {
    FatalError {
        #[from]
        kind: FatalErrorKind,
        backtrace: Backtrace,
    },
    RecoverableError {
        #[from]
        kind: RecoverableErrorKind,
    },
}

#[derive(Debug, ::thiserror::Error)]
pub enum FatalErrorKind {
    #[error(r#"The type name "{name}" is not found in any other input .proto files."#)]
    UnknownTypeName { name: String },
    #[error(r#"The group feature is not yet supported. GIVE ME A DOCUMENT!!!"#)]
    GroupNotSupported,
    #[error(r#"Unknown value for proto file's syntax: "{name}"."#)]
    UnknownProtoSyntax { name: String },
    #[error(r#"An error from formatter: "{source}""#)]
    WriteError {
        #[from]
        source: std::fmt::Error,
    },
    #[error(r#"An error from ParseIntError: "{source}""#)]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error(r#"An error from ParseFloatError: "{source}""#)]
    ParseBoolError {
        #[from]
        source: std::str::ParseBoolError,
    },
    #[error(r#"An error from ParseFloatError: "{source}""#)]
    ParseFloatError {
        #[from]
        source: std::num::ParseFloatError,
    },
    #[error(r#"An error from TryFromIntError: "{source}""#)]
    TryFromIntError {
        #[from]
        source: std::num::TryFromIntError,
    },
    #[error(r#"An error from syn::parse::Error: "{source}""#)]
    SynParseError {
        #[from]
        source: ::syn::parse::Error,
    },
    #[error(r#"Ar error from std::io::Error: "{source}""#)]
    IoError {
        #[from]
        source: ::std::io::Error,
    },
    #[error(r#"Bad format string: "{string}""#)]
    InvalidString { string: String },
    #[error(r#"An error from puroro: "{source}""#)]
    PuroroError {
        #[from]
        source: crate::puroro::PuroroError,
    },
    #[error(r#"Expected the field descriptor's type_name field is filled, but is not"#)]
    MissingTypeName,
    #[error(r#"Enum must have at least one value"#)]
    NoEnumValues,
    #[error(r#"Utf8 error."#)]
    FromUtf8Error {
        #[from]
        source: ::std::string::FromUtf8Error,
    },
    #[error(
        r#"Invalid combination of the field's label and proto syntax.
     label: {label}, syntax: {syntax}, proto3_optional: {proto3_optional}"#
    )]
    InvalidLabel {
        label: String,
        syntax: String,
        proto3_optional: bool,
    },
    #[error(r#"Something went wrong: "{detail}""#)]
    InternalError { detail: String },
}

#[derive(Debug, ::thiserror::Error)]
pub enum RecoverableErrorKind {
    #[error(r#"Unknown enum value: {0}"#)]
    UnknownEnumValue(i32),
}

macro_rules! impl_from_from {
    ($ty:ty) => {
        impl From<$ty> for GeneratorError {
            fn from(e: $ty) -> Self {
                Into::<FatalErrorKind>::into(e).into()
            }
        }
    };
}
impl_from_from!(::std::fmt::Error);
impl_from_from!(::std::io::Error);
impl_from_from!(crate::puroro::PuroroError);
impl_from_from!(::std::string::FromUtf8Error);
impl_from_from!(::std::num::ParseIntError);
impl_from_from!(::std::num::ParseFloatError);
impl_from_from!(::std::str::ParseBoolError);
impl_from_from!(::std::num::TryFromIntError);
impl_from_from!(::syn::parse::Error);
