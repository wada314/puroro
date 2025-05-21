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
use ::puroro::Either;
use ::quote::{format_ident, quote};
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, Ident, Item, Path, Type};
use ::syn::{Lifetime, Signature};
use blanket_both::GenBlanketBothImpls;
use blanket_either::GenBlanketEitherImpls;
use blanket_either_or_both::GenBlanketEitherOrBothImpls;
use blanket_option::GenBlanketOptionImpls;
use blanket_ref::GenBlanketRefImpls;

pub struct GenTraits {
    view_trait_name: Ident,
    try_view_trait_name: Ident,
    fields: Vec<Field>,
    options: Rc<CodeGeneratorOptions>,
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
            view_trait_name: Self::view_trait_name(desc.name())?,
            try_view_trait_name: Self::try_view_trait_name(desc.name())?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|f| Field::try_new(f, Rc::clone(&current_path), Rc::clone(&options)))
                .collect::<Result<Vec<_>>>()?,
            options,
        })
    }

    pub fn view_trait_name(message_name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}View",
            convert_into_case(message_name, Case::CamelCase)
        ))
    }

    pub fn try_view_trait_name(message_name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "Try{}View",
            convert_into_case(message_name, Case::CamelCase)
        ))
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let try_trait_def = self.gen_try_view_trait()?;
        let try_trait_name = &self.try_view_trait_name;
        let try_trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators: Vec<Rc<dyn BlanketImplsGenerator>> = vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
        ];
        let blanket_impls = blanket_impl_generators
            .iter()
            .map(|g| g.generate(&try_trait_path, Box::new(self.fields.iter())))
            .map(|r| match r {
                Ok(vec) => Either::Left(vec.into_iter().map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        Ok(once(try_trait_def).chain(blanket_impls).collect())
    }

    fn gen_try_view_trait(&self) -> Result<Item> {
        let trait_name = &self.try_view_trait_name;
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
    fn from_field_desc(field: &FieldDescriptorExt) -> Self {
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
                        let ident = GenTraits::try_view_trait_name(name)?;
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
}

pub struct Field {
    pub getter_name: Ident,
    pub getter_signature: Signature,
    pub has_method_name: Option<Ident>,
    pub has_method_signature: Option<Signature>,
    pub try_getter_name: Ident,
    pub try_getter_signature: Signature,
    pub try_has_method_name: Option<Ident>,
    pub try_has_method_signature: Option<Signature>,
    pub presense: FieldPresense,
    pub scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

struct FieldFactory {
    current_proto_path: Rc<ProtoPathBuf>,
    options: Rc<CodeGeneratorOptions>,
    lower_cased: String,
    presense: FieldPresense,
    scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl FieldFactory {
    pub fn new(
        desc: &FieldDescriptorExt,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let lower_cased = convert_into_case(&desc.name(), Case::LowerSnakeCase);
        let presense = FieldPresense::from_field_desc(desc);
        let scalar_proto_type = desc.type_with_full_path()?;
        Ok(Self {
            current_proto_path,
            options,
            lower_cased,
            presense,
            scalar_proto_type,
        })
    }

    pub fn build(self) -> Result<Field> {
        let (getter_name, getter_signature) = self.make_getter()?;
        let (has_method_name, has_method_signature) = self
            .make_has_method()?
            .map(|(n, s)| (Some(n), Some(s)))
            .unwrap_or((None, None));
        let (try_getter_name, try_getter_signature) = self.make_try_getter()?;
        let (try_has_method_name, try_has_method_signature) = self
            .make_try_has_method()?
            .map(|(n, s)| (Some(n), Some(s)))
            .unwrap_or((None, None));
        Ok(Field {
            getter_name,
            getter_signature,
            has_method_name,
            has_method_signature,
            try_getter_name,
            try_getter_signature,
            try_has_method_name,
            try_has_method_signature,
            presense: self.presense,
            scalar_proto_type: self.scalar_proto_type,
        })
    }

    fn make_getter(&self) -> Result<(Ident, Signature)> {
        let name = to_ident(&format!("{}", &self.lower_cased));
        let scalar_ref_type = self.scalar_proto_type.gen_scalar_maybe_ref_type(
            &self.current_proto_path,
            None,
            &self.options,
        )?;
        let getter_type = match self.presense {
            FieldPresense::Repeated => {
                let item_type = self.options.result_type(&scalar_ref_type)?;
                parse2(quote! {
                    impl ::puroro::repeated::RepeatedView<Item = #item_type>
                })?
            }
            FieldPresense::Explicit | FieldPresense::Implicit => match self.scalar_proto_type {
                FieldType::Message(_) => self.options.option_type(&scalar_ref_type)?,
                _ => scalar_ref_type,
            },
        };
        let getter_result_type = self.options.result_type(&getter_type)?;
        let sig: Signature = parse2(quote! {
            fn #name(&self) -> #getter_result_type
        })?;
        Ok((name, sig))
    }

    fn make_has_method(&self) -> Result<Option<(Ident, Signature)>> {
        if let FieldPresense::Repeated = self.presense {
            Ok(None)
        } else {
            let name = to_ident(&format!("has_{}", &self.lower_cased));
            let has_result_type = self
                .options
                .result_type(&self.options.primitive_type("bool")?)?;
            let sig: Signature = parse2(quote! {
                fn #name(&self) -> #has_result_type
            })?;
            Ok(Some((name, sig)))
        }
    }

    fn make_try_getter(&self) -> Result<(Ident, Signature)> {
        let name = to_ident(&format!("try_{}", &self.lower_cased));
        let scalar_ref_type = self.scalar_proto_type.gen_scalar_maybe_ref_type(
            &self.current_proto_path,
            None,
            &self.options,
        )?;
        let getter_type = match self.presense {
            FieldPresense::Repeated => {
                let item_type = self.options.result_type(&scalar_ref_type)?;
                parse2(quote! {
                    impl ::puroro::repeated::RepeatedView<Item = #item_type>
                })?
            }
            FieldPresense::Explicit | FieldPresense::Implicit => match self.scalar_proto_type {
                FieldType::Message(_) => self.options.option_type(&scalar_ref_type)?,
                _ => scalar_ref_type,
            },
        };
        let getter_result_type = self.options.result_type(&getter_type)?;
        let sig: Signature = parse2(quote! {
            fn #name(&self) -> #getter_result_type
        })?;
        Ok((name, sig))
    }

    fn make_try_has_method(&self) -> Result<Option<(Ident, Signature)>> {
        if let FieldPresense::Repeated = self.presense {
            Ok(None)
        } else {
            let name = to_ident(&format!("try_has_{}", &self.lower_cased));
            let has_result_type = self
                .options
                .result_type(&self.options.primitive_type("bool")?)?;
            let sig: Signature = parse2(quote! {
                fn #name(&self) -> #has_result_type
            })?;
            Ok(Some((name, sig)))
        }
    }
}

impl Field {
    pub fn try_new<'a>(
        desc: &'a FieldDescriptorExt<'a>,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        FieldFactory::new(desc, current_proto_path, options)?.build()
    }
}
