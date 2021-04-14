use std::borrow::Cow;
use std::fmt::Debug;

use crate::google::protobuf::field_descriptor_proto::Label;
use crate::google::protobuf::FieldDescriptorProto;
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case};
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
    pub fn is_repeated(&self) -> Result<bool> {
        Ok(matches!(self.label()?, FieldLabel::Repeated))
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
            FieldType::Group => Err(ErrorKind::Proto2NotSupported)?,
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

    // Returns type name, which will suit for struct definition.
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
                                "::std::option::Optional<::std::boxed::Box<{name}>>",
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

fn iter_package_to_root(package: &str) -> impl Iterator<Item = &str> {
    std::iter::successors(Some(package), |package| {
        if package.is_empty() {
            None
        } else if let Some((remain, _)) = package.rsplit_once('.') {
            Some(remain)
        } else {
            Some("")
        }
    })
}

#[derive(Debug, Clone)]
pub enum WireType<'c> {
    Variant(VariantFieldType<'c>),
    LengthDelimited(LengthDelimitedFieldType<'c>),
    Bits32(Bits32FieldType),
    Bits64(Bits64FieldType),
    //Group(GroupFieldType),
}

#[derive(Debug, Clone)]
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
            VariantFieldType::Int32 => "::puroro::tags::Int32".into(),
            VariantFieldType::Int64 => "::puroro::tags::Int64".into(),
            VariantFieldType::UInt32 => "::puroro::tags::UInt32".into(),
            VariantFieldType::UInt64 => "::puroro::tags::UInt64".into(),
            VariantFieldType::SInt32 => "::puroro::tags::SInt32".into(),
            VariantFieldType::SInt64 => "::puroro::tags::UInt64".into(),
            VariantFieldType::Bool => "::puroro::tags::Bool".into(),
            VariantFieldType::Enum(e) => format!(
                "::puroro::tags::Enum<{name}>",
                name = e.native_fully_qualified_type_name(path_to_root_mod)
            )
            .into(),
        }
    }
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum FieldLabel {
    Optional,
    Required,
    Repeated,
}
