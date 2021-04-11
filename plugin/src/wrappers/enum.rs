use crate::generators::shared::context::Context;
use crate::generators::shared::utils::{
    get_keyword_safe_ident, snake_case_to_camel_case, FullyQualifiedTypeName, PackagePath,
};
use crate::protos::{EnumDescriptorProto, EnumValueDescriptorProto};
use ::once_cell::unsync::OnceCell;

pub struct EnumDescriptor<'p, 'c> {
    proto: &'p EnumDescriptorProto,
    context: &'c Context,
    package: PackagePath,

    lazy_fq_name: OnceCell<FullyQualifiedTypeName>,
    lazy_native_bare_typename: OnceCell<String>,
    lazy_values: OnceCell<Vec<EnumValueDescriptor<'p>>>,
}
impl<'p, 'c> EnumDescriptor<'p, 'c> {
    pub fn new(proto: &'p EnumDescriptorProto, context: &'c Context, package: PackagePath) -> Self {
        Self {
            proto,
            context,
            package,
            lazy_fq_name: Default::default(),
            lazy_native_bare_typename: Default::default(),
            lazy_values: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn package(&self) -> &PackagePath {
        &self.package
    }
    pub fn fq_name(&self) -> &FullyQualifiedTypeName {
        self.lazy_fq_name
            .get_or_init(|| FullyQualifiedTypeName::new(self.package().clone(), self.name()))
    }
    pub fn values(&self) -> impl Iterator<Item = &EnumValueDescriptor<'p>> {
        self.lazy_values
            .get_or_init(|| {
                self.proto
                    .value
                    .iter()
                    .map(|value| EnumValueDescriptor::new(value))
                    .collect::<Vec<_>>()
            })
            .iter()
    }

    /// Returns a Rust typename without mod path, without wrapped by Result<>,
    /// without distinguishing between repeated / optional labels.
    pub fn native_bare_typename(&self) -> &str {
        self.lazy_native_bare_typename
            .get_or_init(|| get_keyword_safe_ident(&snake_case_to_camel_case(self.name())))
    }
}

pub struct EnumValueDescriptor<'p> {
    proto: &'p EnumValueDescriptorProto,
    lazy_native_name: OnceCell<String>,
}
impl<'p> EnumValueDescriptor<'p> {
    pub fn new(proto: &'p EnumValueDescriptorProto) -> Self {
        Self {
            proto,
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
