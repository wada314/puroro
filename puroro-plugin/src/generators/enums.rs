use super::writer::{func, indent, indent_n, iter, IntoFragment};
use crate::utils::Indentor;
use crate::wrappers::EnumDescriptor;
use crate::Result;

pub fn print_enum<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
    (
        func(|output| print_enum_body(output, enume)),
        func(|output| print_enum_try_from(output, enume)),
        func(|output| print_enum_from(output, enume)),
        func(|output| print_enum_field_new(output, enume)),
    )
        .write_into(output)
}

fn print_enum_body<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
#[derive(Debug, Clone)]
pub enum {name} {{\n",
            name = enume.native_ident()?
        ),
        indent((iter(enume.values().map(|value| {
            Ok(format!(
                "{name} = {value},\n",
                name = value.native_name()?,
                value = value.number()
            ))
        })),)),
        "\
}}\n",
    )
        .write_into(output)
}

fn print_enum_try_from<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
impl ::std::convert::TryFrom<i32> for {name} {{
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {{
        match val {{\n",
            name = enume.native_ident()?
        ),
        indent_n(
            3,
            (
                iter(enume.values().map(|value| {
                    Ok(format!(
                        "{number} => Ok(Self::{name}),\n",
                        number = value.number(),
                        name = value.native_name()?,
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
        .write_into(output)
}

fn print_enum_from<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
    // Possibly failing, not good...
    (format!(
        "\
impl ::std::convert::From<{name}> for i32 {{
    fn from(val: {name}) -> i32 {{
        val as i32
    }}
}}\n",
        name = enume.native_ident()?
    ),)
        .write_into(output)
}

fn print_enum_field_new<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
    (format!(
        "\
impl<'bump> ::puroro_internal::FieldNew<'bump> for {ident} {{
    fn new() -> Self {{
        Self::{value_ident}
    }}
}}\n",
        ident = enume.native_ident()?,
        value_ident = enume.first_value()?.native_name()?,
    ),)
        .write_into(output)
}
