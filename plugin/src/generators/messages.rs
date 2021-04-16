use super::writer::{func, indent, indent_n, iter, Fragment, IntoFragment};
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
        (
            func(|output| print_msg_deser_deserializable(output, msg)),
            func(|output| print_msg_puroro_deserializable(output, msg)),
            func(|output| print_msg_ser_serializable(output, msg)),
            func(|output| print_msg_puroro_serializable(output, msg)),
        ),
        func(|output| print_msg_trait(output, msg)),
        func(|output| print_msg_trait_impl(output, msg)),
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
pub struct {name}<
    #[cfg(feature = \"puroro-nightly\")] A: ::std::alloc::Allocator = ::std::alloc::Global
> {{\n",
            name = msg.native_bare_type_name(),
        ),
        indent(iter(msg.fields().map(|field| {
            Ok(format!(
                "pub {name}: {type_},\n",
                name = field.native_name(),
                type_ = field.native_owned_type_name()?,
            ))
        }))),
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
        use ::std::convert::TryInto;
        Self {{\n",
            name = msg.native_bare_type_name(),
        ),
        indent_n(
            3,
            iter(
                msg.fields()
                    .map(|field| match (field.label()?, field.type_()?) {
                        (FieldLabel::Optional, FieldType::Enum(_))
                        | (FieldLabel::Required, FieldType::Enum(_)) => Ok(format!(
                            "{name}: 0i32.try_into(),\n",
                            name = field.native_name()
                        )),
                        (_, _) => Ok(format!(
                            "{name}: ::std::default::Default::default(),\n",
                            name = field.native_name(),
                        )),
                    }),
            ),
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
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
        indent((
            iter(msg.fields().map(|field| -> Result<Fragment<_>> {
                Ok(match field.wire_type()? {
                    WireType::Variant(field_type) => (
                        format!("{number} => {{\n", number = field.number()),
                        indent(format!(
                            "*self.{name}.push_and_get_mut() \
                            = variant.to_native::<{tag}>()?;\n",
                            name = field.native_name(),
                            tag = field_type.native_tag_type(msg.path_to_root_mod()),
                        )),
                        "}}\n",
                    )
                        .into(),
                    _ => format!(
                        "{number} => Err(::puroro::PuroroError::UnexpectedWireType)?,\n",
                        number = field.number()
                    )
                    .into(),
                })
            })),
            "_ => Err(::puroro::PuroroError::UnexpectedFieldId)?,\n",
        )),
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
                Ok((
                    format!("{number} => {{\n", number = field.number()),
                    indent((match field.wire_type()? {
                        // Deserialize packed variant(s)
                        WireType::Variant(field_type) => format!(
                            "\
let values = ldd.deserialize_as_variants().map(|rv| {{
    rv.and_then(|variant| variant.to_native::<{tag}>())
}}).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
let mut iter = values.into_iter();
let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
MaybeRepeatedVariantField::extend(&mut self.{name}, first, iter);\n",
                            name = field.native_name(),
                            tag = field_type.native_tag_type(msg.path_to_root_mod()),
                        ),
                        WireType::LengthDelimited(field_type) => match field_type {
                            LengthDelimitedFieldType::String => format!(
                                "\
*self.{name}.push_and_get_mut()
    = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;\n",
                                name = field.native_name()
                            ),
                            LengthDelimitedFieldType::Bytes => format!(
                                "\
*self.{name}.push_and_get_mut()
    = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;\n",
                                name = field.native_name()
                            ),
                            LengthDelimitedFieldType::Message(_) => format!(
                                "\
let msg = self.{name}.push_and_get_mut();
ldd.deserialize_as_message(msg)?;\n",
                                name = field.native_name()
                            ),
                        },
                        _ => "Err(::puroro::PuroroError::UnexpectedWireType)?\n".into(),
                    },)),
                    "}}\n",
                )
                    .into())
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
            iter(
                msg.fields()
                    .map(|field| -> Result<_> {
                        Ok(match (bits, field.wire_type()?) {
                            (32, WireType::Bits32(field_type)) => Some(field_type.native_type()),
                            (64, WireType::Bits64(field_type)) => Some(field_type.native_type()),
                            _ => None,
                        }
                        .map(|native_type| {
                            format!(
                                "\
{number} => {{
    *self.{name}.push_and_get_mut() = {type_}::from_le_bytes(bytes);
}}\n",
                                number = field.number(),
                                name = field.native_name(),
                                type_ = native_type
                            )
                        }))
                    })
                    .filter_map(|ro| ro.transpose()),
            ),
            // group the wrong wire type fields.
            {
                let wrong_wire_field_numbers_iter = msg
                    .fields()
                    .map(|field| {
                        Ok(match (bits, field.wire_type()?) {
                            (32, WireType::Bits32(_)) | (64, WireType::Bits64(_)) => None,
                            _ => Some(field.number().to_string()),
                        })
                    })
                    .filter_map(|ro| ro.transpose());
                let pattern =
                    ::itertools::Itertools::intersperse_with(wrong_wire_field_numbers_iter, || {
                        Ok(" | ".to_string())
                    })
                    .collect::<Result<String>>()?;
                format!(
                    "\
{field_numbers} => {{
    Err(::puroro::PuroroError::UnexpectedWireType)?
}}\n",
                    field_numbers = pattern
                )
            },
            // TODO: handle unknown field id
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
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <{name} as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
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
    {{
        use ::puroro::helpers::MaybeRepeatedField;\n",
            name = msg.native_bare_type_name()
        ),
        indent_n(
            2,
            iter(msg.fields().map(|field| -> Result<_> {
                Ok(match field.wire_type()? {
                    WireType::Variant(field_type) => format!(
                        "\
serializer.serialize_variants_twice::<{tag}, _>(
    {number}, 
    self.{name}.iter_for_ser()
        .cloned()
        .map(|v| Ok(v)))?;\n",
                        number = field.number(),
                        tag = field_type.native_tag_type(msg.path_to_root_mod()),
                        name = field.native_name(),
                    ),

                    WireType::LengthDelimited(field_type) => match field_type {
                        LengthDelimitedFieldType::String => format!(
                            "\
for string in self.{name}.iter_for_ser() {{
    serializer.serialize_bytes_twice({number}, string.bytes().map(|b| Ok(b)))?;
}}\n",
                            name = field.native_name(),
                            number = field.number()
                        ),
                        LengthDelimitedFieldType::Bytes => format!(
                            "\
for bytes in self.{name}.iter_for_ser() {{
    serializer.serialize_bytes_twice({number}, bytes.iter().map(|b| Ok(*b)))?;
}}\n",
                            name = field.native_name(),
                            number = field.number()
                        ),
                        LengthDelimitedFieldType::Message(_) => format!(
                            "\
for msg in self.{name}.iter_for_ser() {{
    serializer.serialize_message_twice({number}, msg)?;
}}\n",
                            number = field.number(),
                            name = field.native_name(),
                        ),
                    },
                    WireType::Bits32(_) | WireType::Bits64(_) => format!(
                        "\
for item in self.{name}.iter_for_ser() {{
    serializer.serialize_fixed_bits({number}, item.to_le_bytes())?;
}}\n",
                        name = field.native_name(),
                        number = field.number(),
                    ),
                })
            })),
        ),
        "        \
        Ok(())
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
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }}
}}\n",
        name = msg.native_bare_type_name()
    ),)
        .write_into(output)
}

pub fn print_msg_trait<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
pub trait {name}Trait {{\n",
            name = msg.native_bare_type_name()
        ),
        indent(iter(msg.fields().map(|field| -> Result<_> {
            Ok(match (field.label()?, field.type_()?) {
                (FieldLabel::Optional, FieldType::Message(_)) => {
                    // getter function for optional message field, wrapped by Option.
                    format!(
                        "fn {name}(&self) -> ::std::option::Option<{reftype}>;\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                    )
                }
                (FieldLabel::Required, _) | (FieldLabel::Optional, _) => {
                    // normal getter function.
                    format!(
                        "fn {name}(&self) -> {reftype};\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                    )
                }
                (FieldLabel::Repeated, _) => {
                    // for_each_***:
                    // A generic getter function for repeated field.
                    // Because of some current Rust language limitations we can only
                    // use an internal iterator, which reminds me Rust @ 2013.
                    // https://doc.rust-lang.org/0.6/std/list.html#function-iter
                    // ***_boxed_iter:
                    // Another restricted getter function. Returns an iterator,
                    // but it is wrapped by `Box`.
                    format!(
                        "\
fn for_each_{name}<F>(&self, f: F)
where
    F: FnMut({reftype});
fn {name}_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>;\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                    )
                }
            })
        }))),
        "}}\n",
    )
        .write_into(output)
}
pub fn print_msg_trait_impl<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "impl {name}Trait for {name} {{\n",
            name = msg.native_bare_type_name()
        ),
        indent(iter(msg.fields().map(|field| -> Result<_> {
            Ok(match (field.label()?, field.type_()?) {
                (FieldLabel::Optional, FieldType::Message(_)) => {
                    format!(
                        "\
fn {name}(&self) -> ::std::option::Option<{reftype}> {{
    self.{name}.as_deref()
}}\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                    )
                }
                (FieldLabel::Required, FieldType::Message(_)) => {
                    format!(
                        "\
fn {name}(&self) -> {reftype} {{
    self.{name}.as_ref()
}}\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                    )
                }
                (FieldLabel::Required, _) | (FieldLabel::Optional, _) => {
                    // normal getter function.
                    let process_ref = match field.type_()? {
                        FieldType::String | FieldType::Bytes => ".as_ref()",
                        FieldType::Message(_) => "", // This should be catched by the arm above
                        _ => ".clone()",
                    };
                    format!(
                        "\
fn {name}(&self) -> {reftype} {{
    self.{name}{process_ref}
}}\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                        process_ref = process_ref,
                    )
                }
                (FieldLabel::Repeated, _) => {
                    let process_iter = match field.type_()? {
                        FieldType::Message(_) => "",
                        FieldType::String | FieldType::Bytes => ".map(|v| v.as_ref())",
                        _ => ".cloned()",
                    };
                    format!(
                        "\
fn for_each_{name}<F>(&self, mut f: F)
where
    F: FnMut({reftype}) {{
    for item in (self.{name}).iter(){process_iter} {{
        (f)(item);
    }}
}}
fn {name}_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>> {{
    ::std::boxed::Box::new(self.{name}.iter(){process_iter})
}}\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_ref_type_name()?,
                        process_iter = process_iter,
                    )
                }
            })
        }))),
        "}}\n",
    )
        .write_into(output)
}

pub fn print_msg_mutable_trait<'c, W: std::fmt::Write>(
    output: &mut Indentor<W>,
    msg: &'c MessageDescriptor<'c>,
) -> Result<()> {
    (
        format!(
            "\
pub trait {name}MutTrait {{\n",
            name = msg.native_bare_type_name()
        ),
        indent(iter(msg.fields().map(|field| -> Result<_> {
            Ok(match (field.label()?, field.type_()?) {
                (FieldLabel::Optional, FieldType::Message(_)) => {
                    // getter function for optional message field, wrapped by Option.
                    format!(
                        "fn {name}_mut(&mut self) -> ::std::option::Option<{reftype}>;\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_mut_ref_type_name()?,
                    )
                }
                (FieldLabel::Required, _) | (FieldLabel::Optional, _) => {
                    // normal getter function.
                    format!(
                        "fn {name}_mut(&mut self) -> {reftype};\n",
                        name = field.native_name(),
                        reftype = field.native_scalar_mut_ref_type_name()?,
                    )
                }
                (FieldLabel::Repeated, _) => {
                    format!(
                        "\
fn for_each_{name}_mut<F>(&mut self, f: F)
where
    F: FnMut({reftype});
fn {name}_boxed_iter_mut(&mut self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>;
// We need more! Maybe just expose &mut Vec<T> ? \n",
                        name = field.native_name(),
                        reftype = field.native_scalar_mut_ref_type_name()?,
                    )
                }
            })
        }))),
        "}}\n",
    )
        .write_into(output)
}
