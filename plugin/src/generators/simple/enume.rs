use super::super::shared::utils::{to_enum_value_name, to_type_name};
use super::*;

pub(crate) fn handle_enum<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    write_body(output, context, enume)?;
    write_tryfrom(output, context, enume)?;
    Ok(())
}

// enum body
fn write_body<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &Context<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&enume.name);

    // enum body
    (
        format!(
            "\
#[derive(Debug, Clone)]
pub enum {name} {{\n",
            name = native_type_name
        ),
        indent((iter(enume.value.iter().map(|value| {
            let name = to_enum_value_name(&value.name);
            Ok(format!(
                "{name} = {number},\n",
                name = name,
                number = value.number
            ))
        })),)),
        "}}\n",
    )
        .write_into(output)
}

// TryFrom<i32>
fn write_tryfrom<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &Context<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&enume.name);
    (
        format!(
            "impl std::convert::TryFrom<i32> for {name} {{\n",
            name = native_type_name
        ),
        indent((
            ("type Error = i32; \n\
                fn try_from(val: i32) -> std::result::Result<Self, i32> {{\n"),
            indent((
                "match val {{\n",
                indent((
                    iter(enume.value.iter().map(|value| {
                        let value_name = to_enum_value_name(&value.name);
                        Ok(format!(
                            "{number} => Ok(Self::{name}),\n",
                            number = value.number,
                            name = value_name
                        ))
                    })),
                    "x => Err(x),\n",
                )),
                "}}\n",
            )),
            "}}\n",
        )),
        "}}\n",
    )
        .write_into(output)
}
