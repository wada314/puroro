use super::FileOrMessageRef;
use crate::google::protobuf::{EnumDescriptorProto, EnumValueDescriptorProto};
use crate::utils::{get_keyword_safe_ident, snake_case_to_camel_case};
use crate::Context;
use ::once_cell::unsync::OnceCell;

pub struct EnumDescriptor<'c> {
    proto: &'c EnumDescriptorProto,
    context: &'c Context<'c>,
    parent: FileOrMessageRef<'c>,

    values: Vec<EnumValueDescriptor<'c>>,

    lazy_package: OnceCell<String>,
    lazy_fq_name: OnceCell<String>,
    lazy_native_bare_typename: OnceCell<String>,
}
impl<'c> EnumDescriptor<'c> {
    pub fn new(
        proto: &'c EnumDescriptorProto,
        context: &'c Context<'c>,
        parent: FileOrMessageRef<'c>,
    ) -> Self {
        Self {
            proto,
            context,
            parent,
            values: proto
                .value
                .iter()
                .map(|v| EnumValueDescriptor::new(v, context))
                .collect(),
            lazy_package: Default::default(),
            lazy_fq_name: Default::default(),
            lazy_native_bare_typename: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn package(&'c self) -> &str {
        self.lazy_package
            .get_or_init(|| self.parent.package_for_child() + "." + self.name())
    }
    pub fn fq_name(&'c self) -> &str {
        self.lazy_fq_name
            .get_or_init(|| self.name().to_string() + "." + self.package())
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
