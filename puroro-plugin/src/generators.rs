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
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case, upgrade};
use crate::Result;
use ::askama::Template;
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google;
use ::std::borrow::Cow;
use ::std::rc::{Rc, Weak};

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
    pub wire_type: WireType,
    pub rust_field_type: String,
}

impl Field {
    pub fn try_new(f: &FieldDescriptorExt, resolver: &DescriptorResolver) -> Result<Self> {
        use google::protobuf::field_descriptor_proto::Label::*;
        let ident_lsnake = get_keyword_safe_ident(&to_lower_snake_case(f.name())).into();
        let ident_camel = get_keyword_safe_ident(&to_camel_case(f.name())).into();
        let syntax = f.try_parent()?.try_get_file()?.syntax();
        let rule = match (syntax, f.label(), f.proto3_optional()) {
            ("proto2", LabelOptional | LabelRequired, _) => FieldRule::Optional,
            ("proto3", LabelOptional, false) => FieldRule::Singular,
            ("proto3", LabelOptional, true) => FieldRule::Optional,
            (_, LabelRepeated, _) => FieldRule::Repeated,
            (syntax, label, opt) => Err(ErrorKind::InternalError {
                detail: format!(
                    "Unknown syntax/label/proto3opt combination: ({}, {}, {})",
                    syntax,
                    Into::<i32>::into(label),
                    opt
                ),
            })?,
        };
        let wire_type = WireType::from_proto_type(f.r#type(), f.type_name(), syntax, resolver)?;
        let rust_field_type_name = match (&rule, &wire_type) {
            (FieldRule::Optional, WireType::Variant(_)) => {
                format!("OptionalNumericField<{}, {}, {}>", "i32", "()", 0)
            }
            (FieldRule::Singular, WireType::Variant(_)) => {
                format!("SingularNumericField<{}, {}, {}>", "i32", "()", 0)
            }
            (FieldRule::Optional, WireType::LengthDelimited(LengthDelimitedType::String)) => {
                format!("OptionalStringField<{}>", 0)
            }
            (FieldRule::Singular, WireType::LengthDelimited(LengthDelimitedType::String)) => {
                format!("SingularStringField<{}>", 0)
            }
            (
                FieldRule::Optional | FieldRule::Singular,
                WireType::LengthDelimited(LengthDelimitedType::Message(weak_m)),
            ) => {
                let m = upgrade(weak_m)?;
                format!("ScalarHeapMessageField<{}, {}>", m.name(), 0)
            }
            _ => format!(""),
        };
        let rust_field_type = format!(
            "self::_puroro::internal::field_types::{}",
            rust_field_type_name
        );
        Ok(Self {
            ident_lsnake,
            ident_camel,
            rule,
            wire_type,
            rust_field_type,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FieldRule {
    Optional,
    Singular,
    Repeated,
}

#[derive(Debug, Clone)]
pub enum WireType {
    Variant(VariantType),
    LengthDelimited(LengthDelimitedType),
    Bits32(Bits32Type),
    Bits64(Bits64Type),
}

impl WireType {
    fn from_proto_type(
        r#type: google::protobuf::field_descriptor_proto::Type,
        type_name: &str,
        syntax: &str,
        resolver: &DescriptorResolver,
    ) -> Result<WireType> {
        use crate::descriptor_resolver::RcMessageOrEnum;
        use google::protobuf::field_descriptor_proto::Type::*;
        use Bits32Type::*;
        use Bits64Type::*;
        use LengthDelimitedType::*;
        use VariantType::*;
        use WireType::*;
        Ok(match r#type {
            TypeInt32 => Variant(Int32),
            TypeUint32 => Variant(UInt32),
            TypeSint32 => Variant(SInt32),
            TypeInt64 => Variant(Int64),
            TypeUint64 => Variant(UInt64),
            TypeSint64 => Variant(SInt64),
            TypeBool => Variant(Bool),
            TypeEnum => match resolver.fqtn_to_desc_or_err(type_name)? {
                RcMessageOrEnum::Message(_) => Err(ErrorKind::FqtnNotFound {
                    fqtn: type_name.to_string(),
                })?,
                RcMessageOrEnum::Enum(e) => Variant(match syntax {
                    "proto2" => Enum2(Rc::downgrade(&e)),
                    "proto3" => Enum3(Rc::downgrade(&e)),
                    _ => Err(ErrorKind::UnknownProtoSyntax {
                        name: syntax.to_string(),
                    })?,
                }),
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
            TypeMessage => match resolver.fqtn_to_desc_or_err(type_name)? {
                RcMessageOrEnum::Message(m) => LengthDelimited(Message(Rc::downgrade(&m))),
                RcMessageOrEnum::Enum(_) => Err(ErrorKind::FqtnNotFound {
                    fqtn: type_name.to_string(),
                })?,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub enum VariantType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Bool,
    Enum2(Weak<EnumDescriptorExt>),
    Enum3(Weak<EnumDescriptorExt>),
}

impl VariantType {
    pub fn into_owned_rust_type(&self) -> Cow<'static, str> {
        use VariantType::*;
        match self {
            Int32 => "i32".into(),
            UInt32 => "u32".into(),
            SInt32 => "i32".into(),
            Int64 => "i64".into(),
            UInt64 => "u64".into(),
            SInt64 => "i64".into(),
            Bool => "bool".into(),
            Enum2(_) => todo!(),
            Enum3(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LengthDelimitedType {
    String,
    Bytes,
    Message(Weak<DescriptorExt>),
}

#[derive(Debug, Clone)]
pub enum Bits32Type {
    Fixed32,
    SFixed32,
    Float,
}

#[derive(Debug, Clone)]
pub enum Bits64Type {
    Fixed64,
    SFixed64,
    Double,
}

fn enum_rust_type_full_path(e: &EnumDescriptorExt) -> Result<String> {
    todo!()
}
