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

use crate::descriptor_ext::{FieldDescriptorExt, FileDescriptorExt, FileOrMessage};
use crate::descriptor_resolver::{DescriptorResolver, PackageContents};
use crate::restructure;
use crate::restructure::Syntax;
use crate::utils::{Fqtn, StrExt as _};
use crate::{ErrorKind, Result};
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
    pub fqtn: String,
    pub submodules: Vec<Module>,
    pub messages: Vec<Message>,
    pub enums: Vec<Enum>,
}
impl Module {
    pub fn try_from_package<'a>(
        p: &'a PackageContents<'a>,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Self> {
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
                f.messages()
                    .iter()
                    .map(|m| Module::try_from_message(m, resolver))
            })
            .filter_ok(|m| !m.messages.is_empty() || !m.enums.is_empty())
            .collect::<Result<Vec<_>>>()?;
        let mut submodules = submodules_from_packages;
        submodules.append(&mut submodules_from_messages);
        let messages = p
            .input_files
            .iter()
            .flat_map(|f| f.messages().iter())
            .map(|m| Message::try_new(m, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = p
            .input_files
            .iter()
            .flat_map(|f| f.enums().into_iter())
            .map(|e| Enum::try_new(e, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Module {
            ident,
            is_root_package,
            fqtn: full_path,
            submodules,
            messages,
            enums,
        })
    }

    pub fn try_from_message<'a>(
        m: &'a restructure::Message<'a>,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Self> {
        let ident = m.name().to_lower_snake_case().escape_rust_keywords().into();
        let is_root_package = false;
        let fqtn = m.fqtn().to_string();
        let submodules = m
            .messages()
            .into_iter()
            .map(|d| Module::try_from_message(d, resolver))
            .collect::<Result<Vec<_>>>()?;
        let messages = m
            .messages()
            .into_iter()
            .map(|d| Message::try_new(d, resolver))
            .collect::<Result<Vec<_>>>()?;
        let enums = m
            .enums()
            .into_iter()
            .map(|e| Enum::try_new(e, resolver))
            .collect::<Result<Vec<_>>>()?;
        Ok(Module {
            ident,
            is_root_package,
            fqtn,
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
    pub fn try_new<'a>(
        m: &'a restructure::Message<'a>,
        resolver: &'a DescriptorResolver<'a>,
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
    pub fn try_new(e: &restructure::Enum, resolver: &DescriptorResolver) -> Result<Self> {
        let ident_camel = e.name().to_camel_case().escape_rust_keywords().into_owned();
        Ok(Enum { ident_camel })
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
    pub number: i32,
}

impl Field {
    pub fn try_new<'a>(
        f: &'a restructure::Field<'a>,
        bit_index: &mut usize,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Self> {
        use google::protobuf::field_descriptor_proto::Label::*;

        let ident_camel = f.name().to_camel_case().escape_rust_keywords().into();
        let ident_lsnake = f.name().to_lower_snake_case().escape_rust_keywords().into();
        let syntax = f.parent().file().try_syntax()?;
        let rule = match (syntax, f.label(), f.proto3_optional()) {
            (Syntax::Proto2, LabelOptional | LabelRequired, _) => FieldRule::Optional,
            (Syntax::Proto3, LabelOptional, false) => FieldRule::Singular,
            (Syntax::Proto3, LabelOptional, true) => FieldRule::Optional,
            (_, LabelRepeated, _) => FieldRule::Repeated,
            (syntax, label, opt) => Err(ErrorKind::InternalError {
                detail: format!(
                    "Unknown syntax/label/proto3opt combination: ({:?}, {}, {})",
                    syntax,
                    Into::<i32>::into(label),
                    opt
                ),
            })?,
        };
        let wire_type = WireType::from_proto_type(f.r#type(), f.fqtn_opt(), syntax, resolver)?;
        let bit_index_for_optional = {
            let index = *bit_index;
            if matches!(rule, FieldRule::Optional) {
                *bit_index += 1;
            }
            index
        };
        let rust_field_type_name = {
            use FieldRule::*;
            use LengthDelimitedType::*;
            use WireType::*;
            match (&rule, &wire_type) {
                (Optional, Variant(_) | Bits32(_) | Bits64(_)) => {
                    format!(
                        "OptionalNumericalField<{}, {}, {}>",
                        wire_type.into_owned_rust_type(),
                        wire_type.into_tag_type(),
                        bit_index_for_optional
                    )
                }
                (Singular, Variant(_) | Bits32(_) | Bits64(_)) => {
                    format!(
                        "SingularNumericalField<{}, {}>",
                        wire_type.into_owned_rust_type(),
                        wire_type.into_tag_type(),
                    )
                }
                (Repeated, Variant(_) | Bits32(_) | Bits64(_)) => {
                    format!(
                        "RepeatedNumericalField<{}, {}>",
                        wire_type.into_owned_rust_type(),
                        wire_type.into_tag_type(),
                    )
                }
                (Optional, LengthDelimited(String)) => {
                    format!("OptionalStringField<{}>", bit_index_for_optional)
                }
                (Singular, LengthDelimited(String)) => {
                    format!("SingularStringField")
                }
                (Optional | Singular, LengthDelimited(Message(fqtn))) => {
                    format!("SingularHeapMessageField<{}>", fqtn.to_rust_path())
                }
                _ => format!("Dummy"), // TODO
            }
        };
        let rust_field_type = format!(
            "self::_puroro::internal::field_types::{}",
            rust_field_type_name
        );
        let rust_getter_type = wire_type
            .into_getter_rust_type(matches!(&rule, &FieldRule::Repeated))
            .into_owned();
        let number = f.number();
        Ok(Self {
            ident_lsnake,
            ident_camel,
            rule,
            wire_type,
            rust_field_type,
            rust_getter_type,
            number,
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
        fqtn: Option<Fqtn<&str>>,
        syntax: Syntax,
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
            TypeEnum => match syntax {
                Syntax::Proto2 => {
                    Variant(Enum2(fqtn.ok_or(ErrorKind::MissingTypeName)?.to_owned()))
                }
                Syntax::Proto3 => {
                    Variant(Enum3(fqtn.ok_or(ErrorKind::MissingTypeName)?.to_owned()))
                }
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
            TypeMessage => {
                LengthDelimited(Message(fqtn.ok_or(ErrorKind::MissingTypeName)?.to_owned()))
            }
        })
    }

    pub fn into_getter_rust_type(&self, is_repeated: bool) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        use WireType::*;
        if is_repeated {
            match self {
                Variant(var) => format!("&[{}]", var.into_owned_rust_type()).into(),
                LengthDelimited(_) => "&[()]".into(), // TODO
                Bits32(x) => format!("&[{}]", x.into_owned_rust_type()).into(),
                Bits64(x) => format!("&[{}]", x.into_owned_rust_type()).into(),
            }
        } else {
            match self {
                Variant(var) => var.into_owned_rust_type(),
                LengthDelimited(String) => "&str".into(),
                LengthDelimited(Bytes) => "&[u8]".into(),
                LengthDelimited(Message(fqtn)) => {
                    format!("Option<&{}>", fqtn.to_rust_path()).into()
                }
                Bits32(x) => x.into_owned_rust_type(),
                Bits64(x) => x.into_owned_rust_type(),
            }
        }
    }

    pub fn into_owned_rust_type(&self) -> Cow<'static, str> {
        use WireType::*;
        match self {
            Variant(v) => v.into_owned_rust_type(),
            LengthDelimited(ld) => ld.into_owned_rust_type(),
            Bits32(x) => x.into_owned_rust_type(),
            Bits64(x) => x.into_owned_rust_type(),
        }
    }

    pub fn into_tag_type(&self) -> Cow<'static, str> {
        use WireType::*;
        match self {
            Variant(v) => v.into_tag_type(),
            LengthDelimited(ld) => ld.into_tag_type(),
            Bits32(x) => x.into_tag_type(),
            Bits64(x) => x.into_tag_type(),
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
    Enum2(Fqtn<String>),
    Enum3(Fqtn<String>),
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
            Enum2(fqtn) => fqtn.to_rust_path().into(),
            Enum3(fqtn) => fqtn.to_rust_path().into(),
        }
    }

    pub fn into_tag_type(&self) -> Cow<'static, str> {
        use VariantType::*;
        match self {
            Int32 => "self::_puroro::tags::Int32".into(),
            UInt32 => "self::_puroro::tags::UInt32".into(),
            SInt32 => "self::_puroro::tags::SInt32".into(),
            Int64 => "self::_puroro::tags::Int64".into(),
            UInt64 => "self::_puroro::tags::UInt64".into(),
            SInt64 => "self::_puroro::tags::SInt64".into(),
            Bool => "self::_puroro::tags::Bool".into(),
            Enum2(e) => format!("self::_puroro::tags::Enum2<{}>", e.to_rust_path()).into(),
            Enum3(e) => format!("self::_puroro::tags::Enum3<{}>", e.to_rust_path()).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LengthDelimitedType {
    String,
    Bytes,
    Message(Fqtn<String>),
}

impl LengthDelimitedType {
    pub fn into_owned_rust_type(&self) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        match self {
            Bytes => "::std::vec::Vec<u8>".into(),
            String => "::std::string::String".into(),
            Message(fqtn) => fqtn.to_rust_path().into(),
        }
    }

    pub fn into_tag_type(&self) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        match self {
            String => "self::_puroro::tags::String".into(),
            Bytes => "self::_puroro::tags::Bytes".into(),
            Message(m) => format!("self::_puroro::tags::Message<{}>", m.to_rust_path()).into(),
        }
    }
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

    pub fn into_tag_type(&self) -> Cow<'static, str> {
        use Bits32Type::*;
        match self {
            Fixed32 => "self::_puroro::tags::Fixed32".into(),
            SFixed32 => "self::_puroro::tags::SFixed32".into(),
            Float => "self::_puroro::tags::Float".into(),
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

    pub fn into_tag_type(&self) -> Cow<'static, str> {
        use Bits64Type::*;
        match self {
            Fixed64 => "self::_puroro::tags::Fixed64".into(),
            SFixed64 => "self::_puroro::tags::SFixed64".into(),
            Double => "self::_puroro::tags::Double".into(),
        }
    }
}
