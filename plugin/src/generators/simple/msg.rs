use super::*;

pub(crate) fn handle_msg<'p, W: Write>(
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p, W>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    write_body(context, fc, msg)?;
    write_default(context, fc, msg)?;
    Ok(())
}

// struct body
fn write_body<'p, W: Write>(
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p, W>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    write!(fc.writer(), "pub struct {name} ", name = native_type_name)?;
    fc.indent_with_braces(|fc| {
        for field in &msg.field {
            let field_native_type = gen_field_type(field, context, fc)?;
            writeln!(
                fc.writer(),
                "{name}: {type_},",
                name = to_var_name(&field.name),
                type_ = field_native_type
            )?;
        }
        Ok(())
    })
}

// impl Default
fn write_default<'p, W: Write>(
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p, W>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    // Because enum is Result<enum, i32>, we need a special treatment for it.
    let native_type_name = to_type_name(&msg.name);
    write!(
        fc.writer(),
        "impl ::std::default::Default for {name} ",
        name = native_type_name
    )?;
    fc.indent_with_braces(|fc| {
        write!(fc.writer(), "fn default() -> Self ")?;
        fc.indent_with_braces(|fc| {
            write!(fc.writer(), "Self ")?;
            fc.indent_with_braces(|fc| {
                for field in &msg.field {
                    let native_field_name = to_var_name(&field.name);
                    if let Some(TypeOfIdent::Enum) =
                        context.type_of_ident(fc.package().clone(), &field.type_name)
                    {
                        writeln!(
                            fc.writer(),
                            "{name}: 0i32.try_into(),",
                            name = native_field_name
                        )?;
                    } else {
                        writeln!(
                            fc.writer(),
                            "{name}: std::default::Default::default(),",
                            name = native_field_name
                        )?;
                    }
                }
                Ok(())
            })
        })
    })
}
