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
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, OneofDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

pub trait MessageTrait: Debug {
    fn gen_struct(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub struct MessageImpl {
    name: String,
    submessages: Vec<Rc<dyn MessageTrait>>,
    enums: Vec<Rc<dyn EnumTrait>>,
    oneofs: Vec<Rc<dyn OneofTrait>>,
    fields: Vec<Rc<dyn FieldTrait>>,
}

impl MessageTrait for MessageImpl {
    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = format_ident!(
            "{}",
            self.name.to_camel_case().escape_rust_keywords().to_string()
        );
        let fields = self
            .fields()?
            .into_iter()
            .map(|f| f.gen_struct_field_decl())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            pub struct #ident {
                #(#fields)*
            }
        })
    }
}

impl MessageImpl {
    fn try_new<FM, FE, FO, FF>(
        proto: &DescriptorProto,
        fm: FM,
        fe: FE,
        fo: FO,
        ff: FF,
    ) -> Result<Rc<dyn MessageTrait>>
    where
        FM: FnMut(&DescriptorProto, Weak<dyn MessageTrait>) -> Result<Rc<dyn MessageTrait>>,
        FE: FnMut(&EnumDescriptorProto, Weak<dyn MessageTrait>) -> Result<Rc<dyn EnumTrait>>,
        FO: FnMut(&OneofDescriptorProto, Weak<dyn MessageTrait>) -> Result<Rc<dyn OneofTrait>>,
        FF: FnMut(&FieldDescriptorProto, Weak<dyn MessageTrait>) -> Result<Rc<dyn FieldTrait>>,
    {
        let name = proto.name().to_string();
        let mut maybe_error = None;
        let maybe_message = Rc::new_cyclic(|weak| {
            let res_message: Result<MessageImpl> = (|| {
                Ok(MessageImpl {
                    name,
                    submessages: proto
                        .nested_type()
                        .into_iter()
                        .map(|m| fm(m, Weak::clone(weak)))
                        .collect::<Result<Vec<_>>>()?,
                    enums: proto
                        .enum_type()
                        .into_iter()
                        .map(|e| fe(e, Weak::clone(weak)))
                        .collect::<Result<Vec<_>>>()?,
                    oneofs: proto
                        .oneof_decl()
                        .into_iter()
                        .map(|o| fo(o, Weak::clone(weak)))
                        .collect::<Result<Vec<_>>>()?,
                    fields: proto
                        .field()
                        .into_iter()
                        .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                        .map(|f| ff(f, Weak::clone(weak)))
                        .collect::<Result<Vec<_>>>()?,
                })
            })();
            match res_message {
                Ok(m) => m,
                Err(e) => {
                    maybe_error = Some(e);
                    MessageImpl {
                        name,
                        submessages: Vec::new(),
                        enums: Vec::new(),
                        oneofs: Vec::new(),
                        fields: Vec::new(),
                    }
                }
            }
        });
        match maybe_error {
            Some(e) => Err(e),
            None => Ok(maybe_message),
        }
    }
}

#[cfg(test)]
pub struct MessageFake;

#[cfg(test)]
impl MessageTrait for MessageFake {
    fn try_new(proto: &DescriptorProto, _context: &ContextForMessage) -> Result<Self> {
        Ok(MessageFake)
    }
}
