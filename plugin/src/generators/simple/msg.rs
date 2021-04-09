use super::*;

pub(crate) fn handle_msg<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    write_body(output, context, fc, msg)?;
    /*
    write_default(output, context, fc, msg)?;
    write_deser_stream_handler(output, context, fc, msg)?;*/
    Ok(())
}

// struct body
fn write_body<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    write(
        output,
        (
            format!("pub struct {name} {{\n", name = native_type_name),
            indent((iter(msg.field.iter().map(|field| {
                let field_native_type = gen_field_type(field, context, fc).unwrap();
                format!(
                    "{name}: {type_},\n",
                    name = to_var_name(&field.name),
                    type_ = field_native_type
                )
            })),)),
            "}}\n",
        ),
    )
}
/*
// impl Default
// Because enum is Result<enum, i32>, we need a special treatment for it.
fn write_default<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
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

fn write_deser_stream_handler2<W: Write>(
    context: &InvocationContext,
    out: &mut Indentor<W>,
    msg: &DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";
    write(
        out,
        fr([
            fr(format_args!(
                "impl {d}::MessageDeserializeEventHandler for {name} {{\n",
                d = DESER_MOD,
                name = native_type_name
            )),
            indent([
                fr(format_args!(
                    "\
type Target = Self;
fn finish(self) -> ::puroro::Result<Self::Target> {{ Ok(self) }}
fn met_field<T: {d}::LengthDelimitedDeserializer>(
    &mut self,
    field: {d}::Field<T>,
    field_number: usize,
) -> ::puroro::Result<()> {{\n",
                    d = DESER_MOD
                )),
                indent([
                    fr("match field {{\n"),
                    indent([fr("{d}::Field::Variant(variant) => ")]),
                    fr("}}\n"),
                ]),
                fr("}}\n"),
            ]),
            fr("}}\n"),
        ]),
    )
}

fn write_deser_stream_handler<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";
    write!(
        fc.writer(),
        "impl {d}::MessageDeserializeEventHandler for {name} ",
        d = DESER_MOD,
        name = native_type_name
    )?;
    fc.indent_with_braces(|fc| {
        write!(
            fc.writer(),
            "\
type Target = Self;
fn finish(self) -> ::puroro::Result<Self::Target> {{ Ok(self) }}
fn met_field<T: {d}::LengthDelimitedDeserializer>(
    &mut self,
    field: {d}::Field<T>,
    field_number: usize,
) -> ::puroro::Result<()> ",
            d = DESER_MOD
        )?;
        fc.indent_with_braces(|fc| {
            write!(fc.writer(), "match field ")?;
            fc.indent_with_braces(|fc| {
                write!(
                    fc.writer(),
                    "{d}::Field::Variant(variant) => ", // Go to the next episode...
                    d = DESER_MOD
                )?;
                write_deser_stream_handler_variant(context, fc, msg)?;
                write!(
                    fc.writer(),
                    "{d}::Field::LengthDelimeted(ldd) => ",
                    d = DESER_MOD
                )?;
                write_deser_stream_handler_length_delimited(context, fc, msg)?;
                write!(fc.writer(), "{d}::Field::Bytes32(array) => ", d = DESER_MOD)?;
                write_deser_stream_handler_bytes32(context, fc, msg)?;
                write!(fc.writer(), "{d}::Field::Bytes64(array) => ", d = DESER_MOD)?;
                write_deser_stream_handler_bytes64(context, fc, msg)?;
                Ok(())
            })?;
            writeln!(fc.writer(), "Ok(())")?;
            Ok(())
        })
    })
}

fn write_deser_stream_handler_variant<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    write!(fc.writer(), "match field_number ")?;
    fc.indent_with_braces(|fc| {
        for field in &msg.field {
            if let Some(tag_type) = variant_field_type(field) {
                let is_enum = matches!(
                    context.type_of_ident(fc.package().clone(), &field.type_name),
                    Some(TypeOfIdent::Enum)
                );
                write!(fc.writer(), "{number} => ", number = field.number)?;
                fc.indent_with_braces(|fc| {
                    let maybe_try_into = if is_enum { ".try_into()" } else { "" };
                    if let Ok(FieldDescriptorProto_Label::LABEL_REPEATED) = field.label {
                        writeln!(
                            fc.writer(),
                            "self.{name}.push(\
                                variant.to_native::<::puroro::tags::{tag}>()?\
                                {maybe_try_into});",
                            name = to_var_name(&field.name),
                            tag = tag_type,
                            maybe_try_into = maybe_try_into,
                        )?;
                    } else {
                        writeln!(
                            fc.writer(),
                            "self.{name} = \
                                variant.to_native::<::puroro::tags::{tag}>()?\
                                {maybe_try_into};",
                            name = to_var_name(&field.name),
                            tag = tag_type,
                            maybe_try_into = maybe_try_into,
                        )?;
                    }
                    Ok(())
                })?;
            }
        }
        writeln!(
            fc.writer(),
            "_ => Err(::puroro::PuroroError::UnexpectedWireType)?"
        )?;
        Ok(())
    })
}

fn write_deser_stream_handler_length_delimited<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

fn write_deser_stream_handler_bytes32<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

fn write_deser_stream_handler_bytes64<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: &mut FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

fn variant_field_type(field: &FieldDescriptorProto) -> Option<&'static str> {
    if let Ok(t) = field.type_ {
        match t {
            FieldDescriptorProto_Type::TYPE_INT64 => Some("Int64"),
            FieldDescriptorProto_Type::TYPE_SINT64 => Some("SInt64"),
            FieldDescriptorProto_Type::TYPE_UINT64 => Some("UInt64"),
            FieldDescriptorProto_Type::TYPE_INT32 => Some("Int32"),
            FieldDescriptorProto_Type::TYPE_SINT32 => Some("SInt32"),
            FieldDescriptorProto_Type::TYPE_ENUM => Some("Int32"),
            FieldDescriptorProto_Type::TYPE_BOOL => Some("Bool"),
            FieldDescriptorProto_Type::TYPE_UINT32 => Some("UInt32"),
            _ => None,
        }
    } else {
        None
    }
}
*/
