use super::writer::{func, indent, indent_n, iter, seq, Fragment, TupleOfIntoFragments};
use crate::utils::Indentor;
use crate::wrappers::{
    FieldLabel, FieldType, LengthDelimitedFieldType, MessageDescriptor, WireType,
};
use crate::Result;

const DESER_MOD: &'static str = "::puroro::deserializer::stream";

pub fn print_msg<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        func(|output| print_msg_struct(output, msg)),
        func(|output| print_msg_default(output, msg)),
        func(|output| print_msg_deser_deserializable(output, msg)),
        func(|output| print_msg_puroro_deserializable(output, msg)),
        func(|output| print_msg_ser_serializable(output, msg)),
        func(|output| print_msg_puroro_serializable(output, msg)),
    )
        .write_into(output)
}

pub fn print_msg_struct<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
#[derive(Debug, Clone)]
pub struct {name} {{\n",
            name = msg.native_bare_type_name(),
        ),
        indent((iter(msg.fields().map(|field| {
            Ok(format!(
                "pub {name}: {type_},\n",
                name = field.native_name(),
                type_ = field.native_owned_type_name()?,
            ))
        })),)),
        "\
}}\n",
    )
        .write_into(output)
}

pub fn print_msg_default<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
impl ::std::default::Default for {name} {{
    fn default() -> Self {{
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {{\n",
            name = msg.native_bare_type_name(),
        ),
        indent_n(
            3,
            (iter(msg.fields().map(|field| {
                match (field.label()?, field.type_()?) {
                    (FieldLabel::Optional, FieldType::Enum(_))
                    | (FieldLabel::Required, FieldType::Enum(_)) => Ok(format!(
                        "{name}: 0i32.try_into(),\n",
                        name = field.native_name()
                    )),
                    (_, _) => Ok(format!(
                        "{name}: ::std::default::Default::default(),\n",
                        name = field.native_name(),
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

pub fn print_msg_deser_deserializable<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
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
            name = msg.native_bare_type_name(),
        ),
        indent_n(
            3,
            (
                func(|output| print_msg_deser_deserializable_variant_arm(output, msg)),
                func(|output| print_msg_deser_deserializable_length_delimited_arm(output, msg)),
                func(|output| print_msg_deser_deserializable_bitsxx_arm(output, msg, 32)),
                func(|output| print_msg_deser_deserializable_bitsxx_arm(output, msg, 64)),
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

pub fn print_msg_deser_deserializable_variant_arm<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Variant(variant) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((iter(msg.fields().map(|field| -> Result<Fragment<_>> {
            Ok(match field.wire_type()? {
                WireType::Variant(field_type) => seq((
                    format!("{number} => {{\n", number = field.number()),
                    indent(format!(
                        "*::puroro::helpers::MaybeRepeatedField::last_mut(\
                            &mut self.{name}) = variant.to_native::<{tag}>()?;\n",
                        name = field.native_name(),
                        tag = field_type.native_tag_type(msg.path_to_root_mod()),
                    )),
                    "}}\n",
                ))
                .into(),
                _ => format!(
                    "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                    number = field.number()
                )
                .into(),
            })
        })),)),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_deser_deserializable_length_delimited_arm<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "{d}::Field::LengthDelimited(ldd) => match field_number {{\n",
            d = DESER_MOD
        ),
        indent((
            iter(msg.fields().map(|field| -> Result<Fragment<_>> {
                Ok(seq((
                    format!("{number} => {{\n", number = field.number()),
                    indent((match field.wire_type()? {
                        // Deserialize packed variant(s)
                        WireType::Variant(field_type) => format!(
                            "\
let values = ldd.deserialize_as_variants().map(|rv| {{
    rv.and_then(|variant| variant.to_native::<{tag}>())
}}).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
::puroro::helpers::MaybeRepeatedVariantField::extend(
    &mut self.{name}, values.into_iter());\n",
                            name = field.native_name(),
                            tag = field_type.native_tag_type(msg.path_to_root_mod()),
                        ),
                        WireType::LengthDelimited(field_type) => match field_type {
                            LengthDelimitedFieldType::String => format!(
                                "\
*::puroro::helpers::MaybeRepeatedField::last_mut(&mut self.{name}) 
    = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;\n",
                                name = field.native_name()
                            ),
                            LengthDelimitedFieldType::Bytes => format!(
                                "\
*::puroro::helpers::MaybeRepeatedField::last_mut(&mut self.{name}) 
    = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;\n",
                                name = field.native_name()
                            ),
                            LengthDelimitedFieldType::Message(_) => format!(
                                "\
let msg = ::puroro::helpers::MaybeRepeatedField::last_mut(&mut self.{name});
ldd.deserialize_as_message(msg)?;\n",
                                name = field.native_name()
                            ),
                        },
                        _ => "Err(::puroro::PuroroError::UnexpectedWireType)?\n".into(),
                    },)),
                    "}}\n",
                )))
            })),
            "_ => Err(::puroro::PuroroError::UnexpectedFieldId)?,\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_deser_deserializable_bitsxx_arm<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
    bits: usize,
) -> Result<()> {
    (
        format!(
            "{d}::Field::Bits{bits}(bytes) => match field_number {{\n",
            bits = bits,
            d = DESER_MOD
        ),
        indent((
            iter(msg.fields().map(|field| -> Result<Fragment<_>> {
                Ok(seq((
                    format!("{number} => {{\n", number = field.number()),
                    indent(({
                        let opt_native_type = match (bits, field.wire_type()?) {
                            (32, WireType::Bits32(field_type)) => Some(field_type.native_type()),
                            (64, WireType::Bits64(field_type)) => Some(field_type.native_type()),
                            _ => None,
                        };
                        if let Some(native_type) = opt_native_type {
                            if field.is_repeated()? {
                                format!(
                                    "self.{name}.push({type_}::from_le_bytes(bytes));\n",
                                    name = field.native_name(),
                                    type_ = native_type
                                )
                            } else {
                                format!(
                                    "self.{name} = {type_}::from_le_bytes(bytes);\n",
                                    name = field.native_name(),
                                    type_ = native_type
                                )
                            }
                        } else {
                            "Err(::puroro::PuroroError::UnexpectedWireType)?\n".into()
                        }
                    },)),
                    "}}\n",
                )))
            })),
            "_ => Err(::puroro::PuroroError::UnexpectedFieldId)?,\n",
        )),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_puroro_deserializable<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (format!(
        "\
impl ::puroro::Deserializable for {name} {{
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {{
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(<{name} as ::std::default::Default>::default())
    }}
}}\n",
        name = msg.native_bare_type_name()
    ),)
        .write_into(output)
}

pub fn print_msg_ser_serializable<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
impl ::puroro::serializer::Serializable for {name} {{
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{\n",
            name = msg.native_bare_type_name()
        ),
        indent_n(
            2,
            (iter(msg.fields().map(|field| -> Result<_> {
                Ok(match field.wire_type()? {
                    WireType::Variant(field_type) => format!(
                        "\
serializer.serialize_variants_twice::<{tag}>(
    {number}, 
    ::puroro::helpers::MaybeRepeatedField::iter(&self.{name}))?;\n",
                        number = field.number(),
                        tag = field_type.native_tag_type(msg.path_to_root_mod()),
                        name = field.native_name(),
                    ),

                    WireType::LengthDelimited(field_type) => match field_type {
                        LengthDelimitedFieldType::String => format!(
                            "\
for string in ::puroro::MaybeRepeatedField::iter(&self.{name}) {{
    serializer.serialize_bytes_twice({number}, string.bytes().map(|b| Ok(b)))?;
}}\n",
                            name = field.native_name(),
                            number = field.number()
                        ),
                        LengthDelimitedFieldType::Bytes => format!(
                            "\
for bytes in ::puroro::MaybeRepeatedField::iter(&self.{name}) {{
    serializer.serialize_bytes_twice({number}, bytes.iter().map(|b| Ok(b)))?;
}}\n",
                            name = field.native_name(),
                            number = field.number()
                        ),
                        LengthDelimitedFieldType::Message(_) => format!(
                            "\
for msg in ::puroro::MaybeRepeatedField::iter(&self.{name}) {{
    serializer.serialize_message_twice({number}, msg)?;
}}\n",
                            number = field.number(),
                            name = field.native_name(),
                        ),
                    },
                    WireType::Bits32(_) | WireType::Bits64(_) => format!(
                        "\
for item in ::puroro::MaybeRepeatedField::iter(&self.{name}) {{
    serializer.serialize_fixed_bits({number}, &item.to_le_bytes())?;
}}\n",
                        name = field.native_name(),
                        number = field.number(),
                    ),
                })
            })),),
        ),
        "    \
    }}
}}\n",
    )
        .write_into(output)
}

pub fn print_msg_puroro_serializable<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (format!(
        "\
impl ::puroro::Serializable for {name} {{
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {{
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serialize::Serializable>::serialize(&mut serializer)
    }}
}}\n",
        name = msg.native_bare_type_name()
    ),)
        .write_into(output)
}
