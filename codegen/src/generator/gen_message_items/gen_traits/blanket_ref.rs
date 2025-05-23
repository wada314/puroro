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

use super::{blanket_impls_helper, BlanketImplsGenerator, Field};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Ident, Item, Path};

pub struct GenBlanketRefImpls {
    #[allow(unused)]
    options: Rc<CodeGeneratorOptions>,
}

impl BlanketImplsGenerator for GenBlanketRefImpls {
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let fields = fields.collect::<Vec<_>>();

        let view_methods = blanket_impls_helper(
            fields.iter().copied(),
            |f| self.gen_get_method_body(f, &t, &view_trait_path),
            |f| self.gen_has_method_body(f, &t, &view_trait_path),
            false,
        )?;

        let try_methods = blanket_impls_helper(
            fields.iter().copied(),
            |f| self.gen_try_get_method_body(f, &t, &try_view_trait_path),
            |f| self.gen_try_has_method_body(f, &t, &try_view_trait_path),
            true,
        )?;

        Ok(vec![
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
                impl<#t: #try_view_trait_path> #try_view_trait_path for &#t {
                    #(#try_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #try_view_trait_path> #try_view_trait_path for &mut #t {
                    #(#try_methods)*
                }
            })?,
        ])
    }
}

impl GenBlanketRefImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_get_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Result<Block> {
        let signature = field.getter_signature();
        let getter_name = &signature.ident;
        Ok(parse2(
            quote! {{ <#t as #trait_path>::#getter_name(self) }},
        )?)
    }

    fn gen_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Result<Block> {
        let Some(signature) = field.has_method_signature_if_non_repeated() else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &signature.ident;
        Ok(parse2(quote! {{ <#t as #trait_path>::#has_name(self) }})?)
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let signature = field.try_getter_signature();
        let try_getter_name = &signature.ident;
        Ok(parse2(
            quote! {{ <#t as #trait_path>::#try_getter_name(self) }},
        )?)
    }

    fn gen_try_has_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let Some(signature) = field.try_has_method_signature_if_non_repeated() else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let try_has_name = &signature.ident;
        Ok(parse2(
            quote! {{ <#t as #trait_path>::#try_has_name(self) }},
        )?)
    }
}
