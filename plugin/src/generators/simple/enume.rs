use super::*;

pub(crate) fn handle_enum<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    write_body(output, context, fc, enume)?;
    write_tryfrom(output, context, fc, enume)?;
    Ok(())
}

// enum body
fn write_body<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&enume.name);

    // enum body
    write(
        output,
        (
            format!("pub enum {name} {{\n", name = native_type_name),
            indent((iter(enume.value.iter().map(|value| {
                let name = to_enum_value_name(&value.name);
                format!("{name} = {number},\n", name = name, number = value.number)
            })),)),
            "}}\n",
        ),
    )
}

// TryFrom<i32>
fn write_tryfrom<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&enume.name);
    write(
        output,
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
                            format!(
                                "{number} => Ok(Self::{name}),\n",
                                number = value.number,
                                name = value_name
                            )
                        })),
                        "x => Err(x),\n",
                    )),
                    "}}\n",
                )),
                "}}\n",
            )),
            "}}\n",
        ),
    )
}
