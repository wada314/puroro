use std::borrow::Cow;

use itertools::Itertools;

use crate::context::{AllocatorType, Context, ImplType};
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, MessageDescriptor, NonTrivialFieldType,
};
use crate::{ErrorKind, Result};

pub struct MessageImplFragmentGenerator<'a, 'c> {
    context: &'a Context<'c>,
}
impl<'a, 'c> MessageImplFragmentGenerator<'a, 'c> {
    pub fn new(context: &'a Context<'c>) -> Self {
        Self { context }
    }

    pub fn struct_name(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'c, str>> {
        let postfix1 = match self.context.impl_type() {
            ImplType::Default => "",
            ImplType::SliceRef => "SliceRef",
        };
        let postfix2 = match self.context.alloc_type() {
            AllocatorType::Default => "",
            AllocatorType::Bumpalo => "Bumpalo",
        };
        Ok(format!(
            "{name}{postfix1}{postfix2}",
            name = msg.native_bare_type_name(),
            postfix1 = postfix1,
            postfix2 = postfix2,
        )
        .into())
    }

    pub fn cfg_condition(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Bumpalo => "#[cfg(feature = \"puroro-bumpalo\")]",
            _ => "",
        }
    }

    pub fn is_default_available(&self) -> bool {
        match (self.context.impl_type(), self.context.alloc_type()) {
            (ImplType::SliceRef, _) | (_, AllocatorType::Bumpalo) => false,
            _ => true,
        }
    }

    pub fn field_visibility(&self) -> &'static str {
        match self.context.impl_type() {
            ImplType::Default => "pub ",
            ImplType::SliceRef => "",
        }
    }

    pub fn field_type_for(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>> {
        Ok(match self.context.impl_type() {
            ImplType::Default => {
                let scalar_type: Cow<'static, str> = match field.type_()?.native_trivial_type_name()
                {
                    Ok(name) => name.into(),
                    Err(nontrivial_type) => match nontrivial_type {
                        NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                        NonTrivialFieldType::String => self.string_type().into(),
                        NonTrivialFieldType::Bytes => self.vec_type("u8").into(),

                        NonTrivialFieldType::Enum(e) => format!(
                            "::std::result::Result<{type_}, i32>",
                            type_ = e.native_fully_qualified_type_name(field.path_to_root_mod())
                        )
                        .into(),
                        NonTrivialFieldType::Message(m) => m
                            .native_fully_qualified_type_name(field.path_to_root_mod())
                            .into(),
                    },
                };
                match field.label()? {
                    FieldLabel::Optional => {
                        if matches!(field.type_()?, FieldType::Message(_)) {
                            format!(
                                "::std::option::Option<{boxed_type}>",
                                boxed_type = self.box_type(scalar_type.as_ref()),
                            )
                            .into()
                        } else {
                            scalar_type.into()
                        }
                    }
                    FieldLabel::Required => scalar_type.into(),
                    FieldLabel::Repeated => self.vec_type(scalar_type.as_ref()).into(),
                }
            }
            ImplType::SliceRef => unimplemented!(),
        })
    }

    pub fn struct_generic_params(&self, params: &[&'static str]) -> String {
        let iter = params
            .iter()
            .cloned()
            .chain(match self.context.alloc_type() {
                AllocatorType::Default => None.into_iter(),
                AllocatorType::Bumpalo => Some("'bump").into_iter(),
            });
        if iter.clone().count() == 0 {
            "".to_string()
        } else {
            format!(
                "<{}>",
                Itertools::intersperse(iter, ", ").collect::<String>()
            )
        }
    }

    pub fn struct_generic_params_bounds(&self, params: &[&'static str]) -> String {
        self.struct_generic_params(params)
    }

    pub fn new_method_declaration(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "fn new() -> Self",
            AllocatorType::Bumpalo => "fn new_in(bump: &'bump ::bumpalo::Bump) -> Self",
        }
    }

    pub fn box_type(&self, item: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!("::std::boxed::Box<{item}>", item = item),
            AllocatorType::Bumpalo => format!("::bumpalo::boxed::Box<'bump, {item}>", item = item),
        }
    }

    pub fn vec_type(&self, item: &str) -> String {
        match self.context.alloc_type() {
            AllocatorType::Default => format!("::std::vec::Vec<{item}>", item = item),
            AllocatorType::Bumpalo => {
                format!("::bumpalo::collections::Vec<'bump, {item}>", item = item)
            }
        }
    }

    pub fn string_type(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "::std::string::String",
            AllocatorType::Bumpalo => "::bumpalo::collections::String<'bump>",
        }
    }

    pub fn internal_data_type(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "::puroro::helpers::InternalDataForNormalStruct",
            AllocatorType::Bumpalo => "::puroro::helpers::InternalDataForBumpaloStruct<'bump>",
        }
    }

    pub fn internal_field_init_value(&self) -> &'static str {
        match self.context.alloc_type() {
            AllocatorType::Default => "::puroro::helpers::InternalDataForNormalStruct::new()",
            AllocatorType::Bumpalo => "::puroro::helpers::InternalDataForBumpaloStruct::new(bump)",
        }
    }
}
