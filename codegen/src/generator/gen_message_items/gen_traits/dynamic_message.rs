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

use super::{impls_helper, Field, ImplsGenerator};
use crate::descriptor::{I32Type, I64Type, LenType, VariantType, WireType};
use crate::generator::CodeGeneratorOptions;
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Item, Path};
use ::syn::{Expr, Type};

// Implementation generator for DynamicMessage type using ImplsGenerator trait
pub struct DynamicMessageImplsGenerator {
    options: Rc<CodeGeneratorOptions>,
}

impl ImplsGenerator for DynamicMessageImplsGenerator {
    fn generate<'a>(
        &self,
        _view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Result<Vec<Item>> {
        let methods = impls_helper(
            fields,
            |f| self.gen_try_getter_block(f),
            |f| self.gen_try_has_method_block(f),
            true,
        )?;
        Ok(vec![parse2(quote! {
            impl<A: ::std::alloc::Allocator + ::std::clone::Clone> #try_view_trait_path
            for ::puroro::dynamic::DynamicMessage<A>
            {
                #(#methods)*
            }
        })?])
    }
}

impl DynamicMessageImplsGenerator {
    pub fn new(options: Rc<CodeGeneratorOptions>) -> Self {
        Self { options }
    }

    pub fn gen_try_getter_block(&self, field: &Field) -> Result<Block> {
        let number = field.number();
        let body = self.gen_try_getter_body(field, &parse_str("f_opt")?)?;
        let ok = self.options.ok_path()?;
        Ok(parse2(quote! {
            {
                let f_opt = self.field(#number);
                let result = #body;
                #ok(result)
            }
        })?)
    }

    pub fn gen_try_getter_body(&self, field: &Field, field_opt_expr: &Expr) -> Result<Expr> {
        let wire_type: WireType<_, _> = field.scalar_proto_type().as_ref().into();
        let field_expr: Expr = parse_str("f")?;
        Ok(match field {
            Field::Repeated { .. } => {
                let body = match wire_type {
                    WireType::Variant(t) => {
                        self.gen_repeated_variant_getter_body(field, &field_expr, t)?
                    }
                    WireType::I32(t) => self.gen_repeated_i32_getter_body(field, &field_expr, t)?,
                    WireType::I64(t) => self.gen_repeated_i64_getter_body(field, &field_expr, t)?,
                    WireType::Len(t) => self.gen_repeated_len_getter_body(field, &field_expr, t)?,
                    _ => todo!(),
                };
                parse2(quote! {
                    (#field_opt_expr).map(|f| #body).transpose()?.into_iter().flatten()
                })?
            }
            Field::Implicit { .. } | Field::Explicit { .. } => {
                let body = match wire_type {
                    WireType::Variant(t) => {
                        self.gen_try_non_repeated_varint_getter_body(field, &field_expr, t)?
                    }
                    WireType::I32(t) => {
                        self.gen_try_non_repeated_i32_getter_body(field, &field_expr, t)?
                    }
                    WireType::I64(t) => {
                        self.gen_try_non_repeated_i64_getter_body(field, &field_expr, t)?
                    }
                    WireType::Len(t) => {
                        self.gen_try_non_repeated_len_getter_body(field, &field_expr, t)?
                    }
                    _ => todo!(),
                };
                let unwrap_option = (!matches!(wire_type, WireType::Len(LenType::Message(_))))
                    .then(|| {
                        quote! {
                            .unwrap_or_default() // TODO: default value
                        }
                    })
                    .into_iter();
                parse2(quote! {
                    (#field_opt_expr).map(|f| #body).transpose()?.flatten() #(#unwrap_option)*
                })?
            }
        })
    }

    pub fn gen_try_non_repeated_varint_getter_body(
        &self,
        field: &Field,
        field_expr: &Expr,
        t: VariantType<impl AsRef<ProtoPath>>,
    ) -> Result<Expr> {
        let vt_type: Type =
            t.to_variant_integer_type(field.base_proto_path().as_ref(), &self.options)?;
        Ok(parse2(quote! {
            (#field_expr).as_scalar_variant::<#vt_type>(
                true /* TODO: packed check */,
                ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */,
            )
        })?)
    }
    pub fn gen_try_non_repeated_i32_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: I32Type,
    ) -> Result<Expr> {
        let bytes_expr: Expr = parse2(quote! { (#field_expr).as_scalar_i32(
            ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */,
        ) })?;
        let primitive_type = t.to_primitive_type(&self.options)?;
        Ok(parse2(
            quote! { (#bytes_expr).map(|v_opt| v_opt.map(#primitive_type::from_le_bytes)) },
        )?)
    }
    pub fn gen_try_non_repeated_i64_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: I64Type,
    ) -> Result<Expr> {
        let bytes_expr: Expr = parse2(quote! { (#field_expr).as_scalar_i64(
            ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */,
        ) })?;
        let primitive_type = t.to_primitive_type(&self.options)?;
        Ok(parse2(
            quote! { (#bytes_expr).map(|v_opt| v_opt.map(#primitive_type::from_le_bytes)) },
        )?)
    }
    pub fn gen_try_non_repeated_len_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: LenType<impl AsRef<ProtoPath>>,
    ) -> Result<Expr> {
        Ok(parse2(match t {
            LenType::String => {
                quote! { (#field_expr).as_scalar_string(
                ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */,) }
            }
            LenType::Bytes => {
                quote! { (#field_expr).as_scalar_bytes(
                ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */,) }
            }
            LenType::Message(_) => {
                quote! { (#field_expr).as_scalar_message() }
            }
        })?)
    }

    pub fn gen_repeated_variant_getter_body(
        &self,
        field: &Field,
        field_expr: &Expr,
        t: VariantType<impl AsRef<ProtoPath>>,
    ) -> Result<Expr> {
        let vt_type: Type =
            t.to_variant_integer_type(field.base_proto_path().as_ref(), &self.options)?;
        Ok(parse2(quote! {
            (#field_expr).as_repeated_variant::<#vt_type>(true /* TODO: packed check */)
        })?)
    }
    pub fn gen_repeated_i32_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: I32Type,
    ) -> Result<Expr> {
        let primitive_type = t.to_primitive_type(&self.options)?;
        Ok(parse2(quote! {
            (#field_expr).as_repeated_i32().map(|iter| iter.map(|v_res| v_res.map(#primitive_type::from_le_bytes)))
        })?)
    }
    pub fn gen_repeated_i64_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: I64Type,
    ) -> Result<Expr> {
        let primitive_type = t.to_primitive_type(&self.options)?;
        Ok(parse2(quote! {
            (#field_expr).as_repeated_i64().map(|iter| iter.map(|v_res| v_res.map(#primitive_type::from_le_bytes)))
        })?)
    }
    pub fn gen_repeated_len_getter_body(
        &self,
        _field: &Field,
        field_expr: &Expr,
        t: LenType<impl AsRef<ProtoPath>>,
    ) -> Result<Expr> {
        Ok(parse2(match t {
            LenType::String => quote! { (#field_expr).as_repeated_string() },
            LenType::Bytes => quote! { (#field_expr).as_repeated_bytes() },
            LenType::Message(_) => {
                quote! { (#field_expr).as_repeated_message() }
            }
        })?)
    }

    pub fn gen_try_has_method_block(&self, field: &Field) -> Result<Block> {
        let (Field::Explicit { scalar_proto_type, .. } | Field::Implicit { scalar_proto_type, .. }) =
            field
        else {
            Err("try_has method is not supported for repeated fields")?
        };
        let wire_type: WireType<_, _> = scalar_proto_type.as_ref().into();
        let field_opt: Expr = parse2(quote! { self.field(#number) })?;
        let body = parse2(match wire_type {
            WireType::Variant(variant) => {
                let vt_type: Type = variant
                    .to_variant_integer_type(field.base_proto_path().as_ref(), &self.options)?;
                quote! {
                    #field_opt.and_then(|f| f.as_scalar_variant::<#vt_type>(
                        true, /* TODO: pack check */
                        ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */
                    )).is_some()
                }
            }
            WireType::I32(_) => {
                quote! {
                    #field_opt.and_then(|f| f.as_scalar_i32(
                        ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */
                    )).is_some()
                }
            }
            WireType::I64(_) => {
                quote! {
                    #field_opt.and_then(|f| f.as_scalar_i64(
                        ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */
                    )).is_some()
                }
            }
            WireType::Len(LenType::String) => {
                quote! {
                    #field_opt.and_then(|f| f.as_scalar_string(
                        ::puroro::dynamic::FieldReducingErrorStrategy::Skip /* TODO: needs confirmation */
                    )).is_some()
                }
            }
            
            _ => todo!(),
        })?;

        Ok(parse2(quote! {
            {
                let f_opt = self.field(#number);
                let result = #body;
                #ok(result)
            }
        })?)
    }
}

impl<E: AsRef<ProtoPath>> VariantType<E> {
    fn to_variant_integer_type(
        &self,
        current_path: impl AsRef<ProtoPath>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        Ok(parse2(match self {
            VariantType::Int32 => quote! { ::puroro::variant::variant_types::Int32 },
            VariantType::Int64 => quote! { ::puroro::variant::variant_types::Int64 },
            VariantType::UInt32 => quote! { ::puroro::variant::variant_types::UInt32 },
            VariantType::UInt64 => quote! { ::puroro::variant::variant_types::UInt64 },
            VariantType::SInt32 => quote! { ::puroro::variant::variant_types::SInt32 },
            VariantType::SInt64 => quote! { ::puroro::variant::variant_types::SInt64 },
            VariantType::Bool => quote! { ::puroro::variant::variant_types::Bool },
            VariantType::Enum(path) => {
                let path = path
                    .as_ref()
                    .to_relative_path(current_path.as_ref())
                    .unwrap_or(path.as_ref());
                let enum_path = path.to_rust_path(options)?;
                quote! { ::puroro::variant::variant_types::Enum::<#enum_path> }
            }
        })?)
    }
}
