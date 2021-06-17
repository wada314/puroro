use std::borrow::Cow;

use super::writer::{func, indent, iter, IntoFragment};
use crate::utils::{
    relative_path, relative_path_over_namespaces, to_camel_case, GenericParams, Indentor,
};
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, MessageDescriptor, NonNumericalFieldType,
};
use crate::{ErrorKind, Result};

pub struct MessageTraitCodeGenerator<'c> {
    msg: &'c MessageDescriptor<'c>,
}

impl<'c> MessageTraitCodeGenerator<'c> {
    pub fn new(msg: &'c MessageDescriptor<'c>) -> Self {
        Self { msg }
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
            indent((iter(self.msg.fields().map(|field| -> Result<String> {
                let getter_methods = self.generate_getter_method_decls(field)?;
                let maybe_msg_decl = if let Some(AssociatedType {
                    ident,
                    gp,
                    bound,
                    where_clause,
                }) = getter_methods.maybe_msg_type
                {
                    format!(
                        "type {ident}{gp}: {bound} {where_clause};\n",
                        ident = ident,
                        gp = gp,
                        bound = bound,
                        where_clause = where_clause,
                    )
                } else {
                    "".to_string()
                };
                let body = match getter_methods.field_label_type {
                    FieldLabelType::BareField { get_decl }
                    | FieldLabelType::OptionalField { get_decl } => {
                        format!("{decl};\n", decl = get_decl)
                    }
                    FieldLabelType::RepeatedField {
                        get_decl,
                        repeated_type:
                            AssociatedType {
                                ident,
                                gp,
                                bound,
                                where_clause,
                            },
                    }
                    | FieldLabelType::MapField {
                        get_decl,
                        repeated_type:
                            AssociatedType {
                                ident,
                                gp,
                                bound,
                                where_clause,
                            },
                    } => {
                        format!(
                            "\
type {ident}{gp}: {bound}
    {where_clause};
{get_decl};\n",
                            ident = ident,
                            gp = gp,
                            bound = bound,
                            get_decl = get_decl,
                            where_clause = where_clause,
                        )
                    }
                };
                Ok(format!("{}{}", maybe_msg_decl, body))
            })),)),
            "}}\n",
        )
            .write_into(output)
    }

    pub fn generate_getter_method_decls(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<GetterMethods> {
        let field_label_type = match (field.label()?, field.type_()?) {
            (FieldLabel::Repeated, FieldType::Message(m)) if m.is_map_entry() => {
                // Map.
                let (key_field, value_field) = m.key_value_of_map_entry()?;
                let ident: Cow<'static, str> =
                    format!("{}Map", to_camel_case(field.native_ident()?)).into();
                FieldLabelType::MapField {
                    repeated_type: AssociatedType {
                        ident: ident.clone(),
                        gp: std::array::IntoIter::new(["'this"]).collect(),
                        bound: format!(
                            "::puroro::MapField::<'this, {key}, {value}>",
                            key = self.map_deref_borrowed_key_type_name(key_field)?,
                            value = self.map_value_getter_type_name(value_field)?,
                        )
                        .into(),
                        where_clause: "where Self: 'this".into(),
                    },
                    get_decl: format!(
                        "fn {ident}<'this>(&'this self) -> Self::{type_ident}::<'this>",
                        ident = field.native_ident()?,
                        type_ident = ident,
                    )
                    .into(),
                }
            }
            (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                FieldLabelType::OptionalField {
                    get_decl: format!(
                        "fn {name}<'this>(&'this self) -> ::std::option::Option::<{reftype}>",
                        name = field.native_ident()?,
                        reftype = self.scalar_getter_type_name(field, "'this")?,
                    )
                    .into(),
                }
            }
            (FieldLabel::Repeated, _) => {
                let ident = format!("{}Repeated", to_camel_case(field.native_ident()?));
                FieldLabelType::RepeatedField {
                    repeated_type: AssociatedType {
                        ident: ident.clone().into(),
                        gp: std::array::IntoIter::new(["'this"]).collect(),
                        bound: format!(
                            "::puroro::RepeatedField::<{value}>",
                            value = self.scalar_getter_type_name(field, "'this")?,
                        )
                        .into(),
                        where_clause: "where Self: 'this".into(),
                    },
                    get_decl: format!(
                        "fn {ident}<'this>(&'this self) -> Self::{type_ident}::<'this>",
                        ident = field.native_ident()?,
                        type_ident = ident,
                    )
                    .into(),
                }
            }
            (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => FieldLabelType::BareField {
                get_decl: format!(
                    "fn {name}<'this>(&'this self) -> {reftype}",
                    name = field.native_ident()?,
                    reftype = self.scalar_getter_type_name(field, "'this")?,
                )
                .into(),
            },
        };
        let maybe_message_associated_type = match field.type_()? {
            FieldType::Message(m) => {
                let ident = self.associated_msg_type_ident(field, field.label()?)?;
                Some(AssociatedType {
                    ident: ident.into(),
                    gp: std::array::IntoIter::new(["'this"]).collect(),
                    bound: self.trait_path_from_trait(m, self.msg.package()?)?,
                    where_clause: "where Self: 'this".into(),
                })
            }
            _ => None,
        };
        Ok(GetterMethods {
            maybe_msg_type: maybe_message_associated_type,
            field_label_type,
        })
    }

    pub fn trait_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'static, str>> {
        Ok(format!("{}Trait", msg.native_ident()?).into())
    }

    pub fn trait_path_from_trait(
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

    pub fn trait_path_from_struct(&self, cur_package: &str) -> Result<Cow<'static, str>> {
        Ok(format!(
            "{module}::{ident}",
            module = relative_path_over_namespaces(cur_package, self.msg.package()?, "traits")?,
            ident = self.trait_ident(self.msg)?,
        )
        .into())
    }

    pub fn associated_msg_type_ident(
        &self,
        field: &'c FieldDescriptor<'c>,
        label: FieldLabel,
    ) -> Result<Cow<'static, str>> {
        let postfix = match label {
            FieldLabel::Repeated => "Element",
            _ => "Type",
        };
        Ok(format!(
            "{ident}{postfix}",
            ident = to_camel_case(&field.native_ident()?),
            postfix = postfix,
        )
        .into())
    }

    pub fn associated_msg_type_ident_gp<'b, T>(
        &self,
        field: &'c FieldDescriptor<'c>,
        label: FieldLabel,
        bindings: T,
    ) -> Result<Cow<'static, str>>
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
            "{ident}::<{lt}>",
            ident = self.associated_msg_type_ident(field, label)?,
            lt = lt
        )
        .into())
    }

    pub fn scalar_getter_type_name(
        &self,
        field: &'c FieldDescriptor<'c>,
        this_lifetime: &'static str,
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
                        NonNumericalFieldType::Message(_) => format!(
                            "Self::{name}",
                            name =
                                self.associated_msg_type_ident_gp(field, field.label()?, None,)?,
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
        self.scalar_getter_type_name(field, "'this")
    }
}

pub struct AssociatedType {
    pub ident: Cow<'static, str>,
    pub gp: GenericParams,
    pub bound: Cow<'static, str>,
    pub where_clause: Cow<'static, str>,
}

pub struct GetterMethods {
    pub maybe_msg_type: Option<AssociatedType>,
    pub field_label_type: FieldLabelType,
}

pub enum FieldLabelType {
    BareField {
        get_decl: Cow<'static, str>,
    },
    OptionalField {
        get_decl: Cow<'static, str>,
    },
    RepeatedField {
        repeated_type: AssociatedType,
        get_decl: Cow<'static, str>,
    },
    MapField {
        repeated_type: AssociatedType,
        get_decl: Cow<'static, str>,
    },
}
