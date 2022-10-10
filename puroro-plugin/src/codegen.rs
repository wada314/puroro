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
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorSet;

pub fn generate(request: CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
    let mut response: CodeGeneratorResponse = Default::default();
    *response.supported_features_mut() = Feature::FeatureProto3Optional as u64;

    let mut file_set = FileDescriptorSet::default();
    file_set.file_mut().append(request.proto_file_mut());
    let root_module = get_root_module(&file_set)?;

    let modules = {
        let mut queue = vec![&root_module];
        let mut found_modules = Vec::new();
        while let Some(module) = queue.pop() {
            queue.extend(module.submodules.iter());
            found_modules.push(module);
        }
        found_modules
    };

    for module in modules {
        let file_name = &module.rust_file_path;
        // Do render!
        let unformatted_content = module.render().unwrap();
        let content = format(&unformatted_content)?;

        let mut output_file = <File as Default>::default();
        *output_file.name_mut() = file_name.into();
        *output_file.content_mut() = content.into();
        response.file_mut().push(output_file);
    }

    Ok(response)
}

pub fn generate_single_file(
    request: CodeGeneratorRequest,
    file_name: &str,
) -> Result<CodeGeneratorResponse> {
    let mut response: CodeGeneratorResponse = Default::default();
    *response.supported_features_mut() = Feature::FeatureProto3Optional as u64;

    let mut file_set = FileDescriptorSet::default();
    file_set.file_mut().append(request.proto_file_mut());
    let mut root_module = get_root_module(&file_set)?;
    root_module.output_all_in_one_file = true;

    // Do render!
    let unformatted_content = root_module.render().unwrap();
    let content = format(&unformatted_content)?;

    let mut output_file = <File as Default>::default();
    *output_file.name_mut() = file_name.into();
    *output_file.content_mut() = content.into();
    response.file_mut().push(output_file);

    Ok(response)
}

fn get_root_module(file_set: &FileDescriptorSet) -> Result<Module> {
    let input_files = file_set
        .file()
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
