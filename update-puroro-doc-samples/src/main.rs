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
    // Check if the current directory is puroro's workspace directory
    // (not the crate puroro's directory)."
    let output_rust_path = ["puroro", "src", "doc_samples"]
        .into_iter()
        .collect::<PathBuf>();
    if !output_rust_path.exists() || !output_rust_path.is_dir() {
        eprintln!(
            "Please run this command at the puroro's workspace dir, which contains the
        directories like 'puroro' and 'tests'."
        );
        panic!();
    }

    let temp_dir = env::temp_dir().join("update-puroro-doc-samples");
    if !temp_dir.exists() {
        create_dir_all(&temp_dir).unwrap();
    }
    let temp_output_rust_path = temp_dir.join("generated-rust");
    let file_descriptor_set_file_path = temp_dir.join("file_descriptor_set.pb");

    // Run protoc command, output a temporal file which contains the encoded FileDescriptorSet.
    let protoc_exe = protoc_bin_vendored::protoc_bin_path().unwrap();
    let proto_path = ["update-puroro-doc-samples", "protos"]
        .into_iter()
        .collect::<PathBuf>();
    let plugin_proto_file =
        proto_path.join(["lib-rs-samples.proto"].into_iter().collect::<PathBuf>());

    let protoc_status = Command::new(&protoc_exe)
        .arg(plugin_proto_file.as_os_str())
        .arg(format!("--proto_path={}", proto_path.to_string_lossy()))
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
        eprintln!("Failed to run `protoc` command.");
        panic!("Failed to run `protoc` command.")
    }

    // Decode the FileDescriptorSet output by the above code.
    let fds_file = File::open(&file_descriptor_set_file_path).unwrap();
    let file_descriptor_set = FileDescriptorSet::from_bytes_iter(fds_file.bytes()).unwrap();

    // Generate the code, returned by File proto structs.
    let mut options = CodegenOptions::default();
    options.root_file_name = Some("mod.rs".to_string());
    options.puroro_library_path = Some("crate".to_string());
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
