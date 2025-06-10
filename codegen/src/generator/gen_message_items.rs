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

mod field;

mod blanket_both;
mod blanket_either;
mod blanket_either_or_both;
mod blanket_option;
mod blanket_ref;
mod dynamic_message;

use super::CodeGeneratorOptions;
use crate::cases::{Case, convert_into_case};
use crate::descriptor::{DescriptorExt, FieldType};
use crate::generator::to_ident;
use crate::{Result, ResultExt};
use ::culpa::throws;
use ::quither::Either;
use ::quote::{format_ident, quote};
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::parse::Parser;
use ::syn::{Block, Ident, ImplItemFn, Item, Path, PathArguments, PathSegment, Stmt, Type, parse2};
use field::{Field, RepeatedField, ScalarField};

use blanket_both::GenBlanketBothImpls;
use blanket_either::GenBlanketEitherImpls;
use blanket_either_or_both::GenBlanketEitherOrBothImpls;
use blanket_option::GenBlanketOptionImpls;
use blanket_ref::GenBlanketRefImpls;
use dynamic_message::DynamicMessageImplsGenerator;

type Error = crate::ErrorKind;

pub struct GenMessageItems {
    struct_name: Ident,
    view_trait_name: Ident,
    try_view_trait_name: Ident,
    fields: Vec<Field>,
    options: Rc<CodeGeneratorOptions>,
}

impl GenMessageItems {
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let current_proto_path = Rc::new(desc.current_path().to_owned());
        Ok(Self {
            struct_name: gen_struct_name(desc.name()),
            view_trait_name: gen_view_trait_name(desc.name()),
            try_view_trait_name: gen_try_view_trait_name(desc.name()),
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|f| Field::try_new(f, Rc::clone(&current_proto_path), Rc::clone(&options)))
                .collect::<Result<Vec<_>>>()?,
            options,
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let view_trait_def = self.gen_view_trait()?;
        let try_view_trait_def = self.gen_try_view_trait()?;
        let struct_def = self.gen_struct()?;

        let view_wrapping_struct_impl = self.gen_view_wrapping_struct_impl()?;

        let view_trait_name = &self.view_trait_name;
        let view_trait_path: Path = parse2(quote! { self::#view_trait_name })?;
        let try_trait_name = &self.try_view_trait_name;
        let try_trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators: Vec<Rc<dyn ImplsGenerator>> = vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
            Rc::new(DynamicMessageImplsGenerator::new(Rc::clone(&self.options))),
        ];
        let blanket_impls = blanket_impl_generators
            .iter()
            .map(|g| {
                g.generate(
                    &view_trait_path,
                    &try_trait_path,
                    Box::new(self.fields.iter()),
                )
            })
            .map(|r| match r {
                Ok(vec) => Either::Left(vec.into_iter().map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        items.extend([
            struct_def,
            view_trait_def,
            try_view_trait_def,
            view_wrapping_struct_impl,
        ]);
        items.extend(blanket_impls);

        Ok(items)
    }

    #[throws]
    fn gen_struct(&self) -> Item {
        let struct_name = &self.struct_name;
        let t = format_ident!("T");
        let default_type: Type = parse2(quote! { ::puroro::dynamic::DynamicMessage })?;
        parse2(quote! {
            #[repr(transparent)]
            pub struct #struct_name<#t = #default_type>(pub #t);
        })?
    }

    #[throws]
    fn gen_view_trait(&self) -> Item {
        let trait_name = &self.view_trait_name;
        let try_trait_name = &self.try_view_trait_name;
        let getters = self
            .fields
            .iter()
            .map(|f| f.getter_signatures().trait_getter.clone())
            .collect::<Vec<_>>();
        let has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit(ScalarField {
                    has_method_signatures,
                    ..
                })
                | Field::Explicit(ScalarField {
                    has_method_signatures,
                    ..
                }) => Some(has_method_signatures.has_method.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        parse2(quote! {
            pub trait #trait_name: self::#try_trait_name {
                #(#getters;)*
                #(#has_methods;)*
            }
        })?
    }

    #[throws]
    fn gen_try_view_trait(&self) -> Item {
        let trait_name = &self.try_view_trait_name;
        let try_getters = self
            .fields
            .iter()
            .map(|f| f.getter_signatures().trait_try_getter.clone())
            .collect::<Vec<_>>();
        let try_has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit(ScalarField {
                    has_method_signatures,
                    ..
                })
                | Field::Explicit(ScalarField {
                    has_method_signatures,
                    ..
                }) => Some(has_method_signatures.try_has_method.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        parse2(quote! {
            pub trait #trait_name {
                #(#try_getters;)*
                #(#try_has_methods;)*
            }
        })?
    }

    #[throws]
    fn gen_view_wrapping_struct_impl(&self) -> Item {
        let struct_name = &self.struct_name;
        let t = format_ident!("T");
        let trait_name = &self.view_trait_name;
        let getter_signatures = self
            .fields
            .iter()
            .map(|field| field.getter_signatures().struct_getter.clone())
            .collect::<Vec<_>>();
        let body_stmts = self
            .fields
            .iter()
            .map(|field| self.gen_view_wrapping_method_body(field, &t, trait_name, &self.options))
            .collect::<Result<Vec<_>>>()?;
        parse2(quote! {
            impl<#t> #struct_name<#t>
            where #t: #trait_name
            {
                #(pub #getter_signatures {
                    #(#body_stmts)*
                })*
            }
        })?
    }

    #[throws]
    fn gen_view_wrapping_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_name: &Ident,
        options: &CodeGeneratorOptions,
    ) -> Vec<Stmt> {
        let parser = Block::parse_within;
        let getter_name = &field.getter_signatures().struct_getter.ident;
        let body_tokens = match field {
            Field::Repeated(RepeatedField {
                scalar_proto_type: FieldType::Message(m),
                ..
            }) => {
                let wrapper_type = m
                    .to_relative_path(&field.base_proto_path())
                    .unwrap_or(m.as_ref())
                    .to_rust_path_with(options, |name| {
                        Ok(PathSegment {
                            ident: gen_struct_name(name),
                            arguments: PathArguments::None,
                        })
                    })?;
                // Repeated message field. Need to map the iterator values by the wrapper type.
                quote! {
                    <#t as #trait_name>::#getter_name(&self.0)
                        .into_iter()
                        .map(|value| #wrapper_type(value))
                }
            }
            Field::Explicit(ScalarField {
                scalar_proto_type: FieldType::Message(m),
                ..
            })
            | Field::Implicit(ScalarField {
                scalar_proto_type: FieldType::Message(m),
                ..
            }) => {
                let wrapper_type = m
                    .to_relative_path(&field.base_proto_path())
                    .unwrap_or(m.as_ref())
                    .to_rust_path_with(options, |name| {
                        Ok(PathSegment {
                            ident: gen_struct_name(name),
                            arguments: PathArguments::None,
                        })
                    })?;
                // Scalar message field. Need to map the Option inner value by the wrapper type.
                quote! {
                    <#t as #trait_name>::#getter_name(&self.0)
                        .map(|value| #wrapper_type(value))
                }
            }
            _ => {
                quote! {
                    <#t as #trait_name>::#getter_name(&self.0)
                }
            }
        };
        parser.parse2(body_tokens)?
    }
}

fn gen_view_trait_name(message_name: &str) -> Ident {
    to_ident(&format!(
        "{}View",
        convert_into_case(message_name, Case::CamelCase)
    ))
}

fn gen_try_view_trait_name(message_name: &str) -> Ident {
    to_ident(&format!(
        "Try{}View",
        convert_into_case(message_name, Case::CamelCase)
    ))
}

fn gen_struct_name(message_name: &str) -> Ident {
    to_ident(&convert_into_case(message_name, Case::CamelCase))
}

trait ImplsGenerator {
    #[throws]
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item>;
}

#[throws]
fn impls_helper<'a, F, G>(
    fields: impl Iterator<Item = &'a Field>,
    gen_getter: F,
    gen_has_method: G,
    is_try_trait: bool,
) -> Vec<ImplItemFn>
where
    F: Fn(&Field) -> Result<Block>,
    G: Fn(&Field) -> Result<Block>,
{
    fields
        .map(|f| {
            let get_method: ImplItemFn = {
                let signature = if is_try_trait {
                    f.getter_signatures().trait_try_getter.clone()
                } else {
                    f.getter_signatures().trait_getter.clone()
                };
                let body = gen_getter(f)?;
                parse2(quote! {
                    #signature #body
                })?
            };
            let has_method: Option<ImplItemFn> = match f {
                Field::Explicit(ScalarField {
                    has_method_signatures,
                    ..
                })
                | Field::Implicit(ScalarField {
                    has_method_signatures,
                    ..
                }) => {
                    let signature = if is_try_trait {
                        has_method_signatures.try_has_method.clone()
                    } else {
                        has_method_signatures.has_method.clone()
                    };
                    let body = gen_has_method(f)?;
                    Some(parse2(quote! {
                        #signature #body
                    })?)
                }
                _ => None,
            };
            Ok(once(get_method).chain(has_method.into_iter()))
        })
        .flat_map(ResultExt::transpose_iter)
        .collect::<Result<Vec<_>>>()?
}
