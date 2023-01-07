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

use super::super::util::*;
use super::{
    Bits32Type, Bits64Type, EnumExt, Field, FieldBase, FieldOrOneofExt, FieldRule, FieldType,
    LengthDelimitedType, MessageExt, VariantType, PURORO_INTERNAL,
};
use crate::syn::{
    parse2, Arm, Expr, ExprMethodCall, FieldValue, ImplItemMethod, PathSegment, Stmt, Type,
};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::Span;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;
use ::syn::{LitBool, LitByteStr};

pub trait FieldExt {
    // Message's bitfield allocation
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>>;
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize>;

    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>>;

    fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>>;
    fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue>;
    fn gen_message_struct_impl_deser_arm(&self, field_data_expr: &Expr) -> Result<Arm>;
    fn gen_message_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt>;
    fn gen_message_struct_impl_debug_method_call(&self, receiver: Expr) -> Result<ExprMethodCall>;
    fn gen_message_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr>;
}

#[derive(Debug, Clone, Copy)]
struct FieldBitfieldAllocation {
    // Bit used by optional field.
    maybe_optional: Option<usize>,
    tail: usize,
}

#[derive(Debug, Default)]
struct Cache {
    // Message's bitfield allocation
    allocated_bitfield: OnceCell<FieldBitfieldAllocation>,
}

impl<T: ?Sized + Field> FieldExt for T {
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>> {
        Ok(self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .get()
            .map(|a| a.tail))
    }
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize> {
        // We need to allocate the bit for optional bit.
        let mut alloc = FieldBitfieldAllocation {
            maybe_optional: None,
            tail: head,
        };
        match (self.rule()?, self.r#type()?) {
            (_, FieldType::LengthDelimited(LengthDelimitedType::Message(_))) => {
                // Optional bit not needed for Message field.
                // Do nothing
            }
            (FieldRule::Optional, _) => {
                alloc.maybe_optional = Some(alloc.tail);
                alloc.tail += 1;
            }
            (FieldRule::Singular | FieldRule::Repeated, _) => {
                // Optional bit not needed for singular / repeated field.
                // Do nothing
            }
        }
        match self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .try_insert(alloc)
        {
            Ok(alloc) => Ok(alloc.tail),
            Err(_) => Err(ErrorKind::InternalError {
                detail: "Tried to assign the field's bitfield twice.".to_string(),
            })?,
        }
    }

    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>> {
        use FieldRule::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        let primitive_type = self.r#type()?.rust_type()?;
        let tag_type = self.r#type()?.tag_type()?;
        let bitfield_index = bitfield_index_for_optional(self)?.unwrap_or(usize::MAX);
        let type_name_segment: PathSegment = parse2(match (self.rule()?, self.r#type()?) {
            (Optional | Singular, LengthDelimited(Message(_))) => quote! {
                SingularHeapMessageField::<#primitive_type>
            },
            (Repeated, LengthDelimited(Message(_))) => quote! {
                RepeatedMessageField::<#primitive_type>
            },
            (Optional, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                OptionalNumericalField::<#primitive_type, #tag_type, #bitfield_index>
            },
            (Singular, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                SingularNumericalField::<#primitive_type, #tag_type>
            },
            (Repeated, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                RepeatedNumericalField::<#primitive_type, #tag_type>
            },
            (Optional, LengthDelimited(_)) => quote! {
                OptionalUnsizedField::<#primitive_type, #tag_type, #bitfield_index>
            },
            (Singular, LengthDelimited(_)) => quote! {
                SingularUnsizedField::<#primitive_type, #tag_type>
            },
            (Repeated, LengthDelimited(_)) => quote! {
                RepeatedUnsizedField::<#primitive_type, #tag_type>
            },
        })?;
        Ok(Rc::new(parse2(quote! {
            #PURORO_INTERNAL::#type_name_segment
        })?))
    }

    fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        match self.rule()? {
            FieldRule::Repeated => gen_message_struct_field_methods_for_repeated(self),
            _ => gen_message_struct_field_methods_for_non_repeated(self),
        }
    }
    fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue> {
        let ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(quote! {
            #ident: ::std::clone::Clone::clone(&self.fields.#ident)
        })?)
    }
    fn gen_message_struct_impl_deser_arm(&self, field_data_expr: &Expr) -> Result<Arm> {
        let ident = self.gen_fields_struct_field_ident()?;
        let number = self.number()?;
        Ok(parse2(quote! {
            #number => #PURORO_INTERNAL::FieldType::deser_from_iter(
                &mut self.fields.#ident,
                &mut self.bitfield,
                #field_data_expr,
            )?,
        })?)
    }
    fn gen_message_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt> {
        let ident = self.gen_fields_struct_field_ident()?;
        let number = self.number()?;
        Ok(parse2(quote! {
            #PURORO_INTERNAL::FieldType::ser_to_write(
                &self.fields.#ident,
                &self.bitfield,
                #number,
                #out_expr,
            )?;
        })?)
    }
    fn gen_message_struct_impl_debug_method_call(&self, receiver: Expr) -> Result<ExprMethodCall> {
        let ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(match self.rule()? {
            FieldRule::Repeated => {
                let getter_ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                quote! {
                    #receiver.field(stringify!(#ident), &self.#getter_ident())
                }
            }
            _ => {
                let getter_opt_ident = format_ident!("{}_opt", self.name()?.to_lower_snake_case());
                quote! {
                    #receiver.field(stringify!(#ident), &self.#getter_opt_ident())
                }
            }
        })?)
    }
    fn gen_message_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr> {
        Ok(parse2(match self.rule()? {
            FieldRule::Repeated => {
                let getter_ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                quote! { self.#getter_ident() == #rhs_expr.#getter_ident() }
            }
            _ => {
                let getter_opt_ident = format_ident!("{}_opt", self.name()?.to_lower_snake_case());
                quote! { self.#getter_opt_ident() == #rhs_expr.#getter_opt_ident() }
            }
        })?)
    }
}

fn bitfield_index_for_optional(this: &(impl ?Sized + Field)) -> Result<Option<usize>> {
    let alloc = if let Some(alloc) = this.cache().get::<Cache>()?.allocated_bitfield.get() {
        alloc
    } else {
        let _ = this.message()?.bitfield_size()?;
        let Some(alloc) = this
            .cache()
            .get::<Cache>()?.allocated_bitfield.get() else {
                Err(ErrorKind::InternalError { detail: "field bitfield is not set after the message's bitfield size is calculated.".to_string() })?
            };
        alloc
    };
    Ok(alloc.maybe_optional)
}

fn gen_message_struct_field_methods_for_repeated(
    this: &(impl ?Sized + Field),
) -> Result<Vec<ImplItemMethod>> {
    debug_assert!(matches!(this.rule(), Ok(FieldRule::Repeated)));
    let getter_ident = format_ident!(
        "{}",
        this.name()?.to_lower_snake_case().escape_rust_keywords()
    );
    let getter_mut_ident = format_ident!("{}_mut", this.name()?.to_lower_snake_case());
    let clear_ident = format_ident!("clear_{}", this.name()?.to_lower_snake_case());
    let field_ident = this.gen_fields_struct_field_ident()?;
    let getter_item_type = match this.r#type()? {
        FieldType::LengthDelimited(LengthDelimitedType::String) => Rc::new(parse2(quote! {
            impl ::std::ops::Deref::<Target = str> +
                ::std::fmt::Debug +
                ::std::cmp::PartialEq
        })?),
        FieldType::LengthDelimited(LengthDelimitedType::Bytes) => Rc::new(parse2(quote! {
            impl ::std::ops::Deref::<Target = [u8]> +
                ::std::fmt::Debug +
                ::std::cmp::PartialEq
        })?),
        FieldType::LengthDelimited(LengthDelimitedType::Message(m)) => {
            m.try_upgrade()?.gen_message_struct_type()?
        }
        _ => this.r#type()?.rust_type()?,
    };
    let mut_item_type = this.r#type()?.rust_type()?;
    Ok(vec![
        parse2(quote! {
            pub fn #getter_ident(&self) -> &[#getter_item_type] {
                use #PURORO_INTERNAL::RepeatedFieldType;
                RepeatedFieldType::get_field(
                    &self.fields.#field_ident, &self.bitfield,
                )
            }
        })?,
        parse2(quote! {
            pub fn #getter_mut_ident(&mut self) -> &mut ::std::vec::Vec::<#mut_item_type> {
                use #PURORO_INTERNAL::RepeatedFieldType;
                RepeatedFieldType::get_field_mut(
                    &mut self.fields.#field_ident, &mut self.bitfield,
                )
            }
        })?,
        parse2(quote! {
            pub fn #clear_ident(&mut self) {
                use #PURORO_INTERNAL::RepeatedFieldType;
                RepeatedFieldType::clear(
                    &mut self.fields.#field_ident, &mut self.bitfield,
                )
            }
        })?,
    ])
}

fn gen_message_struct_field_methods_for_non_repeated(
    this: &(impl ?Sized + Field),
) -> Result<Vec<ImplItemMethod>> {
    debug_assert!(matches!(
        this.rule(),
        Ok(FieldRule::Optional | FieldRule::Singular)
    ));
    let getter_ident = format_ident!(
        "{}",
        this.name()?.to_lower_snake_case().escape_rust_keywords()
    );
    let getter_opt_ident = format_ident!("{}_opt", this.name()?.to_lower_snake_case());
    let getter_mut_ident = format_ident!("{}_mut", this.name()?.to_lower_snake_case());
    let getter_has_ident = format_ident!("has_{}", this.name()?.to_lower_snake_case());
    let clear_ident = format_ident!("clear_{}", this.name()?.to_lower_snake_case());
    let field_ident = this.gen_fields_struct_field_ident()?;
    let borrowed_type = this.r#type()?.rust_maybe_borrowed_type(None)?;
    let getter_type = match this.r#type()? {
        FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => Rc::new(parse2(quote! {
            ::std::option::Option::< #borrowed_type >
        })?),
        _ => Rc::clone(&borrowed_type),
    };
    let getter_opt_type = Rc::new(quote! {
        ::std::option::Option::< #borrowed_type >
    });
    let getter_mut_type = this.r#type()?.rust_mut_ref_type()?;
    let default_fn = gen_default_fn(this)?;
    Ok(vec![
        parse2(quote! {
            pub fn #getter_ident(&self) -> #getter_type {
               use #PURORO_INTERNAL::NonRepeatedFieldType;
                NonRepeatedFieldType::get_field_or_else(
                    &self.fields.#field_ident, &self.bitfield, #default_fn,
                )
            }
        })?,
        parse2(quote! {
            pub fn #getter_opt_ident(&self) -> #getter_opt_type {
                use #PURORO_INTERNAL::NonRepeatedFieldType;
                NonRepeatedFieldType::get_field_opt(
                    &self.fields.#field_ident, &self.bitfield,
                )
            }
        })?,
        parse2(quote! {
            pub fn #getter_mut_ident(&mut self) -> #getter_mut_type {
                use #PURORO_INTERNAL::NonRepeatedFieldType;
                NonRepeatedFieldType::get_field_mut(
                    &mut self.fields.#field_ident, &mut self.bitfield, #default_fn,
                )
            }
        })?,
        parse2(quote! {
            pub fn #getter_has_ident(&self) -> bool {
                use #PURORO_INTERNAL::NonRepeatedFieldType;
                NonRepeatedFieldType::get_field_opt(
                    &self.fields.#field_ident, &self.bitfield,
                ).is_some()
            }
        })?,
        parse2(quote! {
            pub fn #clear_ident(&mut self) {
                use #PURORO_INTERNAL::NonRepeatedFieldType;
                NonRepeatedFieldType::clear(
                    &mut self.fields.#field_ident, &mut self.bitfield,
                )
            }
        })?,
    ])
}

pub(crate) fn gen_default_fn(this: &(impl ?Sized + FieldBase)) -> Result<Expr> {
    use ::std::str::FromStr;

    Ok(parse2(
        if let Some(default_value_string) = this.default_value()? {
            use FieldType::*;
            match this.r#type()? {
                // Floats. I believe the rust's `f32::from_str()` method supports
                // all the possible patterns of protoc command output:
                // https://doc.rust-lang.org/std/primitive.f32.html#impl-FromStr-for-f32
                Bits32(Bits32Type::Float) => {
                    let value = f32::from_str(&default_value_string)?;
                    if value.is_infinite() && value.is_sign_positive() {
                        quote! { || f32::INFINITY }
                    } else if value.is_infinite() && value.is_sign_negative() {
                        quote! { || f32::NEG_INFINITY }
                    } else if value.is_nan() {
                        quote! { || f32::NAN }
                    } else {
                        quote! { || #value }
                    }
                }
                Bits64(Bits64Type::Double) => {
                    let value = f64::from_str(&default_value_string)?;
                    if value.is_infinite() && value.is_sign_positive() {
                        quote! { || f64::INFINITY }
                    } else if value.is_infinite() && value.is_sign_negative() {
                        quote! { || f64::NEG_INFINITY }
                    } else if value.is_nan() {
                        quote! { || f64::NAN }
                    } else {
                        quote! { || #value }
                    }
                }
                // Strings and bytes. Strings are okay for as-is, but bytes need to parse
                // the octal escape sequences.
                LengthDelimited(LengthDelimitedType::String) => {
                    quote! { || #default_value_string }
                }
                LengthDelimited(LengthDelimitedType::Bytes) => {
                    let bytes = convert_octal_escapes_to_bytes(&default_value_string)?;
                    let byte_lit = LitByteStr::new(&bytes, Span::call_site());
                    quote! { || #byte_lit }
                }
                LengthDelimited(LengthDelimitedType::Message(_)) => {
                    Err(ErrorKind::InternalError {
                        detail: "The field default value should not be set for the message field."
                            .to_string(),
                    })?
                }
                // Enum. Need to generate the value name.
                FieldType::Variant(VariantType::Enum2(e) | VariantType::Enum3(e)) => {
                    let e = e.try_upgrade()?;
                    let enum_path = e.gen_enum_path()?;
                    let enum_value_ident = format_ident!(
                        "{}",
                        default_value_string.to_camel_case().escape_rust_keywords()
                    );
                    quote! { || #enum_path :: #enum_value_ident }
                }
                // Integers and boolean. For the both cases just convert the given string
                // as-is is okay.
                Variant(VariantType::Bool) => {
                    let value = bool::from_str(&default_value_string)?;
                    let lit_bool = LitBool::new(value, Span::call_site());
                    quote! { || #lit_bool }
                }
                Variant(VariantType::Int32 | VariantType::SInt32)
                | Bits32(Bits32Type::SFixed32) => {
                    let value = i32::from_str(&default_value_string)?;
                    quote! { || #value }
                }
                Variant(VariantType::Int64 | VariantType::SInt64)
                | Bits64(Bits64Type::SFixed64) => {
                    let value = i64::from_str(&default_value_string)?;
                    quote! { || #value }
                }
                Variant(VariantType::UInt32) | Bits32(Bits32Type::Fixed32) => {
                    let value = u32::from_str(&default_value_string)?;
                    quote! { || #value }
                }
                Variant(VariantType::UInt64) | Bits64(Bits64Type::Fixed64) => {
                    let value = u64::from_str(&default_value_string)?;
                    quote! { || #value }
                }
            }
        } else {
            quote! { ::std::default::Default::default }
        },
    )?)
}
