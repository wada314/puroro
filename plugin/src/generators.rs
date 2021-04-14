pub mod shared;
//pub mod simple;

// Too long!
const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";

use std::fmt::Write;

use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
use crate::wrappers::{
    DescriptorVisitor, EnumDescriptor, FieldLabel, FieldType, MessageDescriptor,
    NonvariantFieldType,
};
use crate::Result;
use shared::writers::{func, indent, indent_n, iter, seq, Fragment, TupleOfIntoFragments};

use super::Context;
struct Visitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for Visitor {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> crate::Result<()> {
        (
            format!(
                "\
#[derive(Debug, Clone)]
pub struct {name} {{\n",
                name = msg.native_bare_type_name(),
            ),
            indent((iter(msg.fields().map(|field| {
                Ok(format!(
                    "pub {name}: {type_},\n",
                    name = field.native_name(),
                    type_ = field.native_owned_type_name()?,
                ))
            })),)),
            "\
}}\n",
            format!(
                "\
impl ::std::default::Default for {name} {{
    fn default() -> Self {{
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {{\n",
                name = msg.native_bare_type_name(),
            ),
            indent_n(
                3,
                (iter(msg.fields().map(|field| {
                    match (field.label()?, field.type_()?) {
                        (FieldLabel::Optional, FieldType::Enum(_)) => Ok(format!(
                            "{name}: 0i32.try_into(),\n",
                            name = field.native_name()
                        )),
                        (_, _) => Ok(format!(
                            "{name}: ::std::default::Default::default(),\n",
                            name = field.native_name(),
                        )),
                    }
                })),),
            ),
            "        \
        }}
    }}
}}\n",
            func(|output| print_msg_deserializable(output, msg)),
        )
            .write_into(&mut self.output)
    }

    fn handle_enum(&mut self, enume: &'c EnumDescriptor<'c>) -> crate::Result<()> {
        (
            format!(
                "\
#[derive(Debug, Clone)]
pub enum {name} {{\n",
                name = enume.native_bare_type_name()
            ),
            indent((iter(enume.values().map(|value| {
                Ok(format!(
                    "{name} = {value},\n",
                    name = value.native_name(),
                    value = value.number()
                ))
            })),)),
            "\
}}\n",
            format!(
                "\
impl std::convert::TryFrom<i32> for {name} {{
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {{
        match val {{\n",
                name = enume.native_bare_type_name()
            ),
            indent_n(
                3,
                (
                    iter(enume.values().map(|value| {
                        Ok(format!(
                            "{number} => Ok(Self::{name}),\n",
                            number = value.number(),
                            name = value.native_name(),
                        ))
                    })),
                    "x => Err(x),\n",
                ),
            ),
            "        \
        }}
    }}
}}\n",
            format!(
                "\
impl std::convert::Into<i32> for {name} {{
    fn into(self) -> i32 {{
        self as i32
    }}
}}\n",
                name = enume.native_bare_type_name()
            ),
        )
            .write_into(&mut self.output)
    }

    fn enter_submodule(&mut self, name: &str) -> crate::Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("pub mod {name} {{\n", name = mod_name))?;
        Ok(())
    }

    fn exit_submodule(&mut self, name: &str) -> crate::Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("}} // mod {name}\n", name = mod_name))?;
        Ok(())
    }
}
pub fn do_generate<'c>(context: &'c Context<'c>) -> Result<Vec<(String, String)>> {
    let mut filenames_and_contents = Vec::new();
    for file_desc in context.file_descriptors() {
        let file_name = file_desc.output_file_path_from_root().to_string();
        let mut visitor = Visitor {
            output: Indentor::new(String::new()),
        };
        file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
        filenames_and_contents.push((file_name.clone(), visitor.output.into_inner()));
    }
    Ok(filenames_and_contents)
}

pub fn print_msg_deserializable<'c>(
    output: &mut Indentor<String>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
impl<'a> {d}::MessageDeserializeEventHandler for &'a mut {name} {{
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {{
        Ok(())
    }}
    fn met_field<T: {d}::LengthDelimitedDeserializer>(
        &mut self,
        field: {d}::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {{
        match field {{\n",
            d = DESER_MOD,
            name = msg.native_bare_type_name(),
        ),
        indent_n(
            3,
            (
                func(|output| print_msg_deserializable_variant_arm(output, msg)),
                func(|output| print_msg_deserializable_length_delimited_arm(output, msg)),
                func(|output| print_msg_deserializable_bitsxx_arm(output, msg, 32)),
                func(|output| print_msg_deserializable_bitsxx_arm(output, msg, 64)),
                "_ => Err(::puroro::PuroroError::UnexpectedFieldType)?,\n",
            ),
        ),
        "        \
        }}
        Ok(())
    }}
}}\n",
    )
        .write_into(output)
}

pub fn print_msg_deserializable_variant_arm<'c>(
    output: &mut Indentor<String>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Variant(variant) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((iter(msg.fields().map(|field| -> Result<Fragment<_>> {
            Ok(
                match field
                    .type_()?
                    .native_tag_type_for_variant_types(msg.path_to_root_mod())
                {
                    Ok(tag) => seq((
                        format!("{number} => {{\n", number = field.number()),
                        indent((if field.is_repeated()? {
                            format!(
                                "self.{name}.push(variant.to_native::<{tag}>()?);\n",
                                name = field.native_name(),
                                tag = tag,
                            )
                        } else {
                            format!(
                                "self.{name} = variant.to_native::<{tag}>()?;\n",
                                name = field.native_name(),
                                tag = tag,
                            )
                        },)),
                        "}}\n",
                    ))
                    .into(),
                    Err(_) => format!(
                        "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                        number = field.number()
                    )
                    .into(),
                },
            )
        })),)),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_deserializable_length_delimited_arm<'c>(
    output: &mut Indentor<String>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "{d}::Field::LengthDelimited(ldd) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((
            iter(msg.fields().map(|field| -> Result<Fragment<_>> {
                Ok(seq((
                    format!("{number} => {{\n", number = field.number()),
                    indent(
                        match field
                            .type_()?
                            .native_tag_type_for_variant_types(msg.path_to_root_mod())
                        {
                            Ok(tag) => {
                                if field.is_repeated()? {
                                    format!(
                                        "\
self.{name}.append(&mut ldd.deserialize_as_variants().map(|rv| {{
    rv.and_then(|variant| variant.to_native::<{tag}>())
}}).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);\n",
                                        name = field.native_name(),
                                        tag = tag,
                                    )
                                } else {
                                    format!(
                                        "\
self.{name} = ldd.deserialize_as_variants()
    .last()
    .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
    .and_then(|variant| variant.to_native::<{tag}>())?;\n",
                                        name = field.native_name(),
                                        tag = tag,
                                    )
                                }
                            }
                            Err(NonvariantFieldType::String) => {
                                if field.is_repeated()? {
                                    format!(
                                        "self.{name}.push(ldd.deserialize_as_chars()\
                                        .collect::<::puroro::Result<_>>()?);\n",
                                        name = field.native_name()
                                    )
                                } else {
                                    format!(
                                        "self.{name} = ldd.deserialize_as_chars()\
                                        .collect::<::puroro::Result<_>>()?;\n",
                                        name = field.native_name()
                                    )
                                }
                            }
                            Err(NonvariantFieldType::Bytes) => {
                                if field.is_repeated()? {
                                    format!(
                                        "self.{name}.push(ldd.deserialize_as_bytes()\
                                        .collect::<::puroro::Result<_>>()?);\n",
                                        name = field.native_name()
                                    )
                                } else {
                                    format!(
                                        "self.{name} = ldd.deserialize_as_bytes()\
                                        .collect::<::puroro::Result<_>>()?;\n",
                                        name = field.native_name()
                                    )
                                }
                            }
                            Err(NonvariantFieldType::Message(_)) => {
                                if field.is_repeated()? {
                                    format!(
                                        "\
let mut msg = ::std::default::Default::default();
ldd.deserialize_as_message(&mut msg)?;
self.{name}.push(msg);",
                                        name = field.native_name()
                                    )
                                } else {
                                    format!(
                                        "\
let boxed_msg = self.{name}.get_or_insert_with(
    || ::std::boxed::Box::new(::std::default::Default::default()));
ldd.deserialize_as_message(boxed_msg.as_mut())?;",
                                        name = field.native_name()
                                    )
                                }
                            }
                            _ => "Err(::puroro::PuroroError::UnexpectedWireType)?\n".into(),
                        },
                    ),
                    "}}\n",
                )))
            })),
            "_ => Err(::puroro::PuroroError::UnexpectedFieldId)?,\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_deserializable_bitsxx_arm<'c>(
    output: &mut Indentor<String>,
    msg: &'c MessageDescriptor<'c>,
    bits: usize,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Bits{bits}(bytes) => match field_number {{\n",
            bits = bits,
            d = DESER_MOD
        ),
        indent((
            iter(msg.fields().map(|field| -> Result<Fragment<_>> {
                Ok(seq((
                    format!("{number} => {{\n", number = field.number()),
                    indent(match field.type_()?.native_type_for_bitsxx_types(bits) {
                        Ok(native_type) => {
                            if field.is_repeated()? {
                                format!(
                                    "self.{name}.push({type_}::from_le_bytes(bytes));\n",
                                    name = field.native_name(),
                                    type_ = native_type
                                )
                            } else {
                                format!(
                                    "self.{name} = {type_}::from_le_bytes(bytes);\n",
                                    name = field.native_name(),
                                    type_ = native_type
                                )
                            }
                        }

                        Err(_) => "Err(::puroro::PuroroError::UnexpectedWireType)?\n".into(),
                    }),
                    "}}\n",
                )))
            })),
            "_ => Err(::puroro::PuroroError::UnexpectedFieldId)?,\n",
        )),
        "}}\n",
    )
        .write_into(output)
}
