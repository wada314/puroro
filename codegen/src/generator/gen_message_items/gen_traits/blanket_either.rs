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

use super::{BlanketImplsGenerator, Field, FieldPresense};
use crate::descriptor::FieldType;
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::puroro::Either;
use ::quote::quote;
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Expr, Ident, ImplItemFn, Item, Path};

pub struct GenBlanketEitherImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl BlanketImplsGenerator for GenBlanketEitherImpls {
    fn generate<'a>(
        &self,
        trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t1: Ident = parse_str("T")?;
        let t2: Ident = parse_str("U")?;

        let methods = fields
            .map(|f| {
                let try_getter: ImplItemFn = {
                    let signature = &f.try_getter_signature;
                    let body = self.gen_try_get_method_body(f, &t1, &t2, &trait_path)?;
                    parse2(quote! { #signature #body })?
                };
                let try_has_method: Option<ImplItemFn> = {
                    let signature = &f.try_has_method_signature;
                    let body = self.gen_try_has_method_body(f, &t1, &t2, &trait_path)?;
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
            impl<#t1: #trait_path, #t2: #trait_path> #trait_path for ::puroro::Either<#t1, #t2> {
                #(#methods)*
            }
        })?])
    }
}

impl GenBlanketEitherImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Block> {
        let try_getter_name = &field.try_getter_name;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_getter_name,
                <#t2 as #trait_path>::#try_getter_name)?
        })?;
        let expr = self.options.ok_value(&parse2::<Expr>(
            match (field.presense, &field.scalar_proto_type) {
                (FieldPresense::Repeated, &FieldType::Message(_)) => quote! {
                    #mapped_either.into_iter_either().map(|either_res| either_res.factor_err())
                },
                (FieldPresense::Repeated, _) => quote! {
                    #mapped_either.into_iter_chained()
                },
                (_, FieldType::Message(_)) => quote! {
                    #mapped_either.factor_none()
                },
                _ => quote! {
                    #mapped_either.into_inner()
                },
            },
        )?)?;
        Ok(parse2(quote! {
            { #expr }
        })?)
    }

    fn gen_try_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Option<Block>> {
        if let FieldPresense::Repeated = field.presense {
            return Ok(None);
        }
        let Some(try_has_name) = &field.try_has_method_name else {
            return Ok(None);
        };
        let expr: Expr = parse2(quote! {
            self.as_ref().try_map2(
                |t1| <#t1 as #trait_path>::#try_has_name(t1),
                |t2| <#t2 as #trait_path>::#try_has_name(t2)
            )?.into_inner()
        })?;
        let result_expr = self.options.ok_value(&expr)?;
        Ok(Some(parse2(quote! { { #result_expr } })?))
    }
}
