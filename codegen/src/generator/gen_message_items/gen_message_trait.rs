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
use crate::descriptor::{DescriptorExt, FieldDescriptorExt, FieldLabel, FieldType};
use crate::generator::{to_ident, CodeGeneratorOptions};
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::itertools::Itertools;
use ::quote::{format_ident, quote};
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Expr, Ident, Item, Path, Type};
use ::syn::{Lifetime, Signature};

pub struct GenTrait {
    rust_name: Ident,
    rust_mut_name: Ident,
    rust_app_name: Ident,
    fields: Vec<Field>,
    options: Rc<CodeGeneratorOptions>,
}

impl GenTrait {
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let current_path = Rc::new(desc.current_path().to_owned());
        Ok(Self {
            rust_name: Self::rust_name_from_message_name(desc.name())?,
            rust_mut_name: Self::rust_mut_name_from_message_name(desc.name())?,
            rust_app_name: Self::rust_app_name_from_message_name(desc.name())?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|f| Field::try_new(f, Rc::clone(&current_path), Rc::clone(&options)))
                .collect::<Result<Vec<_>>>()?,
            options,
        })
    }

    pub fn rust_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}Trait",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_app_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}AppTrait",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_mut_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}MutTrait",
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
        let trait_app_def = self.gen_message_app_trait()?;
        let trait_mut_def = self.gen_message_mut_trait()?;
        let mut blanket_impls = Vec::new();
        blanket_impls.append(&mut self.gen_blanket_ref_impls()?);
        blanket_impls.push(self.gen_blanket_option_impl()?);
        Ok([trait_def, trait_app_def, trait_mut_def]
            .into_iter()
            .chain(blanket_impls)
            .collect())
    }

    fn gen_message_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let getters = self
            .fields
            .iter()
            .map(Field::gen_get_method_signature)
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            pub trait #trait_name {
                #(#getters;)*
            }
        })?)
    }

    fn gen_message_app_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_app_name;
        let base_trait_name = &self.rust_name;
        let base_trait_path = self
            .options
            .path_in_self_module(&base_trait_name.clone().into())?;
        let allocator_ident: Ident = parse_str("A")?;
        let allocator: Type = parse2(quote! { #allocator_ident})?;
        let methods = self
            .fields
            .iter()
            .map(|f| f.gen_appendale_methods_signatures(&allocator))
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            pub trait #trait_name < #allocator_ident: ::std::alloc::Allocator >
                : #base_trait_path
            {
                #(#methods;)*
            }
        })?)
    }

    fn gen_message_mut_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_mut_name;
        let base_trait_name = &self.rust_app_name;
        let base_trait_path = self
            .options
            .path_in_self_module(&base_trait_name.clone().into())?;
        let allocator_ident: Ident = parse_str("A")?;
        let allocator: Type = parse2(quote! { #allocator_ident})?;
        let methods = self
            .fields
            .iter()
            .map(|f| f.gen_mutable_methods_signatures(&allocator))
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            pub trait #trait_name < #allocator_ident: ::std::alloc::Allocator >
                : #base_trait_path < #allocator_ident >
            {
                #(#methods;)*
            }
        })?)
    }

    fn gen_blanket_ref_impls(&self) -> Result<Vec<Item>> {
        let trait_name = &self.rust_name;
        let trait_path: Path = self
            .options
            .path_in_self_module(&trait_name.clone().into())?;
        let blanket_type: Ident = parse_str("T")?;
        let getter_signatures = self
            .fields
            .iter()
            .map(Field::gen_get_method_signature)
            .collect::<Result<Vec<_>>>()?;
        let getter_bodies = self
            .fields
            .iter()
            .map(|f| f.gen_blanket_ref_get_method_body(&blanket_type, &trait_path))
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
                impl<T: self::#trait_name> #trait_path for &mut T {
                    #(#getter_signatures {
                        #getter_bodies
                    })*
                }
            })?,
        ])
    }

    fn gen_blanket_option_impl(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let trait_path = self
            .options
            .path_in_self_module(&trait_name.clone().into())?;
        let blanket_type_ident: Ident = parse_str("T")?;
        let blanket_type = parse2(quote! { #blanket_type_ident })?;
        let blanket_opt_type = self.options.option_type(&blanket_type)?;
        let getter_signatures = self
            .fields
            .iter()
            .map(Field::gen_get_method_signature)
            .collect::<Result<Vec<_>>>()?;
        let getter_bodies = self
            .fields
            .iter()
            .map(|f| f.gen_blanket_option_get_method_body(&blanket_type_ident, &trait_path))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            impl<T: #trait_path> #trait_path for #blanket_opt_type {
                #(#getter_signatures {
                    #getter_bodies
                })*
            }
        })?)
    }
}

pub struct Field {
    original_name: String,
    wrapper: FieldPresense,
    current_path: Rc<ProtoPathBuf>,
    scalar_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
    options: Rc<CodeGeneratorOptions>,
}

impl Field {
    pub fn try_new<'a>(
        desc: &'a FieldDescriptorExt<'a>,
        current_path: Rc<ProtoPathBuf>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        Ok(Self {
            original_name: desc.name().to_string(),
            wrapper: FieldPresense::from_field_desc(desc),
            current_path,
            scalar_type: desc.type_with_full_path()?,
            options,
        })
    }

    pub fn scalar_type(&self) -> FieldType<&ProtoPath, &ProtoPath> {
        self.scalar_type.as_deref()
    }

    pub fn wrapper(&self) -> FieldPresense {
        self.wrapper
    }

    // Getters

    fn gen_get_method_name(&self) -> Result<Ident> {
        let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
        Ok(to_ident(&lower_cased))
    }

    pub fn gen_get_method_signature(&self) -> Result<Signature> {
        let getter_name = self.gen_get_method_name()?;
        let lifetime: Option<Lifetime> = None;
        let getter_type = match self.wrapper {
            FieldPresense::Repeated => self.scalar_type.gen_repeated_getter_type(
                &self.current_path,
                lifetime.as_ref(),
                &self.options,
            )?,
            FieldPresense::Explicit => self.scalar_type.gen_optional_getter_type(
                &self.current_path,
                lifetime.as_ref(),
                &self.options,
            )?,
            FieldPresense::Implicit => self.scalar_type.gen_bare_getter_type(
                &self.current_path,
                lifetime.as_ref(),
                &self.options,
            )?,
        };
        Ok(parse2(quote! {
            fn #getter_name(&self) -> #getter_type
        })?)
    }

    fn gen_blanket_ref_get_method_body(
        &self,
        blanket_type: &Ident,
        trait_path: &Path,
    ) -> Result<Expr> {
        let getter_name = self.gen_get_method_name()?;
        Ok(parse2(quote! {
            <#blanket_type as #trait_path>::#getter_name(self)
        })?)
    }

    fn gen_blanket_option_get_method_body(
        &self,
        blanket_type_ident: &Ident,
        trait_path: &Path,
    ) -> Result<Expr> {
        let getter_name = self.gen_get_method_name()?;
        Ok(parse2(match self.wrapper {
            FieldPresense::Repeated => quote! {
                self.as_ref().map(<#blanket_type_ident as #trait_path>::#getter_name).into_iter().flatten()
            },
            FieldPresense::Explicit => quote! {
                self.as_ref().and_then(<#blanket_type_ident as #trait_path>::#getter_name)
            },
            FieldPresense::Implicit => quote! {
                self.as_ref().map(<#blanket_type_ident as #trait_path>::#getter_name).unwrap_or_default()
            },
        })?)
    }

    // Appendables

    pub fn gen_appendale_methods_signatures(&self, allocator: &Type) -> Result<Vec<Signature>> {
        // Bare fields => fn i32_mut(&mut self) -> DerefMut<Target=NonZero<i32>>
        // Optional fields => fn i32_mut(&mut self) -> DerefMut<Target=i32>
        // Vec fields => fn i32_mut(&mut self) -> DerefMut<Target=impl Extend<i32>>
        let signatures = vec![self.gen_append_method_signature(allocator)?];
        Ok(signatures)
    }
    pub fn gen_append_method_signature(&self, allocator: &Type) -> Result<Signature> {
        let append_method_name = self.gen_append_method_name()?;
        let return_type: Type = match self.wrapper {
            FieldPresense::Repeated => {
                let scalar_type = self.scalar_type.gen_mutator_deref_target_type(
                    &self.current_path,
                    allocator,
                    &self.options,
                )?;
                let target_type: Type = parse2(quote! {
                    impl ::puroro::repeated::RepeatedViewApp<Item = #scalar_type>
                })?;
                let deref_mut_trait = self.options.deref_mut_trait(&target_type)?;
                parse2(quote! { impl #deref_mut_trait })?
            }
            FieldPresense::Explicit => {
                let scalar_type = self.scalar_type.gen_mutator_deref_target_type(
                    &self.current_path,
                    allocator,
                    &self.options,
                )?;
                let deref_mut_trait = self.options.deref_mut_trait(&scalar_type)?;
                parse2(quote! { impl #deref_mut_trait })?
            }
            FieldPresense::Implicit => {
                let scalar_type = self.scalar_type.gen_nonzero_mutator_type(
                    &self.current_path,
                    allocator,
                    &self.options,
                )?;
                let deref_mut_trait = self.options.deref_mut_trait(&scalar_type)?;
                parse2(quote! { impl #deref_mut_trait })?
            }
        };
        Ok(parse2(quote! {
            fn #append_method_name(&mut self) -> #return_type
        })?)
    }
    fn gen_append_method_name(&self) -> Result<Ident> {
        let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
        Ok(to_ident(&format!("{}_mut", &lower_cased)))
    }

    // Mutators

    fn gen_mut_method_name(&self) -> Result<Ident> {
        let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
        Ok(to_ident(&format!("{}_mut", &lower_cased)))
    }

    pub fn gen_mutable_methods_signatures(&self, allocator: &Type) -> Result<Vec<Signature>> {
        Ok(vec![self.gen_mut_method_signature(allocator)?])
    }

    pub fn gen_mut_method_signature(&self, allocator: &Type) -> Result<Signature> {
        let mutator_name = self.gen_mut_method_name()?;
        let mutator_type = match self.wrapper {
            FieldPresense::Repeated => self.scalar_type.gen_repeated_mutator_type(
                &self.current_path,
                allocator,
                &self.options,
            )?,
            _ => self.scalar_type.gen_non_repeated_mutator_type(
                &self.current_path,
                allocator,
                &self.options,
            )?,
        };
        Ok(parse2(quote! {
            fn #mutator_name(&mut self) -> #mutator_type
        })?)
    }

    // Others
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
        } else if field.label() == Some(FieldLabel::Repeated) {
            FieldPresense::Repeated
        } else {
            FieldPresense::Implicit
        }
    }
}

impl FieldType<ProtoPathBuf, ProtoPathBuf> {
    fn gen_bare_getter_type(
        &self,
        current_path: &ProtoPath,
        lifetime: Option<&Lifetime>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let lifetime_iter = lifetime.iter();
        Ok(match self {
            FieldType::Message(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path_with(options, |name| {
                    let ident = GenTrait::rust_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { impl #(#lifetime_iter +)* #path })?
            }
            FieldType::Enum(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path(options)?;
                parse2(quote! { #path })?
            }
            FieldType::Int32 => options.primitive_type("i32")?,
            FieldType::Int64 => options.primitive_type("i64")?,
            FieldType::UInt32 => options.primitive_type("u32")?,
            FieldType::UInt64 => options.primitive_type("u64")?,
            FieldType::SInt32 => options.primitive_type("i32")?,
            FieldType::SInt64 => options.primitive_type("i64")?,
            FieldType::Fixed32 => options.primitive_type("u32")?,
            FieldType::Fixed64 => options.primitive_type("u64")?,
            FieldType::SFixed32 => options.primitive_type("i32")?,
            FieldType::SFixed64 => options.primitive_type("i64")?,
            FieldType::Float => options.primitive_type("f32")?,
            FieldType::Double => options.primitive_type("f64")?,
            FieldType::Bool => options.primitive_type("bool")?,
            FieldType::String => {
                let str_type = options.primitive_type("str")?;
                parse2(quote! { & #lifetime #str_type })?
            }
            FieldType::Bytes => {
                let u8_type = options.primitive_type("u8")?;
                parse2(quote! { & #lifetime [#u8_type] })?
            }
            FieldType::Group => Err(format!("Group field is not supported"))?,
        })
    }
    fn gen_optional_getter_type(
        &self,
        current_path: &ProtoPath,
        lifetime: Option<&Lifetime>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(current_path, lifetime, options)?;
        options.option_type(&bare_type)
    }
    fn gen_repeated_getter_type(
        &self,
        current_path: &ProtoPath,
        lifetime: Option<&Lifetime>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(current_path, lifetime, options)?;
        let iter_trait = options.iter_trait(&bare_type)?;
        Ok(parse2(quote! {
            impl #iter_trait
        })?)
    }

    fn gen_nonzero_mutator_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        Ok(match self {
            FieldType::Int32 => parse_str("::std::num::NonZero<i32>")?,
            FieldType::Int64 => parse_str("::std::num::NonZero<i64>")?,
            FieldType::UInt32 => parse_str("::std::num::NonZero<u32>")?,
            FieldType::UInt64 => parse_str("::std::num::NonZero<u64>")?,
            FieldType::SInt32 => parse_str("::std::num::NonZero<i32>")?,
            FieldType::SInt64 => parse_str("::std::num::NonZero<i64>")?,
            FieldType::Fixed32 => parse_str("::std::num::NonZero<u32>")?,
            FieldType::Fixed64 => parse_str("::std::num::NonZero<u64>")?,
            FieldType::SFixed32 => parse_str("::std::num::NonZero<i32>")?,
            FieldType::SFixed64 => parse_str("::std::num::NonZero<i64>")?,
            // TODO: following items
            FieldType::Float => options.primitive_type("f32")?,
            FieldType::Double => options.primitive_type("f64")?,
            FieldType::Bool => options.primitive_type("bool")?,
            FieldType::String => parse2(quote! { ::puroro::string::String<#allocator> })?,
            FieldType::Bytes => {
                let u8_type = options.primitive_type("u8")?;
                options.vec_type(&u8_type, Some(allocator))?
            }
            FieldType::Group => Err(format!("Group field is not supported"))?,
            FieldType::Message(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path_with(options, |name| {
                    let ident = GenTrait::rust_mut_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { impl #path<#allocator> })?
            }
            FieldType::Enum(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path(options)?;
                parse2(quote! { #path })?
            }
        })
    }

    fn gen_mutator_deref_target_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        Ok(match self {
            FieldType::Message(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path_with(options, |name| {
                    let ident = GenTrait::rust_mut_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { impl #path<#allocator> })?
            }
            FieldType::Enum(path) => {
                let path = path.to_relative_path(current_path).unwrap_or(path);
                let path = path.to_rust_path(options)?;
                parse2(quote! { #path })?
            }
            FieldType::Int32 => options.primitive_type("i32")?,
            FieldType::Int64 => options.primitive_type("i64")?,
            FieldType::UInt32 => options.primitive_type("u32")?,
            FieldType::UInt64 => options.primitive_type("u64")?,
            FieldType::SInt32 => options.primitive_type("i32")?,
            FieldType::SInt64 => options.primitive_type("i64")?,
            FieldType::Fixed32 => options.primitive_type("u32")?,
            FieldType::Fixed64 => options.primitive_type("u64")?,
            FieldType::SFixed32 => options.primitive_type("i32")?,
            FieldType::SFixed64 => options.primitive_type("i64")?,
            FieldType::Float => options.primitive_type("f32")?,
            FieldType::Double => options.primitive_type("f64")?,
            FieldType::Bool => options.primitive_type("bool")?,
            FieldType::String => parse2(quote! { ::puroro::string::String<#allocator> })?,
            FieldType::Bytes => {
                let u8_type = options.primitive_type("u8")?;
                options.vec_type(&u8_type, Some(allocator))?
            }
            FieldType::Group => Err(format!("Group field is not supported"))?,
        })
    }
    fn gen_non_repeated_mutator_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let bare_type = self.gen_mutator_deref_target_type(current_path, allocator, options)?;
        let deref_mut_trait = options.deref_mut_trait(&bare_type)?;
        Ok(parse2(quote! {
            impl #deref_mut_trait
        })?)
    }
    fn gen_repeated_mutator_type(
        &self,
        current_path: &ProtoPath,
        allocator: &Type,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        let bare_type = self.gen_mutator_deref_target_type(current_path, allocator, options)?;
        let vec_type = options.vec_type(&bare_type, Some(allocator))?;
        let deref_mut_trait = options.deref_mut_trait(&vec_type)?;
        Ok(parse2(quote! {
            impl #deref_mut_trait
        })?)
    }
}
