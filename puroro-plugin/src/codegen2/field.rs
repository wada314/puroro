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
    r#type: OnceCell<FieldType>,
    proto3_optional: bool,
    label_opt: Option<field_descriptor_proto::Label>,
    type_opt: Option<field_descriptor_proto::Type>,
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
            r#type: OnceCell::new(),
            label_opt: proto.label_opt(),
            proto3_optional: proto.proto3_optional(),
            type_opt: proto.type_opt(),
            number: proto.number(),
            type_name: proto.type_name().to_string(),
        })))
    }

    fn rule(&self) -> Result<FieldRule> {
        self.rule
            .get_or_try_init(|| {
                let syntax = self.message()?.input_file()?.syntax();
                Ok(FieldRule::try_new(
                    self.label_opt.clone(),
                    syntax,
                    self.proto3_optional,
                )?)
            })
            .cloned()
    }

    fn r#type(&self) -> Result<&FieldType> {
        self.r#type.get_or_try_init(|| {
            let syntax = self.message()?.input_file()?.syntax();
            Ok(FieldType::try_new(
                self.type_opt.clone(),
                &self.type_name,
                syntax,
            )?)
        })
    }

    fn gen_struct_field_type(&self) -> Result<::syn::Type> {
        use FieldRule::*;
        use FieldType::*;
        let primitive_type = self.r#type()?.rust_type()?;
        /*
        (Optional, Variant(_) | Bits32(_) | Bits64(_)) => {
            format!(
                "OptionalNumericalField::<{}, {}, {}>",
                wire_type.into_owned_rust_type(),
                wire_type.into_tag_type(),
                bit_index_for_optional
            )
        }
        (Singular, Variant(_) | Bits32(_) | Bits64(_)) => {
            format!(
                "SingularNumericalField::<{}, {}>",
                wire_type.into_owned_rust_type(),
                wire_type.into_tag_type(),
            )
        }
        (Repeated, Variant(_) | Bits32(_) | Bits64(_)) => {
            format!(
                "RepeatedNumericalField::<{}, {}>",
                wire_type.into_owned_rust_type(),
                wire_type.into_tag_type(),
            )
        }
        (Optional, LengthDelimited(String)) => {
            format!("OptionalStringField::<{}>", bit_index_for_optional)
        }
        (Singular, LengthDelimited(String)) => {
            format!("SingularStringField")
        }
        (Repeated, LengthDelimited(String)) => {
            format!("RepeatedStringField")
        }
        (Optional, LengthDelimited(Bytes)) => {
            format!("OptionalBytesField<{}>", bit_index_for_optional)
        }
        (Singular, LengthDelimited(Bytes)) => {
            format!("SingularBytesField")
        }
        (Repeated, LengthDelimited(Bytes)) => {
            format!("RepeatedBytesField")
        }
        (Optional | Singular, LengthDelimited(Message(fqtn))) => {
            format!("SingularHeapMessageField::<{}>", fqtn.to_rust_path())
        }
        (Repeated, LengthDelimited(Message(fqtn))) => {
            format!("RepeatedMessageField::<{}>", fqtn.to_rust_path())
        } */
        match (self.rule()?, self.r#type()?) {
            (Optional, Variant(_)) => todo!(),
            (Optional, Bits32(_)) => todo!(),
            (Optional, Bits64(_)) => todo!(),
            (Optional, LengthDelimited(_)) => todo!(),
            (Singular, Variant(_)) => todo!(),
            (Singular, LengthDelimited(_)) => todo!(),
            (Singular, Bits32(_)) => todo!(),
            (Singular, Bits64(_)) => todo!(),
            (Repeated, Variant(_)) => todo!(),
            (Repeated, LengthDelimited(_)) => todo!(),
            (Repeated, Bits32(_)) => todo!(),
            (Repeated, Bits64(_)) => todo!(),
        }
        todo!()
    }
}
