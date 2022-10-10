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

pub fn generate(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
    let input_files = request
        .proto_file()
        .into_iter()
        .map(|f| crate::codegen::restructure::File::new(f))
        .collect::<Vec<_>>();
    let resolver = DescriptorResolver::new(&input_files)?;

    let mut cgres: CodeGeneratorResponse = Default::default();
    *cgres.supported_features_mut() = Feature::FeatureProto3Optional as u64;

    let mut state = State::default();
    let root_module = Module::try_from_package(&Package::new(""), &resolver, &mut state)?;

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
        let filename = &module.rust_file_path;
        // Do render!
        let mut contents = module.render().unwrap();
        if let Some(new_contents) = format(&contents).ok() {
            contents = new_contents;
        } else {
            dbg!("failed to run rustfmt");
        }

        let mut output_file = <File as Default>::default();
        *output_file.name_mut() = filename.into();
        *output_file.content_mut() = contents.into();
        cgres.file_mut().push(output_file);
    }

    Ok(cgres)
}
