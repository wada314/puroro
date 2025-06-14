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
use super::{ImplsGenerator, view_trait_blanket_impl_helper};
use crate::generator::{CodeGeneratorOptions, TrySwitch};
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, Ident, Item, Path, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketRefImpls {
    #[allow(unused)]
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketRefImpls {
    #[throws]
    fn generate<'a>(
        &self,
        trait_paths: &TrySwitch<Path>,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t: Ident = parse_str("T")?;
        let fields = fields.collect::<Vec<_>>();
        let view_trait_path = &trait_paths[false];
        let try_trait_path = &trait_paths[true];

        let view_methods = view_trait_blanket_impl_helper(
            fields.iter().copied(),
            |f| self.gen_get_method_body(f, &t, view_trait_path, false),
            |f| self.gen_has_method_body(f, &t, view_trait_path, false),
            false,
        )?;

        let try_methods = view_trait_blanket_impl_helper(
            fields.iter().copied(),
            |f| self.gen_get_method_body(f, &t, try_trait_path, true),
            |f| self.gen_has_method_body(f, &t, try_trait_path, true),
            true,
        )?;

        vec![
            parse2(quote! {
                impl<#t: #view_trait_path> #view_trait_path for &#t {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #view_trait_path> #view_trait_path for &mut #t {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #try_trait_path> #try_trait_path for &#t {
                    #(#try_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #try_trait_path> #try_trait_path for &mut #t {
                    #(#try_methods)*
                }
            })?,
        ]
    }
}

impl GenBlanketRefImpls {
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
        parse2(quote! {{ <#t as #trait_path>::#getter_name(self) }})?
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
        parse2(quote! {{ <#t as #trait_path>::#has_name(self) }})?
    }
}
