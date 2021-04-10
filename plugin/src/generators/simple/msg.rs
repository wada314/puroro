use super::*;

// Too long! lol
const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";

pub(crate) fn handle_msg<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    write_body(output, context, msg)?;
    write_default(output, context, msg)?;
    write_deser_stream_handler2(output, context, msg)?;
    Ok(())
}

// struct body
fn write_body<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (
        format!("pub struct {name} {{\n", name = native_type_name),
        indent((iter(msg.field.iter().map(|field| {
            let field_native_type = gen_field_type(field, context)?;
            Ok(format!(
                "{name}: {type_},\n",
                name = to_var_name(&field.name),
                type_ = field_native_type
            ))
        })),)),
        "}}\n",
    )
        .write_into(output)
}

// impl Default
// Because enum is Result<enum, i32>, we need a special treatment for it.
fn write_default<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (
        format!(
            "\
impl ::std::default::Default for {name} {{
    fn default() -> Self {{
        Self {{\n",
            name = native_type_name
        ),
        indent_n(
            3,
            (iter(msg.field.iter().map(|field| {
                let native_field_name = to_var_name(&field.name);
                let is_repeated = is_field_repeated(field);
                let is_enum = is_field_enum(field, context);

                match (is_repeated, is_enum) {
                    (false, true) => Ok(format!(
                        "{name}: 0i32.try_into(),\n",
                        name = native_field_name
                    )),
                    (_, _) => Ok(format!(
                        "{name}: ::std::default::Default::default(),\n",
                        name = native_field_name
                    )),
                }
            })),),
        ),
        "        \
        }}
    }}
}}\n",
    )
        .write_into(output)
}

fn write_deser_stream_handler2<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (
        format!(
            "\
impl {d}::MessageDeserializeEventHandler for {name} {{
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> {{
        Ok(self)
    }}
    fn met_field<T: {d}::LengthDelimitedDeserializer>(
        &mut self,
        field: {d}::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {{
        match field {{\n",
            d = DESER_MOD,
            name = native_type_name
        ),
        indent_n(
            3,
            (
                func(|output| write_deser_stream_handler_variant_arm(output, context, msg)),
                "_ => Err(::puroro::PuroroError::UnexpectedFieldType)?,\n",
            ),
        ),
        "       \
        }}
    }}
}}\n",
    )
        .write_into(output)
}

fn write_deser_stream_handler_variant_arm<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Variant(variant) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((
            iter(msg.field.iter().map(|field| {
                Ok(match variant_field_type(field) {
                    None => {
                        // This is not a variant field, so the output's match-case should fail.
                        format!(
                            "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                            number = field.number
                        )
                    }
                    Some(tag_type) => {
                        let is_enum = is_field_enum(field, context);
                        let is_repeated = is_field_repeated(field);
                        let maybe_try_into = if is_enum { ".try_into()" } else { "" };
                        if is_repeated {
                            format!(
                                "\
{number} => {{
    self.{name}.push(variant.to_native::<{tag}>()?{maybe_try_into});
}}\n",
                                number = field.number,
                                name = to_var_name(&field.name),
                                tag = tag_type,
                                maybe_try_into = maybe_try_into,
                            )
                        } else {
                            format!(
                                "\
{number} => {{
    self.{name} = variant.to_native::<{tag}>()?{maybe_try_into};
}}\n",
                                number = field.number,
                                name = to_var_name(&field.name),
                                tag = tag_type,
                                maybe_try_into = maybe_try_into,
                            )
                        }
                    }
                })
            })),
            "_ => todo!(\"Unknown field number\"),\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

fn write_deser_stream_handler_ld_arm<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

/*
fn write_deser_stream_handler_variant<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: & FileGeneratorContext<'p>,
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
    fc: & FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

fn write_deser_stream_handler_bytes32<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: & FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}

fn write_deser_stream_handler_bytes64<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &InvocationContext,
    fc: & FileGeneratorContext<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    todo!()
}
*/
fn variant_field_type(field: &FieldDescriptorProto) -> Option<&'static str> {
    if let Ok(t) = field.type_ {
        match t {
            FieldDescriptorProto_Type::TYPE_INT64 => Some("::puroro::tags::Int64"),
            FieldDescriptorProto_Type::TYPE_SINT64 => Some("::puroro::tags::SInt64"),
            FieldDescriptorProto_Type::TYPE_UINT64 => Some("::puroro::tags::UInt64"),
            FieldDescriptorProto_Type::TYPE_INT32 => Some("::puroro::tags::Int32"),
            FieldDescriptorProto_Type::TYPE_SINT32 => Some("::puroro::tags::SInt32"),
            FieldDescriptorProto_Type::TYPE_ENUM => Some("::puroro::tags::Int32"),
            FieldDescriptorProto_Type::TYPE_BOOL => Some("::puroro::tags::Bool"),
            FieldDescriptorProto_Type::TYPE_UINT32 => Some("::puroro::tags::UInt32"),
            _ => None,
        }
    } else {
        None
    }
}
