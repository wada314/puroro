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
use crate::generator::message_open_struct::MessageOpenStruct;
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::quote;
use ::syn::{parse2, parse_str, Ident, Type};

pub(crate) trait ProtoPathExt {
    fn into_rust_message_open_struct_path(&self, allocator: &Type) -> Result<Type>;
}

impl ProtoPathExt for ProtoPath {
    fn into_rust_message_open_struct_path(&self, allocator: &Type) -> Result<Type> {
        let modules = self
            .parent()
            .into_iter()
            .flat_map(|p| p.components())
            .map(|name| Ok(parse_str(&convert_into_case(name, Case::LowerSnakeCase))?))
            .collect::<Result<Vec<Ident>>>()?;
        let struct_name = MessageOpenStruct::rust_name_from_message_name(
            self.last_component()
                .ok_or_else(|| format!("Invalid message path: {:?}", self))?,
        )?;
        Ok(parse2(quote! {
            crate #(:: #modules)* :: #struct_name :: <#allocator>
        })?)
    }
}
