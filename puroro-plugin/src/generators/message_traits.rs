use std::borrow::Cow;

use super::writer::{func, indent, iter, IntoFragment};
use crate::context::Context;
use crate::utils::{relative_path, to_camel_case, Indentor};
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, MessageDescriptor, NonNumericalFieldType,
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
                        "\
type {type_ident}<'this>: {trait_path}
    where Self: 'this;\n",
                        type_ident = self.associated_msg_type_ident(msg)?,
                        trait_path =
                            self.trait_ident_with_relative_path(msg, self.msg.package()?)?,
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
                        }
                        | GetterMethods::MapField {
                            return_type_ident_gp: type_ident_gp,
                            return_type_bound: type_bound,
                            get_decl,
                        } => {
                            format!(
                                "\
type {type_ident_gp}: {type_bound}
    where Self: 'this;
{get_decl};\n",
                                type_ident_gp = type_ident_gp,
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
                    return_type_ident_gp: format!("{ident}<'this>", ident = type_ident.clone()),
                    return_type_bound: format!(
                        "::puroro::MapField::<'this, {key}, {value}>",
                        key = self.map_deref_borrowed_key_type_name(key_field)?,
                        value = self.map_value_getter_type_name(value_field)?,
                    ),
                    get_decl: format!(
                        "fn {ident}<'this>(&'this self) -> Self::{type_ident}::<'this>",
                        ident = field.native_ident()?,
                        type_ident = type_ident,
                    ),
                }
            }
            (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                GetterMethods::OptionalField(format!(
                    "fn {name}<'this>(&'this self) -> ::std::option::Option::<{reftype}>",
                    name = field.native_ident()?,
                    reftype = self.scalar_getter_type_name(field, "'this", "'this")?,
                ))
            }
            (FieldLabel::Repeated, _) => {
                let type_ident = format!("{}Repeated", to_camel_case(field.native_ident()?));
                GetterMethods::RepeatedField {
                    return_type_ident_gp: format!("{ident}<'this>", ident = type_ident.clone()),
                    return_type_bound: format!(
                        "::puroro::RepeatedField::<'this, {value}>",
                        value = self.scalar_getter_type_name(field, "'this", "'static")?,
                    ),
                    get_decl: format!(
                        "fn {ident}<'this>(&'this self) -> Self::{type_ident}::<'this>",
                        ident = field.native_ident()?,
                        type_ident = type_ident,
                    ),
                }
            }
            (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
                GetterMethods::BareField(format!(
                    "fn {name}<'this>(&'this self) -> {reftype}",
                    name = field.native_ident()?,
                    reftype = self.scalar_getter_type_name(field, "'this", "'this")?,
                ))
            }
        })
    }

    pub fn trait_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'static, str>> {
        Ok(format!("{}Trait", msg.native_ident()?).into())
    }

    pub fn trait_ident_with_relative_path(
        &self,
        msg: &'c MessageDescriptor<'c>,
        cur_package: &str,
    ) -> Result<Cow<'static, str>> {
        Ok(format!(
            "{module}::{ident}",
            module = relative_path(cur_package, msg.package()?)?,
            ident = self.trait_ident(msg)?,
        )
        .into())
    }

    pub fn associated_msg_type_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Type", msg.native_ident()?))
    }
    pub fn associated_msg_type_ident_gp<'b, T>(
        &self,
        msg: &'c MessageDescriptor<'c>,
        bindings: T,
    ) -> Result<String>
    where
        T: IntoIterator<Item = &'b (&'static str, &'static str)>,
    {
        let mut lt = "'this";
        for &(from, to) in bindings {
            if lt == from {
                lt = to;
            }
        }
        Ok(format!(
            "{ident}Type::<{lt}>",
            ident = msg.native_ident()?,
            lt = lt
        ))
    }

    pub fn scalar_getter_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
        this_lifetime: &'static str,
        message_fields_parent_lifetime: &'static str,
    ) -> Result<Cow<'static, str>> {
        Ok(
            match field
                .type_()?
                .native_numerical_type_name(field.package()?)?
            {
                Ok(name) => name.into(),
                Err(nonnumerical_type) => {
                    let t: Cow<str> = match nonnumerical_type {
                        NonNumericalFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                        NonNumericalFieldType::String => "str".into(),
                        NonNumericalFieldType::Bytes => "[u8]".into(),
                        NonNumericalFieldType::Message(m) => format!(
                            "<Self as {trait_name}>::{name}",
                            trait_name = self.trait_ident(self.msg)?,
                            name = self.associated_msg_type_ident_gp(
                                m,
                                &[("'this", message_fields_parent_lifetime)]
                            )?,
                        )
                        .into(),
                    };
                    format!(
                        "::std::borrow::Cow::<{lt}, {type_}>",
                        lt = this_lifetime,
                        type_ = t,
                    )
                    .into()
                }
            },
        )
    }

    pub fn map_deref_borrowed_key_type_name(
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
                    NonNumericalFieldType::String => "str".into(),
                    _ => Err(ErrorKind::InvalidMapKey {
                        name: field.fully_qualified_type_name()?.to_string(),
                    })?,
                },
            },
        )
    }

    pub fn map_value_getter_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<Cow<'static, str>> {
        self.scalar_getter_type_name(field, "'this", "'static")
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
        return_type_ident_gp: String,
        return_type_bound: String,
        get_decl: String,
    },
}
