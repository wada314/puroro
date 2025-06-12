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
use crate::generator::{CodeGeneratorOptions, TrySwitch};
use ::culpa::throws;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Block, Expr, Ident, Item, Path, parse_str, parse2};

type Error = crate::ErrorKind;

pub struct GenBlanketRefImpls {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for GenBlanketRefImpls {
    #[throws]
    fn generate<'a>(
        &self,
        trait_paths: &TrySwitch<Path>,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item> {
        let t: Ident = parse_str("T")?;
        let fields: Vec<_> = fields.collect();

        let view_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_getter_body(f, &t, &trait_paths[false]),
            |f| self.gen_has_method_body(f, &t, &trait_paths[false]),
            false,
        )?;

        let try_methods = impls_helper(
            fields.iter().copied(),
            |f| self.gen_try_getter_body(f, &t, &trait_paths[true]),
            |f| self.gen_try_has_method_body(f, &t, &trait_paths[true]),
            true,
        )?;

        vec![
            parse2(quote! {
                impl<#t: #(&trait_paths[false])> #(&trait_paths[false]) for &::std::option::Option<#t> {
                    #(#view_methods)*
                }
            })?,
            parse2(quote! {
                impl<#t: #(&trait_paths[true])> #(&trait_paths[true]) for &::std::option::Option<#t> {
                    #(#try_methods)*
                }
            })?,
        ]
    }
}

impl GenBlanketRefImpls {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    #[throws]
    fn gen_getter_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let signature = field.trait_getter_signatures()[false].clone();
        let getter_name = &signature.ident;
        let map_expr = quote! {
            self.as_ref().map(<#t as #trait_path>::#getter_name)
        };
        let expr = match field {
            Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! { #map_expr.into_iter_either() }
            }
            Field::Repeated(RepeatedField { .. }) => quote! { #map_expr.into_iter_chained() },
            Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. })
            | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! { #map_expr.factor_none() }
            }
            Field::Explicit(ScalarField { has_method_signatures, .. })
            | Field::Implicit(ScalarField { has_method_signatures, .. }) => {
                let has_method_name = &has_method_signatures[false].ident;
                quote! {
                    if <#t as #trait_path>::#has_method_name(self.as_ref()) {
                        <#t as #trait_path>::#getter_name(self.as_ref())
                    } else {
                        ::std::default::Default::default()
                    }
                }
            }
        };
        parse2(quote! {{ #expr }})?
    }

    #[throws]
    fn gen_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let (Field::Implicit(ScalarField { has_method_signatures, .. })
        | Field::Explicit(ScalarField { has_method_signatures, .. })) = field
        else {
            Err("this method is not supported for repeated fields".to_string())?
        };
        let has_name = &has_method_signatures[false].ident;
        parse2(quote! {{ <#t as #trait_path>::#has_name(self.as_ref()) }})?
    }

    #[throws]
    fn gen_try_getter_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let signature = field.trait_getter_signatures()[true].clone();
        let try_getter_name = &signature.ident;
        let mapped_either: Expr = parse2(quote! {
            self.as_ref().try_map(<#t as #trait_path>::#try_getter_name)?
        })?;
        let ok = self.options.ok_path();
        parse2(match field {
            Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! {{
                    #ok(#mapped_either.into_iter_either().map(|either_res| either_res.factor_err()))
                }}
            }
            Field::Repeated(RepeatedField { .. }) => quote! {{
                #ok(#mapped_either.into_iter_chained())
            }},
            Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. })
            | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(_), .. }) => {
                quote! {{
                    #ok(#mapped_either.factor_none())
                }}
            }
            Field::Explicit(ScalarField { has_method_signatures, .. })
            | Field::Implicit(ScalarField { has_method_signatures, .. }) => {
                let try_has_method_name = &has_method_signatures[true].ident;
                quote! {{
                    if <#t as #trait_path>::#try_has_method_name(self.as_ref())? {
                        <#t as #trait_path>::#try_getter_name(self.as_ref())
                    } else {
                        #ok(::std::default::Default::default())
                    }
                }}
            }
        })?
    }

    #[throws]
    fn gen_try_has_method_body(&self, field: &Field, t: &Ident, trait_path: &Path) -> Block {
        let try_has_name = match field {
            Field::Implicit(ScalarField { has_method_signatures, .. })
            | Field::Explicit(ScalarField { has_method_signatures, .. }) => {
                &has_method_signatures[true].ident
            }
            _ => Err("this method is not supported for repeated fields".to_string())?,
        };
        parse2(quote! {{ <#t as #trait_path>::#try_has_name(self.as_ref()) }})?
    }
}
