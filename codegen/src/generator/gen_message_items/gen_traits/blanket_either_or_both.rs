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

pub struct GenBlanketEitherOrBothImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketEitherOrBothImpls {
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
                impl<#t1: #view_trait_path, #t2: #view_trait_path> #view_trait_path for ::puroro::EitherOrBoth<#t1, #t2> {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t1: #try_view_trait_path, #t2: #try_view_trait_path> #try_view_trait_path for ::puroro::EitherOrBoth<#t1, #t2> {
                    #(#try_methods)*
                }
            })?,
        ])
    }
}

impl GenBlanketEitherOrBothImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    fn gen_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
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
            Field::Explicit { has_method_signature, .. }
            | Field::Implicit { has_method_signature, .. } => {
                let has_method_name = &has_method_signature.ident;
                quote! {
                    let (left_opt, right_opt) = self.as_ref().left_and_right();
                    if let Some(right) = right_opt {
                        if <#t2 as #trait_path>::#has_method_name(right) {
                            return <#t2 as #trait_path>::#getter_name(right);
                        }
                    }
                    if let Some(left) = left_opt {
                        if <#t1 as #trait_path>::#has_method_name(left) {
                            return <#t1 as #trait_path>::#getter_name(left);
                        }
                    }
                    ::std::default::Default::default()
                }
            }
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
    ) -> Result<Block> {
        let (Field::Implicit { has_method_signature, .. }
        | Field::Explicit { has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signature.ident;
        let block = parse2(quote! {{
            let (left_opt, right_opt) = self.as_ref().left_and_right();
            if let Some(right) = right_opt {
                if <#t2 as #trait_path>::#has_name(right) {
                    return true;
                }
            }
            if let Some(left) = left_opt {
                if <#t1 as #trait_path>::#has_name(left) {
                    return true;
                }
            }
            false
        }})?;
        Ok(block)
    }

    fn gen_try_get_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let signature = field.try_getter_signature();
        let try_getter_name = &signature.ident;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map2(
                <#t1 as #trait_path>::#try_getter_name,
                <#t2 as #trait_path>::#try_getter_name)?
        })?;
        let ok = self.options.ok_path();
        let block = parse2(match field {
            Field::Repeated { scalar_proto_type: FieldType::Message(_), .. } => quote! {{
                #ok(#mapped_either.into_iter_either().map(|either_res| either_res.factor_err()))
            }},
            Field::Repeated { .. } => quote! {{
                #ok(#mapped_either.into_iter_chained())
            }},
            Field::Explicit { scalar_proto_type: FieldType::Message(_), .. }
            | Field::Implicit { scalar_proto_type: FieldType::Message(_), .. } => quote! {{
                #ok(#mapped_either.factor_none())
            }},
            Field::Explicit { try_has_method_signature, .. }
            | Field::Implicit { try_has_method_signature, .. } => {
                let try_has_method_name = &try_has_method_signature.ident;
                quote! {{
                    let (left_opt, right_opt) = self.as_ref().left_and_right();
                    if let Some(right) = right_opt {
                        if <#t2 as #trait_path>::#try_has_method_name(right)? {
                            return <#t2 as #trait_path>::#try_getter_name(right);
                        }
                    }
                    if let Some(left) = left_opt {
                        if <#t1 as #trait_path>::#try_has_method_name(left)? {
                            return <#t1 as #trait_path>::#try_getter_name(left);
                        }
                    }
                    #ok(::std::default::Default::default())
                }}
            }
        })?;
        Ok(block)
    }

    fn gen_try_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> Result<Block> {
        let (Field::Implicit { try_has_method_signature, .. }
        | Field::Explicit { try_has_method_signature, .. }) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let try_has_name = &try_has_method_signature.ident;
        let expr: Expr = parse2(quote! {
            self.as_ref().right().map(<#t2 as #trait_path>::#try_has_name)
                    .transpose()?.unwrap_or(false)
                || self.as_ref().left().map(<#t1 as #trait_path>::#try_has_name)
                    .transpose()?.unwrap_or(false)
        })?;
        let ok = self.options.ok_path();
        Ok(parse2(quote! {
            {
                let result = #expr;
                #ok(result)
            }
        })?)
    }
}
