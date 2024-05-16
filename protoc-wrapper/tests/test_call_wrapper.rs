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

use ::puroro::google::protobuf::compiler::code_generator_response::File as ResFile;
use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro_protoc_wrapper::Protoc;
use ::std::io::Write;
use ::tempfile::{tempdir, NamedTempFile};

const EMPTY_PROTO_FILE: &'static str = r#"
syntax = "proto3";
package empty;
"#;

#[test]
fn test_call_wrapper() {
    let out_dir = tempdir().unwrap();
    let out_file_name = "empty_test.rs";
    let proto_dir = tempdir().unwrap();
    let proto_file = NamedTempFile::new_in(proto_dir.path()).unwrap();
    proto_file
        .as_file()
        .write_all(EMPTY_PROTO_FILE.as_bytes())
        .unwrap();
    Protoc::new()
        .protoc_path("protoc")
        .out_dir(out_dir.path().to_str().unwrap())
        .proto_file(proto_file.path().to_str().unwrap())
        .proto_path(proto_dir.path().to_str().unwrap())
        .run(|req| Ok(test_call_wrapper_inner(req, &out_file_name)))
        .unwrap();
}

fn test_call_wrapper_inner(
    req: CodeGeneratorRequest,
    out_file_name: &str,
) -> CodeGeneratorResponse<'static> {
    assert_eq!(req.proto_file().count(), 1);
    let mut res = CodeGeneratorResponse::default();
    let mut file = ResFile::default();
    file.set_name(out_file_name).unwrap();
    // empty content
    res.push_file(file).unwrap();
    res
}
