use std::borrow::Cow;
use std::fmt::Debug;

use crate::google::protobuf::FieldDescriptorProto;
use crate::protos::enums::google::protobuf::field_descriptor_proto::Label;
use crate::utils::{
    get_keyword_safe_ident, iter_package_to_root, relative_path_over_namespaces,
    to_lower_snake_case,
};
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
            Some(Label::LabelOptional) => match self.message().file().syntax()? {
                super::ProtoSyntax::Proto2 => Ok(FieldLabel::Optional2),
                super::ProtoSyntax::Proto3 => {
                    if self.proto.proto3_optional.unwrap_or_default() {
                        Ok(FieldLabel::Optional2)
                    } else {
                        Ok(FieldLabel::Optional3)
                    }
                }
            },
            Some(Label::LabelRepeated) => Ok(FieldLabel::Repeated),
            Some(Label::LabelRequired) => Ok(FieldLabel::Required),
            None => Err(ErrorKind::UnknownLabelId { id: 0 })?,
        }
    }

    pub fn type_(&'c self) -> Result<FieldType<'c>> {
        Ok(match &self.proto.type_ {
            Some(type_) => {
                use crate::protos::enums::google::protobuf::field_descriptor_proto::Type;
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
                        Some(EnumOrMessageRef::Enum(e)) => FieldType::Enum2(e),
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
            None => match self
                .context
                .fq_name_to_desc(self.fully_qualified_type_name()?)?
            {
                Some(EnumOrMessageRef::Enum(e)) => match self.message().file().syntax()? {
                    super::ProtoSyntax::Proto2 => FieldType::Enum2(e),
                    super::ProtoSyntax::Proto3 => FieldType::Enum3(e),
                },
                Some(EnumOrMessageRef::Message(m)) => FieldType::Message(m),
                _ => Err(ErrorKind::UnknownTypeName {
                    name: self.proto.type_name.clone().unwrap_or("".to_string()),
                })?,
            },
        })
    }

    pub fn label_tag_ident(&'c self) -> Result<&'static str> {
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

    pub fn native_ident(&'c self) -> Result<&str> {
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
    Enum2(&'c super::EnumDescriptor<'c>),
    Enum3(&'c super::EnumDescriptor<'c>),
    Message(&'c super::MessageDescriptor<'c>),
}
impl<'c> FieldType<'c> {
    pub fn native_numerical_type_name(
        &self,
        package: &str,
    ) -> Result<std::result::Result<Cow<'static, str>, NonNumericalFieldType<'c>>> {
        Ok(match self {
            FieldType::Double => Ok("f64".into()),
            FieldType::Float => Ok("f32".into()),
            FieldType::Int32 | FieldType::SInt32 | FieldType::SFixed32 => Ok("i32".into()),
            FieldType::Int64 | FieldType::SInt64 | FieldType::SFixed64 => Ok("i64".into()),
            FieldType::UInt32 | FieldType::Fixed32 => Ok("u32".into()),
            FieldType::UInt64 | FieldType::Fixed64 => Ok("u64".into()),
            FieldType::Bool => Ok("bool".into()),
            FieldType::Group => Err(NonNumericalFieldType::Group),
            FieldType::String => Err(NonNumericalFieldType::String),
            FieldType::Bytes => Err(NonNumericalFieldType::Bytes),
            FieldType::Enum2(e) => Ok(format!(
                "{module}::{ident}",
                module = relative_path_over_namespaces(e.package()?, "enums")?,
                ident = e.native_ident()?,
            )
            .into()),
            FieldType::Enum3(e) => Ok(format!(
                "::std::result::Result<{module}::{ident}, i32>",
                module = relative_path_over_namespaces(e.package()?, "enums")?,
                ident = e.native_ident()?,
            )
            .into()),
            FieldType::Message(m) => Err(NonNumericalFieldType::Message(m)),
        })
    }
}

pub enum NonNumericalFieldType<'c> {
    Group,
    String,
    Bytes,
    Message(&'c super::MessageDescriptor<'c>),
}

#[derive(Debug, Clone, Hash)]
pub enum FieldLabel {
    Optional2,
    Optional3,
    Required,
    Repeated,
}
