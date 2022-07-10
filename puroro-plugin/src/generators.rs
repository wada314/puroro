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

use crate::descriptor_ext::*;
use crate::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::error::ErrorKind;
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case};
use crate::Result;
use ::askama::Template;
use ::itertools::Itertools;

#[derive(Template, Debug)]
#[template(path = "module.rs.txt")]
pub struct Module {
    pub ident: String,
    pub is_root_package: bool,
    pub full_path: String,
    pub submodules: Vec<Module>,
    pub messages: Vec<Message>,
    pub enums: Vec<Enum>,
}
impl Module {
    pub fn try_from_package(p: &PackageContents, resolver: &DescriptorResolver) -> Result<Self> {
        let ident = get_keyword_safe_ident(&to_lower_snake_case(&p.name)).into();
        let is_root_package = p.name.is_empty();
        let full_path = p.full_package.clone();
        let subpackages = p
            .subpackages
            .iter()
            .map(|sp| {
                let new_package = if is_root_package {
                    sp.clone()
                } else {
                    format!("{}.{}", full_path, sp)
                };
                resolver.package_contents_or_err(&new_package)
            })
            .collect::<Result<Vec<_>>>()?;
        let submodules_from_packages = subpackages
            .iter()
            .map(|p| Module::try_from_package(*p, resolver))
            .collect::<Result<Vec<_>>>()?;
        let mut submodules_from_messages = p
            .input_files
            .iter()
            .map(|f| f.message_type().iter())
            .flatten()
            .map(|m| Module::try_from_message(m, resolver))
            .filter_ok(|m| !m.messages.is_empty() || !m.enums.is_empty())
            .collect::<Result<Vec<_>>>()?;
        let mut submodules = submodules_from_packages;
        submodules.append(&mut submodules_from_messages);
        let messages = p
            .input_files
            .iter()
            .map(|f| f.message_type().iter())
            .flatten()
            .map(|m| Message::try_new(m, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = p
            .input_files
            .iter()
            .map(|f| f.enum_type().iter())
            .flatten()
            .map(|e| Enum::try_new(e, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Module {
            ident,
            is_root_package,
            full_path,
            submodules,
            messages,
            enums,
        })
    }

    pub fn try_from_message(m: &DescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        let ident = to_lower_snake_case(m.name());
        let is_root_package = false;
        let full_path = m
            .try_package_opt()?
            .into_iter()
            .chain(m.try_enclosing_messages_opt()?.into_iter())
            .join(".");
        let submodules = m
            .nested_type()
            .into_iter()
            .map(|d| Module::try_from_message(d, resolver))
            .collect::<Result<Vec<_>>>()?;
        let messages = m
            .nested_type()
            .into_iter()
            .map(|d| Message::try_new(d, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = m
            .enum_type()
            .into_iter()
            .map(|e| Enum::try_new(e, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Module {
            ident,
            is_root_package,
            full_path,
            submodules,
            messages,
            enums,
        })
    }
}

#[derive(Template, Debug)]
#[template(path = "message.rs.txt")]
pub struct Message {
    pub ident_camel: String,
    pub ident_lsnake: String,
    pub fields: Vec<Field>,
}

impl Message {
    #[allow(unused)]
    pub fn try_new(m: &DescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        let ident_camel = get_keyword_safe_ident(&to_camel_case(m.name())).into();
        let ident_lsnake = get_keyword_safe_ident(&to_lower_snake_case(m.name())).into();
        let fields = m
            .field()
            .into_iter()
            .map(|f| Field::try_new(f, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Message {
            ident_camel,
            ident_lsnake,
            fields,
        })
    }
}

#[derive(Template, Debug)]
#[template(path = "enum.rs.txt")]
pub struct Enum {
    pub ident_camel: String,
}

impl Enum {
    #[allow(unused)]
    pub fn try_new(e: &EnumDescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Field {
    pub ident_lsnake: String,
    pub ident_camel: String,
    pub rule: FieldRule,
    pub r#type: FieldType,
}

impl Field {
    pub fn try_new(f: &FieldDescriptorExt, _resolver: &DescriptorResolver) -> Result<Self> {
        use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto::Label::*;
        use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto::Type::*;
        let ident_lsnake = get_keyword_safe_ident(&to_lower_snake_case(f.name())).into();
        let ident_camel = get_keyword_safe_ident(&to_camel_case(f.name())).into();
        let rule = match (
            f.try_parent()?.try_get_file()?.syntax(),
            f.label(),
            f.proto3_optional(),
        ) {
            ("proto2", LabelOptional | LabelRequired, _) => FieldRule::Optional,
            ("proto3", LabelOptional, false) => FieldRule::Singular,
            ("proto3", LabelOptional, true) => FieldRule::Optional,
            (_, LabelRepeated, _) => FieldRule::Repeated,
            _ => Err(ErrorKind::InternalError {
                detail: "Unknown syntax/label/proto3opt combination.".to_string(),
            })?,
        };
        let r#type = match f.r#type() {
            TypeString => FieldType::String,
            TypeMessage => FieldType::Message,
            _ => FieldType::Int32,
        };
        Ok(Self {
            ident_lsnake,
            ident_camel,
            rule,
            r#type,
        })
    }
}

#[derive(Debug)]
pub enum FieldRule {
    Optional,
    Singular,
    Repeated,
}

#[allow(unused)]
#[derive(Debug)]
pub enum FieldType {
    Int32,
    UInt32,
    SInt32,
    Fixed32,
    Int64,
    UInt64,
    SInt64,
    Fixed64,
    Bool,
    Float,
    Double,
    Bytes,
    String,
    Message,
}
