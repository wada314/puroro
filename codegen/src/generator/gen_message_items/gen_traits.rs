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

mod blanket_both;
mod blanket_either;
mod blanket_either_or_both;
mod blanket_option;
mod blanket_ref;

use crate::cases::{convert_into_case, Case};
use crate::descriptor::{DescriptorExt, FieldDescriptorExt, FieldLabel, FieldType, LenType};
use crate::generator::{to_ident, CodeGeneratorOptions};
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::itertools::Itertools;
use ::puroro::Either;
use ::quote::{format_ident, quote};
use ::std::cell::OnceCell;
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Expr, Ident, Item, Path, Type};
use ::syn::{Lifetime, Signature};
use blanket_both::GenBlanketBothImpls;
use blanket_either::GenBlanketEitherImpls;
use blanket_either_or_both::GenBlanketEitherOrBothImpls;
use blanket_option::GenBlanketOptionImpls;
use blanket_ref::GenBlanketRefImpls;

pub struct GenTraits {
    rust_name: Ident,
    fields: Vec<Field>,
    options: Rc<CodeGeneratorOptions>,
    try_getter_signatures: OnceCell<Vec<Signature>>,
    try_has_method_signatures: OnceCell<Vec<Signature>>,
}

pub trait BlanketImplsGenerator {
    fn generate<'a>(
        &self,
        trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>>;
}

impl GenTraits {
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let current_path = Rc::new(desc.current_path().to_owned());
        Ok(Self {
            rust_name: Self::rust_name_from_message_name(desc.name())?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|f| Field::try_new(f, Rc::clone(&current_path), Rc::clone(&options)))
                .collect::<Result<Vec<_>>>()?,
            options,
            try_getter_signatures: OnceCell::new(),
            try_has_method_signatures: OnceCell::new(),
        })
    }

    pub fn rust_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}Trait",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_path_from_proto_path(self, path: &ProtoPath) -> Result<Path> {
        path.to_rust_path_with(&self.options, |s| {
            let ident = Self::rust_name_from_message_name(s)?;
            Ok(parse2(quote! { #ident })?)
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let trait_def = self.gen_message_trait()?;
        let try_trait_name = &self.rust_name;
        let trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators: Vec<Rc<dyn BlanketImplsGenerator>> = vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
        ];
        let blanket_impls = blanket_impl_generators
            .iter()
            .map(|g| g.generate(&trait_path, Box::new(self.fields.iter())))
            .map(|r| match r {
                Ok(vec) => Either::Left(vec.into_iter().map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        Ok(once(trait_def).chain(blanket_impls).collect())
    }

    fn gen_message_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let try_getters = self
            .fields
            .iter()
            .map(|f| &f.try_getter_signature)
            .collect::<Vec<_>>();
        let try_has_methods = self
            .fields
            .iter()
            .filter_map(|f| f.try_has_method_signature.as_ref())
            .collect::<Vec<_>>();
        Ok(parse2(quote! {
            pub trait #trait_name {
                #(#try_getters;)*
                #(#try_has_methods;)*
            }
        })?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPresense {
    Implicit,
    Explicit,
    Repeated,
}

impl FieldPresense {
    fn from_field_desc<'a>(field: &'a FieldDescriptorExt<'a>) -> Self {
        if field.has_presence() {
            FieldPresense::Explicit
        } else if field.label() == FieldLabel::Repeated {
            FieldPresense::Repeated
        } else {
            FieldPresense::Implicit
        }
    }
}

impl<M: AsRef<ProtoPath>, E: AsRef<ProtoPath>> FieldType<M, E> {
    pub fn gen_scalar_maybe_ref_type(
        &self,
        current_path: &ProtoPath,
        lifetime: Option<&Lifetime>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let lifetime = lifetime.iter();
        match self
            .as_ref()
            .maybe_into_primitive_type(current_path, options)?
        {
            Ok(primitive_type) => Ok(primitive_type),
            Err(len_type) => match len_type {
                LenType::Message(path) => {
                    let path = path
                        .as_ref()
                        .to_relative_path(current_path)
                        .unwrap_or(path.as_ref());
                    let path = path.to_rust_path_with(options, |name| {
                        let ident = GenTraits::rust_name_from_message_name(name)?;
                        Ok(parse2(quote! { #ident })?)
                    })?;
                    Ok(parse2(quote! { impl #(#lifetime +)* #path })?)
                }
                LenType::String => {
                    let str_type = options.primitive_type("str")?;
                    Ok(parse2(quote! { & #(#lifetime)* #str_type })?)
                }
                LenType::Bytes => {
                    let u8_type = options.primitive_type("u8")?;
                    Ok(parse2(quote! { & #(#lifetime)* [#u8_type] })?)
                }
            },
        }
    }

    pub fn gen_scalar_owned_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        match self
            .as_ref()
            .maybe_into_primitive_type(current_path, options)?
        {
            Ok(primitive_type) => Ok(primitive_type),
            Err(len_type) => match len_type {
                LenType::Message(path) => {
                    let path = path
                        .as_ref()
                        .to_relative_path(current_path)
                        .unwrap_or(path.as_ref());
                    let path = path.to_rust_path_with(options, |name| {
                        let ident = GenTraits::rust_mut_name_from_message_name(name)?;
                        Ok(parse2(quote! { #ident })?)
                    })?;
                    Ok(parse2(quote! { impl #path<#allocator> })?)
                }
                LenType::String => Ok(parse2(quote! { ::puroro::string::String<#allocator> })?),
                LenType::Bytes => {
                    let u8_type = options.primitive_type("u8")?;
                    Ok(options.vec_type(&u8_type, Some(allocator))?)
                }
            },
        }
    }

    pub fn gen_scalar_nonzero_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let scalar_owned_type = self.gen_scalar_owned_type(current_path, allocator, options)?;
        if matches!(self, FieldType::Message(_)) {
            Ok(scalar_owned_type)
        } else {
            Ok(parse2(quote! { ::puroro::NonEmpty<#scalar_owned_type> })?)
        }
    }
}

pub struct Field {
    try_getter_name: Ident,
    try_has_method_name: Option<Ident>,
    try_getter_signature: Signature,
    try_has_method_signature: Option<Signature>,
    presense: FieldPresense,
    scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl Field {
    pub fn try_new<'a>(
        desc: &'a FieldDescriptorExt<'a>,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let lower_cased = convert_into_case(&desc.name(), Case::LowerSnakeCase);
        let presense = FieldPresense::from_field_desc(desc);
        let scalar_proto_type = desc.type_with_full_path()?;

        let try_getter_name = to_ident(&format!("try_{}", &lower_cased));
        let try_has_method_name = if let FieldPresense::Repeated = presense {
            None
        } else {
            Some(to_ident(&format!("try_has_{}", &lower_cased)))
        };

        let scalar_ref_type =
            scalar_proto_type.gen_scalar_maybe_ref_type(&current_proto_path, None, &options)?;
        let getter_type = match presense {
            FieldPresense::Repeated => {
                let item_type = options.result_type(&scalar_ref_type)?;
                parse2(quote! {
                    impl ::puroro::repeated::RepeatedView<Item = #item_type>
                })?
            }
            FieldPresense::Explicit | FieldPresense::Implicit => match scalar_proto_type {
                FieldType::Message(_) => options.option_type(&scalar_ref_type)?,
                _ => scalar_ref_type,
            },
        };
        let getter_result_type = options.result_type(&getter_type)?;
        let try_getter_signature: Signature = parse2(quote! {
            fn #try_getter_name(&self) -> #getter_result_type
        })?;
        let has_result_type = options.result_type(&options.primitive_type("bool")?)?;
        let try_has_method_signature: Option<Signature> = match presense {
            FieldPresense::Repeated => None,
            FieldPresense::Explicit | FieldPresense::Implicit => Some(parse2(
                quote! { fn #try_has_method_name(&self) -> #has_result_type },
            )?),
        };
        Ok(Self {
            try_getter_name,
            try_has_method_name,
            try_getter_signature,
            try_has_method_signature,
            presense,
            scalar_proto_type,
        })
    }
}
