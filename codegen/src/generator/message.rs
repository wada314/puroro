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

use crate::cases::{convert_into_case, Case};
use crate::descriptor::{DescriptorWithContext, FieldDescriptorWithContext};
use crate::Result;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};

pub fn generate_open_struct_from_message<'a>(
    desc: &'a DescriptorWithContext<'a>,
) -> Result<TokenStream> {
    let struct_name = format_ident!("{}", convert_into_case(desc.name()?, Case::CamelCase));
    let field_names = desc
        .non_oneof_fields()?
        .into_iter()
        .map(|field| {
            let name = convert_into_case(field.name()?, Case::LowerSnakeCase);
            Ok(format_ident!("{name}"))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! {
        pub struct #struct_name {
            #(#field_names: String,)*
        }
    })
}

fn generate_open_struct_field_type(field: &FieldDescriptorWithContext) -> Result<TokenStream> {
    todo!()
}
