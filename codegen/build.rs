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
use ::puroro_protoc_wrapper::Protoc;
use ::std::fs;
use ::std::path::PathBuf;

fn main() -> Result<(), Error> {
    if !cfg!(test) {
        return Ok(());
    }

    fs::remove_dir_all("tests/generated").unwrap();

    let file_paths_in_protos = fs::read_dir("tests/protos")
        .unwrap()
        .filter_ok(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false))
        .map_ok(|e| e.path().strip_prefix("tests/protos"))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    Protoc::new().out_dir("tests/generated").proto_files()
}
