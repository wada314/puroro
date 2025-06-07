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

use super::gen_traits::GenTraits;
use crate::ErrorKind;
use crate::Result;
use crate::cases::{Case, convert_into_case};
use crate::descriptor::{DescriptorExt, FieldDescriptorExt, FieldType, LenType};
use crate::generator::{CodeGeneratorOptions, FieldPresense, to_ident};
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use ::culpa::throws;
use ::quote::{format_ident, quote};
use ::syn::parse::Parser;
use ::syn::{
    Block, Expr, ExprPath, GenericArgument, GenericParam, Ident, Item, Lifetime, PathArguments,
    PathSegment, Signature, Stmt, Type, TypePath, parse2,
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
            .map(|field| field.getter_signature())
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
                #(#getter_signatures {
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
        let getter_name = &field.getter_signature().ident;
        let body_tokens = match field {
            Field::Repeated { scalar_proto_type: FieldType::Message(m), .. } => {
                let wrapper_type = m
                    .to_relative_path(&field.current_proto_path())
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
            Field::Explicit { scalar_proto_type: FieldType::Message(m), .. }
            | Field::Implicit { scalar_proto_type: FieldType::Message(m), .. } => {
                let wrapper_type = m
                    .to_relative_path(&field.current_proto_path())
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

#[derive(Debug)]
enum Field {
    Explicit {
        current_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
    Implicit {
        current_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
    Repeated {
        current_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
}

impl Field {
    #[throws]
    fn try_new<'a>(
        desc: &'a FieldDescriptorExt<'a>,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        FieldFactory::new(desc, current_proto_path, options)?.build()?
    }

    fn current_proto_path(&self) -> &ProtoPathBuf {
        match self {
            Field::Explicit { current_proto_path, .. }
            | Field::Implicit { current_proto_path, .. }
            | Field::Repeated { current_proto_path, .. } => current_proto_path,
        }
    }

    fn getter_signature(&self) -> &Signature {
        match self {
            Field::Explicit { getter_signature, .. }
            | Field::Implicit { getter_signature, .. }
            | Field::Repeated { getter_signature, .. } => getter_signature,
        }
    }

    fn scalar_proto_type(&self) -> &FieldType<ProtoPathBuf, ProtoPathBuf> {
        match self {
            Field::Explicit { scalar_proto_type, .. }
            | Field::Implicit { scalar_proto_type, .. }
            | Field::Repeated { scalar_proto_type, .. } => scalar_proto_type,
        }
    }
}

struct FieldFactory {
    number: i32,
    base_proto_path: Rc<ProtoPathBuf>,
    current_proto_path: Rc<ProtoPathBuf>,
    options: Rc<CodeGeneratorOptions>,
    lower_cased: String,
    presense: FieldPresense,
    scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl FieldFactory {
    #[throws]
    pub fn new(
        desc: &FieldDescriptorExt,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        let lower_cased = convert_into_case(&desc.name(), Case::LowerSnakeCase);
        let presense = FieldPresense::from_field_desc(desc);
        let scalar_proto_type = desc.type_with_full_path()?;
        let number = desc.number();
        let base_proto_path = Rc::clone(&current_proto_path);
        Self {
            number,
            base_proto_path,
            current_proto_path,
            options,
            lower_cased,
            presense,
            scalar_proto_type,
        }
    }

    #[throws]
    pub fn build(self) -> Field {
        let getter_signature = self.gen_getter_signature()?;
        let scalar_proto_type = self.scalar_proto_type;
        let current_proto_path = self.current_proto_path;
        match &self.presense {
            FieldPresense::Explicit => {
                Field::Explicit { current_proto_path, getter_signature, scalar_proto_type }
            }
            FieldPresense::Implicit => {
                Field::Implicit { current_proto_path, getter_signature, scalar_proto_type }
            }
            FieldPresense::Repeated => {
                Field::Repeated { current_proto_path, getter_signature, scalar_proto_type }
            }
        }
    }

    #[throws]
    fn gen_getter_signature<'a>(
        &self,
        outer_generic_params: impl Iterator<Item = &'a GenericParam>,
    ) -> Signature {
        let name = to_ident(&format!("{}", &self.lower_cased));
        let scalar_ref_type = gen_scalar_maybe_ref_type(
            &self.scalar_proto_type,
            &self.current_proto_path,
            None,
            outer_generic_params,
            &self.options,
        )?;
        let repeated_view_trait = self.options.puroro_repeated_view_trait(&scalar_ref_type);
        let getter_type = match self.presense {
            FieldPresense::Repeated => parse2(quote! {
                impl #repeated_view_trait
            })?,
            FieldPresense::Explicit | FieldPresense::Implicit => match self.scalar_proto_type {
                FieldType::Message(_) => self.options.option_type(&scalar_ref_type),
                _ => scalar_ref_type,
            },
        };
        parse2(quote! {
            fn #name(&self) -> #getter_type
        })?
    }
}

#[throws]
fn gen_scalar_maybe_ref_type<'a, M, E>(
    field_type: &FieldType<M, E>,
    current_path: &ProtoPath,
    lifetime: Option<&Lifetime>,
    outer_generic_params: impl Iterator<Item = &'a GenericParam>,
    options: &CodeGeneratorOptions,
) -> Type
where
    M: AsRef<ProtoPath>,
    E: AsRef<ProtoPath>,
{
    let lifetime = lifetime.iter();
    match field_type
        .as_ref()
        .maybe_into_primitive_type(current_path, options)
    {
        Ok(primitive_type) => primitive_type,
        Err(len_type) => {
            match len_type {
                LenType::Message(path) => {
                    let path = path
                        .as_ref()
                        .to_relative_path(current_path)
                        .unwrap_or(path.as_ref());
                    let view_trait_path = path.to_rust_path_with(options, |name| {
                        let ident = GenTraits::gen_view_trait_name(name)?;
                        Ok(parse2(quote! { #ident })?)
                    })?;
                    let use_bound_params = outer_generic_params
                        .map(|param| match param {
                            GenericParam::Type(type_param) => {
                                GenericArgument::Type(Type::Path(TypePath {
                                    qself: None,
                                    path: type_param.ident.clone().into(),
                                }))
                            }
                            GenericParam::Lifetime(lt_param) => {
                                GenericArgument::Lifetime(lt_param.lifetime.clone())
                            }
                            GenericParam::Const(const_param) => {
                                GenericArgument::Const(Expr::Path(ExprPath {
                                    attrs: vec![],
                                    qself: None,
                                    path: const_param.ident.clone().into(),
                                }))
                            }
                        })
                        .collect::<Vec<_>>();
                    let struct_path = path.to_rust_path_with(options, |name| {
                    let ident = GenStruct::struct_name(name)?;
                    Ok(parse2(quote! { #ident :: <impl #view_trait_path + use<#(#use_bound_params)*>> })?)
                })?;
                    parse2(quote! { #struct_path })?
                }
                LenType::String => {
                    let str_type = options.primitive_type("str");
                    parse2(quote! { & #(#lifetime)* #str_type })?
                }
                LenType::Bytes => {
                    let u8_type = options.primitive_type("u8");
                    parse2(quote! { & #(#lifetime)* [#u8_type] })?
                }
            }
        }
    }
}
