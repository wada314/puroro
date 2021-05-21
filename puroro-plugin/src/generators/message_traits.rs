use super::writer::{func, indent, iter, IntoFragment};
use crate::context::Context;
use crate::utils::{to_camel_case, Indentor};
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, MessageDescriptor, NonTrivialFieldType,
};
use crate::{ErrorKind, Result};

pub struct MessageTraitCodeGenerator<'a, 'c> {
    #[allow(unused)]
    context: &'a Context<'c>,
    msg: &'c MessageDescriptor<'c>,
}

impl<'a, 'c> MessageTraitCodeGenerator<'a, 'c> {
    pub fn new(context: &'a Context<'c>, msg: &'c MessageDescriptor<'c>) -> Self {
        Self { context, msg }
    }
    pub fn print_msg_traits<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (func(|output| self.print_msg_base_trait(output)),).write_into(output)
    }

    fn print_msg_base_trait<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
pub trait {trait_ident}: ::std::clone::Clone {{\n",
                trait_ident = self.trait_ident(self.msg)?,
            ),
            indent((
                iter(self.msg.unique_msgs_from_fields()?.map(|msg| {
                    // typedefs for message types
                    Ok(format!(
                        "type {type_name}: {trait_rel_ident};\n",
                        type_name = self.associated_msg_type_ident(msg)?,
                        trait_rel_ident = self.trait_relative_ident(msg)?,
                    ))
                })),
                iter(self.msg.fields().map(|field| -> Result<String> {
                    // getter method decls
                    Ok(match self.generate_getter_method_decls(field)? {
                        GetterMethods::BareField(decl) | GetterMethods::OptionalField(decl) => {
                            format!("{decl};\n", decl = decl)
                        }
                        GetterMethods::RepeatedField {
                            return_type_ident_gp: type_ident_gp,
                            return_type_bound: type_bound,
                            get_decl,
                        } => {
                            format!(
                                "type {type_ident_gp}: {type_bound} where Self: 'a;\n{get_decl};\n",
                                type_ident_gp = type_ident_gp,
                                type_bound = type_bound,
                                get_decl = get_decl
                            )
                        }
                        GetterMethods::MapField {
                            return_type_ident: type_ident,
                            return_type_bound: type_bound,
                            get_decl,
                        } => {
                            format!(
                                "type {type_ident}: {type_bound};\n{get_decl};\n",
                                type_ident = type_ident,
                                type_bound = type_bound,
                                get_decl = get_decl
                            )
                        }
                    })
                })),
            )),
            "}}\n",
        )
            .write_into(output)
    }

    pub fn generate_getter_method_decls(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<GetterMethods> {
        Ok(match (field.label()?, field.type_()?) {
            (FieldLabel::Repeated, FieldType::Message(m)) if m.is_map_entry() => {
                // Map.
                let (key_field, value_field) = m.key_value_of_map_entry()?;
                let type_ident = format!("{}Map", to_camel_case(field.native_ident()?));
                GetterMethods::MapField {
                    return_type_ident: type_ident.clone(),
                    return_type_bound: format!(
                        "::puroro::MapField<{key}, {value}>",
                        key = self.scalar_deref_type_name(key_field)?,
                        value = self.scalar_deref_type_name(value_field)?,
                    ),
                    get_decl: format!(
                        "fn {ident}(&self) -> &Self::{type_ident}",
                        ident = field.native_ident()?,
                        type_ident = type_ident,
                    ),
                }
            }
            (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                GetterMethods::OptionalField(format!(
                    "fn {name}(&self) -> ::std::option::Option<{reftype}>",
                    name = field.native_ident()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ))
            }
            (FieldLabel::Repeated, _) => {
                let type_ident = format!("{}Repeated", to_camel_case(field.native_ident()?));
                GetterMethods::RepeatedField {
                    return_type_ident_gp: format!("{ident}<'a>", ident = type_ident.clone()),
                    return_type_bound: format!(
                        "::puroro::RepeatedField<'a, {value}>",
                        value = self.repeated_item_type_name(field)?,
                    ),
                    get_decl: format!(
                        "fn {ident}<'a>(&'a self) -> Self::{type_ident}<'a>",
                        ident = field.native_ident()?,
                        type_ident = type_ident,
                    ),
                }
            }
            (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
                GetterMethods::BareField(format!(
                    "fn {name}(&self) -> {reftype}",
                    name = field.native_ident()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ))
            }
        })
    }

    pub fn trait_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Trait", msg.native_ident()?))
    }
    pub fn trait_relative_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<String> {
        Ok(format!(
            "{}Trait",
            msg.native_ident_with_relative_path(self.msg.package()?)?
        ))
    }
    pub fn associated_msg_type_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Type", msg.native_ident()?))
    }

    pub fn scalar_deref_type_name(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => format!("str").into(),
                NonTrivialFieldType::Bytes => format!("[u8]").into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_ident_with_relative_path(self.msg.package()?)?
                )
                .into(),
                NonTrivialFieldType::Message(m) => {
                    format!("Self::{name}", name = self.associated_msg_type_ident(m)?).into()
                }
            },
        })
    }

    pub fn repeated_item_type_name(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => format!("::std::borrow::Cow<'a, str>").into(),
                NonTrivialFieldType::Bytes => format!("::std::borrow::Cow<'a, [u8]>").into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_ident_with_relative_path(self.msg.package()?)?
                )
                .into(),
                NonTrivialFieldType::Message(m) => format!(
                    "::std::borrow::Cow<'a, Self::{name}>",
                    name = self.associated_msg_type_ident(m)?
                )
                .into(),
            },
        })
    }

    pub fn scalar_maybe_ref_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
        lifetime: &str,
    ) -> Result<String> {
        Ok(match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => format!("&{lt} str", lt = lifetime).into(),
                NonTrivialFieldType::Bytes => format!("&{lt} [u8]", lt = lifetime).into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_ident_with_relative_path(field.package()?)?
                )
                .into(),
                NonTrivialFieldType::Message(m) => format!(
                    "&{lt} Self::{name}",
                    lt = lifetime,
                    name = self.associated_msg_type_ident(m)?
                )
                .into(),
            },
        })
    }
}

pub enum GetterMethods {
    BareField(String),
    OptionalField(String),
    RepeatedField {
        return_type_ident_gp: String,
        return_type_bound: String,
        get_decl: String,
    },
    MapField {
        return_type_ident: String,
        return_type_bound: String,
        get_decl: String,
    },
}
