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
pub mod proto_path;

use self::descriptor::{FileDescriptor, RootContext};
use ::itertools::Itertools;
use ::puroro::google::protobuf::compiler::{
    code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse,
};
use ::puroro::message::MessageLite;
use ::puroro::Result as PResult;
use ::std::collections::HashMap;
use ::thiserror::Error;

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
    #[error("ProtoPath strip prefix error: Tried to split \"{1}\" from \"{0}\"")]
    ProtoPathStripPrefixError(String, String),
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

struct GeneratedFile {
    name: String,
    sources: Vec<String>,
    content: String,
}
impl GeneratedFile {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sources: Vec::new(),
            content: String::new(),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn append(&mut self, source: impl AsRef<str>) {
        self.content.push_str(source.as_ref());
    }
    fn add_source(&mut self, source: impl Into<String>) {
        self.sources.push(source.into());
    }
}
impl TryFrom<GeneratedFile> for code_generator_response::File<'_> {
    type Error = ErrorKind;
    fn try_from(from: GeneratedFile) -> Result<Self> {
        let mut file = code_generator_response::File::default();
        file.set_name(from.name())?;
        let source_list = from
            .sources
            .into_iter()
            .map(|s| format!("//   {}\n", s))
            .join("");
        file.set_content(&format!(
            "\
            // THIS FILE IS A GENERATED FILE! DO NOT EDIT!\n\
            // Source(s):\n\
            {}\n\
            \n\
            {}\n",
            source_list, from.content
        ))?;
        Ok(file)
    }
}

pub fn compile(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse<'static>> {
    let mut response = CodeGeneratorResponse::default();

    let descriptors: Vec<FileDescriptor> = request
        .proto_file()
        .map_ok(TryInto::try_into)
        .collect::<PResult<Result<Vec<_>>>>()??;

    let root_context: RootContext = descriptors.into();
    let mut out_files = HashMap::new();

    for fd in root_context.files() {
        let file_name = if let Some(package) = fd.package()? {
            package.split('.').join("/") + ".rs"
        } else {
            "mod.rs".to_string()
        };
        let file = out_files
            .entry(file_name.clone())
            .or_insert_with(|| GeneratedFile::new(file_name));
        file.append("pub fn yeah() { }");
        file.add_source(fd.name()?);
    }

    for file in out_files.into_values() {
        response.push_file(file.try_into()?)?;
    }

    Ok(response)
}

pub fn compile_binary(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let request = CodeGeneratorRequest::deser_from_read(input.as_ref()).unwrap();
    let response = compile(&request).unwrap();
    let mut output_buffer = Vec::new();
    response.write(&mut output_buffer).unwrap();
    Ok(output_buffer)
}
