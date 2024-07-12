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

pub mod r#enum;
pub mod message_trait;
pub mod untyped_message_impls;

use self::message_trait::MessageTrait;
use self::untyped_message_impls::UntypedMessageImpls;
use crate::descriptor::{FileDescriptor, RootContext};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::prettyplease::unparse;
use ::proc_macro2::TokenStream;
use ::puroro::google::protobuf::compiler::code_generator_response;
use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::puroro::Result as PResult;
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::collections::HashSet;
use ::std::collections::{BTreeSet, HashMap};

pub fn compile(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse<'static>> {
    let mut response = CodeGeneratorResponse::default();

    let descriptors: Vec<FileDescriptor> = request
        .proto_file()
        .map_ok(TryInto::try_into)
        .collect::<PResult<Result<Vec<_>>>>()??;

    let root_context: RootContext = descriptors.into();
    let mut out_files = GeneratedFileSet::new();

    let messages = root_context
        .files()
        .into_iter()
        .map(|fd| fd.all_messages())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for message in &messages {
        let file_path = if let Some(package) = message.full_path()?.parent() {
            package.to_rust_file_path()
        } else {
            "mod.rs".to_string()
        };
        let file = out_files.file_mut(file_path);
        file.add_source(message.file()?.name()?);

        let trait_item = MessageTrait::try_new(message)?.gen_message_trait()?;
        file.append(quote! { #trait_item });
        let untyped_message_impl =
            UntypedMessageImpls::try_new(message)?.gen_impl_message_trait()?;
        file.append(quote! { #untyped_message_impl });
    }

    let enums = root_context
        .files()
        .into_iter()
        .map(|fd| fd.all_enums())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for e in &enums {
        let file_path = if let Some(package) = e.full_path()?.parent() {
            package.to_rust_file_path()
        } else {
            "mod.rs".to_string()
        };
        let file = out_files.file_mut(file_path);
        file.add_source(e.file()?.name()?);

        let enum_ = r#enum::Enum::try_new(e)?.rust_items()?;
        file.append(quote! { #(#enum_)* });
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

    /// The body part of the file, except:
    /// - The header comments
    /// - The submodule definitions (like "pub mod foo;")
    body: Vec<TokenStream>,
}
impl GeneratedFile {
    fn new(full_path: impl Into<String>) -> Self {
        Self {
            full_path: full_path.into(),
            sources: BTreeSet::new(),
            submodules: BTreeSet::new(),
            body: Vec::new(),
        }
    }
    fn full_path(&self) -> &str {
        &self.full_path
    }
    fn append(&mut self, source: TokenStream) {
        self.body.push(source);
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
            .iter()
            .map(|s| {
                let doc = format!("   {s}");
                quote! {
                    #![doc=#doc]
                }
            })
            .collect::<Vec<_>>();
        let submodule_decls = from
            .submodules
            .iter()
            .map(|s| {
                let id = format_ident!("{}", s);
                quote! { pub mod #id; }
            })
            .collect::<Vec<_>>();
        let puroro_root = if from.full_path() == "mod.rs" {
            quote! { pub(crate) mod _puroro_root { pub(crate) use super::*; } }
        } else {
            quote! { pub(crate) mod _puroro_root { pub(crate) use super::super::_puroro_root::*; } }
        };
        let body = from.body;
        let content = quote! {
            #![doc=" THIS FILE IS A GENERATED FILE! DO NOT EDIT!"]
            #![doc=" Source(s):"]
            #(#source_list)*

            #(#submodule_decls)*
            #puroro_root
            #(#body)*
        };
        let syn_file: ::syn::File = syn::parse2(content)?;
        file.set_content(&unparse(&syn_file))?;
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
            self.files
                .entry(format!("{parent}.rs"))
                .or_insert_with(|| GeneratedFile::new(format!("{parent}.rs")))
                .add_submodule(submodule);
            module_path = parent;
        }
        self.files
            .entry("mod.rs".to_string())
            .or_insert_with(|| GeneratedFile::new("mod.rs"))
            .add_submodule(module_path);

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

// List of the Rust "strict" and "reserved" keywords except not-"r#" prefixed ones.
const KEYWORDS_LIST: &[&str] = &[
    "abstract", "alignof", "as", "become", "box", "break", "const", "continue",
    /*"crate",*/ "do", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv",
    "proc", "pub", "pure", "ref", "return", /*"Self",*/ /*"self",*/ "sizeof", "static",
    "struct", /*"super",*/ "trait", "true", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while", "yield",
];
const KEYWORDS: LazyCell<HashSet<&'static str>> =
    LazyCell::new(|| KEYWORDS_LIST.iter().copied().collect());
const NOT_RAWNIZEABLE_KEYWORDS: LazyCell<HashSet<&'static str>> =
    LazyCell::new(|| ["crate", "Self", "self", "super"].iter().copied().collect());

pub fn avoid_reserved_keywords(s: &str) -> Cow<str> {
    if KEYWORDS.contains(&s) {
        return Cow::Owned(format!("r#{}", s));
    }
    if NOT_RAWNIZEABLE_KEYWORDS.contains(&s) {
        return Cow::Owned(format!("_{}", s));
    }
    return Cow::Borrowed(s);
}
