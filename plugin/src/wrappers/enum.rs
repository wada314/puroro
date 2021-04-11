use crate::generators::shared::context::Context;
use crate::generators::shared::utils::{
    get_keyword_safe_ident, snake_case_to_camel_case, FullyQualifiedTypeName, PackagePath,
};
use crate::protos::{EnumDescriptorProto, EnumValueDescriptorProto};
use ::once_cell::unsync::OnceCell;

pub struct EnumDescriptor<'c> {
    proto: &'c EnumDescriptorProto,
    context: &'c Context<'c>,
    values: Vec<EnumValueDescriptor<'c>>,

    lazy_fq_name: OnceCell<FullyQualifiedTypeName>,
    lazy_native_bare_typename: OnceCell<String>,
}
impl<'c> EnumDescriptor<'c> {
    pub fn new(proto: &'c EnumDescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,
            values: proto
                .value
                .iter()
                .map(|v| EnumValueDescriptor::new(v, context))
                .collect(),
            lazy_fq_name: Default::default(),
            lazy_native_bare_typename: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn fq_name(&self) -> &FullyQualifiedTypeName {
        todo!()
        //self.lazy_fq_name
        //    .get_or_init(|| FullyQualifiedTypeName::new(self.package().clone(), self.name()))
    }
    pub fn values(&self) -> impl Iterator<Item = &EnumValueDescriptor<'c>> {
        self.values.iter()
    }

    /// Returns a Rust typename without mod path, without wrapped by Result<>,
    /// without distinguishing between repeated / optional labels.
    pub fn native_bare_typename(&self) -> &str {
        self.lazy_native_bare_typename
            .get_or_init(|| get_keyword_safe_ident(&snake_case_to_camel_case(self.name())))
    }
}

pub struct EnumValueDescriptor<'c> {
    proto: &'c EnumValueDescriptorProto,
    context: &'c Context<'c>,
    lazy_native_name: OnceCell<String>,
}
impl<'c> EnumValueDescriptor<'c> {
    pub fn new(proto: &'c EnumValueDescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,
            lazy_native_name: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn number(&self) -> i32 {
        self.proto.number
    }
    pub fn native_name(&self) -> &str {
        self.lazy_native_name
            .get_or_init(|| get_keyword_safe_ident(&snake_case_to_camel_case(self.name())))
    }
}
