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
use ::quote::{format_ident, quote, ToTokens, TokenStreamExt};
use ::syn::{parse_str, Ident};

pub struct MessageOpenStruct {
    name: Ident,
    fields: Vec<Field>,
}

struct Field {
    name: Ident,
}

impl MessageOpenStruct {
    pub fn try_new<'a>(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            name: parse_str(&convert_into_case(desc.name()?, Case::CamelCase))?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|field| {
                    Ok(Field {
                        name: parse_str(&convert_into_case(field.name()?, Case::LowerSnakeCase))?,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl ToTokens for MessageOpenStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = &self.fields;
        tokens.append_all(quote! {
            pub struct #name {
                #(#fields)*
            }
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        tokens.append_all(quote! {
            #name: String,
        })
    }
}
