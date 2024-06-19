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
use crate::descriptor::{DescriptorWithContext, FieldDescriptorWithContext, FieldLabel, FieldType};
use crate::generator::r#enum::Enum;
use crate::proto_path::ProtoPath;
use crate::Result;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote, ToTokens, TokenStreamExt};
use ::std::cell::OnceCell;
use ::syn::{parse2, parse_str, Ident, Item, Type};

pub struct MessageOpenStruct<'a> {
    rust_name: Ident,
    fields: Vec<Field<'a>>,
    cache: MessageOpenStructCache,
}
#[derive(Default)]
struct MessageOpenStructCache {}

struct Field<'a> {
    desc: &'a FieldDescriptorWithContext<'a>,
    rust_name: Ident,
    cache: FieldCache,
}
#[derive(Default)]
struct FieldCache {
    rust_type: OnceCell<Type>,
    rust_field_wrapper: OnceCell<FieldWrapper>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldWrapper {
    Bare,
    Optional,
    OptionalBoxed,
    Vec,
}

impl<'a> MessageOpenStruct<'a> {
    pub fn try_new(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            rust_name: Self::rust_name_from_message_name(desc.name()?)?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
            cache: Default::default(),
        })
    }

    pub fn rust_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}Struct",
            convert_into_case(name, Case::CamelCase)
        ))
    }

    pub fn rust_path_from_message_path(
        path: impl AsRef<ProtoPath>,
        allocator: &Type,
    ) -> Result<Type> {
        let modules = path
            .as_ref()
            .parent()
            .into_iter()
            .flat_map(|p| p.components())
            .map(|name| Ok(parse_str(&convert_into_case(name, Case::LowerSnakeCase))?))
            .collect::<Result<Vec<Ident>>>()?;
        let struct_name = Self::rust_name_from_message_name(
            path.as_ref()
                .last_component()
                .ok_or_else(|| format!("Invalid message path: {:?}", path.as_ref()))?,
        )?;
        Ok(parse2(quote! {
            crate #(:: #modules)* :: #struct_name :: <#allocator>
        })?)
    }

    pub fn rust_items(&self) -> Result<Vec<Item>> {
        Ok(vec![
            self.rust_struct_item()?,
            self.rust_impl_message_lite()?,
        ])
    }

    fn rust_struct_item(&self) -> Result<Item> {
        let name = &self.rust_name;
        let fields = &self.fields;
        Ok(parse2(quote! {
            pub struct #name<#[cfg(allocator)]A: ::std::alloc::Allocator = ::std::alloc::Global> {
                #(#fields)*
            }
        })?)
    }

    fn rust_impl_message_lite(&self) -> Result<Item> {
        let name = &self.rust_name;
        Ok(parse2(quote! {
        impl<#[cfg(allocator)]A: ::std::alloc::Allocator> ::puroro::MessageLite<A> for self::#name<A> {
            fn merge_from_bufread<R: ::std::io::BufRead>(
                &mut self, _bufread: &mut R,
            ) -> ::puroro::Result<Self> {
                use ::puroro::Result;
                use ::puroro::internal::deser::{DeserMessageHandlerBase, DeserMessageHandlerForRead};
                use ::puroro::variant::Variant;
                impl DeserMessageHandlerBase for Self {
                    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
                        todo!()
                    }
                    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
                        todo!()
                    }
                    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
                        todo!()
                    }
                    fn is_message_field(&self, num: i32) -> bool {
                        todo!()
                    }
                    fn start_message(&mut self, num: i32) -> Result<()> {
                        todo!()
                    }
                    fn end_message(&mut self) -> Result<()> {
                        todo!()
                    }
                }
                impl<R: ::std::io::Read> DeserMessageHandlerForRead<R> for Self {
                    fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
                        todo!()
                    }
                }
                todo!()
            }
            fn write<W: ::std::io::Write>(&self, _write: &mut W) -> Result<usize> {
                unimplemented!()
            }
        }
                    })?)
    }
}

impl<'a> Field<'a> {
    pub fn try_new(desc: &'a FieldDescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            desc,
            rust_name: parse_str(&convert_into_case(desc.name()?, Case::LowerSnakeCase))?,
            cache: Default::default(),
        })
    }

    pub fn rust_type(&self) -> Result<Type> {
        let body = || {
            let ty = self.desc.r#type()?;
            match (self.desc.label()?, self.desc.is_proto3_optional()?) {
                (Some(FieldLabel::Repeated), _) => Self::gen_repeated_type(ty),
                (None, false) => Self::gen_scalar_type(ty),
                _ => Self::gen_optional_type(ty),
            }
        };
        self.cache.rust_type.get_or_try_init(body).cloned()
    }

    pub fn rust_field_wrapper(&self) -> Result<FieldWrapper> {
        let body = || {
            let result = match (
                self.desc.label()?,
                self.desc.r#type()?,
                self.desc.is_proto3_optional()?,
            ) {
                (Some(FieldLabel::Repeated), _, _) => FieldWrapper::Vec,
                (_ /* Not repeated */, FieldType::Message(_), _) => FieldWrapper::OptionalBoxed,
                (None, _ /* Not message */, false) => FieldWrapper::Bare,
                _ => FieldWrapper::Optional,
            };
            Ok(result)
        };
        self.cache.rust_field_wrapper.get_or_try_init(body).cloned()
    }

    fn gen_scalar_type(ty: FieldType) -> Result<Type> {
        Ok(parse2(match ty {
            FieldType::Bool => quote! { bool },
            FieldType::Bytes => quote! { ::std::vec::Vec<u8, A> },
            FieldType::Double => quote! { f64 },
            FieldType::Enum(e) => {
                let enum_path = Enum::rust_path_from_enum_path(e.full_path()?)?;
                quote! { #enum_path }
            }
            FieldType::Fixed32 => quote! { u32 },
            FieldType::Fixed64 => quote! { u64 },
            FieldType::Float => quote! { f32 },
            FieldType::Group => todo!(),
            FieldType::Int32 => quote! { i32 },
            FieldType::Int64 => quote! { i64 },
            FieldType::Message(m) => {
                let struct_path = MessageOpenStruct::rust_path_from_message_path(
                    m.full_path()?,
                    &parse_str("A")?,
                )?;
                quote! {
                    ::std::boxed::Box::<#struct_path, A>
                }
            }
            FieldType::SFixed32 => quote! { i32 },
            FieldType::SFixed64 => quote! { i64 },
            FieldType::SInt32 => quote! { i32 },
            FieldType::SInt64 => quote! { i64 },
            FieldType::String => quote! { ::puroro::string::String<A> },
            FieldType::UInt32 => quote! { u32 },
            FieldType::UInt64 => quote! { u64 },
        })?)
    }

    fn gen_optional_type(ty: FieldType) -> Result<Type> {
        let scalar = Self::gen_scalar_type(ty)?;
        Ok(parse2(quote! {
            ::std::option::Option::<#scalar>
        })?)
    }

    fn gen_repeated_type(ty: FieldType) -> Result<Type> {
        let scalar_type = match ty {
            FieldType::Message(m) => {
                MessageOpenStruct::rust_path_from_message_path(m.full_path()?, &parse_str("A")?)?
            }
            _ => Self::gen_scalar_type(ty)?,
        };
        Ok(parse2(quote! { ::std::vec::Vec::<#scalar_type, A> })?)
    }
}

impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.rust_name;
        let ty = &self.rust_type().unwrap();
        tokens.append_all(quote! {
            pub #name: #ty,
        })
    }
}
