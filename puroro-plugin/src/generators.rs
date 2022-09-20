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
use crate::restructure as re;
use crate::restructure::Syntax;
use crate::utils::{Fqtn, Package, StrExt as _};
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google;
use ::std::borrow::Cow;

#[derive(Template, Debug)]
#[template(path = "module.rs.txt")]
pub struct Module {
    pub ident_module: String,
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
        let ident_module = p.package_name.as_ref().map_or_else(Default::default, |s| {
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
                resolver.package_contents_or_err(&Package::new(new_package))
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
            ident_module,
            is_root_package,
            fqtn: full_path,
            submodules,
            messages,
            enums,
        })
    }

    pub fn try_from_message<'a>(
        m: &'a re::Message<'a>,
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
            ident_module: ident,
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
    pub ident_struct: String,
    pub ident_module: String,
    pub fields: Vec<Field>,
    pub oneofs: Vec<Oneof>,
    pub bits_length: usize,
}
impl Message {
    #[allow(unused)]
    pub fn try_new<'a>(
        m: &'a re::Message<'a>,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Self> {
        let ident_struct = m.name().to_camel_case().escape_rust_keywords().into();
        let ident_module = m.name().to_lower_snake_case().escape_rust_keywords().into();
        let mut bits_index = 0usize;
        let fields = m
            .field()
            .into_iter()
            .map(|f| Field::try_new(f, &mut bits_index, resolver))
            .collect::<Result<Vec<_>>>()?;
        let oneofs = m
            .oneofs()
            .into_iter()
            .map(|o| Oneof::try_new(o, &mut bits_index, resolver))
            .collect::<Result<Vec<_>>>()?;
        let bits_length = bits_index;
        Ok(Message {
            ident_struct,
            ident_module,
            fields,
            oneofs,
            bits_length,
        })
    }
}

#[derive(Template, Debug)]
#[template(path = "enum.rs.txt")]
pub struct Enum {
    pub ident_enum: String,
}

impl Enum {
    #[allow(unused)]
    pub fn try_new(e: &re::Enum, resolver: &DescriptorResolver) -> Result<Self> {
        let ident_enum = e.name().to_camel_case().escape_rust_keywords().into_owned();
        Ok(Enum { ident_enum })
    }
}

#[derive(Debug)]
pub struct Field {
    pub ident_struct_field: String,
    pub ident_getter: String,
    pub ident_getter_opt: String,
    pub ident_has: String,
    pub ident_clear: String,
    pub ident_getter_mut: String,
    pub rule: FieldRule,
    pub wire_type: WireType,
    pub rust_field_type: String,
    pub rust_getter_type: String,
    pub rust_getter_opt_type: String,
    pub rust_getter_mut_type: String,
    pub number: i32,
}

impl Field {
    pub fn try_new<'a>(
        f: &'a re::Field<'a>,
        bit_index: &mut usize,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Self> {
        use google::protobuf::field_descriptor_proto::Label::*;

        let ident_struct_field = f.name().to_lower_snake_case().escape_rust_keywords().into();
        let ident_getter = f.name().to_lower_snake_case().escape_rust_keywords().into();
        let ident_getter_opt = format!("{}_opt", f.name().to_lower_snake_case());
        let ident_has = format!("has_{}", f.name().to_lower_snake_case());
        let ident_clear = format!("clear_{}", f.name().to_lower_snake_case());
        let ident_getter_mut = format!("{}_mut", f.name().to_lower_snake_case());

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
        let wire_type = WireType::from_field(f, resolver)?;
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
            "self::_puroro::internal::field_type::{}",
            rust_field_type_name
        );
        let rust_getter_type = wire_type
            .into_getter_rust_type(matches!(&rule, &FieldRule::Repeated))
            .into_owned();
        let rust_getter_opt_type = wire_type.into_opt_getter_rust_type().into_owned();
        let rust_getter_mut_type = wire_type
            .into_mut_getter_rust_type(matches!(&rule, &FieldRule::Repeated))
            .into_owned();
        let number = f.number();
        Ok(Self {
            ident_struct_field,
            ident_getter,
            ident_getter_opt,
            ident_has,
            ident_clear,
            ident_getter_mut,
            rule,
            wire_type,
            rust_field_type,
            rust_getter_type,
            rust_getter_opt_type,
            rust_getter_mut_type,
            number,
        })
    }
}

#[derive(Debug)]
pub struct Oneof {
    pub ident_union: String,
    pub ident_case: String,
    pub ident_case_ref: String,
    pub fields: Vec<OneofField>,
    pub has_ld_type: bool,
    pub bitfield_start: usize,
    pub bitfield_end: usize,
    pub num_fields: usize,
}
impl Oneof {
    pub fn try_new<'a>(
        o: &'a re::Oneof<'a>,
        bit_index: &mut usize,
        resolver: &'a DescriptorResolver,
    ) -> Result<Self> {
        let ident_union = o.name().to_camel_case().escape_rust_keywords().to_string();
        let ident_case = format!("{}Case", o.name().to_camel_case());
        let ident_case_ref = format!("{}CaseRef", o.name().to_camel_case());
        let fields = o
            .fields()
            .into_iter()
            .enumerate()
            .map(|(index, f)| OneofField::try_new(f, index, resolver))
            .collect::<Result<Vec<_>>>()?;
        use google::protobuf::field_descriptor_proto::Type::*;
        let has_ld_type = o
            .fields()
            .into_iter()
            .any(|f| matches!(f.r#type(), TypeBytes | TypeString | TypeMessage));
        let num_fields = fields.len();
        let num_bits = usize::BITS - num_fields.leading_zeros();
        let bitfield_start = *bit_index;
        *bit_index += num_bits as usize;
        let bitfield_end = *bit_index;
        let num_fields = fields.len();

        Ok(Self {
            ident_union,
            ident_case,
            ident_case_ref,
            fields,
            has_ld_type,
            bitfield_start,
            bitfield_end,
            num_fields,
        })
    }
}

#[derive(Debug)]
pub struct OneofField {
    pub ident_camel: String,
    pub ident_union_item: String,
    pub ident_opt_getter: String,
    pub ident_enum_item: String,
    pub index: usize,
    pub rust_field_type: String,
    pub rust_field_inner_type: String,
    pub rust_getter_type: String,
    pub rust_opt_getter_type: String,
    pub rust_oneof_getter_type: String,
}
impl OneofField {
    pub fn try_new<'a>(
        f: &'a re::OneofField<'a>,
        index: usize,
        resolver: &'a DescriptorResolver,
    ) -> Result<Self> {
        let ident_camel = f.name().to_camel_case().escape_rust_keywords().to_string();
        let ident_union_item = f
            .name()
            .to_lower_snake_case()
            .escape_rust_keywords()
            .to_string();
        let ident_opt_getter = format!("{}_opt", f.name().to_lower_snake_case());
        let ident_enum_item = f.name().to_camel_case().escape_rust_keywords().to_string();
        let wire_type = WireType::from_oneof_field(f, resolver)?;
        let rust_field_inner_type_name = {
            use LengthDelimitedType::*;
            use WireType::*;
            match &wire_type {
                Variant(_) | Bits32(_) | Bits64(_) => {
                    format!(
                        "NumericalField<{}, {}>",
                        wire_type.into_owned_rust_type(),
                        wire_type.into_tag_type(),
                    )
                }
                LengthDelimited(String) => {
                    format!("StringField")
                }
                LengthDelimited(Message(m)) => {
                    format!("HeapMessageField<{}>", m.to_rust_path())
                }
                _ => format!("Dummy"), // TODO
            }
        };
        let rust_field_inner_type = format!(
            "self::_puroro::internal::oneof_field_type::{}",
            rust_field_inner_type_name
        );
        let rust_field_type = format!("::std::mem::ManuallyDrop<{}>", rust_field_inner_type);

        let rust_getter_type = wire_type.into_getter_rust_type(false).into_owned();
        let rust_opt_getter_type = wire_type.into_opt_getter_rust_type().into_owned();
        let rust_oneof_getter_type = wire_type.into_oneof_getter_rust_type("'a").into_owned();
        Ok(Self {
            ident_camel,
            ident_union_item,
            ident_opt_getter,
            ident_enum_item,
            index,
            rust_field_type,
            rust_field_inner_type,
            rust_getter_type,
            rust_opt_getter_type,
            rust_oneof_getter_type,
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
    fn from_field<'a>(field: &'a re::Field<'a>, _resolver: &'a DescriptorResolver) -> Result<Self> {
        let fqtn = field.fqtn().clone();
        let syntax = field.parent().file().try_syntax()?;
        Self::try_new(field.proto().r#type(), fqtn.as_ref(), syntax)
    }
    fn from_oneof_field<'a>(
        field: &'a re::OneofField<'a>,
        _resolver: &'a DescriptorResolver,
    ) -> Result<Self> {
        let fqtn = field.fqtn().clone();
        let syntax = field.parent().parent().file().try_syntax()?;
        Self::try_new(field.proto().r#type(), fqtn.as_ref(), syntax)
    }
    fn try_new(
        r#type: google::protobuf::field_descriptor_proto::Type,
        fqtn: Option<&Fqtn<String>>,
        syntax: Syntax,
    ) -> Result<Self> {
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

    pub fn into_opt_getter_rust_type(&self) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        use WireType::*;
        let inner_type = match self {
            Variant(var) => var.into_owned_rust_type(),
            LengthDelimited(String) => "&str".into(),
            LengthDelimited(Bytes) => "&[u8]".into(),
            LengthDelimited(Message(fqtn)) => format!("&{}", fqtn.to_rust_path()).into(),
            Bits32(x) => x.into_owned_rust_type(),
            Bits64(x) => x.into_owned_rust_type(),
        };
        format!("::std::option::Option<{}>", inner_type).into()
    }

    pub fn into_mut_getter_rust_type(&self, is_repeated: bool) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        use WireType::*;
        let target_scalar_type = match self {
            Variant(var) => var.into_owned_rust_type(),
            LengthDelimited(String) => "::std::string::String".into(),
            LengthDelimited(Bytes) => "::std::vec::Vec<u8>".into(),
            LengthDelimited(Message(fqtn)) => fqtn.to_rust_path().into(),
            Bits32(x) => x.into_owned_rust_type(),
            Bits64(x) => x.into_owned_rust_type(),
        };
        if is_repeated {
            format!("&mut ::std::vec::Vec<{}>", target_scalar_type).into()
        } else {
            format!("&mut {}", target_scalar_type).into()
        }
    }

    pub fn into_oneof_getter_rust_type(&self, lt: &str) -> Cow<'static, str> {
        use LengthDelimitedType::*;
        use WireType::*;
        match self {
            Variant(var) => var.into_owned_rust_type(),
            LengthDelimited(String) => format!("&{} str", lt).into(),
            LengthDelimited(Bytes) => format!("&{} [u8]", lt).into(),
            LengthDelimited(Message(fqtn)) => format!("&{} {}", lt, fqtn.to_rust_path()).into(),
            Bits32(x) => x.into_owned_rust_type(),
            Bits64(x) => x.into_owned_rust_type(),
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
