use std::fmt::Debug;
use std::hash::Hash;

use super::FileOrMessageRef;
use crate::google::protobuf::{EnumDescriptorProto, EnumValueDescriptorProto};
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case};
use crate::{Context, ErrorKind, Result};
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
    pub fn name(&self) -> Result<&str> {
        Ok(self.proto.name.as_ref().ok_or(ErrorKind::InternalError {
            detail: "Missing enum name".to_string(),
        })?)
    }
    pub fn package(&'c self) -> Result<&str> {
        Ok(self
            .lazy_package
            .get_or_try_init(|| -> Result<_> { Ok(self.parent.package_for_child()?) })?)
    }
    pub fn fully_qualified_name(&'c self) -> Result<&str> {
        Ok(self.lazy_fq_name.get_or_try_init(|| -> Result<_> {
            Ok(format!(
                "{package}.{name}",
                package = self.package()?,
                name = self.name()?
            ))
        })?)
    }
    pub fn values(&self) -> impl Iterator<Item = &EnumValueDescriptor<'c>> {
        self.values.iter()
    }

    /// Returns a Rust identifier which can be used for enum definition:
    /// ```
    /// pub enum HERE {
    ///     //...
    /// }
    /// ```
    /// without mod path, without wrapped by Result<>,
    /// without distinguishing between repeated / optional labels.
    pub fn native_ident(&self) -> Result<&str> {
        Ok(self
            .lazy_native_bare_type_name
            .get_or_try_init(|| -> Result<_> {
                Ok(get_keyword_safe_ident(&to_camel_case(self.name()?)))
            })?)
    }

    pub fn native_ident_with_relative_path(&'c self, cur_package: &str) -> Result<String> {
        let enum_name = self.native_ident()?;
        let mut struct_package_iter = self.package()?.split('.').peekable();
        let mut cur_package_iter = cur_package.split('.').peekable();
        while let (Some(p1), Some(p2)) = (struct_package_iter.peek(), cur_package_iter.peek()) {
            if *p1 == *p2 {
                struct_package_iter.next();
                cur_package_iter.next();
            } else {
                break;
            }
        }
        Ok(format!(
            "{supers}{mods}{name}",
            name = enum_name,
            supers = std::iter::repeat("super::")
                .take(cur_package_iter.count())
                .collect::<String>(),
            mods = struct_package_iter
                .map(|s| get_keyword_safe_ident(&to_lower_snake_case(s)) + "::")
                .collect::<String>(),
        ))
    }

    /// Returns a Rust typename qualified with a mod path from an arbitral mod location,
    /// without wrapped by Result<>, without distinguishing between repeated / optional labels.
    pub fn native_fully_qualified_ident(&'c self, path_to_root_mod: &str) -> Result<String> {
        let native_type_name_from_root =
            self.lazy_native_type_name_from_root
                .get_or_try_init(|| -> Result<_> {
                    let mod_path = itertools::Itertools::intersperse(
                        self.package()?
                            .split('.')
                            .map(|p| get_keyword_safe_ident(&to_lower_snake_case(p))),
                        "::".to_string(),
                    )
                    .collect::<String>();
                    Ok(format!(
                        "{mod_path}::{bare_type}",
                        mod_path = mod_path,
                        bare_type = self.native_ident()?
                    ))
                })?;
        Ok(format!(
            "{path_to_root_mod}::{type_name}",
            path_to_root_mod = path_to_root_mod,
            type_name = native_type_name_from_root
        ))
    }
}

impl Hash for EnumDescriptor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent
            .package_for_child()
            .unwrap_or_default()
            .hash(state);
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
    pub fn name(&self) -> Result<&str> {
        Ok(self.proto.name.as_ref().ok_or(ErrorKind::InternalError {
            detail: "Empty enum value name".to_string(),
        })?)
    }
    pub fn number(&self) -> i32 {
        // TODO: maybe need to check the default value?
        self.proto.number.unwrap_or_default()
    }
    pub fn native_name(&self) -> Result<&str> {
        Ok(self.lazy_native_name.get_or_try_init(|| -> Result<_> {
            Ok(get_keyword_safe_ident(&to_camel_case(self.name()?)))
        })?)
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
