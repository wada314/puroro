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
pub trait {name}Trait {{\n",
                name = self.msg.native_ident()?
            ),
            indent((iter(self.msg.fields().map(|field| -> Result<_> {
                Ok((match (field.label()?, field.type_()?) {
                    (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                        // getter function for optional field, wrapped by Option.
                        format!(
                            "fn {name}(&'_ self) -> ::std::option::Option<{reftype}>;\n",
                            name = field.native_name()?,
                            reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                        )
                    }
                    (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
                        // normal getter function.
                        format!(
                            "fn {name}(&'_ self) -> {reftype};\n",
                            name = field.native_name()?,
                            reftype = field.native_maybe_ref_type("'_")?,
                        )
                    }
                    (FieldLabel::Repeated, _) => {
                        // for_each_***:
                        // A generic getter function for repeated field.
                        // Because of some current Rust language limitations we can only
                        // use an internal iterator, which reminds me Rust @ 2013.
                        // https://doc.rust-lang.org/0.6/std/list.html#function-iter
                        // ***_boxed_iter:
                        // Another restricted getter function. Returns an iterator,
                        // but it is wrapped by `Box`.
                        format!(
                            "\
fn for_each_{name}<F>(&self, f: F)
where
    F: FnMut({reftype});
fn {name}_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>;
#[cfg(feature = \"puroro-nightly\")]
type {camel_name}Iter<'a>: Iterator<Item={reftype_lt_a}>;
#[cfg(feature = \"puroro-nightly\")]
fn {name}_iter(&self) -> Self::{camel_name}Iter<'_>;\n",
                            name = field.native_name()?,
                            camel_name = to_camel_case(field.native_name()?),
                            reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                            reftype_lt_a = self.scalar_maybe_ref_type_name(field, "'a")?,
                        )
                    }
                },))
            })),)),
            "}}\n",
        )
            .write_into(output)
    }

    pub fn generate_getter_method_decls(
        &self,
        field: &'c FieldDescriptor<'c>,
    ) -> Result<GetterMethods> {
        Ok(match (field.label()?, field.type_()?) {
            (FieldLabel::Optional2, _) | (FieldLabel::Optional3, FieldType::Message(_)) => {
                GetterMethods::OptionalField(format!(
                    "fn {name}(&'_ self) -> ::std::option::Option<{reftype}>",
                    name = field.native_name()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ))
            }
            (FieldLabel::Repeated, _) => GetterMethods::RepeatedField {
                for_each: format!(
                    "\
fn for_each_{name}<F>(&self, f: F)
where
    F: FnMut({reftype})",
                    name = field.native_name()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ),
                boxed_iter: format!(
                    "\
fn {name}_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>",
                    name = field.native_name()?,
                    reftype = self.scalar_maybe_ref_type_name(field, "'_")?,
                ),
                iter: format!(
                    "\
#[cfg(feature = \"puroro-nightly
fn {name}_iter(&self) -> Self::{iter_name}<'_>;\n",
                    name = field.native_name()?,
                    iter_name = self.associated_iter_type_ident(field)?,
                ),
            },
            (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
                GetterMethods::ScalarField(format!(
                    "fn {name}(&'_ self) -> {reftype}",
                    name = field.native_name()?,
                    reftype = field.native_maybe_ref_type("'_")?,
                ))
            }
        })
    }

    pub fn associated_msg_type_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Type", msg.native_ident()?))
    }
    pub fn associated_iter_type_ident(&self, field: &'c FieldDescriptor<'c>) -> Result<String> {
        Ok(format!("{}Iter", to_camel_case(field.native_name()?)))
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

enum GetterMethods {
    ScalarField(String),
    OptionalField(String),
    RepeatedField {
        for_each: String,
        boxed_iter: String,
        iter: String,
    },
}
