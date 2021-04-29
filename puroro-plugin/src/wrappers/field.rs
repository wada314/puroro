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
    pub fn name(&self) -> Result<&str> {
        Ok(self.proto.name.as_ref().ok_or(ErrorKind::InternalError {
            detail: "Empty field name".to_string(),
        })?)
    }
    pub fn number(&self) -> i32 {
        // TODO: maybe check the default value?
        self.proto.number.unwrap_or_default()
    }
    pub fn label(&'c self) -> Result<FieldLabel> {
        match self.proto.label {
            Some(Ok(Label::LabelOptional)) => match self.message().file().syntax()? {
                super::ProtoSyntax::Proto2 => Ok(FieldLabel::Optional2),
                super::ProtoSyntax::Proto3 => {
                    if self.proto.proto3_optional.unwrap_or_default() {
                        Ok(FieldLabel::Optional2)
                    } else {
                        Ok(FieldLabel::Optional3)
                    }
                }
            },
            Some(Ok(Label::LabelRepeated)) => Ok(FieldLabel::Repeated),
            Some(Ok(Label::LabelRequired)) => Ok(FieldLabel::Required),
            Some(Err(id)) => Err(ErrorKind::UnknownLabelId { id })?,
            None => Err(ErrorKind::UnknownLabelId { id: 0 })?,
        }
    }

    pub fn type_(&'c self) -> Result<FieldType<'c>> {
        Ok(match &self.proto.type_ {
            Some(Ok(type_)) => {
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
                                    self.parent.fully_qualified_name()?,
                                    self.name()?,
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
                                self.parent.fully_qualified_name()?,
                                self.name()?,
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
            Some(Err(0)) | None => match self
                .context
                .fq_name_to_desc(self.fully_qualified_type_name()?)?
            {
                Some(EnumOrMessageRef::Enum(e)) => FieldType::Enum(e),
                Some(EnumOrMessageRef::Message(m)) => FieldType::Message(m),
                _ => Err(ErrorKind::UnknownTypeName {
                    name: self.proto.type_name.clone().unwrap_or("".to_string()),
                })?,
            },
            Some(Err(id)) => Err(ErrorKind::UnknownFieldTypeId { id: *id })?,
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

    pub fn type_tag(&'c self) -> Result<String> {
        Ok(match self.type_()? {
            FieldType::Double => "Double".into(),
            FieldType::Float => "Float".into(),
            FieldType::Int32 => "Int32".into(),
            FieldType::Int64 => "Int64".into(),
            FieldType::UInt32 => "UInt32".into(),
            FieldType::UInt64 => "UInt64".into(),
            FieldType::SInt32 => "SInt32".into(),
            FieldType::SInt64 => "SInt64".into(),
            FieldType::Fixed32 => "Fixed32".into(),
            FieldType::Fixed64 => "Fixed64".into(),
            FieldType::SFixed32 => "SFixed32".into(),
            FieldType::SFixed64 => "SFixed64".into(),
            FieldType::Bool => "Bool".into(),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "String".into(),
            FieldType::Bytes => "Bytes".into(),
            FieldType::Enum(e) => format!(
                "Enum<{}>",
                e.native_fully_qualified_type_name(self.path_to_root_mod()?)?
            ),
            FieldType::Message(m) => format!(
                "Message<{}>",
                // TODO: Wrong! Need a type name depending on the implementation detail.
                // e.g. super::DescriptorProtoBumpalo<'bump>
                m.native_fully_qualified_type_name(self.path_to_root_mod()?)?
            ),
        })
    }

    pub fn label_tag(&'c self) -> Result<&'static str> {
        Ok(match self.label()? {
            FieldLabel::Optional2 => "Optional2",
            FieldLabel::Optional3 => "Optional3",
            FieldLabel::Required => "Required",
            FieldLabel::Repeated => "Repeated",
        })
    }

    pub fn package(&'c self) -> Result<&str> {
        self.parent.package()
    }

    pub fn message(&'c self) -> &'c MessageDescriptor<'c> {
        self.parent
    }

    pub fn path_to_root_mod(&'c self) -> Result<&str> {
        self.parent.path_to_root_mod()
    }

    pub fn fully_qualified_type_name(&'c self) -> Result<&str> {
        let type_name_in_proto = match self.proto.type_name.as_deref() {
            None | Some("") => Err(ErrorKind::InternalError {
                detail: "Empty field type name".to_string(),
            })?,
            Some(s) => s,
        };
        Ok(self.lazy_fq_type_name.get_or_try_init(|| -> Result<_> {
            // If the type name starts with ".", then just return the remaining part.
            if let Some(fq_name) = type_name_in_proto.strip_prefix('.') {
                return Ok(fq_name.to_string());
            }
            // If the type name is not fully-qualified, search the known types
            // with climbing up the package to the root package.
            for package in self::iter_package_to_root(self.parent.package()?) {
                let fq_name_candidate = package.to_string() + "." + type_name_in_proto;
                if let Some(_) = self.context.fq_name_to_desc(&fq_name_candidate)? {
                    return Ok(fq_name_candidate);
                }
            }
            Err(ErrorKind::InternalError {
                detail: format!(
                    "The type \"{}.{}\" was not found in the Context's type list. \
                    Maybe it is not an enum or a message?",
                    self.parent.package()?,
                    type_name_in_proto,
                ),
            })?
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
                    name = e.native_type_name_with_relative_path(self.package()?)?
                )
                .into(),
                NonTrivialFieldType::Message(m) => format!(
                    "&{lt} {name}",
                    lt = lifetime,
                    name = m.native_type_name_with_relative_path(self.package()?)?
                )
                .into(),
            },
        })
    }

    pub fn native_name(&'c self) -> Result<&str> {
        Ok(self.lazy_native_name.get_or_try_init(|| -> Result<_> {
            Ok(get_keyword_safe_ident(&to_lower_snake_case(self.name()?)))
        })?)
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
    pub fn native_tag_type(&self, path_to_root_mod: &str) -> Result<Cow<'static, str>> {
        Ok(match self {
            VariantFieldType::Int32 => "::puroro_internal::tags::Int32".into(),
            VariantFieldType::Int64 => "::puroro_internal::tags::Int64".into(),
            VariantFieldType::UInt32 => "::puroro_internal::tags::UInt32".into(),
            VariantFieldType::UInt64 => "::puroro_internal::tags::UInt64".into(),
            VariantFieldType::SInt32 => "::puroro_internal::tags::SInt32".into(),
            VariantFieldType::SInt64 => "::puroro_internal::tags::UInt64".into(),
            VariantFieldType::Bool => "::puroro_internal::tags::Bool".into(),
            VariantFieldType::Enum(e) => format!(
                "::puroro_internal::tags::Enum<{name}>",
                name = e.native_fully_qualified_type_name(path_to_root_mod)?
            )
            .into(),
        })
    }
}

#[derive(Debug, Clone, Hash)]
pub enum LengthDelimitedFieldType<'c> {
    String,
    Bytes,
    Message(&'c super::MessageDescriptor<'c>),
}
impl<'c> LengthDelimitedFieldType<'c> {}

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
    Optional2,
    Optional3,
    Required,
    Repeated,
}
