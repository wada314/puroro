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

//! A wrapper around protoc command.
//! Stragety: As an initial implementation step, let's start form a simple code,
//! and then aggeregate those into a strucutered api.

use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorSet;
use ::std::io::Read;

pub fn generate_fds_from_input_reads_using_protoc_naively<'a, I>(input: I) -> Result<FileDescriptorSet>
where
    I: Iterator<Item = (String, &'a dyn Read)>,
{
    todo!()
}
