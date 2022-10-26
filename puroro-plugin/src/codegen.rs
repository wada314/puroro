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

pub mod descriptor_resolver;
pub mod generators;
pub mod generators2;
pub mod restructure;
pub mod utils;

use crate::codegen::descriptor_resolver::DescriptorResolver;
use crate::codegen::generators::{Module, State};
use crate::codegen::utils::Package;
use crate::rustfmt::format;
use crate::Result;
use ::askama::Template as _;
use ::puroro_protobuf_compiled::google::protobuf::compiler::code_generator_response::{
    Feature, File,
};
use ::puroro_protobuf_compiled::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;

use ::prettyplease;
use ::syn;

#[derive(Default)]
pub struct Config {
    pub single_output_file: bool,
    pub root_file_name: Option<String>,
}

#[allow(unused)]
pub fn generate_response_from_request(
    request: CodeGeneratorRequest,
    config: &Config,
) -> Result<CodeGeneratorResponse> {
    let mut response: CodeGeneratorResponse = Default::default();
    *response.supported_features_mut() = Feature::FeatureProto3Optional as u64;

    let output_files = generate_output_files_from_file_descriptors(request.proto_file(), config)?;
    response.file_mut().extend(output_files);

    Ok(response)
}

pub fn generate_output_files_from_file_descriptors<'a>(
    files: impl IntoIterator<Item = &'a FileDescriptorProto>,
    config: &Config,
) -> Result<impl IntoIterator<Item = File>> {
    let mut root_module = get_root_module(files)?;
    if config.single_output_file {
        root_module.output_all_in_one_file = true;
    }
    if let Some(ref root_file_name) = &config.root_file_name {
        root_module.rust_file_path = root_file_name.clone();
    }

    let modules = {
        let mut queue = vec![&root_module];
        let mut found_modules = Vec::new();
        while let Some(module) = queue.pop() {
            queue.extend(module.submodules.iter());
            found_modules.push(module);
        }
        found_modules
    };

    let output_files = modules
        .into_iter()
        .map(|module| {
            let file_name = &module.rust_file_path;
            // Do render!
            let unformatted_content = module.render().unwrap();
            let content =
                format(&unformatted_content).unwrap_or_else(|_| unformatted_content.clone());

            let mut output_file = <File as Default>::default();
            *output_file.name_mut() = file_name.into();
            *output_file.content_mut() = content.into();
            Ok(output_file)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(output_files)
}

fn get_root_module<'a>(files: impl IntoIterator<Item = &'a FileDescriptorProto>) -> Result<Module> {
    let input_files = files
        .into_iter()
        .map(|f| crate::codegen::restructure::File::new(f))
        .collect::<Vec<_>>();
    let resolver = DescriptorResolver::new(&input_files)?;

    let mut state = State::default();
    Ok(Module::try_from_package(
        &Package::new(""),
        &resolver,
        &mut state,
    )?)
}

pub fn generate_output_files_from_file_descriptors2<'a>(
    files: impl IntoIterator<Item = &'a FileDescriptorProto>,
) -> Result<impl IntoIterator<Item = File>> {
    let input_files = files
        .into_iter()
        .map(|f| crate::codegen::restructure::File::new(f))
        .collect::<Vec<_>>();
    let resolver = DescriptorResolver::new(&input_files)?;
    let root_pc = resolver.package_contents_or_err(&Package::new(""))?;

    let (file_name, ts) = generators2::gen_module_from_package(&root_pc)?;

    let syn_file = syn::parse2::<syn::File>(ts).unwrap();
    let formatted = prettyplease::unparse(&syn_file);

    let mut output_file = File::default();
    *output_file.name_mut() = file_name;
    *output_file.content_mut() = formatted;

    Ok(vec![output_file])
}
