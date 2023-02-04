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

mod data;
mod gen;
mod util;

use self::data::{Package, PackageOrMessage};
use self::gen::PackageOrMessageExt as _;

use crate::Result;
use ::itertools::Itertools;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::compiler::code_generator_response::File;
use ::puroro_protobuf_compiled::google::protobuf::compiler::CodeGeneratorResponse;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::quote::quote;
use ::std::iter;
use ::std::rc::Rc;

/// Configurates the generated code.
pub struct CodegenOptions {
    /// Replaces the generated root .rs file name. If not specified, "lib.rs" is used.
    /// Example: "my_library.rs"
    pub root_file_name: Option<String>,

    /// If specified, all generated rust files except the root files are prefixed
    /// by the value + "/".
    /// This field will be combined with the `root_file_name` field to generate a
    /// module-level output code (instead of crate-level output code).
    ///
    /// # Example
    ///
    /// Imagine if you have a single input proto file with
    /// `package = my_package;`.
    ///
    /// If you don't specify this field (neither `root_file_name`), then
    /// `lib.rs` and `my_package.rs` will be generated.
    ///
    /// If you set `"my_module"` to this field, and set the `root_file_name`
    /// field as `"my_module.rs"`, then `my_module.rs` and
    /// `my_module/my_package.rs` files will be generated.
    pub subdirectory_except_for_root_file: Option<String>,
}

pub fn generate_file_names_and_tokens<'a>(
    files: impl Iterator<Item = &'a FileDescriptorProto>,
) -> Result<impl IntoIterator<Item = (String, TokenStream)>> {
    let root_package = Package::new_root(files);

    let file_generating_items =
        iter::once(Ok(Rc::clone(&root_package) as Rc<dyn PackageOrMessage>))
            .chain(
                // Packages except root
                root_package
                    .all_child_packages()?
                    .into_iter()
                    .map(|p| Ok(p as Rc<dyn PackageOrMessage>)),
            )
            .chain(
                root_package
                    .all_child_messages()?
                    .into_iter()
                    .filter_map(|m| {
                        // messages which contains subitems
                        match m.should_generate_module_file() {
                            Ok(true) => Some(Ok(m as Rc<dyn PackageOrMessage>)),
                            Ok(false) => None,
                            Err(e) => Some(Err(e)),
                        }
                    }),
            );
    file_generating_items
        .map_ok(|item| {
            let file_name = item.module_file_path()?.to_string();
            let file_content = item.gen_module_file()?;
            Ok((file_name, quote! { #file_content }))
        })
        .map(|rr| rr.flatten())
        .collect::<Result<Vec<_>>>()
}

pub fn generate_output_file_protos<'a>(
    files: impl Iterator<Item = &'a FileDescriptorProto>,
) -> Result<CodeGeneratorResponse> {
    let mut cgr = CodeGeneratorResponse::default();
    *cgr.file_mut() = generate_file_names_and_tokens(files)?
        .into_iter()
        .map(|(file_name, ts)| {
            let formatted = if let Ok(syn_file) = syn::parse2::<syn::File>(ts.clone()) {
                prettyplease::unparse(&syn_file)
            } else {
                // Parse failed route. Print the output file for debugging.
                eprintln!("Generated code error!");
                format!("{}", ts)
            };
            let mut output_file = File::default();
            *output_file.name_mut() = file_name;
            *output_file.content_mut() = formatted;
            Ok(output_file)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(cgr)
}

pub fn generate_tokens_for_inline<'a>(
    files: impl Iterator<Item = &'a FileDescriptorProto>,
) -> Result<TokenStream> {
    let root_package = Package::new_root(files);
    root_package.gen_inline_code()
}
