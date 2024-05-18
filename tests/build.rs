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

use ::anyhow::Error;
use ::itertools::Itertools;
use ::puroro_codegen::compile;
use ::puroro_protoc_wrapper::Protoc;
use ::std::fs;
use ::std::path::Path;

fn main() -> Result<(), Error> {
    print!("cargo::rerun-if-changed=tests/protos");
    print!("cargo::rerun-if-changed=../codegen");

    let _ = fs::remove_dir_all("tests/generated"); // Allow error: directory may not exist
    fs::create_dir("tests/generated").unwrap();

    let file_paths_in_protos_dir = fs::read_dir("tests/protos")
        .unwrap()
        .filter_ok(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false))
        .map_ok(|e| e.path().strip_prefix("tests/protos").map(Path::to_owned))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    Protoc::new()
        .out_dir("tests/generated")
        .proto_files(file_paths_in_protos_dir)
        .proto_path("tests/protos")
        .run(|req| compile(&req).map_err(|e| e.to_string()))?;
    Ok(())
}
