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

#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(generic_associated_types)]
#![feature(is_some_with)]

mod codegen;
mod error;
mod rustfmt;

use ::puroro::Message;
use ::puroro_protobuf_compiled::google::protobuf::compiler::CodeGeneratorRequest;
use ::std::io::{stdin, stdout, Read};

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

fn main() -> Result<()> {
    let request = CodeGeneratorRequest::from_bytes(&mut stdin().bytes()).unwrap();
    let response = codegen::generate(&request)?;
    response.ser(&mut stdout())?;
    Ok(())
}
