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

use ::itertools::Itertools;
use ::puroro::google::protobuf::compiler::{
    code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse,
};
use ::puroro::message::MessageLite;
use ::puroro::Result as PResult;
use ::thiserror::Error;
use descriptor::{Context, FileDescriptor};

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Unknown compile error")]
    CompileError,
    #[error("puroro error")]
    PuroroError(#[from] ::puroro::ErrorKind),
    #[error("Unknown Edition.")]
    UnknownEdition,
    #[error("Error while validating the input descriptor protos. {0}")]
    DescriptorProtoValidationError(String),
    #[error("std::num::TryFromIntError")]
    StdTryFromIntError(#[from] ::std::num::TryFromIntError),
    #[error("Error while constructing a descriptor tree structure. {0}")]
    DescriptorStructureError(String),
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub fn compile(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse<'static>> {
    let mut response = CodeGeneratorResponse::default();

    let descriptors: Vec<FileDescriptor> = request
        .proto_file()
        .map_ok(TryInto::try_into)
        .collect::<PResult<Result<Vec<_>>>>()??;

    let root_context: Context<'static> = descriptors.into();

    let mut file = code_generator_response::File::default();
    file.set_name("test.rs")?;
    file.set_content("pub fn yeah() { }")?;
    response.push_file(file)?;

    Ok(response)
}

pub fn compile_binary(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let request = CodeGeneratorRequest::deser_from_read(input.as_ref()).unwrap();
    let response = compile(&request).unwrap();
    let mut output_buffer = Vec::new();
    response.write(&mut output_buffer).unwrap();
    Ok(output_buffer)
}
