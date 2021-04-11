use super::super::shared::utils::{to_type_name, to_var_name};
use super::*;

// Too long! lol
const DESER_MOD: &'static str = "::puroro_serializer::deserializer::stream";

pub fn handle_msg<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    write_body(output, context, msg)?;
    write_default(output, context, msg)?;
    write_deser_stream_handler(output, context, msg)?;
    write_deserializable(output, context, msg)?;
    write_ser_serializer(output, context, msg)?;
    write_puroro_serializable(output, context, msg)?;
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
        format!(
            "\
#[derive(Debug, Clone)]
pub struct {name} {{\n",
            name = native_type_name
        ),
        indent((iter(msg.field.iter().map(|field| {
            let field_native_type = gen_field_type(field, context)?;
            Ok(format!(
                "pub {name}: {type_},\n",
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
        #[allow(unused)]
        use ::std::convert::TryInto;
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

fn write_deser_stream_handler<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
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
            name = native_type_name
        ),
        indent_n(
            3,
            (
                func(|output| write_deser_stream_handler_variant_arm(output, context, msg)),
                func(|output| write_deser_stream_handler_ld_arm(output, context, msg)),
                func(|output| write_deser_stream_handler_bitsxx_arm(32, output, context, msg)),
                func(|output| write_deser_stream_handler_bitsxx_arm(64, output, context, msg)),
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
                Ok(match variant_field_type_tag(field) {
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
    #[allow(unused)]
    use ::std::convert::TryInto;
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
    #[allow(unused)]
    use ::std::convert::TryInto;
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
    (
        format!(
            "{d}::Field::LengthDelimited(ldd) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((
            iter(msg.field.iter().map(|field| {
                let native_field_name = to_var_name(&field.name);
                let is_repeated = is_field_repeated(field);
                Ok(if is_field_msg(field, context) {
                    // Message
                    let field_native_type = gen_field_bare_type(
                        field.type_.clone(),
                        &field.type_name,
                        context.path_to_package_root(),
                    )?;
                    if is_repeated {
                        format!(
                            "\
{number} => {{
    let mut msg = ::std::default::Default::default();
    ldd.deserialize_as_message(&mut msg)?;
    self.{name}.push(msg);
}}\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    } else {
                        format!(
                            "\
{number} => {{
    let boxed_msg = self.{name}.get_or_insert_with(
        || ::std::boxed::Box::new(::std::default::Default::default()));
    ldd.deserialize_as_message(boxed_msg.as_mut())?;
}}\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    }
                } else if let Some(tag_type) = variant_field_type_tag(field) {
                    // packed variant, maybe enum
                    let maybe_try_into = if is_field_enum(field, context) {
                        ".try_into()"
                    } else {
                        ""
                    };
                    if is_repeated {
                        format!(
                            "\
{number} => {{
    #[allow(unused)]
    use ::std::convert::TryInto;
    self.{name}.append(&mut ldd.deserialize_as_variants().map(|rv| {{
        rv.and_then(|variant| variant.to_native::<{tag}>()){maybe_try_into}
    }}).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
}}\n",
                            number = field.number,
                            name = native_field_name,
                            tag = tag_type,
                            maybe_try_into = maybe_try_into,
                        )
                    } else {
                        // a packed variant is coming for singular field...??
                        // It's soooo weird but I'm not sure if that is illegal...
                        format!(
                            "\
{number} => {{
    #[allow(unused)]
    use ::std::convert::TryInto;
    self.{name} = ldd.deserialize_as_variants()
        .last()
        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
        .and_then(|variant| variant.to_native::<{tag}>())?{maybe_try_into};
}}\n",
                            number = field.number,
                            name = native_field_name,
                            tag = tag_type,
                            maybe_try_into = maybe_try_into,
                        )
                    }
                } else {
                    match field.type_ {
                        Ok(field_descriptor_proto::Type::TypeString) => {
                            // string
                            if is_repeated {
                                format!(
                                    "\
{number} => {{
    self.{name}.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
}}\n",
                                    number = field.number,
                                    name = native_field_name,
                                )
                            } else {
                                format!(
                                    "\
{number} => {{
    self.{name} = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
}}\n",
                                    number = field.number,
                                    name = native_field_name,
                                )
                            }
                        }
                        Ok(field_descriptor_proto::Type::TypeBytes) => {
                            // bytes
                            if is_repeated {
                                format!(
                                    "\
{number} => {{
    self.{name}.push(ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?);
}}\n",
                                    number = field.number,
                                    name = native_field_name,
                                )
                            } else {
                                format!(
                                    "\
{number} => {{
    self.{name} = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;
}}\n",
                                    number = field.number,
                                    name = native_field_name,
                                )
                            }
                        }
                        _ => {
                            // else
                            format!(
                                "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                                number = field.number
                            )
                        }
                    }
                })
            })),
            "_ => todo!(\"Unknown filed number\"),\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

fn write_deser_stream_handler_bitsxx_arm<'p, W: Write>(
    bits: usize,
    output: &mut Indentor<W>,
    _context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Bits{bits}(bytes) => match field_number {{\n",
            bits = bits,
            d = DESER_MOD
        ),
        indent((
            iter(msg.field.iter().map(|field| {
                let opt_native_type = if bits == 32 {
                    bits32_field_native_type(field)
                } else {
                    bits64_field_native_type(field)
                };
                Ok(if let Some(native_type) = opt_native_type {
                    let native_field_name = to_var_name(&field.name);
                    if is_field_repeated(field) {
                        format!(
                            "\
{number} => {{
    self.{name}.push({type_}::from_le_bytes(bytes));
}}\n",
                            number = field.number,
                            name = native_field_name,
                            type_ = native_type
                        )
                    } else {
                        format!(
                            "\
{number} => {{
    self.{name} = {type_}::from_le_bytes(bytes);
}}\n",
                            number = field.number,
                            name = native_field_name,
                            type_ = native_type
                        )
                    }
                } else {
                    format!(
                        "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                        number = field.number
                    )
                })
            })),
            "_ => todo!(\"Unknown filed number\"),\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

fn variant_field_type_tag(field: &FieldDescriptorProto) -> Option<&'static str> {
    if let Ok(t) = &field.type_ {
        match t {
            field_descriptor_proto::Type::TypeInt64 => Some("::puroro::tags::Int64"),
            field_descriptor_proto::Type::TypeSint64 => Some("::puroro::tags::SInt64"),
            field_descriptor_proto::Type::TypeUint64 => Some("::puroro::tags::UInt64"),
            field_descriptor_proto::Type::TypeInt32 => Some("::puroro::tags::Int32"),
            field_descriptor_proto::Type::TypeSint32 => Some("::puroro::tags::SInt32"),
            field_descriptor_proto::Type::TypeEnum => Some("::puroro::tags::Int32"),
            field_descriptor_proto::Type::TypeBool => Some("::puroro::tags::Bool"),
            field_descriptor_proto::Type::TypeUint32 => Some("::puroro::tags::UInt32"),
            _ => None,
        }
    } else {
        None
    }
}

fn bits32_field_native_type(field: &FieldDescriptorProto) -> Option<&'static str> {
    if let Ok(t) = &field.type_ {
        match t {
            field_descriptor_proto::Type::TypeFloat => Some("f32"),
            field_descriptor_proto::Type::TypeFixed32 => Some("u32"),
            field_descriptor_proto::Type::TypeSfixed32 => Some("i32"),
            _ => None,
        }
    } else {
        None
    }
}

fn bits64_field_native_type(field: &FieldDescriptorProto) -> Option<&'static str> {
    if let Ok(t) = &field.type_ {
        match t {
            field_descriptor_proto::Type::TypeDouble => Some("f64"),
            field_descriptor_proto::Type::TypeFixed64 => Some("u64"),
            field_descriptor_proto::Type::TypeSfixed64 => Some("i64"),
            _ => None,
        }
    } else {
        None
    }
}

// Deserializable trait
fn write_deserializable<'p, W: Write>(
    output: &mut Indentor<W>,
    _context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (format!(
        "\
impl ::puroro::Deserializable for {name} {{
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {{
        use {d}::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        {d}::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }}
}}\n",
        name = native_type_name,
        d = DESER_MOD,
    ),)
        .write_into(output)
}

// ::puroro_serialize::Seriazilazble trait
fn write_ser_serializer<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (
        format!(
            "\
impl ::puroro_serializer::serializer::Serializable for {name} {{
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{\n",
            name = native_type_name
        ),
        indent_n(
            2,
            (iter(msg.field.iter().map(|field| {
                let native_field_name = to_var_name(&field.name);
                let is_repeated = is_field_repeated(field);
                Ok(if is_field_enum(field, context) {
                    // Enum
                    if is_repeated {
                        format!(
                            "\
serializer.serialize_variants_twice::<::puroro::tags::Int32>(
    {number},
    self.{name}.iter().map(|rv| Ok(rv.map_or_else(|e| e, |v| v as i32)))
)?;\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    } else {
                        format!(
                            "\
serializer.serialize_variant::<::puroro::tags::Int32>(
    {number}, 
    self.{name}.clone().map_or_else(|e| e, |v| v as i32)
)?;\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    }
                } else if is_field_msg(field, context) {
                    // Message
                    let field_native_type = gen_field_bare_type(
                        field.type_.clone(),
                        &field.type_name,
                        context.path_to_package_root(),
                    )?;
                    if is_repeated {
                        format!(
                            "\
for msg in &self.{name} {{
    serializer.serialize_message_twice::<{type_}>({number}, msg)?;
}}\n",
                            number = field.number,
                            name = native_field_name,
                            type_ = field_native_type,
                        )
                    } else {
                        format!(
                            "\
if let Some(msg) = &self.{name} {{
    serializer.serialize_message_twice::<{type_}>({number}, msg)?;
}}\n",
                            number = field.number,
                            name = native_field_name,
                            type_ = field_native_type,
                        )
                    }
                } else if let Ok(field_descriptor_proto::Type::TypeString) = field.type_ {
                    // String
                    if is_repeated {
                        format!(
                            "\
for string in &self.{name} {{
    serializer.serialize_bytes_twice({number}, string.bytes().map(|b| Ok(b)))?;
}}\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    } else {
                        format!(
                            "\
serializer.serialize_bytes_twice({number}, self.{name}.bytes().map(|b| Ok(b)))?;\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    }
                } else if let Ok(field_descriptor_proto::Type::TypeBytes) = field.type_ {
                    // Bytes
                    if is_repeated {
                        format!(
                            "\
for bytes in &self.{name} {{
    serializer.serialize_bytes_twice({number}, bytes.iter().map(|b| Ok(*b)))?;
}}\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    } else {
                        format!(
                            "\
serializer.serialize_bytes_twice({number}, self.{name}.iter().map(|b| Ok(*b)))?;\n",
                            number = field.number,
                            name = native_field_name,
                        )
                    }
                } else if let Some(_) = bits32_field_native_type(field) {
                    "".to_string() //format!("unimplemented!(\"Serializer for bits32\");\n")
                } else if let Some(_) = bits64_field_native_type(field) {
                    "".to_string() //format!("unimplemented!(\"Serializer for bits64\");\n")
                } else {
                    "".to_string() //format!("unimplemented!(\"Serializer for something else\");\n")
                })
            })),),
        ),
        "        \
        Ok(())
    }}
}}\n",
    )
        .write_into(output)
}

// ::puroro::Serializable
fn write_puroro_serializable<'p, W: Write>(
    output: &mut Indentor<W>,
    context: &Context<'p>,
    msg: &'p DescriptorProto,
) -> Result<()> {
    let native_type_name = to_type_name(&msg.name);
    (format!(
        "\
impl ::puroro::Serializable for {name} {{
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {{
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }}
}}\n",
        name = native_type_name
    ),)
        .write_into(output)
}
