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
use ::protoc_plugin_by_closure::Protoc;
use ::puroro_codegen::compile_binary;
use ::std::fs;
use ::std::path::Path;
use ::std::time::Duration;

fn main() -> Result<(), Error> {
    print!("cargo::rerun-if-changed=tests/protos");
    print!("cargo::rerun-if-changed=../codegen/src");

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
        .run(
            Duration::from_secs(3),
            |req| match ::std::panic::catch_unwind(|| compile_binary(&req)) {
                Ok(Ok(r)) => Ok(r),
                Ok(Err(e)) => Err(e.to_string()),
                Err(e) => {
                    let e = match e.downcast::<String>() {
                        Ok(s) => return Err(*s),
                        Err(e) => e,
                    };
                    let _ = match e.downcast::<&str>() {
                        Ok(s) => return Err(s.to_string()),
                        Err(e) => e,
                    };
                    Err("Unknown panic".to_string())
                }
            },
        )?;
    Ok(())
}
