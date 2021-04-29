use std::fmt::Debug;
use std::hash::Hash;

use super::{EnumDescriptor, FieldDescriptor, FileDescriptor, FileOrMessageRef};
use crate::google::protobuf::DescriptorProto;
use crate::utils::{get_keyword_safe_ident, to_camel_case, to_lower_snake_case};
use crate::Context;
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

    pub fn name(&self) -> &str {
        &self.proto.name
    }
    pub fn parent(&'c self) -> &'c FileOrMessageRef<'c> {
        &self.parent
    }
    pub fn file(&'c self) -> &'c FileDescriptor<'c> {
        self.parent.file_descriptor()
    }
    pub fn is_map_entry(&self) -> bool {
        if let Some(options) = &self.proto.options {
            options.map_entry
        } else {
            false
        }
    }
    pub fn package(&'c self) -> &str {
        self.lazy_package
            .get_or_init(|| self.parent.package_for_child())
    }
    pub fn path_to_root_mod(&'c self) -> &str {
        self.lazy_path_to_root_mod.get_or_init(|| {
            let depth = self.package().split('.').count();
            if depth != 0 {
                itertools::Itertools::intersperse(std::iter::repeat("super").take(depth), "::")
                    .collect::<String>()
            } else {
                "self".to_string()
            }
        })
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

    /// Returns a Rust typename which can be used for struct definition:
    /// ```
    /// pub struct HERE {
    ///     //...
    /// }
    /// ```
    /// Returns a Rust typename without mod path,
    /// without distinguishing between repeated / optional labels.
    pub fn native_bare_type_name(&self) -> &str {
        self.lazy_native_bare_type_name
            .get_or_init(|| get_keyword_safe_ident(&to_camel_case(self.name())))
    }

    pub fn native_type_name_with_relative_path(&'c self, cur_package: &str) -> String {
        let struct_name = self.native_bare_type_name();
        let mut struct_package_iter = self.package().split('.').peekable();
        let mut cur_package_iter = cur_package.split('.').peekable();
        while let (Some(p1), Some(p2)) = (struct_package_iter.peek(), cur_package_iter.peek()) {
            if *p1 == *p2 {
                struct_package_iter.next();
                cur_package_iter.next();
            } else {
                break;
            }
        }
        format!(
            "{supers}{mods}{name}",
            name = struct_name,
            supers = std::iter::repeat("super::")
                .take(cur_package_iter.count())
                .collect::<String>(),
            mods = struct_package_iter
                .map(|s| get_keyword_safe_ident(&to_lower_snake_case(s)) + "::")
                .collect::<String>(),
        )
    }

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

impl Debug for MessageDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageDescriptor").finish()
    }
}

impl Hash for MessageDescriptor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.package_for_child().hash(state);
        self.proto.name.hash(state);
    }
}
