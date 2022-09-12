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

#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(generic_associated_types)]

mod error;
mod generators;
mod utils;
mod wrappers;

use ::askama::Template as _;
use ::itertools::Itertools;
use ::puroro::Message;
use ::puroro_protobuf_compiled::google;
use ::puroro_protobuf_compiled::google::protobuf::compiler::code_generator_response::{
    Feature, File,
};
use ::puroro_protobuf_compiled::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
use ::std::collections::{HashMap, HashSet};
use ::std::env;
use ::std::io::{stdin, stdout, Read};
use ::std::process::Command;
use ::std::process::Stdio;

use error::{ErrorKind, GeneratorError};
type Result<T> = std::result::Result<T, GeneratorError>;

fn make_package_to_subpackages_map(
    files: &[google::protobuf::FileDescriptorProto],
) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for file in files {
        let package_string = file.package();
        let package_vec = package_string
            .split('.')
            .filter_map(|p| {
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
            .collect::<Vec<_>>();
        for i in 0..(package_vec.len()) {
            let cur_package = package_vec.iter().take(i).join(".");
            let subpackage = package_vec[i].clone();
            map.entry(cur_package)
                .or_insert(HashSet::new())
                .insert(subpackage);
        }
    }
    map
}

fn package_to_filename(package: &str) -> String {
    if package.is_empty() {
        "lib.rs".to_string()
    } else {
        package
            .split('.')
            .map(|f| utils::get_keyword_safe_ident(f))
            .join("/")
            + ".rs"
    }
}

fn format_rust_file(input: &str) -> Option<String> {
    use ::std::io::Write as _;
    if input.is_empty() {
        return None;
    }

    let rustfmt_exe = env::var("RUSTFMT").unwrap_or("rustfmt".to_string());
    let mut rustfmt = Command::new(&rustfmt_exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = rustfmt.stdin {
        //let input_string = input.to_string();
        //::std::thread::spawn(move || stdin.write_all(input_string.as_bytes()));
        stdin.write_all(input.as_bytes()).ok()?;
    }

    if let Some(ref mut stdout) = rustfmt.stdout {
        let mut out = String::new();
        stdout.read_to_string(&mut out).ok()?;
        if !out.is_empty() {
            return Some(out);
        }
    }

    return None;
}

fn main() -> Result<()> {
    let cgreq = CodeGeneratorRequest::from_bytes(&mut stdin().bytes()).unwrap();

    let wrapped_cgreq = wrappers::Context::try_from_proto(cgreq.clone())?;

    let mut cgres: CodeGeneratorResponse = Default::default();
    *cgres.supported_features_mut() = Feature::FeatureProto3Optional as u64;

    let package_to_subpackage_map = make_package_to_subpackages_map(cgreq.proto_file());
    let package_to_file_descriptor_map = wrapped_cgreq
        .input_files()
        .iter()
        .map(|file| {
            Ok((
                file.package().iter().join("."),
                generators::MessagesAndEnums::try_new(file)?,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    // merge 2 hashmaps, create a HashMap of OutputFile
    let mut output_file_contexts = HashMap::<String, generators::OutputFile>::new();
    for (package, subpackages) in package_to_subpackage_map {
        let mut v = subpackages.into_iter().collect_vec();
        v.sort();
        output_file_contexts
            .entry(package.clone())
            .or_insert_with(|| generators::OutputFile::new(&package))
            .subpackages = v;
    }
    for (package, file) in package_to_file_descriptor_map {
        output_file_contexts
            .entry(package.clone())
            .or_insert_with(|| generators::OutputFile::new(&package))
            .input_file = Some(file);
    }

    for output_contexts in output_file_contexts.values() {
        let filename = package_to_filename(&output_contexts.package);
        // Do render!
        let mut contents = output_contexts.render().unwrap();
        if let Some(new_contents) = format_rust_file(&contents) {
            contents = new_contents;
        } else {
            dbg!("failed to run rustfmt");
        }

        let mut output_file = <File as Default>::default();
        *output_file.name_mut() = filename.into();
        *output_file.content_mut() = contents.into();
        cgres.file_mut().push(output_file);
    }

    cgres.ser(&mut stdout())?;
    Ok(())
}
