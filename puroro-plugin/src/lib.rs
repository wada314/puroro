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
#![feature(is_some_and)]
#![feature(slice_group_by)]
#![feature(new_uninit)]

mod codegen;
mod codegen2;
mod error;
mod rustfmt;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

pub use crate::codegen::Config;
pub use ::puroro_protobuf_compiled::google::protobuf::compiler::code_generator_response::File;
pub use ::puroro_protobuf_compiled::google::protobuf::{FileDescriptorProto, FileDescriptorSet};
pub use ::puroro_protobuf_compiled::puroro;

pub use crate::codegen::generate_output_files_from_file_descriptors;
pub use crate::codegen::generate_output_files_from_file_descriptors2;
pub use crate::codegen2::generate_output_file_protos;
