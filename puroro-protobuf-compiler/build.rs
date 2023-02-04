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
use std::fs::{create_dir_all, remove_dir_all, rename, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../puroro");
    println!("cargo:rerun-if-changed=../puroro-plugin");
    println!("cargo:rerun-if-changed=../protobuf");
    println!("cargo:rerun-if-changed=build.rs");

    let output_rust_path = ["../puroro/src/protobuf"].iter().collect::<PathBuf>();
    let temp_output_rust_path = [env::var("OUT_DIR").unwrap(), "tmp-src".to_string()]
        .iter()
        .collect::<PathBuf>();
    let file_descriptor_set_file_path = [
        env::var("OUT_DIR").unwrap(),
        "file_descriptor_set.pb".to_string(),
    ]
    .iter()
    .collect::<PathBuf>();

    // Run protoc command, output a temporal file which contains the encoded FileDescriptorSet.
    let protoc_exe = env::var("PURORO_PROTOC_PATH").unwrap_or("protoc".to_string());
    let protoc_status = Command::new(&protoc_exe)
        .arg("../protobuf/src/google/protobuf/compiler/plugin.proto")
        .arg(format!("--proto_path={}", "../protobuf/src/"))
        .arg("--include_imports")
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
    let mut options = CodegenOptions::default();
    options.root_file_name = Some("mod.rs".to_string());
    options.puroro_library_path = Some("crate::puroro_for_protobuf".to_string());
    let cgr =
        generate_output_file_protos(file_descriptor_set.file().into_iter(), &options).unwrap();
    let output_files = cgr.file();

    // Delete the all contents of the temp dir, so that we can output into clean dir.
    if temp_output_rust_path.is_dir() {
        remove_dir_all(&temp_output_rust_path).unwrap();
    }
    create_dir_all(&temp_output_rust_path).unwrap();

    // Output the File proto structs into the temp dir.
    for output_file in output_files {
        let file_path = temp_output_rust_path.join(output_file.name());
        create_dir_all(file_path.parent().unwrap()).unwrap();
        let mut file = File::create(&file_path).unwrap();
        write!(&mut file, "{}", output_file.content()).unwrap();
    }

    // If we survive (not panic!ed) until here, then the temp output dir should be well-formed.
    // Move it into the actual output directory.
    if output_rust_path.is_dir() {
        remove_dir_all(&output_rust_path).unwrap();
    }
    rename(&temp_output_rust_path, &output_rust_path).unwrap();
}
