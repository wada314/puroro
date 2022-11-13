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
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, FieldDescriptorProto};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait FieldTrait: Debug {
    fn gen_struct_field_decl(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub(super) struct Field {
    name: String,
    message: Weak<Box<dyn MessageTrait>>,
}

impl FieldTrait for Field {
    fn gen_struct_field_decl(&self) -> Result<TokenStream> {
        let name = format_ident!("{}", self.name.to_lower_snake_case().escape_rust_keywords());
        Ok(quote! {
            #name: (),
        })
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
        })))
    }
}
