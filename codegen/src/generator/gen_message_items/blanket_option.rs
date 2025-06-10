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
use super::{ImplsGenerator, impls_helper};
use crate::generator::CodeGeneratorOptions;
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, Ident, Item, Path, TypePath, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketOptionImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketOptionImpls {
    #[throws]
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t: Ident = parse_str("T")?;
        let t_opt = self
            .options
            .option_type(&(TypePath { qself: None, path: t.clone().into() }.into()));
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

        vec![
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
        ]
    }
}

impl GenBlanketOptionImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    #[throws]
    fn gen_get_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let signature = field.getter_signatures().trait_getter.clone();
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
        parse2(quote! {
            { #stmts }
        })?
    }

    #[throws]
    fn gen_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let (Field::Implicit(ScalarField { has_method_signatures, .. })
        | Field::Explicit(ScalarField { has_method_signatures, .. })) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signatures.has_method.ident;
        parse2(quote! {{
            self.as_ref()
                .map(<#t as #trait_path>::#has_name)
                .unwrap_or(false)
        }})?
    }

    #[throws]
    fn gen_try_get_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let signature = field.getter_signatures().trait_try_getter.clone();
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
        parse2(quote! {
            { #stmts }
        })?
    }

    #[throws]
    fn gen_try_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let try_has_name = match field {
            Field::Implicit(ScalarField { has_method_signatures, .. })
            | Field::Explicit(ScalarField { has_method_signatures, .. }) => {
                &has_method_signatures.try_has_method.ident
            }
            _ => Err("this method is not supported for repeated fields".to_string())?,
        };
        let ok = self.options.ok_path();
        parse2(quote! { {
            #ok(self.as_ref()
                .map(<#t as #trait_path>::#try_has_name)
                .transpose()?
                .unwrap_or_default())
        } })?
    }
}
