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
use crate::descriptor::FieldType;
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Expr, Ident, Item, Path};

pub struct GenBlanketEitherImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl BlanketImplsGenerator for GenBlanketEitherImpls {
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let t1: Ident = parse_str("T")?;
        let t2: Ident = parse_str("U")?;

        let methods = blanket_impls_helper(
            fields,
            |f| self.gen_try_get_method_body(f, &t1, &t2, &try_view_trait_path),
            |f| self.gen_try_has_method_body(f, &t1, &t2, &try_view_trait_path),
            true,
        )?;

        Ok(vec![parse2(quote! {
            impl<#t1: #try_view_trait_path, #t2: #try_view_trait_path> #try_view_trait_path for ::puroro::Either<#t1, #t2> {
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
        let signature = field.try_getter_signature();
        let try_getter_name = &signature.ident;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_getter_name,
                <#t2 as #trait_path>::#try_getter_name)?
        })?;
        let expr = self.options.ok_value(&parse2::<Expr>(match field {
            Field::Repeated {
                scalar_proto_type: FieldType::Message(_),
                ..
            } => quote! {
                #mapped_either.into_iter_either().map(|either_res| either_res.factor_err())
            },
            Field::Repeated { .. } => quote! {
                #mapped_either.into_iter_chained()
            },
            Field::Explicit {
                scalar_proto_type: FieldType::Message(_),
                ..
            }
            | Field::Implicit {
                scalar_proto_type: FieldType::Message(_),
                ..
            } => quote! {
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
        let Some(signature) = field.try_has_method_signature_if_non_repeated() else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let try_has_name = &signature.ident;
        let expr: Expr = parse2(quote! {
            self.as_ref().try_map2(
                |t1| <#t1 as #trait_path>::#try_has_name(t1),
                |t2| <#t2 as #trait_path>::#try_has_name(t2)
            )?.into_inner()
        })?;
        let result_expr = self.options.ok_value(&expr)?;
        Ok(parse2(quote! { { #result_expr } })?)
    }
}
