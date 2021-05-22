use super::writer::{indent, indent_n, iter, IntoFragment};
use crate::utils::Indentor;
use crate::wrappers::EnumDescriptor;
use crate::Result;

pub fn print_enum<'proto, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'proto EnumDescriptor<'proto>,
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
        format!(
            "\
impl ::std::convert::From<{name}> for i32 {{
    fn from(val: {name}) -> i32 {{
        val as i32
    }}
}}\n",
            name = enume.native_ident()?
        ),
    )
        .write_into(output)
}
