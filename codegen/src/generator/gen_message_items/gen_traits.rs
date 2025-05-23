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
use crate::{Result, ResultExt};
use ::puroro::Either;
use ::quote::{format_ident, quote};
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, Block, Ident, ImplItemFn, Item, Path, Type};
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

pub trait ImplsGenerator {
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>>;
}

fn impls_helper<'a, F, G>(
    fields: impl Iterator<Item = &'a Field>,
    gen_getter: F,
    gen_has_method: G,
    is_try_trait: bool,
) -> Result<Vec<ImplItemFn>>
where
    F: Fn(&Field) -> Result<Block>,
    G: Fn(&Field) -> Result<Block>,
{
    fields
        .map(|f| {
            let get_method: ImplItemFn = {
                let signature = if is_try_trait {
                    f.try_getter_signature()
                } else {
                    f.getter_signature()
                };
                let body = gen_getter(f)?;
                parse2(quote! {
                    #signature #body
                })?
            };
            let has_method: Option<ImplItemFn> = {
                if let Some(signature) = if is_try_trait {
                    f.try_has_method_signature_if_non_repeated()
                } else {
                    f.has_method_signature_if_non_repeated()
                } {
                    let body = gen_has_method(f)?;
                    Some(parse2(quote! {
                        #signature #body
                    })?)
                } else {
                    None
                }
            };
            Ok(once(get_method).chain(has_method.into_iter()))
        })
        .flat_map(ResultExt::transpose_iter)
        .collect::<Result<Vec<_>>>()
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
        let view_trait_def = self.gen_view_trait()?;
        let view_trait_name = &self.view_trait_name;
        let view_trait_path: Path = parse2(quote! { self::#view_trait_name })?;
        let try_trait_def = self.gen_try_view_trait()?;
        let try_trait_name = &self.try_view_trait_name;
        let try_trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators: Vec<Rc<dyn ImplsGenerator>> = vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
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

        Ok([view_trait_def, try_trait_def]
            .into_iter()
            .chain(blanket_impls)
            .collect())
    }

    fn gen_view_trait(&self) -> Result<Item> {
        let trait_name = &self.view_trait_name;
        let try_trait_name = &self.try_view_trait_name;
        let getters = self
            .fields
            .iter()
            .map(|f| f.getter_signature())
            .collect::<Vec<_>>();
        let has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit { has_method_signature, .. }
                | Field::Explicit { has_method_signature, .. } => Some(has_method_signature),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(parse2(quote! {
            pub trait #trait_name: self::#try_trait_name {
                #(#getters;)*
                #(#has_methods;)*
            }
        })?)
    }

    fn gen_try_view_trait(&self) -> Result<Item> {
        let trait_name = &self.try_view_trait_name;
        let try_getters = self
            .fields
            .iter()
            .map(|f| f.try_getter_signature())
            .collect::<Vec<_>>();
        let try_has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit { try_has_method_signature, .. }
                | Field::Explicit { try_has_method_signature, .. } => {
                    Some(try_has_method_signature)
                }
                _ => None,
            })
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

#[derive(Debug)]
pub enum Field {
    Repeated {
        number: i32,
        /// The protobuf-style absolute path of the parent scope (package or enclosing message),
        /// used as the base for generating relative paths to other items.
        base_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        try_getter_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
    Explicit {
        number: i32,
        base_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        has_method_signature: Signature,
        try_getter_signature: Signature,
        try_has_method_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
    Implicit {
        number: i32,
        base_proto_path: Rc<ProtoPathBuf>,
        getter_signature: Signature,
        has_method_signature: Signature,
        try_getter_signature: Signature,
        try_has_method_signature: Signature,
        scalar_proto_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    },
}

impl Field {
    pub fn getter_signature(&self) -> &Signature {
        match self {
            Field::Implicit { getter_signature, .. }
            | Field::Explicit { getter_signature, .. }
            | Field::Repeated { getter_signature, .. } => getter_signature,
        }
    }

    pub fn try_getter_signature(&self) -> &Signature {
        match self {
            Field::Implicit { try_getter_signature, .. }
            | Field::Explicit { try_getter_signature, .. }
            | Field::Repeated { try_getter_signature, .. } => try_getter_signature,
        }
    }

    pub fn has_method_signature_if_non_repeated(&self) -> Option<&Signature> {
        match self {
            Field::Implicit { has_method_signature, .. }
            | Field::Explicit { has_method_signature, .. } => Some(has_method_signature),
            _ => None,
        }
    }

    pub fn try_has_method_signature_if_non_repeated(&self) -> Option<&Signature> {
        match self {
            Field::Implicit { try_has_method_signature, .. }
            | Field::Explicit { try_has_method_signature, .. } => Some(try_has_method_signature),
            _ => None,
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
    pub fn new(
        desc: &FieldDescriptorExt,
        current_proto_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let lower_cased = convert_into_case(&desc.name(), Case::LowerSnakeCase);
        let presense = FieldPresense::from_field_desc(desc);
        let scalar_proto_type = desc.type_with_full_path()?;
        let number = desc.number();
        let base_proto_path = Rc::clone(&current_proto_path);
        Ok(Self {
            number,
            base_proto_path,
            current_proto_path,
            options,
            lower_cased,
            presense,
            scalar_proto_type,
        })
    }

    pub fn build(self) -> Result<Field> {
        let getter_signature = self.make_getter()?;
        let try_getter_signature = self.make_try_getter()?;
        let scalar_proto_type = self.scalar_proto_type.clone();
        let number = self.number;
        let base_proto_path = Rc::clone(&self.base_proto_path);
        match &self.presense {
            FieldPresense::Implicit => {
                let has_method_signature = self.make_has_method()?;
                let try_has_method_signature = self.make_try_has_method()?;
                Ok(Field::Implicit {
                    number,
                    base_proto_path,
                    getter_signature,
                    has_method_signature,
                    try_getter_signature,
                    try_has_method_signature,
                    scalar_proto_type,
                })
            }
            FieldPresense::Explicit => {
                let has_method_signature = self.make_has_method()?;
                let try_has_method_signature = self.make_try_has_method()?;
                Ok(Field::Explicit {
                    number,
                    base_proto_path,
                    getter_signature,
                    has_method_signature,
                    try_getter_signature,
                    try_has_method_signature,
                    scalar_proto_type,
                })
            }
            FieldPresense::Repeated => Ok(Field::Repeated {
                number,
                base_proto_path,
                getter_signature,
                try_getter_signature,
                scalar_proto_type,
            }),
        }
    }

    fn make_getter(&self) -> Result<Signature> {
        let name = to_ident(&format!("{}", &self.lower_cased));
        let scalar_ref_type = self.scalar_proto_type.gen_scalar_maybe_ref_type(
            &self.current_proto_path,
            None,
            &self.options,
        )?;
        let getter_type = match self.presense {
            FieldPresense::Repeated => parse2(quote! {
                impl ::puroro::repeated::RepeatedView<Item = #scalar_ref_type>
            })?,
            FieldPresense::Explicit | FieldPresense::Implicit => match self.scalar_proto_type {
                FieldType::Message(_) => self.options.option_type(&scalar_ref_type)?,
                _ => scalar_ref_type,
            },
        };
        let sig: Signature = parse2(quote! {
            fn #name(&self) -> #getter_type
        })?;
        Ok(sig)
    }

    fn make_has_method(&self) -> Result<Signature> {
        if let FieldPresense::Repeated = self.presense {
            Err("has method is not allowed for repeated fields".to_string())?
        }
        let name = to_ident(&format!("has_{}", &self.lower_cased));
        let bool_type = self.options.primitive_type("bool")?;
        let sig: Signature = parse2(quote! {
            fn #name(&self) -> #bool_type
        })?;
        Ok(sig)
    }

    fn make_try_getter(&self) -> Result<Signature> {
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
        Ok(sig)
    }

    fn make_try_has_method(&self) -> Result<Signature> {
        if let FieldPresense::Repeated = self.presense {
            Err("try_has method is not allowed for repeated fields".to_string())?
        }
        let name = to_ident(&format!("try_has_{}", &self.lower_cased));
        let has_result_type = self
            .options
            .result_type(&self.options.primitive_type("bool")?)?;
        let sig: Signature = parse2(quote! {
            fn #name(&self) -> #has_result_type
        })?;
        Ok(sig)
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
