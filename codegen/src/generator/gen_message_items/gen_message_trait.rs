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

use crate::cases::{convert_into_case, Case};
use crate::descriptor::{DescriptorExt, FieldDescriptorExt, FieldLabel, FieldType, FieldTypeCase};
use crate::generator::avoid_reserved_keywords;
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::quote::{format_ident, quote};
use ::std::iter;
use ::syn::{parse2, parse_str, Expr, Ident, Item, Path, Type};
use ::syn::{Lifetime, Signature};

pub struct GenTrait {
    rust_name: Ident,
    rust_mut_name: Ident,
    fields: Vec<Field>,
}

impl GenTrait {
    pub fn try_new<'a>(desc: &'a DescriptorExt<'a>) -> Result<Self> {
        Ok(Self {
            rust_name: Self::rust_name_from_message_name(desc.name())?,
            rust_mut_name: Self::rust_mut_name_from_message_name(desc.name())?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn rust_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}Trait",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_mut_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}MutTrait",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_path_from_proto_path(path: &ProtoPath) -> Result<Path> {
        path.to_rust_path_with(|s| {
            let ident = Self::rust_name_from_message_name(s)?;
            Ok(parse2(quote! { #ident })?)
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let trait_def = self.gen_message_trait()?;
        let trait_mut_def = self.gen_message_mut_trait()?;
        let mut blanket_impls = Vec::new();
        blanket_impls.append(&mut self.gen_blanket_ref_impls()?);
        blanket_impls.push(self.gen_blanket_option_impl()?);
        Ok([trait_def, trait_mut_def]
            .into_iter()
            .chain(blanket_impls)
            .collect())
    }

    fn gen_message_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let getters = self
            .fields
            .iter()
            .map(Field::gen_getter_signature)
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            pub trait #trait_name {
                #(#getters;)*
            }
        })?)
    }

    fn gen_message_mut_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_mut_name;
        let allocator_ident: Ident = parse_str("A")?;
        let allocator: Type = parse2(quote! { #allocator_ident})?;
        let methods = self
            .fields
            .iter()
            .map(|f| f.gen_mutator_signature(&allocator))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            pub trait #trait_name < #allocator_ident: ::std::alloc::Allocator > {
                #(#methods;)*
            }
        })?)
    }

    fn gen_blanket_ref_impls(&self) -> Result<Vec<Item>> {
        let trait_name = &self.rust_name;
        let trait_path: Path = parse2(quote! { self::#trait_name })?;
        let blanket_type: Ident = parse_str("T")?;
        let getter_signatures = self
            .fields
            .iter()
            .map(Field::gen_getter_signature)
            .collect::<Result<Vec<_>>>()?;
        let getter_bodies = self
            .fields
            .iter()
            .map(|f| f.gen_blanket_ref_getter_body(&blanket_type, &trait_path))
            .collect::<Result<Vec<_>>>()?;
        Ok(vec![
            parse2(quote! {
                impl<T: #trait_path> #trait_path for &T {
                    #(#getter_signatures {
                        #getter_bodies
                    })*
                }
            })?,
            parse2(quote! {
                impl<T: self::#trait_name> self::#trait_name for &mut T {
                    #(#getter_signatures {
                        #getter_bodies
                    })*
                }
            })?,
        ])
    }

    fn gen_blanket_option_impl(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let trait_path: Path = parse2(quote! { self::#trait_name })?;
        let blanket_type: Ident = parse_str("T")?;
        let getter_signatures = self
            .fields
            .iter()
            .map(Field::gen_getter_signature)
            .collect::<Result<Vec<_>>>()?;
        let getter_bodies = self
            .fields
            .iter()
            .map(|f| f.gen_blanket_option_getter_body(&blanket_type, &trait_path))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            impl<T: #trait_path> #trait_path for ::std::option::Option<T> {
                #(#getter_signatures {
                    #getter_bodies
                })*
            }
        })?)
    }
}

pub struct Field {
    original_name: String,
    wrapper: FieldWrapper,
    scalar_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl Field {
    pub fn try_new<'a>(desc: &'a FieldDescriptorExt<'a>) -> Result<Self> {
        Ok(Self {
            original_name: desc.name().to_string(),
            wrapper: FieldWrapper::from_field_desc(desc),
            scalar_type: desc.type_with_full_path()?,
        })
    }

    pub fn scalar_type(&self) -> FieldType<&ProtoPath, &ProtoPath> {
        self.scalar_type.as_deref()
    }

    // Getters

    fn gen_getter_name(&self) -> Result<Ident> {
        let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
        Ok(parse_str(&avoid_reserved_keywords(&lower_cased))?)
    }

    pub fn gen_getter_signature(&self) -> Result<Signature> {
        let getter_name = self.gen_getter_name()?;
        let lifetime: Option<Lifetime> = Some(parse_str("'_")?);
        let getter_type = match self.wrapper {
            FieldWrapper::Vec => self
                .scalar_type
                .gen_repeated_getter_type(lifetime.as_ref())?,
            FieldWrapper::Optional | FieldWrapper::OptionalBoxed => self
                .scalar_type
                .gen_optional_getter_type(lifetime.as_ref())?,
            FieldWrapper::Bare => self.scalar_type.gen_bare_getter_type(lifetime.as_ref())?,
        };
        Ok(parse2(quote! {
            fn #getter_name(&self) -> #getter_type
        })?)
    }

    fn gen_blanket_ref_getter_body(&self, blanket_type: &Ident, trait_path: &Path) -> Result<Expr> {
        let getter_name = self.gen_getter_name()?;
        Ok(parse2(quote! {
            <#blanket_type as #trait_path>::#getter_name(self)
        })?)
    }

    fn gen_blanket_option_getter_body(
        &self,
        blanket_type: &Ident,
        trait_path: &Path,
    ) -> Result<Expr> {
        let getter_name = self.gen_getter_name()?;
        Ok(parse2(match self.wrapper {
            FieldWrapper::Vec => quote! {
                self.as_ref().map(<#blanket_type as #trait_path>::#getter_name).into_iter().flatten()
            },
            FieldWrapper::Optional | FieldWrapper::OptionalBoxed => quote! {
                self.as_ref().and_then(<#blanket_type as #trait_path>::#getter_name)
            },
            FieldWrapper::Bare => quote! {
                self.as_ref().map(<#blanket_type as #trait_path>::#getter_name).unwrap_or_default()
            },
        })?)
    }

    // Appendings

    // Mutators

    fn gen_mutator_name(&self) -> Result<Ident> {
        let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
        Ok(parse_str(&format!(
            "{}_mut",
            avoid_reserved_keywords(&lower_cased)
        ))?)
    }

    pub fn gen_mutator_signature(&self, allocator: &Type) -> Result<Signature> {
        let mutator_name = self.gen_mutator_name()?;
        let mutator_type = match self.wrapper {
            FieldWrapper::Vec => parse2(quote! { () /* TODO */ })?,
            _ => self.scalar_type.gen_non_repeated_mutator_type(allocator)?,
        };
        Ok(parse2(quote! {
            fn #mutator_name(&mut self) -> #mutator_type
        })?)
    }

    // Others

    pub fn wrapper(&self) -> FieldWrapper {
        self.wrapper
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldWrapper {
    Bare,
    Optional,
    OptionalBoxed,
    Vec,
}

impl FieldWrapper {
    fn from_field_desc<'a>(field: &'a FieldDescriptorExt<'a>) -> Self {
        if field.has_presence() {
            if field.type_case() == Some(FieldTypeCase::Message) {
                FieldWrapper::OptionalBoxed
            } else {
                FieldWrapper::Optional
            }
        } else if field.label() == Some(FieldLabel::Repeated) {
            FieldWrapper::Vec
        } else {
            FieldWrapper::Bare
        }
    }
}

impl FieldType<ProtoPathBuf, ProtoPathBuf> {
    fn gen_bare_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        let lifetime_iter = lifetime.iter();
        Ok(match self {
            FieldType::Message(path) => {
                let path = path.to_rust_path_with(|name| {
                    let ident = GenTrait::rust_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { impl #(#lifetime_iter +)* #path })?
            }
            FieldType::Enum(path) => {
                let path = path.to_rust_path()?;
                parse2(quote! { #path })?
            }
            FieldType::Int32 => parse_str("::std::primitive::i32")?,
            FieldType::Int64 => parse_str("::std::primitive::i64")?,
            FieldType::UInt32 => parse_str("::std::primitive::u32")?,
            FieldType::UInt64 => parse_str("::std::primitive::u64")?,
            FieldType::SInt32 => parse_str("::std::primitive::i32")?,
            FieldType::SInt64 => parse_str("::std::primitive::i64")?,
            FieldType::Fixed32 => parse_str("::std::primitive::u32")?,
            FieldType::Fixed64 => parse_str("::std::primitive::u64")?,
            FieldType::SFixed32 => parse_str("::std::primitive::i32")?,
            FieldType::SFixed64 => parse_str("::std::primitive::i64")?,
            FieldType::Float => parse_str("::std::primitive::f32")?,
            FieldType::Double => parse_str("::std::primitive::f64")?,
            FieldType::Bool => parse_str("::std::primitive::bool")?,
            FieldType::String => parse2(quote! { & #lifetime ::std::primitive::str })?,
            FieldType::Bytes => parse2(quote! { & #lifetime [::std::primitive::u8] })?,
            FieldType::Group => Err(format!("Group field is not supported"))?,
        })
    }
    fn gen_optional_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(lifetime)?;
        Ok(parse2(quote! {
            ::std::option::Option::< #bare_type >
        })?)
    }
    fn gen_repeated_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(lifetime)?;
        Ok(parse2(quote! {
            impl ::std::iter::Iterator::<Item = #bare_type >
        })?)
    }

    fn gen_bare_mutator_type(&self, allocator: &Type) -> Result<Type> {
        Ok(match self {
            FieldType::Message(path) => {
                let path = path.to_rust_path_with(|name| {
                    let ident = GenTrait::rust_mut_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { impl #path<#allocator> })?
            }
            FieldType::Enum(path) => {
                let path = path.to_rust_path()?;
                parse2(quote! { #path })?
            }
            FieldType::Int32 => parse_str("::std::primitive::i32")?,
            FieldType::Int64 => parse_str("::std::primitive::i64")?,
            FieldType::UInt32 => parse_str("::std::primitive::u32")?,
            FieldType::UInt64 => parse_str("::std::primitive::u64")?,
            FieldType::SInt32 => parse_str("::std::primitive::i32")?,
            FieldType::SInt64 => parse_str("::std::primitive::i64")?,
            FieldType::Fixed32 => parse_str("::std::primitive::u32")?,
            FieldType::Fixed64 => parse_str("::std::primitive::u64")?,
            FieldType::SFixed32 => parse_str("::std::primitive::i32")?,
            FieldType::SFixed64 => parse_str("::std::primitive::i64")?,
            FieldType::Float => parse_str("::std::primitive::f32")?,
            FieldType::Double => parse_str("::std::primitive::f64")?,
            FieldType::Bool => parse_str("::std::primitive::bool")?,
            FieldType::String => parse_str("::std::string::String")?,
            FieldType::Bytes => {
                parse2(quote! { ::std::vec::Vec<::std::primitive::u8, #allocator> })?
            }
            FieldType::Group => Err(format!("Group field is not supported"))?,
        })
    }
    fn gen_non_repeated_mutator_type(&self, allocator: &Type) -> Result<Type> {
        let bare_type = self.gen_bare_mutator_type(allocator)?;
        Ok(parse2(quote! {
            impl ::std::ops::DerefMut<Target = #bare_type>
        })?)
    }
}
