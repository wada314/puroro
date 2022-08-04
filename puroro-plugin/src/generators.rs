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

use crate::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::error::ErrorKind;
use crate::utils::StrExt as _;
use crate::Result;
use ::askama::Template;
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google;
use ::std::borrow::Cow;
use google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};

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
        let ident = p.package_name.as_ref().map_or_else(Default::default, |s| {
            s.to_lower_snake_case().escape_rust_keywords().into()
        });
        let is_root_package = p.package_name.is_none();
        let full_path = p.full_package.as_str().to_string();
        let subpackages = p
            .subpackages
            .iter()
            .map(|sp| {
                let new_package = if is_root_package {
                    sp.clone()
                } else {
                    format!("{}.{}", full_path.as_str(), sp)
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
            .flat_map(|f| {
                f.message_type()
                    .iter()
                    .map(|m| Module::try_from_message(m, f, &full_path, resolver))
            })
            .filter_ok(|m| !m.messages.is_empty() || !m.enums.is_empty())
            .collect::<Result<Vec<_>>>()?;
        let mut submodules = submodules_from_packages;
        submodules.append(&mut submodules_from_messages);
        let messages = p
            .input_files
            .iter()
            .flat_map(|f| {
                f.message_type()
                    .iter()
                    .map(|m| Message::try_new(m, f, resolver))
            })
            .collect::<Result<Vec<_>>>()?;
        let enums = p
            .input_files
            .iter()
            .flat_map(|f| f.enum_type().iter().map(|e| Enum::try_new(e, f, resolver)))
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

    pub fn try_from_message(
        m: &DescriptorProto,
        f: &FileDescriptorProto,
        package: &str,
        resolver: &DescriptorResolver,
    ) -> Result<Self> {
        let ident = m.name().to_lower_snake_case().escape_rust_keywords().into();
        let is_root_package = false;
        let full_path = if package.is_empty() {
            m.name().to_string()
        } else {
            format!("{}.{}", package, m.name())
        };
        let submodules = m
            .nested_type()
            .into_iter()
            .map(|d| Module::try_from_message(d, f, &full_path, resolver))
            .collect::<Result<Vec<_>>>()?;
        let messages = m
            .nested_type()
            .into_iter()
            .map(|d| Message::try_new(d, f, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = m
            .enum_type()
            .into_iter()
            .map(|e| Enum::try_new(e, f, resolver))
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
    pub bits_length: usize,
}

impl Message {
    #[allow(unused)]
    pub fn try_new(
        m: &DescriptorProto,
        f: &FileDescriptorProto,
        resolver: &DescriptorResolver,
    ) -> Result<Self> {
        let ident_camel = m.name().to_camel_case().escape_rust_keywords().into();
        let ident_lsnake = m.name().to_lower_snake_case().escape_rust_keywords().into();
        let mut bits_index = 0usize;
        let fields = m
            .field()
            .into_iter()
            .map(|f| Field::try_new(f, &mut bits_index, resolver))
            .collect::<Result<Vec<_>>>()?;
        let bits_length = bits_index;
        Ok(Message {
            ident_camel,
            ident_lsnake,
            fields,
            bits_length,
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
    pub fn try_new(
        e: &EnumDescriptorProto,
        f: &FileDescriptorProto,
        resolver: &DescriptorResolver,
    ) -> Result<Self> {
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
    pub rust_getter_type: String,
}

impl Field {
    pub fn try_new(
        f: &FieldDescriptorProto,
        bit_index: &mut usize,
        resolver: &DescriptorResolver,
    ) -> Result<Self> {
        use google::protobuf::field_descriptor_proto::Label::*;

        let ident_camel = f.name().to_camel_case().escape_rust_keywords().into();
        let ident_lsnake = f.name().to_lower_snake_case().escape_rust_keywords().into();
        let syntax: String = todo!();
        let rule = match (syntax.as_str(), f.label(), f.proto3_optional()) {
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
        let wire_type = WireType::from_proto_type(f.r#type(), f.type_name(), &syntax, resolver)?;
        let bit_index_for_optional = {
            let index = *bit_index;
            if matches!(rule, FieldRule::Optional) {
                *bit_index += 1;
            }
            index
        };
        let rust_field_type_name = match (&rule, &wire_type) {
            (FieldRule::Optional, WireType::Variant(v)) => {
                format!(
                    "OptionalNumericField<{}, {}, {}>",
                    v.into_owned_rust_type(),
                    "()",
                    bit_index_for_optional
                )
            }
            (FieldRule::Singular, WireType::Variant(v)) => {
                format!(
                    "SingularNumericField<{}, {}>",
                    v.into_owned_rust_type(),
                    "()"
                )
            }
            (FieldRule::Optional, WireType::LengthDelimited(LengthDelimitedType::String)) => {
                format!("OptionalStringField<{}>", bit_index_for_optional)
            }
            (FieldRule::Singular, WireType::LengthDelimited(LengthDelimitedType::String)) => {
                format!("SingularStringField")
            }
            (
                FieldRule::Optional | FieldRule::Singular,
                WireType::LengthDelimited(LengthDelimitedType::Message),
            ) => {
                format!("SingularHeapMessageField<{}>", todo!())
            }
            _ => format!(""),
        };
        let rust_field_type = format!(
            "self::_puroro::internal::field_types::{}",
            rust_field_type_name
        );
        let rust_getter_type = wire_type
            .into_getter_rust_type(matches!(&rule, &FieldRule::Repeated))
            .into_owned();
        Ok(Self {
            ident_lsnake,
            ident_camel,
            rule,
            wire_type,
            rust_field_type,
            rust_getter_type,
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
            TypeEnum => todo!(),
            TypeFixed32 => Bits32(Fixed32),
            TypeSfixed32 => Bits32(SFixed32),
            TypeFloat => Bits32(Float),
            TypeFixed64 => Bits64(Fixed64),
            TypeSfixed64 => Bits64(SFixed64),
            TypeDouble => Bits64(Double),
            TypeString => LengthDelimited(String),
            TypeBytes => LengthDelimited(Bytes),
            TypeGroup => Err(ErrorKind::GroupNotSupported)?,
            TypeMessage => todo!(),
        })
    }

    pub fn into_getter_rust_type(&self, is_repeated: bool) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        use WireType::*;
        if is_repeated {
            todo!()
        } else {
            match self {
                Variant(v) => v.into_owned_rust_type(),
                LengthDelimited(String) => "&str".into(),
                LengthDelimited(Bytes) => "&[u8]".into(),
                LengthDelimited(Message) => todo!(),
                Bits32(b) => b.into_owned_rust_type(),
                Bits64(b) => b.into_owned_rust_type(),
            }
        }
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
    Enum2,
    Enum3,
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
            Enum2 => todo!(),
            Enum3 => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LengthDelimitedType {
    String,
    Bytes,
    Message,
}

#[derive(Debug, Clone)]
pub enum Bits32Type {
    Fixed32,
    SFixed32,
    Float,
}

impl Bits32Type {
    fn into_owned_rust_type(&self) -> Cow<'static, str> {
        use Bits32Type::*;
        match self {
            Fixed32 => "u32".into(),
            SFixed32 => "i32".into(),
            Float => "f32".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Bits64Type {
    Fixed64,
    SFixed64,
    Double,
}

impl Bits64Type {
    fn into_owned_rust_type(&self) -> Cow<'static, str> {
        use Bits64Type::*;
        match self {
            Fixed64 => "u64".into(),
            SFixed64 => "i64".into(),
            Double => "f64".into(),
        }
    }
}
