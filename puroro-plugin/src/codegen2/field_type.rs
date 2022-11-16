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

use super::Syntax;
use crate::{ErrorKind, Result};
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto;
use ::quote::quote;

#[derive(Debug, Clone)]
pub(super) enum FieldType {
    Variant(VariantType),
    LengthDelimited(LengthDelimitedType),
    Bits32(Bits32Type),
    Bits64(Bits64Type),
}
#[derive(Debug, Clone)]
pub(super) enum VariantType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Bool,
    Enum2(String),
    Enum3(String),
}
#[derive(Debug, Clone)]
pub(super) enum LengthDelimitedType {
    String,
    Bytes,
    Message(String),
}
#[derive(Debug, Clone)]
pub(super) enum Bits32Type {
    Fixed32,
    SFixed32,
    Float,
}
#[derive(Debug, Clone)]
pub(super) enum Bits64Type {
    Fixed64,
    SFixed64,
    Double,
}

impl FieldType {
    pub(super) fn try_new(
        type_opt: Option<field_descriptor_proto::Type>,
        type_name: &str,
        syntax: Syntax,
    ) -> Result<Self> {
        use field_descriptor_proto::Type::*;
        use Bits32Type::*;
        use Bits64Type::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        use VariantType::*;

        let Some(r#type) = type_opt else {
            unimplemented!("No field type enum case. 
            Though the descriptor.proto says the field might not be filled,
            actually the protoc's generated proto always have this field so
            I skip this (complex) implementation.")
        };

        Ok(match r#type {
            TypeInt32 => Variant(Int32),
            TypeUint32 => Variant(UInt32),
            TypeSint32 => Variant(SInt32),
            TypeInt64 => Variant(Int64),
            TypeUint64 => Variant(UInt64),
            TypeSint64 => Variant(SInt64),
            TypeBool => Variant(Bool),
            TypeEnum => match syntax {
                Syntax::Proto2 => Variant(Enum2(type_name.to_string())),
                Syntax::Proto3 => Variant(Enum3(type_name.to_string())),
            },
            TypeFixed32 => Bits32(Fixed32),
            TypeSfixed32 => Bits32(SFixed32),
            TypeFloat => Bits32(Float),
            TypeFixed64 => Bits64(Fixed64),
            TypeSfixed64 => Bits64(SFixed64),
            TypeDouble => Bits64(Double),
            TypeString => LengthDelimited(String),
            TypeBytes => LengthDelimited(Bytes),
            TypeGroup => Err(ErrorKind::GroupNotSupported)?,
            TypeMessage => LengthDelimited(Message(type_name.to_string())),
        })
    }

    pub(super) fn rust_type(&self) -> Result<TokenStream> {
        match self {
            FieldType::Variant(v) => v.rust_type(),
            FieldType::LengthDelimited(ld) => ld.rust_type(),
            FieldType::Bits32(b) => b.rust_type(),
            FieldType::Bits64(b) => b.rust_type(),
        }
    }
}

impl VariantType {
    fn rust_type(&self) -> Result<TokenStream> {
        use VariantType::*;
        Ok(match self {
            Int32 | SInt32 => quote! { i32 },
            UInt32 => quote! {u32},
            Int64 | SInt64 => quote! {i64},
            UInt64 => quote! {u64},
            Bool => quote! {bool},
            Enum2(_) => todo!(),
            Enum3(_) => todo!(),
        })
    }
}
impl LengthDelimitedType {
    fn rust_type(&self) -> Result<TokenStream> {
        use LengthDelimitedType::*;
        Ok(match self {
            String => quote! { ::std::string::String },
            Bytes => quote! { ::std::vec::Vec<u8> },
            Message(_) => todo!(),
        })
    }
}
impl Bits32Type {
    fn rust_type(&self) -> Result<TokenStream> {
        use Bits32Type::*;
        Ok(match self {
            Fixed32 => quote! {u32},
            SFixed32 => quote! {i32},
            Float => quote! {f32},
        })
    }
}
impl Bits64Type {
    fn rust_type(&self) -> Result<TokenStream> {
        use Bits64Type::*;
        Ok(match self {
            Fixed64 => quote! {u64},
            SFixed64 => quote! {i64},
            Double => quote! {f64},
        })
    }
}
