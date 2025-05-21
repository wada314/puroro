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
use crate::{Result, ResultExt};
use ::puroro::Either;
use ::quote::quote;
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Ident, ImplItemFn, Item, Path, TypePath};

pub struct GenBlanketOptionImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl BlanketImplsGenerator for GenBlanketOptionImpls {
    fn generate<'a>(
        &self,
        trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let t_opt = self.options.option_type(
            &(TypePath {
                qself: None,
                path: t.clone().into(),
            }
            .into()),
        )?;

        let methods = blanket_impls_helper(
            fields,
            |f| self.gen_try_get_method_body(f, &t, &trait_path),
            |f| self.gen_try_has_method_body(f, &t, &trait_path),
        )?;

        Ok(vec![parse2(quote! {
            impl<#t: #trait_path> #trait_path for #t_opt {
                #(#methods)*
            }
        })?])
    }
}

impl GenBlanketOptionImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let (try_getter_name, _) = field.try_getter_name_and_signature();
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
        let Some((try_has_name, _)) = field.try_has_method_name_and_signature_if_non_repeated()
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let ok_false = self.options.ok_value(&parse2(quote! { false })?)?;
        Ok(parse2(quote! { {
            self.as_ref()
                .map(<#t as #trait_path>::#try_has_name)
                .unwrap_or(#ok_false)
        } })?)
    }
}
