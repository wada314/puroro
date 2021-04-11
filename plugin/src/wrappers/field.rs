use crate::generators::shared::context::{Context, TypeOfIdent};
use crate::generators::shared::utils::{
    get_keyword_safe_ident, snake_case_to_camel_case, FullyQualifiedTypeName,
    MaybeFullyQualifiedTypeName, PackagePath,
};
use crate::protos::field_descriptor_proto::Label;
use crate::protos::FieldDescriptorProto;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

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
    Enum(&'c super::EnumDescriptor<'c>),
    //Message(super::MessageDescriptor<'c>),
}

pub enum FieldLabel {
    Optional,
    Required,
    Repeated,
}

pub struct FieldDescriptor<'c> {
    proto: &'c FieldDescriptorProto,
    context: &'c Context<'c>,
}
impl<'c> FieldDescriptor<'c> {
    pub fn new(proto: &'c FieldDescriptorProto, context: &'c Context<'c>) -> Self {
        Self { proto, context }
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
    pub fn r#type(&self) -> Result<FieldType<'c>> {
        todo!()
    }
}
