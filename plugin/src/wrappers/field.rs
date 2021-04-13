use std::borrow::Cow;

use crate::google::protobuf::field_descriptor_proto::Label;
use crate::google::protobuf::FieldDescriptorProto;
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case};
use crate::Context;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

use super::{EnumOrMessageRef, MessageDescriptor};

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
    pub fn native_type_for_numerical_types(
        &self,
    ) -> std::result::Result<&'static str, NonnumericalFieldType<'c>> {
        match self {
            FieldType::Double => Ok("f64"),
            FieldType::Float => Ok("f32"),
            FieldType::Int32 | FieldType::SInt32 | FieldType::SFixed32 => Ok("i32"),
            FieldType::Int64 | FieldType::SInt64 | FieldType::SFixed64 => Ok("i64"),
            FieldType::UInt32 | FieldType::Fixed32 => Ok("u32"),
            FieldType::UInt64 | FieldType::Fixed64 => Ok("u64"),
            FieldType::Bool => Ok("bool"),
            FieldType::Group => Err(NonnumericalFieldType::Group),
            FieldType::String => Err(NonnumericalFieldType::String),
            FieldType::Bytes => Err(NonnumericalFieldType::Bytes),
            FieldType::Enum(e) => Err(NonnumericalFieldType::Enum(e)),
            FieldType::Message(m) => Err(NonnumericalFieldType::Message(m)),
        }
    }
}
pub enum NonnumericalFieldType<'c> {
    Group,
    String,
    Bytes,
    Enum(&'c super::EnumDescriptor<'c>),
    Message(&'c super::MessageDescriptor<'c>),
}

pub enum FieldLabel {
    Optional,
    Required,
    Repeated,
}

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
    pub fn r#type(&'c self) -> Result<FieldType<'c>> {
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
                                    self.proto.type_name
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
                                "The field desc for {}::{} says its `type` is `TYPE_ENUM`, \
                                    but we couldn't find the enum named \"{}\" in the inputs.",
                                self.parent.fully_qualified_name(),
                                self.name(),
                                self.proto.type_name
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
                let native_bare_fully_qualified_type: Cow<'static, str> =
                    match self.r#type()?.native_type_for_numerical_types() {
                        Ok(s) => s.into(),
                        Err(t) => match t {
                            NonnumericalFieldType::Group => Err(ErrorKind::Proto2NotSupported)?,
                            NonnumericalFieldType::String => "::std::string::String".into(),
                            NonnumericalFieldType::Bytes => "::std::vec::Vec<u8>".into(),
                            NonnumericalFieldType::Enum(e) => e
                                .native_fully_qualified_typename(self.parent.path_to_root_mod())
                                .into(),
                            NonnumericalFieldType::Message(m) => m
                                .native_fully_qualified_typename(self.parent.path_to_root_mod())
                                .into(),
                        },
                    };
                Ok(match self.label()? {
                    FieldLabel::Optional => {
                        if let FieldType::Message(_) = self.r#type()? {
                            format!(
                                "::std::option::Optional<{name}>",
                                name = native_bare_fully_qualified_type
                            )
                        } else {
                            native_bare_fully_qualified_type.into_owned()
                        }
                    }
                    FieldLabel::Required => Err(ErrorKind::Proto2NotSupported)?,
                    FieldLabel::Repeated => {
                        format!(
                            "::std::vec::Vec<{name}",
                            name = native_bare_fully_qualified_type
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
