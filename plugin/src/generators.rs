pub mod shared;
//pub mod simple;

// Too long!
const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";

use std::fmt::Write;

use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
use crate::wrappers::{
    DescriptorVisitor, EnumDescriptor, FieldLabel, FieldType, MessageDescriptor,
};
use crate::Result;
use shared::writers::{indent, indent_n, iter, TupleOfIntoFragments};

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
                name = msg.native_bare_typename(),
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
                name = msg.native_bare_typename(),
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
        )
            .write_into(&mut self.output)
    }

    fn handle_enum(&mut self, enume: &'c EnumDescriptor<'c>) -> crate::Result<()> {
        (
            format!(
                "\
#[derive(Debug, Clone)]
pub enum {name} {{\n",
                name = enume.native_bare_typename()
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
                name = enume.native_bare_typename()
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
        )
            .write_into(&mut self.output)
    }

    fn enter_submodule(&mut self, name: &str) -> crate::Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("pub mod {name} {{\n", name = mod_name))?;
        Ok(())
    }

    fn exit_submodule(&mut self, _name: &str) -> crate::Result<()> {
        self.output.write_str("}\n")?;
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
    context: &'c Context<'c>,
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
            name = msg.native_bare_typename(),
        ),
        indent_n(
            3,
            (
                format!(
                    "{d}::Field::Variant(variant) => match field_number {{\n",
                    d = DESER_MOD
                ),
                indent((iter(msg.fields().map(|field| -> Result<_> {
                    match field.type_()? {
                        FieldType::Double => {}
                        FieldType::Float => {}
                        FieldType::Int32 => {}
                        FieldType::Int64 => {}
                        FieldType::UInt32 => {}
                        FieldType::UInt64 => {}
                        FieldType::SInt32 => {}
                        FieldType::SInt64 => {}
                        FieldType::Fixed32 => {}
                        FieldType::Fixed64 => {}
                        FieldType::SFixed32 => {}
                        FieldType::SFixed64 => {}
                        FieldType::Bool => {}
                        FieldType::Group => {}
                        FieldType::String => {}
                        FieldType::Bytes => {}
                        FieldType::Enum(_) => {}
                        FieldType::Message(_) => {}
                    };
                    Ok("")
                })),)),
                //func(|output| write_deser_stream_handler_variant_arm(output, context, msg)),
                //func(|output| write_deser_stream_handler_ld_arm(output, context, msg)),
                //func(|output| write_deser_stream_handler_bitsxx_arm(32, output, context, msg)),
                //func(|output| write_deser_stream_handler_bitsxx_arm(64, output, context, msg)),
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
