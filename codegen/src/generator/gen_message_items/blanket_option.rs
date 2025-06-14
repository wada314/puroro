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

use super::field::{Field, ScalarField};
use super::{ImplsGenerator, view_trait_blanket_impl_helper2};
use crate::generator::{CodeGeneratorOptions, TrySwitch};
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, Ident, Item, Path, Stmt, TypePath, parse::Parser, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketOptionImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketOptionImpls {
    #[throws]
    fn generate<'a>(
        &self,
        trait_paths: &TrySwitch<Path>,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t: Ident = parse_str("T")?;
        let t_opt = self
            .options
            .option_type(&(TypePath { qself: None, path: t.clone().into() }.into()));
        let fields = fields.collect::<Vec<_>>();
        let view_trait_path = &trait_paths[false];
        let try_trait_path = &trait_paths[true];

        let methods = view_trait_blanket_impl_helper2(
            fields.iter().copied(),
            |f, is_try| self.gen_get_method_body(f, &t, &trait_paths[is_try], is_try),
            |f, is_try| self.gen_has_method_body(f, &t, &trait_paths[is_try], is_try),
        )?;

        let view_methods = &methods[false];
        let try_methods = &methods[true];

        vec![
            parse2(quote! {
                impl<#t: #view_trait_path> #view_trait_path for #t_opt {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #try_trait_path> #try_trait_path for #t_opt {
                    #(#try_methods)*
                }
            })?,
        ]
    }
}

impl GenBlanketOptionImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    #[throws]
    fn gen_get_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
        is_try: bool,
    ) -> Block {
        let signature = field.trait_getter_signatures()[is_try].clone();
        let getter_name = &signature.ident;
        let stmts = Block::parse_within.parse2(match (field, is_try) {
            (Field::Repeated { .. }, true) => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name).transpose()
                .map(|iter_opt| iter_opt.into_iter().flatten())
            },
            (Field::Repeated { .. }, false) => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name)
                    .into_iter().flatten()
            },
            (Field::Explicit { .. } | Field::Implicit { .. }, true) => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name).transpose()
                .map(|opt| opt.unwrap_or_default())
            },
            (Field::Explicit { .. } | Field::Implicit { .. }, false) => quote! {
                self.as_ref().map(<#t as #trait_path>::#getter_name)
                    .unwrap_or_default()
            },
        })?;
        Block { stmts, brace_token: Default::default() }
    }

    #[throws]
    fn gen_has_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
        is_try: bool,
    ) -> Block {
        let (Field::Implicit(ScalarField { has_method_signatures, .. })
        | Field::Explicit(ScalarField { has_method_signatures, .. })) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signatures[is_try].ident;
        let stmts = Block::parse_within.parse2(if is_try {
            let ok = self.options.ok_path();
            quote! {
                #ok(self.as_ref()
                    .map(<#t as #trait_path>::#has_name)
                    .transpose()?
                    .unwrap_or_default())
            }
        } else {
            quote! {
                self.as_ref()
                    .map(<#t as #trait_path>::#has_name)
                    .unwrap_or(false)
            }
        })?;
        Block { stmts, brace_token: Default::default() }
    }
}
