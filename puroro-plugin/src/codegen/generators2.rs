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

use crate::codegen::utils::StrExt;
use crate::Result;
use ::itertools::Itertools;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn;

use crate::codegen::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::codegen::restructure::Message;

pub fn gen_module_from_package<'a>(pc: &'a PackageContents<'a>) -> Result<TokenStream> {
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
                pub mod #ident;
            }
        })
        .collect_vec();

    Ok(quote! {
        #(#submodules)*
        #(#structs)*
    })
}

pub fn gen_struct_from_message(m: &Message) -> Result<TokenStream> {
    let ident = format_ident!("{}", m.name().to_upper_snake_case().escape_rust_keywords());
    Ok(quote! {
        pub struct #ident {
            fields: (),
        }
    })
}
