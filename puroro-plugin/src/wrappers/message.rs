use std::fmt::Debug;
use std::hash::Hash;

use super::{
    EnumDescriptor, FieldDescriptor, FieldLabel, FieldType, FileDescriptor, FileOrMessageRef,
};
use crate::google::protobuf::DescriptorProto;
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case};
use crate::{Context, ErrorKind, Result};
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;

#[derive(Clone)]
pub struct MessageDescriptor<'c> {
    proto: &'c DescriptorProto,
    context: &'c Context<'c>,
    parent: FileOrMessageRef<'c>,

    lazy_fields: OnceCell<Vec<FieldDescriptor<'c>>>,
    lazy_nested_messages: OnceCell<Vec<MessageDescriptor<'c>>>,
    lazy_enums: OnceCell<Vec<EnumDescriptor<'c>>>,

    lazy_package: OnceCell<String>,
    lazy_path_to_root_mod: OnceCell<String>,
    lazy_fq_name: OnceCell<String>,
    lazy_native_bare_type_name: OnceCell<String>,
    lazy_native_type_name_from_root: OnceCell<String>,
}
impl<'c> MessageDescriptor<'c> {
    pub fn new(
        proto: &'c DescriptorProto,
        context: &'c Context<'c>,
        parent: FileOrMessageRef<'c>,
    ) -> Self {
        Self {
            proto,
            context,
            parent,
            lazy_fields: Default::default(),
            lazy_nested_messages: Default::default(),
            lazy_enums: Default::default(),
            lazy_package: Default::default(),
            lazy_path_to_root_mod: Default::default(),
            lazy_fq_name: Default::default(),
            lazy_native_bare_type_name: Default::default(),
            lazy_native_type_name_from_root: Default::default(),
        }
    }
    pub fn fields(&'c self) -> impl Iterator<Item = &FieldDescriptor<'c>> {
        self.lazy_fields
            .get_or_init(|| {
                self.proto
                    .field
                    .iter()
                    .map(|f| FieldDescriptor::new(f, self.context, self))
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
                    .map(|m| {
                        MessageDescriptor::new(m, self.context, FileOrMessageRef::Message(self))
                    })
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
                    .map(|e| EnumDescriptor::new(e, self.context, FileOrMessageRef::Message(self)))
                    .collect()
            })
            .iter()
    }

    pub fn name(&self) -> Result<&str> {
        Ok(self.proto.name.as_ref().ok_or(ErrorKind::InternalError {
            detail: "Empty message name".to_string(),
        })?)
    }
    pub fn parent(&'c self) -> &'c FileOrMessageRef<'c> {
        &self.parent
    }
    pub fn file(&'c self) -> &'c FileDescriptor<'c> {
        self.parent.file_descriptor()
    }
    pub fn is_map_entry(&self) -> bool {
        if let Some(options) = &self.proto.options {
            // TODO: Maybe need to check the default value?
            options.map_entry.unwrap_or_default()
        } else {
            false
        }
    }
    pub fn package(&'c self) -> Result<&str> {
        Ok(self
            .lazy_package
            .get_or_try_init(|| -> Result<_> { self.parent.package_for_child() })?)
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

    /// Returns a Rust identifier which can be used for struct definition:
    /// ```
    /// pub struct HERE {
    ///     //...
    /// }
    /// ```
    /// Returns a Rust identifier without mod path,
    /// without distinguishing between repeated / optional labels.
    pub fn native_ident(&self) -> Result<&str> {
        Ok(self
            .lazy_native_bare_type_name
            .get_or_try_init(|| -> Result<_> {
                Ok(get_keyword_safe_ident(&to_camel_case(self.name()?)))
            })?)
    }

    pub fn native_ident_with_relative_path(&'c self, cur_package: &str) -> Result<String> {
        let struct_name = self.native_ident()?;
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
            "self::{supers}{mods}{name}",
            name = struct_name,
            supers = std::iter::repeat("super::")
                .take(cur_package_iter.count())
                .collect::<String>(),
            mods = struct_package_iter
                .map(|s| get_keyword_safe_ident(&to_lower_snake_case(s)) + "::")
                .collect::<String>(),
        ))
    }

    pub fn unique_msgs_from_fields(
        &'c self,
    ) -> Result<impl Iterator<Item = &'c MessageDescriptor<'c>>> {
        Ok(self
            .fields()
            .filter_map(|field| {
                if let Ok(FieldType::Message(m)) = field.type_() {
                    Some(m)
                } else {
                    None
                }
            })
            .unique_by(|msg| msg.fully_qualified_name().unwrap_or_default()))
    }
}

impl Debug for MessageDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageDescriptor").finish()
    }
}

impl Hash for MessageDescriptor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent
            .package_for_child()
            .unwrap_or_default()
            .hash(state);
        self.proto.name.hash(state);
    }
}
