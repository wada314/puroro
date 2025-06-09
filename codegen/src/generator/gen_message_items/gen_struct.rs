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

use ::std::rc::Rc;

use super::field::Field;
use super::gen_traits::GenTraits;
use crate::ErrorKind;
use crate::Result;
use crate::cases::{Case, convert_into_case};
use crate::descriptor::{DescriptorExt, FieldType};
use crate::generator::gen_message_items::field::RepeatedField;
use crate::generator::gen_message_items::field::ScalarField;
use crate::generator::{CodeGeneratorOptions, to_ident};
use ::culpa::throws;
use ::quote::{format_ident, quote};
use ::syn::parse::Parser;
use ::syn::{
    Block, Ident, Item, PathArguments, PathSegment, Stmt, Type, parse2,
};

type Error = ErrorKind;

pub struct GenStruct {
    struct_name: Ident,
    options: Rc<CodeGeneratorOptions>,
    gen_traits: Rc<GenTraits>,
    fields: Vec<Field>,
}

impl GenStruct {
    #[throws]
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        gen_traits: Rc<GenTraits>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        let current_proto_path = Rc::new(desc.current_path().to_owned());
        let fields = desc
            .non_oneof_fields()?
            .into_iter()
            .map(|desc| Field::try_new(desc, Rc::clone(&current_proto_path), options.clone()))
            .collect::<Result<Vec<_>>>()?;
        Self {
            struct_name: Self::struct_name(desc.name())?,
            options,
            gen_traits,
            fields,
        }
    }

    #[throws]
    pub fn gen_items(&self) -> Vec<Item> {
        vec![self.gen_struct()?, self.gen_view_wrapping_impl()?]
    }

    #[throws]
    fn struct_name(message_name: &str) -> Ident {
        to_ident(&convert_into_case(message_name, Case::CamelCase))
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
    fn gen_view_wrapping_impl(&self) -> Item {
        let struct_name = &self.struct_name;
        let t = format_ident!("T");
        let trait_name = self.gen_traits.view_trait_name();
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
            Field::Repeated(RepeatedField { scalar_proto_type: FieldType::Message(m), .. }) => {
                let wrapper_type = m
                    .to_relative_path(&field.base_proto_path())
                    .unwrap_or(m.as_ref())
                    .to_rust_path_with(options, |name| {
                        Ok(PathSegment {
                            ident: GenStruct::struct_name(name)?,
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
            Field::Explicit(ScalarField { scalar_proto_type: FieldType::Message(m), .. })
            | Field::Implicit(ScalarField { scalar_proto_type: FieldType::Message(m), .. }) => {
                let wrapper_type = m
                    .to_relative_path(&field.base_proto_path())
                    .unwrap_or(m.as_ref())
                    .to_rust_path_with(options, |name| {
                        Ok(PathSegment {
                            ident: GenStruct::struct_name(name)?,
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
