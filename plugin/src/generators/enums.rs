use super::writer::{indent, indent_n, iter, IntoFragment};
use crate::utils::Indentor;
use crate::wrappers::EnumDescriptor;
use crate::Result;

pub fn print_enum<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    enume: &'c EnumDescriptor<'c>,
) -> Result<()> {
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
impl ::std::convert::TryFrom<i32> for {name} {{
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
impl ::std::convert::Into<i32> for {name} {{
    fn into(self) -> i32 {{
        self as i32
    }}
}}\n",
            name = enume.native_bare_type_name()
        ),
    )
        .write_into(output)
}
