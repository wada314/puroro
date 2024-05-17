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

pub mod descriptor;

use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro::message::MessageLite;
use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Unknown compile error")]
    CompileError,
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub fn compile(_input: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse<'static>> {
    let response = CodeGeneratorResponse::default();
    Ok(response)
}

pub fn compile_binary(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let request = CodeGeneratorRequest::deser_from_read(input.as_ref()).unwrap();
    let response = compile(&request).unwrap();
    let mut output_buffer = Vec::new();
    response.write(&mut output_buffer).unwrap();
    Ok(output_buffer)
}
