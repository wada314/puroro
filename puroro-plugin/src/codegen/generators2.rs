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

use crate::codegen::descriptor_resolver::PackageContents;
use crate::codegen::restructure::Message;
use crate::codegen::utils::StrExt;
use crate::Result;
use ::itertools::Itertools;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};

pub fn gen_module_from_package<'a>(pc: &'a PackageContents<'a>) -> Result<(String, TokenStream)> {
    let messages = pc
        .input_files
        .iter()
        .flat_map(|f| f.messages().into_iter())
        .collect_vec();

    let structs = messages
        .iter()
        .map(|m| gen_struct_from_message(m))
        .collect::<Result<Vec<_>>>()?;

    let submodules = pc
        .subpackages
        .iter()
        .map(|sp| {
            let ident = format_ident!("{}", sp.to_lower_snake_case().escape_rust_keywords());
            quote! {
                // pub mod #ident;
            }
        })
        .collect_vec();

    let module_header = if let Some(package_name) = &pc.package_name {
        let comment = format!("Generated from package \"{}\"", package_name);
        quote! {
            #![doc = #comment]
            pub mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub mod _puroro {
                pub use ::puroro::*;
            }
        }
    } else {
        quote! {
            //! "Generated from root package"
            #![feature(generic_associated_types)]
            /// re-export puroro.
            pub use ::puroro;
            /// re-export the primitive types in puroro namespace.
            /// by using the "*", it can be hidden by the same typename explicitly defined in this file.
            pub use ::puroro::*;
            pub mod _puroro_root {
                pub use super::*;
            }
            pub mod _puroro {
                pub use ::puroro::*;
            }
        }
    };

    let rust_file_path = if pc.package_name.is_none() {
        "lib.rs".to_string()
    } else {
        pc.full_package
            .full_package_path()
            .split('.')
            .map(|s| s.to_lower_snake_case().into_owned())
            .join("/")
            + ".rs"
    };

    let output_tokens = quote! {
        #module_header

        #(#submodules)*
        #(#structs)*
    };

    Ok((rust_file_path, output_tokens))
}

pub fn gen_struct_from_message<'a>(m: &'a Message<'a>) -> Result<TokenStream> {
    let ident = format_ident!("{}", m.name().to_camel_case().escape_rust_keywords());

    let struct_fields = m
        .fields()
        .iter()
        .map(|f| {
            let ident_struct_field =
                format_ident!("{}", f.name().to_lower_snake_case().escape_rust_keywords());
            Ok(quote! {
                #ident_struct_field: (),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        pub struct #ident {
            #(#struct_fields)*
        }
    })
}
