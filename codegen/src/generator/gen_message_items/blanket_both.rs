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
use super::{ImplsGenerator, view_trait_blanket_impl_helper};
use crate::descriptor::FieldType;
use crate::generator::{CodeGeneratorOptions, TrySwitch};
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, ExprPath, Ident, Item, Path, parse::Parser, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketBothImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketBothImpls {
    #[throws]
    fn generate<'a>(
        &self,
        trait_paths: &TrySwitch<Path>,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t1: Ident = parse_str("T")?;
        let t2: Ident = parse_str("U")?;
        let fields: Vec<_> = fields.collect();
        let view_trait_path = &trait_paths[false];
        let try_trait_path = &trait_paths[true];

        let methods = view_trait_blanket_impl_helper(
            fields.iter().copied(),
            |f, is_try| self.gen_get_method_body(f, &t1, &t2, &trait_paths[is_try], is_try),
            |f, is_try| self.gen_has_method_body(f, &t1, &t2, &trait_paths[is_try], is_try),
        )?;

        let view_methods = &methods[false];
        let try_methods = &methods[true];

        vec![
            parse2(quote! {
                impl<#t1: #view_trait_path, #t2: #view_trait_path> #view_trait_path for ::puroro::Both<#t1, #t2> {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t1: #try_trait_path, #t2: #try_trait_path> #try_trait_path for ::puroro::Both<#t1, #t2> {
                    #(#try_methods)*
                }
            })?,
        ]
    }
}

impl GenBlanketBothImpls {
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
        is_try: bool,
    ) -> Block {
        let signature = field.trait_getter_signatures()[is_try].clone();
        let getter_name = &signature.ident;
        let t1_getter: ExprPath = parse2(quote! { <#t1 as #trait_path>::#getter_name })?;
        let t2_getter: ExprPath = parse2(quote! { <#t2 as #trait_path>::#getter_name })?;
        let stmts = Block::parse_within.parse2(match (field, is_try) {
            // Message repeated field with try
            (
                Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(_), .. }),
                true,
            ) => {
                let ok = self.options.ok_path();
                quote! {
                    #ok(self.as_ref().try_map2(#t1_getter, #t2_getter)?.into_iter_either()
                        .map(|either_res| either_res.factor_err()))
                }
            }
            // Message repeated field without try
            (
                Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(_), .. }),
                false,
            ) => {
                quote! {
                    self.as_ref().map2(#t1_getter, #t2_getter).into_iter_either()
                }
            }
            // Non-message repeated field with try
            (Field::Repeated(RepeatedField { .. }), true) => {
                let ok = self.options.ok_path();
                quote! {
                    #ok(self.as_ref().try_map2(#t1_getter, #t2_getter)?.into_iter_chained())
                }
            }
            // Non-message repeated field without try
            (Field::Repeated(RepeatedField { .. }), false) => quote! {
                self.as_ref().map2(#t1_getter, #t2_getter).into_iter_chained()
            },
            // Message scalar field with try
            (
                Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. })
                | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. }),
                true,
            ) => {
                let ok = self.options.ok_path();
                quote! {
                    #ok(self.as_ref().try_map2(#t1_getter, #t2_getter)?.factor_none())
                }
            }
            // Message scalar field without try
            (
                Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. })
                | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. }),
                false,
            ) => {
                quote! {
                    self.as_ref().map2(#t1_getter, #t2_getter).factor_none()
                }
            }
            // Non-message scalar field with try
            (
                Field::Explicit(ScalarField { has_method_signatures, .. })
                | Field::Implicit(ScalarField { has_method_signatures, .. }),
                true,
            ) => {
                let try_has_method_name = &has_method_signatures[true].ident;
                let ok = self.options.ok_path();
                quote! {
                    let ::puroro::Both::Both(left, right) = self;
                    if <#t2 as #trait_path>::#try_has_method_name(&right)? {
                        return #t2_getter(&right);
                    }
                    if <#t1 as #trait_path>::#try_has_method_name(&left)? {
                        return #t1_getter(&left);
                    }
                    #ok(::std::default::Default::default())
                }
            }
            // Non-message scalar field without try
            (
                Field::Explicit(ScalarField { has_method_signatures, .. })
                | Field::Implicit(ScalarField { has_method_signatures, .. }),
                false,
            ) => {
                let has_method_name = &has_method_signatures[false].ident;
                quote! {
                    let ::puroro::Both::Both(left, right) = self;
                    if <#t2 as #trait_path>::#has_method_name(&right) {
                        return #t2_getter(&right);
                    }
                    if <#t1 as #trait_path>::#has_method_name(&left) {
                        return #t1_getter(&left);
                    }
                    ::std::default::Default::default()
                }
            }
        })?;
        Block { stmts, brace_token: Default::default() }
    }

    #[throws]
    fn gen_has_method_body(
        &self,
        field: &Field,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
        is_try: bool,
    ) -> Block {
        let (Field::Implicit(ScalarField { has_method_signatures, .. })
        | Field::Explicit(ScalarField { has_method_signatures, .. })) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signatures[is_try].ident;
        let stmts = Block::parse_within.parse2(if is_try {
            let ok = self.options.ok_path();
            quote! {
                let ::puroro::Both::Both(left, right) = self;
                #ok(<#t2 as #trait_path>::#has_name(&right)?
                    || <#t1 as #trait_path>::#has_name(&left)?)
            }
        } else {
            quote! {
                let ::puroro::Both::Both(left, right) = self;
                <#t2 as #trait_path>::#has_name(&right) || <#t1 as #trait_path>::#has_name(&left)
            }
        })?;
        Block { stmts, brace_token: Default::default() }
    }
}
