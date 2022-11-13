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

use super::util::WeakExt;
use super::*;
use crate::codegen::utils::StrExt;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{field_descriptor_proto, FieldDescriptorProto};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait FieldTrait: Debug {
    fn gen_struct_field_decl(&self) -> Result<TokenStream>;
    fn message(&self) -> Result<Rc<Box<dyn MessageTrait>>>;
}

#[derive(Debug)]
pub(super) struct Field {
    name: String,
    message: Weak<Box<dyn MessageTrait>>,
    rule: OnceCell<FieldRule>,
    proto3_optional: bool,
    label: field_descriptor_proto::Label,
    r#type: field_descriptor_proto::Type,
    number: i32,
    type_name: String,
}

impl FieldTrait for Field {
    fn gen_struct_field_decl(&self) -> Result<TokenStream> {
        let name = format_ident!("{}", self.name.to_lower_snake_case().escape_rust_keywords());
        Ok(quote! {
            #name: (),
        })
    }
    fn message(&self) -> Result<Rc<Box<dyn MessageTrait>>> {
        Ok(self.message.try_upgrade()?)
    }
}

impl Field {
    pub(super) fn try_new(
        proto: &FieldDescriptorProto,
        message: &Weak<Box<dyn MessageTrait>>,
    ) -> Result<Rc<Box<dyn FieldTrait>>> {
        Ok(Rc::new(Box::new(Field {
            name: proto.name().to_string(),
            message: Weak::clone(message),
            rule: OnceCell::new(),
            label: proto.label(),
            proto3_optional: proto.proto3_optional(),
            r#type: proto.r#type(),
            number: proto.number(),
            type_name: proto.type_name().to_string(),
        })))
    }

    fn rule(&self) -> Result<FieldRule> {
        self.rule
            .get_or_try_init(|| {
                let syntax = self.message()?.input_file()?.syntax();
                Ok(FieldRule::try_new(
                    self.label.clone(),
                    syntax,
                    self.proto3_optional,
                )?)
            })
            .cloned()
    }
}
