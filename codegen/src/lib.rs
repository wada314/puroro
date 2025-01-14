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

#![feature(once_cell_try)]
#![feature(error_generic_member_access)]
#![feature(assert_matches)]

pub mod cases;
pub mod descriptor;
pub mod generator;
pub mod proto_path;

pub use crate::generator::compile;
use ::puroro::dynamic::DynamicMessage;
use ::puroro::google::protobuf::compiler::CodeGeneratorRequest;
use ::puroro::message::{Message, MessageMut};
use ::std::backtrace::Backtrace;
use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Compile error: {0}\n{1}")]
    CompileError(String, Backtrace),
    #[error("::puroro error: {0}\n{1}")]
    PuroroError(#[from] ::puroro::ErrorKind, Backtrace),
    #[error("::std::num::TryFromIntError: {0}\n{1}")]
    StdTryFromIntError(#[from] ::std::num::TryFromIntError, Backtrace),
    #[error("::syn error: {0}\n{1}")]
    SynParseError(#[from] ::syn::Error, Backtrace),
}
impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        ErrorKind::CompileError(s, Backtrace::capture())
    }
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub fn compile_binary(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let request: CodeGeneratorRequest = DynamicMessage::deser_from_read(input.as_ref())
        .unwrap()
        .into();
    let response = compile(&request).unwrap();
    let mut output_buffer = Vec::new();
    response.write(&mut output_buffer).unwrap();
    Ok(output_buffer)
}
