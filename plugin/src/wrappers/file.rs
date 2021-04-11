use crate::protos::FileDescriptorProto;
use crate::Context;

use super::EnumDescriptor;
use super::MessageDescriptor;

pub struct FileDescriptor<'c> {
    proto: &'c FileDescriptorProto,
    context: &'c Context<'c>,
    messages: Vec<MessageDescriptor<'c>>,
    enums: Vec<EnumDescriptor<'c>>,
}
impl<'c> FileDescriptor<'c> {
    pub fn new(proto: &'c FileDescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,
            messages: proto
                .message_type
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
    pub fn path_from_root(&self) -> &str {
        &self.proto.name
    }
    pub fn messages(&self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.messages.iter()
    }
    pub fn enums(&self) -> impl Iterator<Item = &EnumDescriptor<'c>> {
        self.enums.iter()
    }
}
