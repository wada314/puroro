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

pub mod message;

use crate::descriptor::{FileDescriptor, RootContext};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro::google::protobuf::compiler::code_generator_response;
use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro::Result as PResult;
use ::std::collections::{BTreeSet, HashMap};

pub fn compile(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse<'static>> {
    let mut response = CodeGeneratorResponse::default();

    let descriptors: Vec<FileDescriptor> = request
        .proto_file()
        .map_ok(TryInto::try_into)
        .collect::<PResult<Result<Vec<_>>>>()??;

    let root_context: RootContext = descriptors.into();
    let mut out_files = GeneratedFileSet::new();

    for fd in root_context.files() {
        let file_path = if let Some(package) = fd.package()? {
            package.to_rust_file_path()
        } else {
            "mod.rs".to_string()
        };
        let file = out_files.file_mut(file_path);
        file.append("pub fn yeah() { }");
        file.add_source(fd.name()?);
    }

    for file in out_files {
        response.push_file(file.try_into()?)?;
    }

    Ok(response)
}

struct GeneratedFile {
    full_path: String,
    sources: BTreeSet<String>,
    submodules: BTreeSet<String>,
    content: String,
}
impl GeneratedFile {
    fn new(full_path: impl Into<String>) -> Self {
        Self {
            full_path: full_path.into(),
            sources: BTreeSet::new(),
            submodules: BTreeSet::new(),
            content: String::new(),
        }
    }
    fn full_path(&self) -> &str {
        &self.full_path
    }
    fn append(&mut self, source: impl AsRef<str>) {
        self.content.push_str(source.as_ref());
    }
    fn add_source(&mut self, source: impl Into<String>) {
        self.sources.insert(source.into());
    }
    fn add_submodule(&mut self, submodule: impl Into<String>) {
        self.submodules.insert(submodule.into());
    }
}
impl TryFrom<GeneratedFile> for code_generator_response::File<'_> {
    type Error = ErrorKind;
    fn try_from(from: GeneratedFile) -> Result<Self> {
        let mut file = code_generator_response::File::default();
        file.set_name(from.full_path())?;
        let source_list = from
            .sources
            .into_iter()
            .map(|s| format!("//   {}\n", s))
            .join("");
        let submodule_decls = from
            .submodules
            .into_iter()
            .map(|s| format!("pub mod {};\n", s))
            .join("");
        file.set_content(&format!(
            "\
            // THIS FILE IS A GENERATED FILE! DO NOT EDIT!\n\
            // Source(s):\n\
            {source_list}\
            \n\
            {submodule_decls}\
            \n\
            {}\n",
            from.content
        ))?;
        Ok(file)
    }
}

struct GeneratedFileSet {
    files: HashMap<String, GeneratedFile>,
}
impl GeneratedFileSet {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    // Get file by full path.
    // Also make sure that the parent modules are created.
    fn file_mut(&mut self, full_path: impl Into<String>) -> &mut GeneratedFile {
        let full_path: String = full_path.into();

        // if the input file path is "mod.rs", then we skip the parent module creation.
        if full_path == "mod.rs" {
            return self
                .files
                .entry("mod.rs".to_string())
                .or_insert_with(|| GeneratedFile::new("mod.rs"));
        }

        // create parent modules.
        let mut module_path = full_path.trim_end_matches(".rs");
        while let Some((parent, submodule)) = module_path.rsplit_once('/') {
            self.files.entry(format!("{parent}.rs")).or_insert_with(|| {
                let mut file = GeneratedFile::new(format!("{parent}.rs"));
                file.add_submodule(submodule);
                file
            });
            module_path = parent;
        }
        self.files.entry("mod.rs".to_string()).or_insert_with(|| {
            let mut file = GeneratedFile::new("mod.rs");
            file.add_submodule(module_path);
            file
        });

        // return the target file.
        self.files
            .entry(full_path.clone())
            .or_insert_with(|| GeneratedFile::new(full_path))
    }
}
impl IntoIterator for GeneratedFileSet {
    type Item = GeneratedFile;
    type IntoIter = ::std::collections::hash_map::IntoValues<String, GeneratedFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_values()
    }
}
