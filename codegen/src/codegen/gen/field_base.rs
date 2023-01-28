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
use super::{Bits32Type, Bits64Type, FieldType, LengthDelimitedType, VariantType};
use crate::codegen::data::FieldBase;
use crate::syn::{parse2, Expr};
use crate::{FatalErrorKind, Result};
use ::proc_macro2::Span;
use ::quote::{format_ident, quote};
use ::syn::{LitBool, LitByteStr};

pub(crate) trait FieldBaseExt: FieldBase {
    fn gen_default_fn(&self) -> Result<Expr> {
        use ::std::str::FromStr;

        Ok(parse2(
            if let Some(default_value_string) = self.default_value()? {
                use FieldType::*;
                match self.r#type()? {
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
                        Err(FatalErrorKind::InternalError {
                            detail:
                                "The field default value should not be set for the message field."
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
}

impl<T: FieldBase> FieldBaseExt for T {}
