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

pub mod gen_enum_items;
pub mod gen_message_items;
pub mod module;

use crate::descriptor::RootContext;
use crate::{ErrorKind, Result};
use ::prettyplease::unparse;
use ::proc_macro2::TokenStream;
use ::puroro::google::protobuf::compiler::code_generator_response::{self, Feature};
use ::puroro::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::collections::HashSet;
use ::std::collections::{BTreeSet, HashMap};
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Ident, ItemUse, Path, Type};

pub fn compile(request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
    let mut response = CodeGeneratorResponse::default();
    response.set_supported_features(Into::<i32>::into(Feature::FeatureProto3Optional) as u64)?;

    let options = Rc::new({
        let mut options = CodeGeneratorOptions::default();
        options.strict_type_path = false;
        options.allow_import_common_types = true;
        options
    });

    let root_context: RootContext = request.proto_file().cloned().into();
    let mut out_files = GeneratedFileSet::new(Rc::clone(&options));

    let messages = root_context
        .files()
        .flat_map(|fd| fd.all_messages())
        .collect::<Vec<_>>();
    for message in &messages {
        let file_path = if let Some(package) = message.full_path().parent() {
            package.to_rust_file_path()
        } else {
            "mod.rs".to_string()
        };
        let file = out_files.file_mut(file_path);
        file.add_source(message.file().name());

        let trait_items =
            gen_message_items::gen_message_trait::GenTrait::try_new(message, Rc::clone(&options))?
                .gen_items()?;
        file.append(quote! { #(#trait_items)* });
        let untyped_message_impl =
            gen_message_items::gen_dynamic_message_impl::GenDynamicMessageImpls::try_new(
                message,
                Rc::clone(&options),
            )?
            .gen_impl_message_trait()?;
        file.append(quote! { #untyped_message_impl });
    }

    let enums = root_context
        .files()
        .flat_map(|fd| fd.all_enums())
        .collect::<Vec<_>>();
    for e in &enums {
        let file_path = if let Some(package) = e.full_path().parent() {
            package.to_rust_file_path()
        } else {
            "mod.rs".to_string()
        };
        let file = out_files.file_mut(file_path);
        file.add_source(e.file().name());

        let enum_ = gen_enum_items::GenEnumItems::try_new(e, Rc::clone(&options))?.rust_items()?;
        file.append(quote! { #(#enum_)* });
    }

    for file in out_files {
        response.push_file(file.try_into()?)?;
    }

    Ok(response)
}

#[derive(Clone)]
pub struct CodeGeneratorOptions {
    /// Should the generated code's type name be fully-qualified type name?
    /// e.g. should we just use `i32` or `::std::primitive::i32` ?
    /// Default to true.
    pub strict_type_path: bool,

    /// If true, in the generated code modules, we `use` the common types which
    /// are not included in the prelude: e.g. `::std::ops::Deref`.
    pub allow_import_common_types: bool,
}
impl Default for CodeGeneratorOptions {
    fn default() -> Self {
        Self {
            strict_type_path: true,
            allow_import_common_types: false,
        }
    }
}
impl CodeGeneratorOptions {
    pub fn primitive_type(&self, ty: &str) -> Result<Type> {
        let ident: Ident = parse_str(ty)?;
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::primitive::#ident }
        } else {
            quote! { #ident }
        })?)
    }
    pub fn imports(&self) -> Result<Vec<ItemUse>> {
        if self.allow_import_common_types {
            Ok(vec![
                parse2(quote! { #[allow(unused)] use ::std::ops::Deref; })?,
                parse2(quote! { #[allow(unused)] use ::std::ops::DerefMut; })?,
            ])
        } else {
            Ok(vec![])
        }
    }
    pub fn clone_trait(&self) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::clone::Clone }
        } else {
            quote! { Clone }
        })?)
    }
    pub fn vec_type(&self, elem_type: &Type, alloc: Option<&Type>) -> Result<Type> {
        let generic_params = if let Some(alloc) = alloc {
            quote! { #elem_type, #alloc }
        } else {
            quote! { #elem_type }
        };
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::vec::Vec<#generic_params> }
        } else {
            quote! { Vec<#generic_params> }
        })?)
    }
    pub fn option_type(&self, elem_type: &Type) -> Result<Type> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::option::Option<#elem_type> }
        } else {
            quote! { Option<#elem_type> }
        })?)
    }
    pub fn iter_trait(&self, elem_type: &Type) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::iter::Iterator<Item=#elem_type> }
        } else {
            quote! { Iterator<Item=#elem_type> }
        })?)
    }
    pub fn deref_mut_trait(&self, target: &Type) -> Result<Path> {
        Ok(parse2(
            if !self.strict_type_path && self.allow_import_common_types {
                quote! { DerefMut<Target=#target> }
            } else {
                quote! { ::std::ops::DerefMut<Target=#target> }
            },
        )?)
    }
    pub fn path_in_self_module(&self, path: &Path) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { self::#path }
        } else {
            quote! { #path }
        })?)
    }
}

struct GeneratedFile {
    full_path: String,
    sources: BTreeSet<String>,
    submodules: BTreeSet<String>,

    options: Rc<CodeGeneratorOptions>,

    /// The body part of the file, except:
    /// - The header comments
    /// - The submodule definitions (like "pub mod foo;")
    body: Vec<TokenStream>,
}
impl GeneratedFile {
    fn new(full_path: impl Into<String>, options: Rc<CodeGeneratorOptions>) -> Self {
        Self {
            full_path: full_path.into(),
            sources: BTreeSet::new(),
            submodules: BTreeSet::new(),
            options,
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
impl TryFrom<GeneratedFile> for code_generator_response::File {
    type Error = ErrorKind;
    fn try_from(from: GeneratedFile) -> Result<Self> {
        let is_root_file = from.full_path() == "mod.rs";
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
        let imports = from.options.imports()?;
        let puroro_root = if is_root_file {
            quote! {
                #[allow(unused)]
                pub(crate) mod _puroro_root {
                    #[allow(unused)]
                    pub(crate) use super::*;
                }
            }
        } else {
            quote! {
                #[allow(unused)]
                pub(crate) mod _puroro_root {
                    #[allow(unused)]
                    pub(crate) use super::super::_puroro_root::*;
                }
            }
        };
        let body = from.body;
        let content = quote! {
            #![doc=" THIS FILE IS A GENERATED FILE! DO NOT EDIT!"]
            #![doc=" Source(s):"]
            #(#source_list)*

            #(#submodule_decls)*
            #(#imports)*
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
    options: Rc<CodeGeneratorOptions>,
}
impl GeneratedFileSet {
    fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self {
            files: HashMap::new(),
            options,
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
                .or_insert_with(|| GeneratedFile::new("mod.rs", Rc::clone(&self.options)));
        }

        // create parent modules.
        let mut module_path = full_path.trim_end_matches(".rs");
        while let Some((parent, submodule)) = module_path.rsplit_once('/') {
            self.files
                .entry(format!("{parent}.rs"))
                .or_insert_with(|| {
                    GeneratedFile::new(format!("{parent}.rs"), Rc::clone(&self.options))
                })
                .add_submodule(submodule);
            module_path = parent;
        }
        self.files
            .entry("mod.rs".to_string())
            .or_insert_with(|| GeneratedFile::new("mod.rs", Rc::clone(&self.options)))
            .add_submodule(module_path);

        // return the target file.
        self.files
            .entry(full_path.clone())
            .or_insert_with(|| GeneratedFile::new(full_path, Rc::clone(&self.options)))
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

pub fn to_ident(s: &str) -> Ident {
    format_ident!("{}", avoid_reserved_keywords(s))
}
pub fn to_ident_without_keyword_check(s: &str) -> Ident {
    format_ident!("{}", s)
}
