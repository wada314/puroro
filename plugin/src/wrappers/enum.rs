use crate::generators::shared::context::Context;
use crate::generators::shared::utils::FullyQualifiedTypeName;
use crate::protos::EnumDescriptorProto;

pub struct EnumDescriptor {
    proto: EnumDescriptorProto,
}
impl EnumDescriptor {
    pub fn new(proto: EnumDescriptorProto) -> Self {
        Self { proto }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn fq_name(&self) -> &FullyQualifiedTypeName {
        todo!()
    }
}
