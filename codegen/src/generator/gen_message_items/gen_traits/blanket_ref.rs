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

use super::{BlanketImplsGenerator, Field2};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::puroro::Either;
use ::quote::quote;
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Ident, ImplItemFn, Item, Path};

pub struct GenBlanketRefImpls {
    #[allow(unused)]
    options: Rc<CodeGeneratorOptions>,
}

impl BlanketImplsGenerator for GenBlanketRefImpls {
    fn generate<'a>(
        &self,
        trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field2>>,
    ) -> Result<Vec<Item>> {
        let t: Ident = parse_str("T")?;
        let methods = fields
            .map(|f| {
                let try_getter: ImplItemFn = {
                    let signature = &f.try_getter_signature;
                    let try_getter_name = &f.try_getter_name;
                    parse2(quote! {
                        #signature {
                            <#t as #trait_path>::#try_getter_name(self)
                        }
                    })?
                };
                let try_has_method: Option<ImplItemFn> = {
                    let signature = &f.try_has_method_signature;
                    let try_has_name = f.try_has_method_name.as_ref();
                    if let (Some(signature), Some(try_has_name)) = (signature, try_has_name) {
                        Some(parse2(quote! {
                            #signature {
                                <#t as #trait_path>::#try_has_name(self)
                            }
                        })?)
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

        Ok(vec![
            parse2(quote! {
                impl<#t: #trait_path> #trait_path for &#t {
                    #(#methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #trait_path> #trait_path for &mut #t {
                    #(#methods)*
                }
            })?,
        ])
    }
}

impl GenBlanketRefImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }
}
