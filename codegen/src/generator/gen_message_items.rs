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
use crate::generator::{TrySwitch, to_ident};
use crate::proto_path::ProtoPathBuf;
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
    view_trait_names: TrySwitch<Ident>,
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
            view_trait_names: TrySwitch::new(
                gen_view_trait_name(desc.name()),
                gen_try_view_trait_name(desc.name()),
            ),
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

        // Generate basic items
        items.extend(self.gen_basic_items()?);

        // Generate blanket implementations
        items.extend(self.gen_blanket_impls()?);

        Ok(items)
    }

    fn gen_basic_items(&self) -> Result<Vec<Item>> {
        let view_trait_def = self.gen_view_trait()?;
        let try_view_trait_def = self.gen_try_view_trait()?;
        let struct_def = self.gen_struct()?;
        let view_wrapping_struct_impl = self.gen_wrapping_struct_impl(false)?;
        let try_view_wrapping_struct_impl = self.gen_wrapping_struct_impl(true)?;

        Ok(vec![
            struct_def,
            view_trait_def,
            try_view_trait_def,
            view_wrapping_struct_impl,
            try_view_wrapping_struct_impl,
        ])
    }

    fn gen_blanket_impls(&self) -> Result<Vec<Item>> {
        let view_trait_name = &self.view_trait_names[false];
        let view_trait_path: Path = parse2(quote! { self::#view_trait_name })?;
        let try_trait_name = &self.view_trait_names[true];
        let try_trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators = self.create_blanket_impl_generators();
        let trait_paths = TrySwitch::new(view_trait_path, try_trait_path);
        let blanket_impls = blanket_impl_generators
            .iter()
            .map(|g| g.generate(&trait_paths, Box::new(self.fields.iter())))
            .map(|r| match r {
                Ok(vec) => Either::Left(vec.into_iter().map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        Ok(blanket_impls)
    }

    fn create_blanket_impl_generators(&self) -> Vec<Rc<dyn ImplsGenerator>> {
        vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
            Rc::new(DynamicMessageImplsGenerator::new(Rc::clone(&self.options))),
        ]
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
        let trait_name = &self.view_trait_names[false];
        let try_trait_name = &self.view_trait_names[true];
        let getters = self
            .fields
            .iter()
            .map(|f| f.trait_getter_signatures()[false].clone())
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
                }) => Some(has_method_signatures[false].clone()),
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
        let trait_name = &self.view_trait_names[true];
        let try_getters = self
            .fields
            .iter()
            .map(|f| f.trait_getter_signatures()[true].clone())
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
                }) => Some(has_method_signatures[true].clone()),
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
    fn gen_wrapping_struct_impl(&self, is_try: bool) -> Item {
        let struct_name = &self.struct_name;
        let t = format_ident!("T");
        let trait_name = &self.view_trait_names[is_try];
        let getter_signatures = self
            .fields
            .iter()
            .map(|field| field.struct_getter_signatures()[is_try].clone())
            .collect::<Vec<_>>();
        let body_stmts = self
            .fields
            .iter()
            .map(|field| {
                self.gen_wrapping_method_body(field, &t, trait_name, &self.options, is_try)
            })
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
    fn gen_wrapping_method_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_name: &Ident,
        options: &CodeGeneratorOptions,
        is_try: bool,
    ) -> Vec<Stmt> {
        let parser = Block::parse_within;
        let getter_name = &field.struct_getter_signatures()[is_try].ident;
        let body_tokens =
            self.gen_field_body(field, t, trait_name, getter_name, options, is_try)?;
        parser.parse2(body_tokens)?
    }

    fn gen_field_body(
        &self,
        field: &Field,
        t: &Ident,
        trait_name: &Ident,
        getter_name: &Ident,
        options: &CodeGeneratorOptions,
        is_try: bool,
    ) -> Result<proc_macro2::TokenStream> {
        match field {
            Field::Repeated(RepeatedField {
                scalar_proto_type: FieldType::Message(m),
                ..
            }) => self.gen_repeated_message_getter_body(
                &FieldType::Message(m.clone()),
                t,
                trait_name,
                getter_name,
                options,
                is_try,
                field,
            ),
            Field::Explicit(ScalarField {
                scalar_proto_type: FieldType::Message(m),
                ..
            })
            | Field::Implicit(ScalarField {
                scalar_proto_type: FieldType::Message(m),
                ..
            }) => self.gen_scalar_message_getter_body(
                &FieldType::Message(m.clone()),
                t,
                trait_name,
                getter_name,
                options,
                is_try,
                field,
            ),
            _ => self.gen_simple_field_getter_body(t, trait_name, getter_name),
        }
    }

    fn gen_repeated_message_getter_body(
        &self,
        message_type: &FieldType<ProtoPathBuf, ProtoPathBuf>,
        t: &Ident,
        trait_name: &Ident,
        getter_name: &Ident,
        options: &CodeGeneratorOptions,
        is_try: bool,
        field: &Field,
    ) -> Result<proc_macro2::TokenStream> {
        let wrapper_type = self.generate_wrapper_type(message_type, field, options)?;
        Ok(if is_try {
            // Repeated message field with try semantics
            quote! {
                <#t as #trait_name>::#getter_name(&self.0)
                    .map(|rep| rep
                        .into_iter()
                        .map(|res| res.map(|value| #wrapper_type(value))))
            }
        } else {
            // Repeated message field without try semantics
            quote! {
                <#t as #trait_name>::#getter_name(&self.0)
                    .into_iter()
                    .map(|value| #wrapper_type(value))
            }
        })
    }

    fn gen_scalar_message_getter_body(
        &self,
        message_type: &FieldType<ProtoPathBuf, ProtoPathBuf>,
        t: &Ident,
        trait_name: &Ident,
        getter_name: &Ident,
        options: &CodeGeneratorOptions,
        is_try: bool,
        field: &Field,
    ) -> Result<proc_macro2::TokenStream> {
        let wrapper_type = self.generate_wrapper_type(message_type, field, options)?;
        Ok(if is_try {
            // Scalar message field with try semantics
            quote! {
                <#t as #trait_name>::#getter_name(&self.0)
                    .map(|opt| opt.map(|value| #wrapper_type(value)))
            }
        } else {
            // Scalar message field without try semantics
            quote! {
                <#t as #trait_name>::#getter_name(&self.0)
                    .map(|value| #wrapper_type(value))
            }
        })
    }

    fn gen_simple_field_getter_body(
        &self,
        t: &Ident,
        trait_name: &Ident,
        getter_name: &Ident,
    ) -> Result<proc_macro2::TokenStream> {
        Ok(quote! {
            <#t as #trait_name>::#getter_name(&self.0)
        })
    }

    fn generate_wrapper_type(
        &self,
        message_type: &FieldType<ProtoPathBuf, ProtoPathBuf>,
        field: &Field,
        options: &CodeGeneratorOptions,
    ) -> Result<Path> {
        let message_type = match message_type {
            FieldType::Message(m) => m,
            _ => unreachable!(),
        };
        Ok(message_type
            .to_relative_path(&field.base_proto_path())
            .unwrap_or(message_type.as_ref())
            .to_rust_path_with(options, |name| {
                Ok(PathSegment {
                    ident: gen_struct_name(name),
                    arguments: PathArguments::None,
                })
            })?)
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
        view_trait_paths: &TrySwitch<Path>,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item>;
}

/// A helper function to generate blanket impls for view traits.
///
/// Essentially, this function takes a generator for the function body,
/// then invokes it for each field, and appends the function signatures for each.
///
/// # Arguments
///
/// * `fields` - The field data structs to generate the blanket impls for.
/// * `gen_getter` - A function to generate the getter method body block.
/// * `gen_has_method` - A function to generate the has method body block.
/// * `is_try_trait` - Whether the trait is a `TryView` trait or not (View trait otherwise).
///
/// # Returns
///
/// A vector of ImplItemFn; i.e. the methods in the impl block.

#[throws]
fn view_trait_blanket_impl_helper<'a, F, G>(
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
                let signature = f.trait_getter_signatures()[is_try_trait].clone();
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
                    let signature = has_method_signatures[is_try_trait].clone();
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

/// Generates blanket implementations for both View and TryView traits.
///
/// This function takes generators for the function bodies and creates implementations
/// for both View and TryView traits simultaneously. For each field, it generates
/// getter methods and optional has-methods, maintaining the distinction between
/// View and TryView semantics.
///
/// # Arguments
///
/// * `fields` - The field data structs to generate the blanket impls for.
/// * `gen_getter` - A function to generate the getter method body block.
/// * `gen_has_method` - A function to generate the has method body block.
///
/// # Returns
///
/// A TrySwitch containing vectors of ImplItemFn for both View and TryView traits.
#[throws]
fn view_trait_blanket_impl_helper2<'a>(
    fields: impl Iterator<Item = &'a Field>,
    gen_getter: TrySwitch<Box<dyn Fn(&Field, bool) -> Result<Block>>>,
    gen_has_method: TrySwitch<Box<dyn Fn(&Field, bool) -> Result<Block>>>,
) -> TrySwitch<Vec<ImplItemFn>> {
    let mut view_impls = Vec::new();
    let mut try_view_impls = Vec::new();

    for f in fields {
        // Generate View trait implementations
        let view_get_method: ImplItemFn = {
            let signature = f.trait_getter_signatures()[false].clone();
            let body = gen_getter(f, false)?;
            parse2(quote! {
                #signature #body
            })?
        };
        let view_has_method: Option<ImplItemFn> = match f {
            Field::Explicit(ScalarField {
                has_method_signatures,
                ..
            })
            | Field::Implicit(ScalarField {
                has_method_signatures,
                ..
            }) => {
                let signature = has_method_signatures[false].clone();
                let body = gen_has_method(f, false)?;
                Some(parse2(quote! {
                    #signature #body
                })?)
            }
            _ => None,
        };
        view_impls.extend(once(view_get_method).chain(view_has_method.into_iter()));

        // Generate TryView trait implementations
        let try_view_get_method: ImplItemFn = {
            let signature = f.trait_getter_signatures()[true].clone();
            let body = gen_getter(f, true)?;
            parse2(quote! {
                #signature #body
            })?
        };
        let try_view_has_method: Option<ImplItemFn> = match f {
            Field::Explicit(ScalarField {
                has_method_signatures,
                ..
            })
            | Field::Implicit(ScalarField {
                has_method_signatures,
                ..
            }) => {
                let signature = has_method_signatures[true].clone();
                let body = gen_has_method(f, true)?;
                Some(parse2(quote! {
                    #signature #body
                })?)
            }
            _ => None,
        };
        try_view_impls.extend(once(try_view_get_method).chain(try_view_has_method.into_iter()));
    }

    TrySwitch::new(view_impls, try_view_impls)
}
