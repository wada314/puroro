use super::writer::{func, indent, iter, IntoFragment};
use crate::context::Context;
use crate::utils::{to_camel_case, Indentor};
use crate::wrappers::{FieldLabel, FieldType, MessageDescriptor};
use crate::Result;

pub struct MessageTraitCodeGenerator<'c> {
    context: &'c Context<'c>,
    msg: &'c MessageDescriptor<'c>,
}

impl<'c> MessageTraitCodeGenerator<'c> {
    pub fn new(context: &'c Context<'c>, msg: &'c MessageDescriptor<'c>) -> Self {
        Self { context, msg }
    }
    pub fn print_msg_traits<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            func(|output| self.print_msg_base_trait(output)),
            func(|output| self.print_msg_mutable_trait(output)),
        )
            .write_into(output)
    }

    fn print_msg_base_trait<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
pub trait {name}Trait {{\n",
                name = self.msg.native_bare_type_name()
            ),
            indent(iter(self.msg.fields().map(|field| -> Result<_> {
                Ok(match (field.label()?, field.type_()?) {
                    (FieldLabel::Optional, FieldType::Message(_)) => {
                        // getter function for optional message field, wrapped by Option.
                        format!(
                            "fn {name}(&self) -> ::std::option::Option<{reftype}>;\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_ref_type_name("")?,
                        )
                    }
                    (FieldLabel::Required, _) | (FieldLabel::Optional, _) => {
                        // normal getter function.
                        format!(
                            "fn {name}(&self) -> {reftype};\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_ref_type_name("")?,
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
                            name = field.native_name(),
                            camel_name = to_camel_case(field.native_name()),
                            reftype = field.native_scalar_ref_type_name("")?,
                            reftype_lt_a = field.native_scalar_ref_type_name("'a")?,
                        )
                    }
                })
            }))),
            "}}\n",
        )
            .write_into(output)
    }

    fn print_msg_mutable_trait<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
pub trait {name}MutTrait {{\n",
                name = self.msg.native_bare_type_name()
            ),
            indent(iter(self.msg.fields().map(|field| -> Result<_> {
                Ok(match (field.label()?, field.type_()?) {
                    (FieldLabel::Optional, FieldType::Message(_)) => {
                        // getter function for optional message field, wrapped by Option.
                        format!(
                            "fn {name}_mut(&self) -> ::std::option::Option<{reftype}>;\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_mut_ref_type_name()?,
                        )
                    }
                    (FieldLabel::Required, _) | (FieldLabel::Optional, _) => {
                        // normal getter function.
                        format!(
                            "fn {name}_mut(&self) -> {reftype};\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_mut_ref_type_name()?,
                        )
                    }
                    (FieldLabel::Repeated, _) => {
                        format!(
                            "\
fn for_each_{name}_mut<F>(&self, f: F)
where
    F: FnMut({reftype});
fn {name}_boxed_iter_mut(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>;
// We need more! Maybe just expose &mut Vec<T> ? \n",
                            name = field.native_name(),
                            reftype = field.native_scalar_mut_ref_type_name()?,
                        )
                    }
                })
            }))),
            "}}\n",
        )
            .write_into(output)
    }
}
