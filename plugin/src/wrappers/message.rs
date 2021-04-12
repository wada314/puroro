use super::{r#enum, EnumDescriptor, FieldDescriptor, FileDescriptor};
use crate::google::protobuf::DescriptorProto;
use crate::Context;
use ::once_cell::unsync::OnceCell;

pub enum Parent<'c> {
    File(&'c FileDescriptor<'c>),
    Message(&'c MessageDescriptor<'c>),
}

pub struct MessageDescriptor<'c> {
    proto: &'c DescriptorProto,
    context: &'c Context<'c>,
    parent: Parent<'c>,

    lazy_fields: OnceCell<Vec<FieldDescriptor<'c>>>,
    lazy_nested_messages: OnceCell<Vec<MessageDescriptor<'c>>>,
    lazy_enums: OnceCell<Vec<EnumDescriptor<'c>>>,
}
impl<'c> MessageDescriptor<'c> {
    pub fn new(proto: &'c DescriptorProto, context: &'c Context<'c>, parent: Parent<'c>) -> Self {
        Self {
            proto,
            context,
            parent,
            lazy_fields: Default::default(),
            lazy_nested_messages: Default::default(),
            lazy_enums: Default::default(),
        }
    }
    pub fn fields(&'c self) -> impl Iterator<Item = &FieldDescriptor<'c>> {
        self.lazy_fields
            .get_or_init(|| {
                self.proto
                    .field
                    .iter()
                    .map(|f| FieldDescriptor::new(f, self.context))
                    .collect()
            })
            .iter()
    }
    pub fn nested_messages(&'c self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.lazy_nested_messages
            .get_or_init(|| {
                self.proto
                    .nested_type
                    .iter()
                    .map(|m| MessageDescriptor::new(m, self.context, Parent::Message(self)))
                    .collect()
            })
            .iter()
    }
    pub fn enums(&'c self) -> impl Iterator<Item = &EnumDescriptor<'c>> {
        self.lazy_enums
            .get_or_init(|| {
                self.proto
                    .enum_type
                    .iter()
                    .map(|e| EnumDescriptor::new(e, self.context, r#enum::Parent::Message(self)))
                    .collect()
            })
            .iter()
    }

    pub fn name(&self) -> &str {
        &self.proto.name
    }
}
