use crate::generators::shared::context::Context;
use crate::generators::shared::utils::{
    get_keyword_safe_ident, snake_case_to_camel_case, FullyQualifiedTypeName,
    MaybeFullyQualifiedTypeName, PackagePath,
};
use crate::protos::field_descriptor_proto::Label;
use crate::protos::FieldDescriptorProto;
use ::once_cell::unsync::OnceCell;

enum FieldType {
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
    Enum(MaybeFullyQualifiedTypeName),
    /*super::message::MessageDescriptor<'p, 'c>*/
    Message(),
}

struct FieldDescriptor<'p, 'c> {
    proto: &'p FieldDescriptorProto,
    context: &'c Context,
}
impl<'p, 'c> FieldDescriptor<'p, 'c> {
    fn new(proto: &'p FieldDescriptorProto, context: &'c Context) -> Self {
        Self { proto, context }
    }
    fn name(&self) -> &str {
        &self.proto.name
    }
}
