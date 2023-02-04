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

use puroro_codegen::puroro::Message;
use puroro_codegen::{generate_output_file_protos, CodegenOptions, FileDescriptorSet};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../puroro");
    println!("cargo:rerun-if-changed=../puroro-codegen");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/*.proto");

    let output_rust_path = ["src"].iter().collect::<PathBuf>();
    let file_descriptor_set_file_path = [
        env::var("OUT_DIR").unwrap(),
        "file_descriptor_set.pb".to_string(),
    ]
    .iter()
    .collect::<PathBuf>();

    // Run protoc command, output a temporal file which contains the encoded FileDescriptorSet.
    let protoc_exe = protoc_bin_vendored::protoc_bin_path().unwrap();
    let protoc_status = Command::new(&protoc_exe)
        .arg("protos/*.proto")
        .arg(format!("--proto_path={}", "./protos/"))
        .arg("--include_source_info")
        .arg("--experimental_allow_proto3_optional")
        .arg(format!(
            "--descriptor_set_out={}",
            file_descriptor_set_file_path.to_string_lossy()
        ))
        .status()
        .unwrap();
    if !protoc_status.success() {
        println!("cargo:warning=Failed to run `protoc` command.");
        panic!("Failed to run `protoc` command.")
    }

    // Decode the FileDescriptorSet output by the above code.
    let fds_file = File::open(&file_descriptor_set_file_path).unwrap();
    let file_descriptor_set = FileDescriptorSet::from_bytes_iter(fds_file.bytes()).unwrap();

    // Generate the code, returned by File proto structs.
    let options = CodegenOptions::default();
    let cgr =
        generate_output_file_protos(file_descriptor_set.file().into_iter(), &options).unwrap();

    // Output the File proto structs into the actual filesystem.
    for output_file in cgr.file() {
        let file_path = output_rust_path.join(output_file.name());
        let mut file = File::create(&file_path).unwrap();
        write!(&mut file, "{}", output_file.content()).unwrap();
    }
}
