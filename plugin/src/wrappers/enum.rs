use std::fmt::Debug;
use std::hash::Hash;

use super::FileOrMessageRef;
use crate::google::protobuf::{EnumDescriptorProto, EnumValueDescriptorProto};
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case};
use crate::Context;
use ::once_cell::unsync::OnceCell;

#[derive(Clone)]
pub struct EnumDescriptor<'c> {
    proto: &'c EnumDescriptorProto,
    context: &'c Context<'c>,
    parent: FileOrMessageRef<'c>,

    values: Vec<EnumValueDescriptor<'c>>,

    lazy_package: OnceCell<String>,
    lazy_fq_name: OnceCell<String>,
    lazy_native_bare_type_name: OnceCell<String>,
    lazy_native_type_name_from_root: OnceCell<String>,
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
            lazy_native_bare_type_name: Default::default(),
            lazy_native_type_name_from_root: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn package(&'c self) -> &str {
        self.lazy_package
            .get_or_init(|| self.parent.package_for_child())
    }
    pub fn fully_qualified_name(&'c self) -> &str {
        self.lazy_fq_name.get_or_init(|| {
            format!(
                "{package}.{name}",
                package = self.package(),
                name = self.name()
            )
        })
    }
    pub fn values(&self) -> impl Iterator<Item = &EnumValueDescriptor<'c>> {
        self.values.iter()
    }

    /// Returns a Rust typename which can be used for enum definition:
    /// ```
    /// pub enum HERE {
    ///     //...
    /// }
    /// ```
    /// without mod path, without wrapped by Result<>,
    /// without distinguishing between repeated / optional labels.
    pub fn native_bare_type_name(&self) -> &str {
        self.lazy_native_bare_type_name
            .get_or_init(|| get_keyword_safe_ident(&to_camel_case(self.name())))
    }

    /// Returns a Rust typename qualified with a mod path from an arbitral mod location,
    /// without wrapped by Result<>, without distinguishing between repeated / optional labels.
    pub fn native_fully_qualified_type_name(&'c self, path_to_root_mod: &str) -> String {
        let native_type_name_from_root = self.lazy_native_type_name_from_root.get_or_init(|| {
            let mod_path = itertools::Itertools::intersperse(
                self.package()
                    .split('.')
                    .map(|p| get_keyword_safe_ident(&to_lower_snake_case(p))),
                "::".to_string(),
            )
            .collect::<String>();
            format!(
                "{mod_path}::{bare_type}",
                mod_path = mod_path,
                bare_type = self.native_bare_type_name()
            )
        });
        format!(
            "{path_to_root_mod}::{type_name}",
            path_to_root_mod = path_to_root_mod,
            type_name = native_type_name_from_root
        )
    }
}

impl Hash for EnumDescriptor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.package_for_child().hash(state);
        self.proto.name.hash(state);
    }
}

#[derive(Clone)]
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
            .get_or_init(|| get_keyword_safe_ident(&to_camel_case(self.name())))
    }
}

impl Debug for EnumDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumDescriptor").finish()
    }
}
impl Debug for EnumValueDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumValueDescriptor").finish()
    }
}
