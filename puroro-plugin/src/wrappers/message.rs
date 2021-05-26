use std::fmt::Debug;
use std::hash::Hash;

use super::{EnumDescriptor, FieldDescriptor, FieldType, FileDescriptor, FileOrMessageRef};
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
            // TODO: Maybe need to check the [default="***"] value?
            options.map_entry.unwrap_or_default()/* TODO */ && false
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

    pub fn unique_msgs_from_fields(
        &'c self,
    ) -> Result<impl Iterator<Item = &'c MessageDescriptor<'c>>> {
        // Not only the fields directly owned by this message,
        // we need to cehck the fields of the map field's keys and value fields.
        let maps_fields = self
            .fields()
            .filter_map(|field| match field.type_() {
                Ok(FieldType::Message(m)) if m.is_map_entry() => Some(m),
                _ => None,
            })
            .flat_map(|msg| {
                match msg.key_value_of_map_entry() {
                    Ok((key_field, value_field)) => {
                        Some(std::array::IntoIter::new([key_field, value_field]))
                    }
                    _ => None,
                }
                .into_iter()
                .flatten()
            });

        Ok(self
            .fields()
            .chain(maps_fields)
            .filter_map(|field| {
                if let Ok(FieldType::Message(m)) = field.type_() {
                    Some(m)
                } else {
                    None
                }
            })
            .unique_by(|msg| msg.fully_qualified_name().unwrap_or_default()))
    }

    pub fn key_value_of_map_entry(
        &'c self,
    ) -> Result<(&'c FieldDescriptor<'c>, &'c FieldDescriptor<'c>)> {
        debug_assert!(self.is_map_entry());
        let key_field =
            self.fields()
                .find(|field| field.number() == 1)
                .ok_or(ErrorKind::InvalidMapEntry {
                    name: self.fully_qualified_name()?.to_string(),
                })?;
        let value_field =
            self.fields()
                .find(|field| field.number() == 2)
                .ok_or(ErrorKind::InvalidMapEntry {
                    name: self.fully_qualified_name()?.to_string(),
                })?;
        Ok((key_field, value_field))
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
