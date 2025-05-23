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

use super::{impls_helper, Field, ImplsGenerator};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Ident, Item, Path, TypePath};

pub struct GenBlanketOptionImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketOptionImpls {
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let t_opt = self
            .options
            .option_type(&(TypePath { qself: None, path: t.clone().into() }.into()))?;
        let fields = fields.collect::<Vec<_>>();

        let view_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_get_method_body(f, &t, &view_trait_path),
            |f| self.gen_has_method_body(f, &t, &view_trait_path),
            false,
        )?;

        let try_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_try_get_method_body(f, &t, &try_view_trait_path),
            |f| self.gen_try_has_method_body(f, &t, &try_view_trait_path),
            true,
        )?;

        Ok(vec![
            parse2(quote! {
                impl<#t: #view_trait_path> #view_trait_path for #t_opt {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #try_view_trait_path> #try_view_trait_path for #t_opt {
                    #(#try_methods)*
                }
            })?,
        ])
    }
}

impl GenBlanketOptionImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_get_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Result<Block> {
        let signature = field.getter_signature();
        let getter_name = &signature.ident;
        let stmts = match field {
            Field::Repeated { .. } => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name)
                    .into_iter().flatten()
            },
            Field::Explicit { .. } | Field::Implicit { .. } => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name)
                    .unwrap_or_default()
            },
        };
        Ok(parse2(quote! {
            { #stmts }
        })?)
    }

    fn gen_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Result<Block> {
        let (Field::Implicit { has_method_signature, .. }
        | Field::Explicit { has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signature.ident;
        Ok(parse2(quote! {{
            self.as_ref()
                .map(<#t as #trait_path>::#has_name)
                .unwrap_or(false)
        }})?)
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
                self.as_ref().map(<#t as #trait_path>::#try_getter_name).transpose()
                .map(|iter_opt| iter_opt.into_iter().flatten())
            },
            Field::Explicit { .. } | Field::Implicit { .. } => quote! {
                self.as_ref().map(<#t as #trait_path>::#try_getter_name).transpose()
                .map(|opt| opt.unwrap_or_default())
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
                .map(<#t as #trait_path>::#try_has_name)
                .unwrap_or(#ok_false)
        } })?)
    }
}
