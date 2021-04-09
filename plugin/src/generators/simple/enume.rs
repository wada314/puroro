use super::*;

pub(crate) fn handle_enum<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    write_body(output, context, fc, enume)?;
    //write_tryfrom(output, context, fc, enume)?;
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
/*
// TryFrom<i32>
fn write_tryfrom<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    enume: &'p EnumDescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&enume.name);
    write!(
        fc.writer(),
        "impl std::convert::TryFrom<i32> for {name} ",
        name = native_type_name
    )?;
    fc.indent_with_braces(|fc| {
        write!(
            fc.writer(),
            "type Error = i32; \
             fn try_from(val: i32) -> std::result::Result<Self, i32> "
        )?;
        fc.indent_with_braces(|fc| {
            write!(fc.writer(), "match val ")?;
            fc.indent_with_braces(|fc| {
                for value in &enume.value {
                    let value_name = to_enum_value_name(&value.name);
                    writeln!(
                        fc.writer(),
                        "{number} => Ok(Self::{name}),",
                        number = value.number,
                        name = value_name
                    )?;
                }
                writeln!(fc.writer(), "x => Err(x),")?;
                Ok(())
            })
        })
    })
}
*/
