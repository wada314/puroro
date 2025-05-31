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

use super::impls_helper;
use crate::generator::gen_message_items::gen_traits::{Field, ImplsGenerator};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::quote::quote;
use ::syn::{parse2, parse_str, Block, Ident, Item, Path};
use std::rc::Rc;

pub struct GenBlanketResultImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketResultImpls {
    fn generate<'a>(
        &self,
        _view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let t_expr = parse2(quote! { #t })?;
        let result_type = self.options.result_type(&t_expr)?;
        let methods = impls_helper(
            fields,
            |f| self.gen_try_get_method_body(f, &t, try_view_trait_path),
            |f| self.gen_try_has_method_body(f, &t, try_view_trait_path),
            true,
        )?;

        Ok(vec![parse2(quote! {
            impl<T: #try_view_trait_path> #try_view_trait_path for #result_type {
                #(#methods)*
            }
        })?])
    }
}

impl GenBlanketResultImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let signature = field.try_getter_signature();
        let try_getter_name = &signature.ident;
        let stmts = match field {
            Field::Repeated { .. } => quote! {
                self.as_ref().and_then(<#t as #trait_path>::#try_getter_name)
            },
            Field::Explicit { .. } | Field::Implicit { .. } => quote! {
                self.as_ref().and_then(<#t as #trait_path>::#try_getter_name)
            },
        };
        Ok(parse2(quote! {
            { #stmts }
        })?)
    }

    fn gen_try_has_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let (Field::Implicit { try_has_method_signature, .. }
        | Field::Explicit { try_has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let try_has_name = &try_has_method_signature.ident;
        let ok_false = self.options.ok_value(&parse2(quote! { false })?)?;
        Ok(parse2(quote! { {
            self.as_ref()
                .and_then(<#t as #trait_path>::#try_has_name)
                .unwrap_or(#ok_false)
        } })?)
    }
}
