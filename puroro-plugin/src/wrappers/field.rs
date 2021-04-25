use std::borrow::Cow;
use std::fmt::Debug;

use crate::google::protobuf::field_descriptor_proto::Label;
use crate::google::protobuf::FieldDescriptorProto;
use crate::once_map::OnceCellMap2;
use crate::utils::{get_keyword_safe_ident, iter_package_to_root, to_lower_snake_case};
use crate::Context;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

use super::{EnumOrMessageRef, MessageDescriptor};

#[derive(Clone)]
pub struct FieldDescriptor<'c> {
    proto: &'c FieldDescriptorProto,
    context: &'c Context<'c>,
    parent: &'c MessageDescriptor<'c>,

    lazy_type: OnceCell<FieldType<'c>>,
    lazy_fq_type_name: OnceCell<String>,
    lazy_native_owned_type_name: OnceCell<String>,
    lazy_native_scalar_ref_type_name: OnceCellMap2<&'static str, String>,
    lazy_native_scalar_mut_ref_type_name: OnceCellMap2<&'static str, String>,
    lazy_native_name: OnceCell<String>,
}
impl<'c> FieldDescriptor<'c> {
    pub fn new(
        proto: &'c FieldDescriptorProto,
        context: &'c Context<'c>,
        parent: &'c MessageDescriptor<'c>,
    ) -> Self {
        Self {
            proto,
            context,
            parent,
            lazy_type: Default::default(),
            lazy_fq_type_name: Default::default(),
            lazy_native_owned_type_name: Default::default(),
            lazy_native_scalar_ref_type_name: Default::default(),
            lazy_native_scalar_mut_ref_type_name: Default::default(),
            lazy_native_name: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn number(&self) -> i32 {
        self.proto.number
    }
    pub fn label(&self) -> Result<FieldLabel> {
        match self.proto.label {
            Ok(Label::LabelOptional) => Ok(FieldLabel::Optional),
            Ok(Label::LabelRepeated) => Ok(FieldLabel::Repeated),
            Ok(Label::LabelRequired) => Ok(FieldLabel::Required),
            Err(id) => Err(ErrorKind::UnknownLabelId { id })?,
        }
    }

    pub fn type_(&'c self) -> Result<FieldType<'c>> {
        Ok(match &self.proto.type_ {
            Ok(type_) => {
                use crate::protos::google::protobuf::field_descriptor_proto::Type;
                match type_ {
                    Type::TypeDouble => FieldType::Double,
                    Type::TypeFloat => FieldType::Float,
                    Type::TypeInt64 => FieldType::Int64,
                    Type::TypeUint64 => FieldType::UInt64,
                    Type::TypeInt32 => FieldType::Int32,
                    Type::TypeFixed64 => FieldType::Fixed64,
                    Type::TypeFixed32 => FieldType::Fixed32,
                    Type::TypeBool => FieldType::Bool,
                    Type::TypeString => FieldType::String,
                    Type::TypeGroup => FieldType::Group,
                    Type::TypeMessage => {
                        match self
                            .context
                            .fq_name_to_desc(self.fully_qualified_type_name()?)?
                        {
                            Some(EnumOrMessageRef::Message(m)) => FieldType::Message(m),
                            _ => Err(ErrorKind::InternalError {
                                detail: format!(
                                    "The field desc for {}.{} says its `type` is `TYPE_MESSAGE`, \
                                    but we couldn't find the message named \"{}\" in the inputs.",
                                    self.parent.fully_qualified_name(),
                                    self.name(),
                                    self.fully_qualified_type_name()?
                                ),
                            })?,
                        }
                    }
                    Type::TypeBytes => FieldType::Bytes,
                    Type::TypeUint32 => FieldType::UInt32,
                    Type::TypeEnum => match self
                        .context
                        .fq_name_to_desc(self.fully_qualified_type_name()?)?
                    {
                        Some(EnumOrMessageRef::Enum(e)) => FieldType::Enum(e),
                        _ => Err(ErrorKind::InternalError {
                            detail: format!(
                                "The field desc for {}.{} says its `type` is `TYPE_ENUM`, \
                                    but we couldn't find the enum named \"{}\" in the inputs.",
                                self.parent.fully_qualified_name(),
                                self.name(),
                                self.fully_qualified_type_name()?
                            ),
                        })?,
                    },
                    Type::TypeSfixed32 => FieldType::SFixed32,
                    Type::TypeSfixed64 => FieldType::SFixed64,
                    Type::TypeSint32 => FieldType::SInt32,
                    Type::TypeSint64 => FieldType::SInt64,
                }
            }
            Err(0) => match self
                .context
                .fq_name_to_desc(self.fully_qualified_type_name()?)?
            {
                Some(EnumOrMessageRef::Enum(e)) => FieldType::Enum(e),
                Some(EnumOrMessageRef::Message(m)) => FieldType::Message(m),
                _ => Err(ErrorKind::UnknownTypeName {
                    name: self.proto.type_name.clone(),
                })?,
            },
            Err(id) => Err(ErrorKind::UnknownFieldTypeId { id: *id })?,
        })
    }

    pub fn wire_type(&'c self) -> Result<WireType<'c>> {
        Ok(match self.type_()? {
            FieldType::Int32 => WireType::Variant(VariantFieldType::Int32),
            FieldType::Int64 => WireType::Variant(VariantFieldType::Int64),
            FieldType::UInt32 => WireType::Variant(VariantFieldType::UInt32),
            FieldType::UInt64 => WireType::Variant(VariantFieldType::UInt64),
            FieldType::SInt32 => WireType::Variant(VariantFieldType::SInt32),
            FieldType::SInt64 => WireType::Variant(VariantFieldType::SInt64),
            FieldType::Enum(e) => WireType::Variant(VariantFieldType::Enum(e)),
            FieldType::Bool => WireType::Variant(VariantFieldType::Bool),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => WireType::LengthDelimited(LengthDelimitedFieldType::String),
            FieldType::Bytes => WireType::LengthDelimited(LengthDelimitedFieldType::Bytes),
            FieldType::Message(m) => {
                WireType::LengthDelimited(LengthDelimitedFieldType::Message(m))
            }
            FieldType::Float => WireType::Bits32(Bits32FieldType::Float),
            FieldType::Fixed32 => WireType::Bits32(Bits32FieldType::Fixed32),
            FieldType::SFixed32 => WireType::Bits32(Bits32FieldType::SFixed32),
            FieldType::Double => WireType::Bits64(Bits64FieldType::Double),
            FieldType::Fixed64 => WireType::Bits64(Bits64FieldType::Fixed64),
            FieldType::SFixed64 => WireType::Bits64(Bits64FieldType::SFixed64),
        })
    }

    pub fn package(&'c self) -> &str {
        self.parent.package()
    }

    pub fn path_to_root_mod(&'c self) -> &str {
        self.parent.path_to_root_mod()
    }

    pub fn fully_qualified_type_name(&'c self) -> Result<&str> {
        Ok(self.lazy_fq_type_name.get_or_try_init(|| -> Result<_> {
            // If the type name starts with ".", then just return the remaining part.
            if let Some(fq_name) = self.proto.type_name.strip_prefix('.') {
                return Ok(fq_name.to_string());
            }
            // If the type name is not fully-qualified, search the known types
            // with climbing up the package to the root package.
            for package in self::iter_package_to_root(self.parent.package()) {
                let fq_name_candidate = package.to_string() + "." + &self.proto.type_name;
                if let Some(_) = self.context.fq_name_to_desc(&fq_name_candidate)? {
                    return Ok(fq_name_candidate);
                }
            }
            Err(ErrorKind::InternalError {
                detail: format!(
                    "The type \"{}.{}\" was not found in the Context's type list. \
                    Maybe it is not an enum or a message?",
                    self.parent.package(),
                    &self.proto.type_name
                ),
            })?
        })?)
    }

    // Returns type name, which will suit for the struct's field definition.
    pub fn native_owned_type_name(&'c self) -> Result<&str> {
        Ok(self
            .lazy_native_owned_type_name
            .get_or_try_init(|| -> Result<_> {
                // enum: Result<xxx, i32>
                // msg: xxx
                let native_fully_qualified_type: Cow<'static, str> = match self.wire_type()? {
                    WireType::Variant(field_type) => {
                        field_type.native_type(self.parent.path_to_root_mod())
                    }
                    WireType::LengthDelimited(field_type) => {
                        field_type.native_owned_type(self.parent.path_to_root_mod())
                    }
                    WireType::Bits32(field_type) => field_type.native_type().into(),
                    WireType::Bits64(field_type) => field_type.native_type().into(),
                };
                Ok(match self.label()? {
                    FieldLabel::Optional => {
                        if let FieldType::Message(_) = self.type_()? {
                            format!(
                                "::std::option::Option<::std::boxed::Box<{name}>>",
                                name = native_fully_qualified_type
                            )
                        } else {
                            native_fully_qualified_type.into_owned()
                        }
                    }
                    FieldLabel::Required => {
                        if let FieldType::Message(_) = self.type_()? {
                            format!(
                                "::std::boxed::Box<{name}>",
                                name = native_fully_qualified_type
                            )
                        } else {
                            native_fully_qualified_type.into_owned()
                        }
                    }
                    FieldLabel::Repeated => {
                        format!(
                            "::std::vec::Vec<{name}>",
                            name = native_fully_qualified_type
                        )
                    }
                })
            })?)
    }

    pub fn native_maybe_ref_type(&'c self, lifetime: &str) -> Result<Cow<'static, str>> {
        Ok(match self.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => format!("&{lt} str", lt = lifetime).into(),
                NonTrivialFieldType::Bytes => format!("&{lt} [u8]", lt = lifetime).into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{name}, i32>",
                    name = e.native_type_name_with_relative_path(self.package())
                )
                .into(),
                NonTrivialFieldType::Message(m) => format!(
                    "&{lt} {name}",
                    lt = lifetime,
                    name = m.native_type_name_with_relative_path(self.package())
                )
                .into(),
            },
        })
    }

    // Returns a ref type name for required field.
    pub fn native_scalar_ref_type_name(&'c self, lifetime: &'static str) -> Result<&str> {
        Ok(self
            .lazy_native_scalar_ref_type_name
            .get_or_try_init(lifetime, || -> Result<_> {
                Ok(match self.wire_type()? {
                    WireType::Variant(field_type) => field_type
                        .native_type(self.parent.path_to_root_mod())
                        .into_owned(),
                    WireType::LengthDelimited(field_type) => field_type
                        .native_ref_type(self.parent.path_to_root_mod(), lifetime)
                        .into_owned(),
                    WireType::Bits32(field_type) => field_type.native_type().to_string(),
                    WireType::Bits64(field_type) => field_type.native_type().to_string(),
                })
            })?)
    }

    // Returns a mutable ref type name for required field.
    pub fn native_scalar_mut_ref_type_name(&'c self, lifetime: &'static str) -> Result<&str> {
        Ok(self.lazy_native_scalar_mut_ref_type_name.get_or_try_init(
            lifetime,
            || -> Result<_> {
                Ok(match self.wire_type()? {
                    WireType::Variant(field_type) => format!(
                        "&mut {name}",
                        name = field_type.native_type(self.parent.path_to_root_mod())
                    ),
                    WireType::LengthDelimited(field_type) => field_type
                        .native_mut_ref_type(self.parent.path_to_root_mod())
                        .into_owned(),
                    WireType::Bits32(field_type) => {
                        format!("&mut {name}", name = field_type.native_type().to_string())
                    }
                    WireType::Bits64(field_type) => {
                        format!("&mut {name}", name = field_type.native_type().to_string())
                    }
                })
            },
        )?)
    }

    pub fn native_name(&'c self) -> &str {
        self.lazy_native_name
            .get_or_init(|| get_keyword_safe_ident(&to_lower_snake_case(self.name())))
    }
}

impl Debug for FieldDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldDescriptor").finish()
    }
}

#[derive(Debug, Clone, Hash)]
pub enum WireType<'c> {
    Variant(VariantFieldType<'c>),
    LengthDelimited(LengthDelimitedFieldType<'c>),
    Bits32(Bits32FieldType),
    Bits64(Bits64FieldType),
    //Group(GroupFieldType),
}

#[derive(Debug, Clone, Hash)]
pub enum VariantFieldType<'c> {
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Bool,
    Enum(&'c super::EnumDescriptor<'c>),
}
impl<'c> VariantFieldType<'c> {
    pub fn native_type(&self, path_to_root_mod: &str) -> Cow<'static, str> {
        match self {
            VariantFieldType::Int32 | VariantFieldType::SInt32 => "i32".into(),
            VariantFieldType::Int64 | VariantFieldType::SInt64 => "i64".into(),
            VariantFieldType::UInt32 => "u32".into(),
            VariantFieldType::UInt64 => "u64".into(),
            VariantFieldType::Bool => "bool".into(),
            VariantFieldType::Enum(e) => format!(
                "::std::result::Result<{name}, i32>",
                name = e.native_fully_qualified_type_name(path_to_root_mod)
            )
            .into(),
        }
    }
    pub fn native_tag_type(&self, path_to_root_mod: &str) -> Cow<'static, str> {
        match self {
            VariantFieldType::Int32 => "::puroro_internal::tags::Int32".into(),
            VariantFieldType::Int64 => "::puroro_internal::tags::Int64".into(),
            VariantFieldType::UInt32 => "::puroro_internal::tags::UInt32".into(),
            VariantFieldType::UInt64 => "::puroro_internal::tags::UInt64".into(),
            VariantFieldType::SInt32 => "::puroro_internal::tags::SInt32".into(),
            VariantFieldType::SInt64 => "::puroro_internal::tags::UInt64".into(),
            VariantFieldType::Bool => "::puroro_internal::tags::Bool".into(),
            VariantFieldType::Enum(e) => format!(
                "::puroro_internal::tags::Enum<{name}>",
                name = e.native_fully_qualified_type_name(path_to_root_mod)
            )
            .into(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum LengthDelimitedFieldType<'c> {
    String,
    Bytes,
    Message(&'c super::MessageDescriptor<'c>),
}
impl<'c> LengthDelimitedFieldType<'c> {
    pub fn native_owned_type(&self, path_to_root_mod: &str) -> Cow<'static, str> {
        match self {
            LengthDelimitedFieldType::String => "::std::string::String".into(),
            LengthDelimitedFieldType::Bytes => "::std::vec::Vec<u8>".into(),
            LengthDelimitedFieldType::Message(m) => {
                m.native_fully_qualified_type_name(path_to_root_mod).into()
            }
        }
    }
    pub fn native_ref_type(&self, path_to_root_mod: &str, lifetime: &str) -> Cow<'static, str> {
        let lt: Cow<'static, str> = if lifetime.is_empty() {
            "".into()
        } else {
            format!("{} ", lifetime).into()
        };
        match self {
            LengthDelimitedFieldType::String => format!("&{lt}str", lt = lt).into(),
            LengthDelimitedFieldType::Bytes => format!("&{lt}[u8]", lt = lt).into(),
            LengthDelimitedFieldType::Message(m) => format!(
                "&{lt}{name}",
                lt = lt,
                name = m.native_fully_qualified_type_name(path_to_root_mod)
            )
            .into(),
        }
    }
    pub fn native_mut_ref_type(&self, path_to_root_mod: &str) -> Cow<'static, str> {
        match self {
            LengthDelimitedFieldType::String => "&mut String".into(),
            LengthDelimitedFieldType::Bytes => "&mut Vec<u8>".into(),
            LengthDelimitedFieldType::Message(m) => format!(
                "&mut {name}",
                name = m.native_fully_qualified_type_name(path_to_root_mod)
            )
            .into(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Bits32FieldType {
    Float,
    Fixed32,
    SFixed32,
}
impl Bits32FieldType {
    pub fn native_type(&self) -> &'static str {
        match self {
            Bits32FieldType::Float => "f32",
            Bits32FieldType::Fixed32 => "u32",
            Bits32FieldType::SFixed32 => "i32",
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Bits64FieldType {
    Double,
    Fixed64,
    SFixed64,
}
impl Bits64FieldType {
    pub fn native_type(&self) -> &'static str {
        match self {
            Bits64FieldType::Double => "f64",
            Bits64FieldType::Fixed64 => "u64",
            Bits64FieldType::SFixed64 => "i64",
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum FieldType<'c> {
    Double,
    Float,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    Group,
    String,
    Bytes,
    Enum(&'c super::EnumDescriptor<'c>),
    Message(&'c super::MessageDescriptor<'c>),
}
impl<'c> FieldType<'c> {
    pub fn native_trivial_type_name(
        &self,
    ) -> std::result::Result<&'static str, NonTrivialFieldType<'c>> {
        match self {
            FieldType::Double => Ok("f64"),
            FieldType::Float => Ok("f32"),
            FieldType::Int32 | FieldType::SInt32 | FieldType::SFixed32 => Ok("i32"),
            FieldType::Int64 | FieldType::SInt64 | FieldType::SFixed64 => Ok("i64"),
            FieldType::UInt32 | FieldType::Fixed32 => Ok("u32"),
            FieldType::UInt64 | FieldType::Fixed64 => Ok("u64"),
            FieldType::Bool => Ok("bool"),
            FieldType::Group => Err(NonTrivialFieldType::Group),
            FieldType::String => Err(NonTrivialFieldType::String),
            FieldType::Bytes => Err(NonTrivialFieldType::Bytes),
            FieldType::Enum(e) => Err(NonTrivialFieldType::Enum(e)),
            FieldType::Message(m) => Err(NonTrivialFieldType::Message(m)),
        }
    }
}

pub enum NonTrivialFieldType<'c> {
    Group,
    String,
    Bytes,
    Enum(&'c super::EnumDescriptor<'c>),
    Message(&'c super::MessageDescriptor<'c>),
}

#[derive(Debug, Clone, Hash)]
pub enum FieldLabel {
    Optional,
    Required,
    Repeated,
}
