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

use puroro_old::Message;
use puroro_plugin::{generate_output_files_from_file_descriptors, Config, FileDescriptorSet};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../puroro");
    println!("cargo:rerun-if-changed=../puroro-internal");
    println!("cargo:rerun-if-changed=../puroro-plugin");
    println!("cargo:rerun-if-changed=../purotobuf");
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
    let protoc_exe = env::var("PURORO_PROTOC_PATH").unwrap_or("protoc".to_string());
    let protoc_status = Command::new(&protoc_exe)
        .arg("protos/*.proto")
        .arg(format!("--proto_path={}", "./protos/"))
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
    let file_descriptor_set = FileDescriptorSet::from_bytes(fds_file.bytes()).unwrap();

    // Generate the code, returned by File proto structs.
    let output_files =
        generate_output_files_from_file_descriptors(file_descriptor_set.file(), &Config::default())
            .unwrap();

    // Output the File proto structs into the actual filesystem.
    for output_file in output_files {
        dbg!(output_file.name());
        let file_path = output_rust_path.join(output_file.name());
        dbg!(&file_path);
        let mut file = File::create(&file_path).unwrap();
        write!(&mut file, "{}", output_file.content()).unwrap();
    }
}
