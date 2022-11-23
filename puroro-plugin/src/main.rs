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

#![allow(incomplete_features)]
#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(is_some_and)]
#![feature(try_find)]
#![feature(trait_upcasting)]

mod codegen2;
mod error;

use crate::error::{ErrorKind, GeneratorError};
type Result<T> = ::std::result::Result<T, GeneratorError>;
use ::puroro_protobuf_compiled::google::protobuf::compiler::CodeGeneratorRequest;
use ::puroro_protobuf_compiled::puroro;
use ::puroro_protobuf_compiled::Message;
use ::std::io::Read;
use ::std::io::{stdin, stdout};

fn main() -> Result<()> {
    let request = CodeGeneratorRequest::from_bytes_iter(&mut stdin().bytes()).unwrap();
    let response = codegen2::generate_output_file_protos(request.proto_file().into_iter())?;
    response.to_bytes(&mut stdout())?;
    Ok(())
}
