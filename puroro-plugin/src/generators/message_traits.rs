use super::writer::{func, indent, iter, Fragment, IntoFragment};
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
pub trait {trait_ident} {{\n",
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
                iter(
                    // typedefs for iterator types
                    self.msg
                        .fields()
                        .filter(|field| matches!(field.label(), Ok(FieldLabel::Repeated)))
                        .map(|field| {
                            Ok(format!(
                                "\
#[cfg(feature = \"puroro-nightly\")]
type {iter_ident}<'a>: ::std::iter::Iterator<Item={reftype}>
    where {scalartype}: 'a;\n",
                                iter_ident = self.associated_iter_type_ident(field)?,
                                reftype = self.scalar_maybe_ref_type_name(field, "'a")?,
                                scalartype = self.scalar_type_name(field)?,
                            ))
                        }),
                ),
                iter(self.msg.fields().map(|field| -> Result<Fragment<W>> {
                    // getter method decls
                    Ok(match self.generate_getter_method_decls(field, false)? {
                        GetterMethods::ScalarField(decl) | GetterMethods::OptionalField(decl) => {
                            format!("{decl};\n", decl = decl).into()
                        }
                        GetterMethods::RepeatedField {
                            for_each,
                            boxed_iter,
                            iter,
                        } => (
                            format!("{decl};\n", decl = for_each),
                            format!("{decl};\n", decl = boxed_iter),
                            format!("{decl};\n", decl = iter),
                        )
                            .into(),
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
        has_body: bool,
    ) -> Result<GetterMethods> {
        Ok(match (field.label()?, field.type_()?) {
            (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                GetterMethods::OptionalField(format!(
                    "fn {name}(&'_ self) -> ::std::option::Option<{reftype}>",
                    name = field.native_ident()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ))
            }
            (FieldLabel::Repeated, _) => GetterMethods::RepeatedField {
                for_each: format!(
                    "\
fn for_each_{name}<F>(&self, {maybe_mut}f: F)
where
    F: FnMut({reftype})",
                    name = field.native_ident()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                    maybe_mut = if has_body { "mut " } else { "" }
                ),
                boxed_iter: format!(
                    "\
fn {name}_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>",
                    name = field.native_ident()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ),
                iter: format!(
                    "\
#[cfg(feature = \"puroro-nightly\")]
fn {name}_iter(&self) -> Self::{iter_name}<'_>",
                    name = field.native_ident()?,
                    iter_name = self.associated_iter_type_ident(field)?,
                ),
            },
            (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
                GetterMethods::ScalarField(format!(
                    "fn {name}(&'_ self) -> {reftype}",
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
    pub fn associated_iter_type_ident(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Iter", to_camel_case(field.native_ident()?)))
    }

    pub fn scalar_type_name(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => format!("str").into(),
                NonTrivialFieldType::Bytes => format!("[u8]").into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_ident_with_relative_path(field.package()?)?
                )
                .into(),
                NonTrivialFieldType::Message(m) => {
                    format!("Self::{name}", name = self.associated_msg_type_ident(m)?).into()
                }
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
    ScalarField(String),
    OptionalField(String),
    RepeatedField {
        for_each: String,
        boxed_iter: String,
        iter: String,
    },
}
