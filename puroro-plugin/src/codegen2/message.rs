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

use super::*;
use crate::codegen::utils::StrExt;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, FieldDescriptorProto};
use ::quote::{format_ident, quote};

pub trait MessageTrait: Sized {
    fn try_new(proto: &DescriptorProto) -> Result<Self>;
    fn gen_struct(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub struct MessageImpl<EnumType, OneofType, FieldType> {
    name: String,
    submessages: Vec<Self>,
    enums: Vec<EnumType>,
    oneofs: Vec<OneofType>,
    fields: Vec<FieldType>,
}

pub type Message = MessageImpl<Enum, Oneof, Field>;

impl<EnumType: EnumTrait, OneofType: OneofTrait, FieldType: FieldTrait> MessageTrait
    for MessageImpl<EnumType, OneofType, FieldType>
{
    fn try_new(proto: &DescriptorProto) -> Result<Self> {
        let name = proto.name().to_string();
        Ok(MessageImpl {
            name,
            submessages: proto
                .nested_type()
                .into_iter()
                .map(|m| Self::try_new(m))
                .collect::<Result<Vec<_>>>()?,
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| EnumType::try_new(e))
                .collect::<Result<Vec<_>>>()?,
            oneofs: proto
                .oneof_decl()
                .into_iter()
                .map(|o| OneofType::try_new(o))
                .collect::<Result<Vec<_>>>()?,
            fields: Vec::new(), // TODO
        })
    }

    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = format_ident!(
            "{}",
            self.name.to_camel_case().escape_rust_keywords().to_string()
        );
        Ok(quote! {
            pub struct #ident {

            }
        })
    }
}

impl<EnumType, OneofType, FieldType> MessageImpl<EnumType, OneofType, FieldType> {
    pub fn submessages(&self) -> Result<&[Self]> {
        Ok(&self.submessages)
    }
    pub fn enums(&self) -> Result<&[EnumType]> {
        Ok(&self.enums)
    }
    pub fn oneofs(&self) -> Result<&[OneofType]> {
        Ok(&self.oneofs)
    }
    pub fn fields(&self) -> Result<&[FieldType]> {
        Ok(&self.fields)
    }
}

#[cfg(test)]
pub struct MessageFake;

#[cfg(test)]
impl MessageTrait for MessageFake {
    fn try_new(proto: &DescriptorProto) -> Result<Self> {
        Ok(MessageFake)
    }

    fn gen_struct(&self) -> Result<TokenStream> {
        Ok(quote! {})
    }
}
