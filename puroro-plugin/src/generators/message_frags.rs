use std::borrow::Cow;

use itertools::Itertools;

use crate::context::{AllocatorType, Context, ImplType};
use crate::utils::{relative_path, relative_path_over_namespaces, GenericParams};
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, MessageDescriptor, NonNumericalFieldType,
};
use crate::{ErrorKind, Result};

pub struct MessageImplFragmentGenerator<'a, 'c> {
    context: &'a Context<'c>,
    msg: &'c MessageDescriptor<'c>,
}
impl<'a, 'c> MessageImplFragmentGenerator<'a, 'c> {
    pub fn new(context: &'a Context<'c>, msg: &'c MessageDescriptor<'c>) -> Self {
        Self { context, msg }
    }

    /// A type name of the struct with a relative path from the current msg.
    /// Includes generic param bounds if there is any.
    pub fn type_name_of_msg<'b, T>(
        &self,
        msg: &'c MessageDescriptor<'c>,
        bindings: T,
    ) -> Result<String>
    where
        T: IntoIterator<Item = &'b (&'static str, &'static str)>,
    {
        let generic_args_iter1 = match self.context.alloc_type() {
            AllocatorType::Default => None,
            AllocatorType::Bumpalo => Some("'bump"),
        }
        .into_iter();
        let generic_args_iter2 = match self.context.impl_type() {
            ImplType::Default => None,
            ImplType::SliceView => Some(std::array::IntoIter::new(["'slice", "S"])),
        }
        .into_iter()
        .flatten();
        // Replace bindings
        let mut generic_args_vec = generic_args_iter1
            .chain(generic_args_iter2)
            .collect::<Vec<_>>();
        for &(from, to) in bindings {
            for i in 0..generic_args_vec.len() {
                if generic_args_vec[i] == from {
                    generic_args_vec[i] = to;
                }
            }
        }
        let generic_args =
            Itertools::intersperse(generic_args_vec.into_iter(), ", ").collect::<String>();
        if generic_args.is_empty() {
            Ok(format!(
                "{module}::{ident}",
                module = relative_path(self.msg.package()?, msg.package()?)?,
                ident = msg.native_ident()?,
            ))
        } else {
            Ok(format!(
                "{module}::{ident}::<{gargs}>",
                module = relative_path(self.msg.package()?, msg.package()?)?,
                ident = msg.native_ident()?,
                gargs = generic_args,
            ))
        }
    }

    pub fn is_default_available(&self) -> bool {
        match (self.context.impl_type(), self.context.alloc_type()) {
            (_, AllocatorType::Bumpalo) => false,
            _ => true,
        }
    }

    pub fn is_merge_from_iter_available(&self) -> bool {
        match self.context.impl_type() {
            ImplType::Default => true,
            ImplType::SliceView => false,
        }
    }

    pub fn field_visibility(&self) -> &'static str {
        match self.context.impl_type() {
            ImplType::Default => "pub ",
            ImplType::SliceView => "",
        }
    }

    pub fn field_scalar_item_type(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<Cow<'static, str>> {
        Ok(match self.context.impl_type() {
            ImplType::Default => match field
                .type_()?
                .native_numerical_type_name(field.package()?)?
            {
                Ok(name) => name,
                Err(nonnumerical_type) => match nonnumerical_type {
                    NonNumericalFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                    NonNumericalFieldType::String => self.string_type().into(),
                    NonNumericalFieldType::Bytes => self.vec_type("u8").into(),
                    NonNumericalFieldType::Message(m) => self.type_name_of_msg(m, None)?.into(),
                },
            },
            ImplType::SliceView => match field
                .type_()?
                .native_numerical_type_name(field.package()?)?
            {
                Ok(name) => name,
                Err(nonnumerical_type) => match nonnumerical_type {
                    NonNumericalFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                    NonNumericalFieldType::String => "::std::borrow::Cow<'slice, str>".into(),
                    NonNumericalFieldType::Bytes => "::std::borrow::Cow<'slice, [u8]>".into(),
                    NonNumericalFieldType::Message(m) => self.type_name_of_msg(m, None)?.into(),
                },
            },
        })
    }

    pub fn field_type_name(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>> {
        let scalar_type = self.field_scalar_item_type(field)?;
        Ok(match self.context.impl_type() {
            ImplType::Default => {
                if let FieldType::Message(m) = field.type_()? {
                    if m.is_map_entry() {
                        // Special treatment for map field
                        let (key_field, value_field) = m.key_value_of_map_entry()?;
                        let key_type = self.field_scalar_item_type(key_field)?;
                        let value_type = self.field_scalar_item_type(value_field)?;
                        return Ok(match self.context.alloc_type() {
                            AllocatorType::Default => format!(
                                "::std::collections::HashMap<{key}, {value}>",
                                key = key_type,
                                value = value_type,
                            ),
                            AllocatorType::Bumpalo => format!(
                                "\
::puroro_internal::hashbrown::HashMap<
    {key},
    {value},
    ::puroro_internal::hashbrown::hash_map::DefaultHashBuilder,
    ::puroro_internal::hashbrown::BumpWrapper<'bump>,
>",
                                key = key_type,
                                value = value_type,
                            ),
                        }
                        .into());
                    }
                }
                // Non-map normal fields
                match field.label()? {
                    FieldLabel::Optional2 => {
                        if matches!(field.type_()?, FieldType::Message(_)) {
                            self.option_type(&self.boxed_type(&scalar_type)).into()
                        } else {
                            self.option_type(&scalar_type).into()
                        }
                    }
                    FieldLabel::Optional3 => {
                        if matches!(field.type_()?, FieldType::Message(_)) {
                            self.option_type(&self.boxed_type(&scalar_type)).into()
                        } else {
                            scalar_type.into()
                        }
                    }
                    FieldLabel::Required => scalar_type.into(),
                    FieldLabel::Repeated => self.vec_type(scalar_type.as_ref()).into(),
                }
            }
            ImplType::SliceView => match (field.label()?, field.type_()?) {
                (FieldLabel::Repeated, _) | (_, FieldType::Message(_)) => {
                    let item = "::puroro_internal::SliceViewField::<'slice>";
                    self.option_type(item).into()
                }
                (FieldLabel::Optional2, _) => self.option_type(&scalar_type).into(),
                _ => scalar_type.into(),
            },
        })
    }

    pub fn type_tag_ident_gp<'b, T>(
        &self,
        field: &'c FieldDescriptor<'c>,
        bindings: T,
    ) -> Result<String>
    where
        T: IntoIterator<Item = &'b (&'static str, &'static str)>,
    {
        Ok(match field.type_()? {
            FieldType::Double => "Double".into(),
            FieldType::Float => "Float".into(),
            FieldType::Int32 => "Int32".into(),
            FieldType::Int64 => "Int64".into(),
            FieldType::UInt32 => "UInt32".into(),
            FieldType::UInt64 => "UInt64".into(),
            FieldType::SInt32 => "SInt32".into(),
            FieldType::SInt64 => "SInt64".into(),
            FieldType::Fixed32 => "Fixed32".into(),
            FieldType::Fixed64 => "Fixed64".into(),
            FieldType::SFixed32 => "SFixed32".into(),
            FieldType::SFixed64 => "SFixed64".into(),
            FieldType::Bool => "Bool".into(),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "String".into(),
            FieldType::Bytes => "Bytes".into(),
            FieldType::Enum(e) => format!(
                "Enum::<{module}::{ident}>",
                module = relative_path_over_namespaces(self.msg.package()?, e.package()?, "enums")?,
                ident = e.native_ident()?,
            ),
            FieldType::Message(m) => {
                format!("Message::<{}>", self.type_name_of_msg(m, bindings)?)
            }
        })
    }

    pub fn default_func_for(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>> {
        Ok(match self.context.impl_type() {
            ImplType::Default => match self.context.alloc_type() {
                AllocatorType::Default => match field.type_()? {
                    FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                    FieldType::Enum(_) => "|| 0i32.try_into()".into(),
                    _ => "::std::default::Default::default".into(),
                },
                AllocatorType::Bumpalo => match field.type_()? {
                    // Before this place, a local var puroro_internal is bound.
                    // let puroro_internal = &self.puroro_internal;
                    // We need this proxy var to tell the borrow checker that we are splitting
                    // the `self`.
                    FieldType::String => {
                        "|| ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump)"
                            .into()
                    }
                    FieldType::Bytes => {
                        "|| ::puroro::bumpalo::collections::Vec::new_in(&puroro_internal.bump)"
                            .into()
                    }
                    FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                    FieldType::Enum(_) => "|| 0i32.try_into()".into(),
                    FieldType::Message(m) => format!(
                        "|| {msg}::new_in(&puroro_internal.bump)",
                        msg = self.type_name_of_msg(m, None)?,
                    )
                    .into(),
                    _ => "::std::default::Default::default".into(),
                },
            },
            ImplType::SliceView => {
                unimplemented!()
            }
        })
    }

    pub fn struct_generic_params(&self) -> GenericParams {
        match self.context.alloc_type() {
            AllocatorType::Default => None,
            AllocatorType::Bumpalo => Some("'bump"),
        }
        .into_iter()
        .chain(
            match self.context.impl_type() {
                ImplType::Default => None,
                ImplType::SliceView => Some(["'slice", "S"].iter()),
            }
            .into_iter()
            .flatten()
            .cloned(),
        )
        .unique()
        .collect()
    }

    pub fn struct_generic_params_bounds(&self) -> GenericParams {
        self.struct_generic_params()
    }

    pub fn new_method_declaration(&self) -> &'static str {
        match (self.context.impl_type(), self.context.alloc_type()) {
            (ImplType::Default, AllocatorType::Default) => "fn new() -> Self",
            (ImplType::Default, AllocatorType::Bumpalo) => {
                "fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self"
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn field_new(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "::puroro_internal::FieldNew::new()",
            AllocatorType::Bumpalo => "::puroro_internal::FieldNew::new_in_bumpalo(bump)",
        }
    }

    pub fn field_clone(&self, field_ident: &str, field_type: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!(
                "<{field_type} as FieldClone>::clone(&self.{field_ident})",
                field_ident = field_ident,
                field_type = field_type
            ),
            AllocatorType::Bumpalo => format!(
                "<{field_type} as FieldClone>::clone_in_bumpalo(\
                    &self.{field_ident}, &self.puroro_internal.bump)",
                field_ident = field_ident,
                field_type = field_type
            ),
        }
    }

    pub fn field_take_or_init(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(match self.context.alloc_type() {
            AllocatorType::Default => format!(
                "<{field_type} as FieldTakeOrInit<{taken_type}>>\
                    ::take_or_init(self.{ident})",
                ident = field.native_ident()?,
                field_type = self.field_type_name(field)?,
                taken_type = self.field_scalar_item_type(field)?,
            ),
            AllocatorType::Bumpalo => format!(
                "<{field_type} as FieldTakeOrInit<{taken_type}>>\
                    ::take_or_init_in_bumpalo(self.{ident}, self.puroro_internal.bumpalo())",
                ident = field.native_ident()?,
                field_type = self.field_type_name(field)?,
                taken_type = self.field_scalar_item_type(field)?,
            ),
        })
    }

    pub fn boxed_type(&self, item: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!("::std::boxed::Box::<{item}>", item = item),
            AllocatorType::Bumpalo => {
                format!(
                    "::puroro::bumpalo::boxed::Box::<'bump, {item}>",
                    item = item
                )
            }
        }
    }

    pub fn box_new(&self, item: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!("::std::boxed::Box::new({item})", item = item),
            AllocatorType::Bumpalo => format!(
                "{{
    let bump = self.puroro_internal.bump;
    ::puroro::bumpalo::boxed::Box::new_in({item}, bump)
}}\n",
                item = item
            ),
        }
    }

    fn vec_type(&self, item: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!("::std::vec::Vec::<{item}>", item = item),
            AllocatorType::Bumpalo => {
                format!(
                    "::puroro::bumpalo::collections::Vec::<'bump, {item}>",
                    item = item
                )
            }
        }
    }

    fn string_type(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "::std::string::String",
            AllocatorType::Bumpalo => "::puroro::bumpalo::collections::String::<'bump>",
        }
    }

    fn option_type(&self, item: &str) -> String {
        format!("::std::option::Option::<{item}>", item = item)
    }

    pub fn internal_data_type(&self) -> &'static str {
        match (self.context.impl_type(), self.context.alloc_type()) {
            (ImplType::Default, AllocatorType::Default) => {
                "::puroro_internal::InternalDataForNormalStruct"
            }
            (ImplType::Default, AllocatorType::Bumpalo) => {
                "::puroro_internal::InternalDataForBumpaloStruct<'bump>"
            }
            (ImplType::SliceView, AllocatorType::Default) => {
                "::puroro_internal::InternalDataForSliceViewStruct<'slice, S>"
            }
            _ => unimplemented!(),
        }
    }

    pub fn map_owned_key_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<Cow<'static, str>> {
        Ok(
            match field
                .type_()?
                .native_numerical_type_name(field.package()?)?
            {
                Ok(name) => name,
                Err(nonnumerical_type) => match nonnumerical_type {
                    NonNumericalFieldType::String => self.string_type().into(),
                    _ => Err(ErrorKind::InvalidMapKey {
                        name: field.fully_qualified_type_name()?.to_string(),
                    })?,
                },
            },
        )
    }

    pub fn map_owned_value_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<Cow<'static, str>> {
        Ok(self.field_scalar_item_type(field)?)
    }
}
