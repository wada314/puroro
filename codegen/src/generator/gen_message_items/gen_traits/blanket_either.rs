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
use crate::descriptor::FieldType;
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Expr, Ident, Item, Path};

pub struct GenBlanketEitherImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketEitherImpls {
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
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

        Ok(vec![
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
        ])
    }
}

impl GenBlanketEitherImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Block> {
        let signature = field.getter_signature();
        let getter_name = &signature.ident;
        let map2_expr = quote! {
            self.as_ref().map2(
                <#t1 as #trait_path>::#getter_name,
                <#t2 as #trait_path>::#getter_name
            )
        };
        let expr = match field {
            Field::Repeated { scalar_proto_type: FieldType::Message(_), .. } => {
                quote! { #map2_expr.into_iter_either() }
            }
            Field::Repeated { .. } => quote! { #map2_expr.into_iter_chained() },
            Field::Explicit { scalar_proto_type: FieldType::Message(_), .. }
            | Field::Implicit { scalar_proto_type: FieldType::Message(_), .. } => {
                quote! { #map2_expr.factor_none() }
            }
            _ => quote! { #map2_expr.into_inner() },
        };
        let block = parse2(quote! {{ #expr }})?;
        Ok(block)
    }

    fn gen_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Block> {
        let (Field::Implicit { has_method_signature, .. }
        | Field::Explicit { has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signature.ident;
        let block = parse2(quote! {{
            self.as_ref().map2(
                <#t1 as #trait_path>::#has_name,
                <#t2 as #trait_path>::#has_name
            ).into_inner()
        }})?;
        Ok(block)
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Block> {
        let signature = field.try_getter_signature();
        let try_getter_name = &signature.ident;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_getter_name,
                <#t2 as #trait_path>::#try_getter_name)?
        })?;
        let expr = self.options.ok_value(&parse2::<Expr>(match field {
            Field::Repeated { scalar_proto_type: FieldType::Message(_), .. } => quote! {
                #mapped_either.into_iter_either().map(|either_res| either_res.factor_err())
            },
            Field::Repeated { .. } => quote! {
                #mapped_either.into_iter_chained()
            },
            Field::Explicit { scalar_proto_type: FieldType::Message(_), .. }
            | Field::Implicit { scalar_proto_type: FieldType::Message(_), .. } => quote! {
                #mapped_either.factor_none()
            },
            _ => quote! {
                #mapped_either.into_inner()
            },
        })?)?;
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
    ) -> crate::Result<Block> {
        let (Field::Implicit { try_has_method_signature, .. }
        | Field::Explicit { try_has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let try_has_name = &try_has_method_signature.ident;
        let expr: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_has_name,
                <#t2 as #trait_path>::#try_has_name
            )?.into_inner()
        })?;
        let result_expr = self.options.ok_value(&expr)?;
        Ok(parse2(quote! { { #result_expr } })?)
    }
}
