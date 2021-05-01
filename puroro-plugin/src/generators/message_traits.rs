use super::writer::{func, indent, iter, IntoFragment};
use crate::context::Context;
use crate::utils::{to_camel_case, Indentor};
use crate::wrappers::{FieldLabel, FieldType, MessageDescriptor};
use crate::Result;

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
            indent(iter(self.msg.fields().map(|field| -> Result<_> {
                Ok((
                    if let FieldType::Message(m) = field.type_()? {
                        // Associated Type for the message type
                        format!(
                            "type {camel_name}Type: {submsg_name}Trait;\n",
                            camel_name = to_camel_case(field.native_name()?),
                            submsg_name = m.native_ident_with_relative_path(field.package()?)?,
                        )
                    } else {
                        "".to_string()
                    },
                    match (field.label()?, field.type_()?) {
                        (FieldLabel::Optional2, FieldType::Message(_))
                        | (FieldLabel::Optional3, FieldType::Message(_)) => {
                            // getter function for optional message field, wrapped by Option.
                            format!(
                                "fn {name}(&'_ self) -> ::std::option::Option<{reftype}>;\n",
                                name = field.native_name()?,
                                reftype = field.native_maybe_ref_type("'_")?,
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
                        (FieldLabel::Optional2, _) => {
                            // getter function with Optional.
                            format!(
                                "fn {name}(&'_ self) -> ::std::option::Option<{reftype}>;\n",
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
                                reftype = field.native_maybe_ref_type("'_")?,
                                reftype_lt_a = field.native_maybe_ref_type("'a")?,
                            )
                        }
                    },
                ))
            }))),
            "}}\n",
        )
            .write_into(output)
    }
}
