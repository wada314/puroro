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
// limitations under the License.<

use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro::message::MessageLite;
use ::std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input_buffer = Vec::new();
    stdin().read_to_end(&mut input_buffer).unwrap();
    let request = CodeGeneratorRequest::deser_from_read(input_buffer.as_slice()).unwrap();
}