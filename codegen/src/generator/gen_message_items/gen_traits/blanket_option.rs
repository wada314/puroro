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

use super::{BlanketImplsGenerator, Field2, FieldPresense};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
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
        fields: impl Iterator<Item = &'a Field2>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let t_opt = self.options.option_type(
            &(TypePath {
                qself: None,
                path: t.clone().into(),
            }
            .into()),
        )?;

        let methods = fields
            .map(|f| {
                let try_getter: ImplItemFn = {
                    let signature = &f.try_getter_signature;
                    let body = self.gen_try_get_method_body(f, &t, &trait_path)?;
                    parse2(quote! { #signature #body })?
                };
                let try_has_method: Option<ImplItemFn> = {
                    let signature = &f.try_has_method_signature;
                    let body = self.gen_try_has_method_body(f, &t, &trait_path)?;
                    if let (Some(signature), Some(body)) = (signature, body) {
                        Some(parse2(quote! { #signature #body })?)
                    } else {
                        None
                    }
                };
                Ok(once(try_getter).chain(try_has_method.into_iter()))
            })
            .map(|r| match r {
                Ok(it) => Either::Left(it.map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

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
        field: &Field2,
        blanket_type_ident: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let try_getter_name = &field.try_getter_name;
        let stmts = match field.presense {
            FieldPresense::Repeated => quote! {
                self.as_ref().map(<#blanket_type_ident as #trait_path>::#try_getter_name).transpose()
                .map(|iter_opt| iter_opt.into_iter().flatten())
            },
            FieldPresense::Explicit | FieldPresense::Implicit => quote! {
                self.as_ref().map(<#blanket_type_ident as #trait_path>::#try_getter_name).transpose()
                .map(|opt| opt.unwrap_or_default())
            },
        };
        Ok(parse2(quote! {
            { #stmts }
        })?)
    }

    fn gen_try_has_method_body(
        &self,
        field: &Field2,
        blanket_type_ident: &Ident,
        trait_path: &Path,
    ) -> Result<Option<Block>> {
        if field.presense == FieldPresense::Repeated {
            return Ok(None);
        }
        let Some(try_has_name) = &field.try_has_method_name else {
            return Ok(None);
        };
        Ok(Some(parse2(quote! {
            {
                self.as_ref().map(<#blanket_type_ident as #trait_path>::#try_has_name)
                    .unwrap_or(Ok(false))
            }
        })?))
    }
}
