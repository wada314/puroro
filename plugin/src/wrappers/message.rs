use crate::generators::shared::context::{Context, TypeOfIdent};
use crate::protos::DescriptorProto;

use super::{EnumDescriptor, FieldDescriptor};

pub struct MessageDescriptor<'c> {
    proto: &'c DescriptorProto,
    context: &'c Context<'c>,
    fields: Vec<FieldDescriptor<'c>>,
    nested_messages: Vec<MessageDescriptor<'c>>,
    enums: Vec<EnumDescriptor<'c>>,
}
impl<'c> MessageDescriptor<'c> {
    pub fn new(proto: &'c DescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,
            fields: proto
                .field
                .iter()
                .map(|f| FieldDescriptor::new(f, context))
                .collect(),
            nested_messages: proto
                .nested_type
                .iter()
                .map(|m| MessageDescriptor::new(m, context))
                .collect(),
            enums: proto
                .enum_type
                .iter()
                .map(|e| EnumDescriptor::new(e, context))
                .collect(),
        }
    }
    pub fn fields(&self) -> impl Iterator<Item = &FieldDescriptor<'c>> {
        self.fields.iter()
    }
    pub fn nested_messages(&self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.nested_messages.iter()
    }
    pub fn enums(&self) -> impl Iterator<Item = &EnumDescriptor<'c>> {
        self.enums.iter()
    }
}
