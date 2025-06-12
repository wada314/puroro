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

use super::field::{Field, RepeatedField, ScalarField};
use super::{ImplsGenerator, impls_helper};
use crate::descriptor::FieldType;
use crate::generator::CodeGeneratorOptions;
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, Expr, Ident, Item, Path, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketEitherImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketEitherImpls {
    #[throws]
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t1: Ident = parse_str("T")?;
        let t2: Ident = parse_str("U")?;
        let fields: Vec<_> = fields.collect();

        let view_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_get_method_body(f, &t1, &t2, &view_trait_path),
            |f| self.gen_has_method_body(f, &t1, &t2, &view_trait_path),
            false,
        )?;

        let try_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_try_get_method_body(f, &t1, &t2, &try_view_trait_path),
            |f| self.gen_try_has_method_body(f, &t1, &t2, &try_view_trait_path),
            true,
        )?;

        vec![
            parse2(quote! {
                impl<#t1: #view_trait_path, #t2: #view_trait_path> #view_trait_path for ::puroro::Either<#t1, #t2> {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t1: #try_view_trait_path, #t2: #try_view_trait_path> #try_view_trait_path for ::puroro::Either<#t1, #t2> {
                    #(#try_methods)*
                }
            })?,
        ]
    }
}

impl GenBlanketEitherImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    #[throws]
    fn gen_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Block {
        let signature = field.trait_getter_signatures()[false].clone();
        let getter_name = &signature.ident;
        let map2_expr = quote! {
            self.as_ref().map2(
                <#t1 as #trait_path>::#getter_name,
                <#t2 as #trait_path>::#getter_name
            )
        };
        let expr = match field {
            Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! { #map2_expr.into_iter_either() }
            }
            Field::Repeated(RepeatedField { .. }) => quote! { #map2_expr.into_iter_chained() },
            Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. })
            | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! { #map2_expr.factor_none() }
            }
            _ => quote! { #map2_expr.into_inner() },
        };
        parse2(quote! {{ #expr }})?
    }

    #[throws]
    fn gen_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Block {
        let has_name = match field {
            Field::Implicit(ScalarField { has_method_signatures, .. })
            | Field::Explicit(ScalarField { has_method_signatures, .. }) => {
                &has_method_signatures[false].ident
            }
            _ => Err("this method is not supported for repeated fields".to_string())?,
        };
        parse2(quote! {{
            self.as_ref().map2(
                <#t1 as #trait_path>::#has_name,
                <#t2 as #trait_path>::#has_name
            ).into_inner()
        }})?
    }

    #[throws]
    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Block {
        let signature = field.trait_getter_signatures()[true].clone();
        let try_getter_name = &signature.ident;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_getter_name,
                <#t2 as #trait_path>::#try_getter_name)?
        })?;
        let expr = self.options.ok_value(
            &parse2::<Expr>(match field {
                Field::Repeated(RepeatedField {
                    scalar_proto_type: FieldType::Message(_), ..
                }) => quote! {
                    #mapped_either.into_iter_either().map(|either_res| either_res.factor_err())
                },
                Field::Repeated(RepeatedField { .. }) => quote! {
                    #mapped_either.into_iter_chained()
                },
                Field::Explicit(ScalarField {
                    scalar_proto_type: FieldType::Message(_), ..
                })
                | Field::Implicit(ScalarField {
                    scalar_proto_type: FieldType::Message(_), ..
                }) => quote! {
                    #mapped_either.factor_none()
                },
                _ => quote! {
                    #mapped_either.into_inner()
                },
            })
            .unwrap_or_else(|e| panic!("parse2 failed: {}", e)),
        );
        parse2(quote! {
            { #expr }
        })?
    }

    #[throws]
    fn gen_try_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Block {
        let try_has_name = match field {
            Field::Implicit(ScalarField { has_method_signatures, .. })
            | Field::Explicit(ScalarField { has_method_signatures, .. }) => {
                &has_method_signatures[true].ident
            }
            _ => Err("this method is not supported for repeated fields".to_string())?,
        };
        let expr: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_has_name,
                <#t2 as #trait_path>::#try_has_name
            )?.into_inner()
        })?;
        let result_expr = self.options.ok_value(&expr);
        parse2(quote! { { #result_expr } })?
    }
}
