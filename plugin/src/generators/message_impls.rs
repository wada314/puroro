use super::message_frags::MessageImplFragmentGenerator;
use super::writer::{func, indent, indent_n, iter, Fragment, IntoFragment};
use crate::context::Context;
use crate::utils::{to_camel_case, Indentor};
use crate::wrappers::{
    FieldLabel, FieldType, LengthDelimitedFieldType, MessageDescriptor, WireType,
};
use crate::Result;

const DESER_MOD: &'static str = "::puroro::deserializer::bytes";

pub struct MessageImplCodeGenerator<'a, 'c> {
    context: &'a Context<'c>,
    msg: &'c MessageDescriptor<'c>,
    frag_gen: MessageImplFragmentGenerator<'a, 'c>,
}

impl<'a, 'c> MessageImplCodeGenerator<'a, 'c> {
    pub fn new(context: &'a Context<'c>, msg: &'c MessageDescriptor<'c>) -> Self {
        Self {
            context,
            msg,
            frag_gen: MessageImplFragmentGenerator::new(context),
        }
    }

    pub fn print_msg<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            func(|output| self.print_msg_struct(output)),
            func(|output| self.print_msg_new(output)),
            (
                func(|output| self.print_msg_deser_deserializable(output)),
                func(|output| self.print_msg_puroro_deserializable(output)),
                func(|output| self.print_msg_ser_serializable(output)),
                func(|output| self.print_msg_puroro_serializable(output)),
            ),
            func(|output| self.print_msg_trait_impl(output)),
        )
            .write_into(output)
    }

    fn print_msg_struct<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
#[derive(Debug, Clone)]
pub struct {name}{gp} {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
            ),
            indent((
                iter(self.msg.fields().map(|field| {
                    Ok(format!(
                        "pub {name}: {type_},\n",
                        name = field.native_name(),
                        type_ = self.frag_gen.field_type_for(field)?,
                    ))
                })),
                format!(
                    "puroro_internal: {type_},\n",
                    type_ = self.frag_gen.internal_data_type()
                ),
            )),
            "\
}}\n",
        )
            .write_into(output)
    }

    pub fn print_msg_new<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} {name}{gpb} {{
    pub {new_decl} {{
        use ::std::convert::TryInto;
        Self {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
                new_decl = self.frag_gen.new_method_declaration(),
            ),
            indent_n(
                3,
                (
                    iter(
                        self.msg
                            .fields()
                            .map(|field| match (field.label()?, field.type_()?) {
                                (FieldLabel::Optional, FieldType::Enum(_))
                                | (FieldLabel::Required, FieldType::Enum(_)) => Ok(format!(
                                    "{name}: 0i32.try_into(),\n",
                                    name = field.native_name()
                                )),
                                (FieldLabel::Required, FieldType::Message(m)) => Ok(format!(
                                    "{name}: {msg_type}::{call_new},\n",
                                    name = field.native_name(),
                                    msg_type = self.frag_gen.field_type_for(field)?,
                                    call_new = self.frag_gen.call_new_from_new(),
                                )),
                                (_, _) => Ok(format!(
                                    "{name}: ::std::default::Default::default(),\n",
                                    name = field.native_name(),
                                )),
                            }),
                    ),
                    format!(
                        "puroro_internal: {value},\n",
                        value = self.frag_gen.internal_field_init_value()
                    ),
                ),
            ),
            "        \
        }}
    }}
}}\n",
            if self.frag_gen.is_default_available() {
                format!(
                    "\
{cfg}
impl{gp} ::std::default::Default for {name}{gpb} {{
    fn default() -> Self {{
        Self::new()
    }}
}}\n",
                    name = self.frag_gen.struct_name(self.msg)?,
                    cfg = self.frag_gen.cfg_condition(),
                    gp = self.frag_gen.struct_generic_params(),
                    gpb = self.frag_gen.struct_generic_params_bounds(),
                )
            } else {
                "".to_string()
            },
        )
            .write_into(output)
    }

    pub fn print_msg_deser_deserializable<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro::deser::DeserializeMessageFromBytesEventHandler for {name}{gpb} {{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {{
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                3,
                (
                    func(|output| self.print_msg_deser_deserializable_variant_arm(output)),
                    func(|output| self.print_msg_deser_deserializable_length_delimited_arm(output)),
                    func(|output| self.print_msg_deser_deserializable_bitsxx_arm(output, 32)),
                    func(|output| self.print_msg_deser_deserializable_bitsxx_arm(output, 64)),
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

    pub fn print_msg_deser_deserializable_variant_arm<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            "::puroro::types::FieldData::Variant(variant) => match field_number {{\n",
            indent((
                iter(self.msg.fields().map(|field| -> Result<Fragment<_>> {
                    Ok(match field.wire_type()? {
                        WireType::Variant(field_type) => (
                            format!("{number} => {{\n", number = field.number()),
                            indent(format!(
                                "*self.{name}.push_and_get_mut() \
                            = variant.to_native::<{tag}>()?;\n",
                                name = field.native_name(),
                                tag = field_type.native_tag_type(self.msg.path_to_root_mod()),
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

    pub fn print_msg_deser_deserializable_length_delimited_arm<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            "::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {{\n",
            indent((
                iter(self.msg.fields().map(|field| -> Result<Fragment<_>> {
                    Ok((
                        format!("{number} => {{\n", number = field.number()),
                        indent((match field.wire_type()? {
                            // Deserialize packed variant(s)
                            WireType::Variant(field_type) => format!(
                                "\
let values = bytes_iter.variants().map(|rv| {{
    rv.and_then(|variant| variant.to_native::<{tag}>())
}}).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
let mut iter = values.into_iter();
let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
MaybeRepeatedVariantField::extend(&mut self.{name}, first, iter);\n",
                                name = field.native_name(),
                                tag = field_type.native_tag_type(self.msg.path_to_root_mod()),
                            ),
                            WireType::LengthDelimited(field_type) => match field_type {
                                LengthDelimitedFieldType::String => format!(
                                    "\
*self.{name}.push_and_get_mut()
    = bytes_iter.chars().collect::<::puroro::Result<_>>()?;\n",
                                    name = field.native_name()
                                ),
                                LengthDelimitedFieldType::Bytes => format!(
                                    "\
*self.{name}.push_and_get_mut()
    = bytes_iter.bytes().collect::<::puroro::Result<_>>()?;\n",
                                    name = field.native_name()
                                ),
                                LengthDelimitedFieldType::Message(_) => format!(
                                    "\
let msg = self.{name}.push_and_get_mut2(&self.puroro_internal);
bytes_iter.deser_message(msg)?;\n",
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

    pub fn print_msg_deser_deserializable_bitsxx_arm<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
        bits: usize,
    ) -> Result<()> {
        (
            format!(
                "::puroro::types::FieldData::Bits{bits}(bytes) => match field_number {{\n",
                bits = bits,
            ),
            indent((
                iter(
                    self.msg
                        .fields()
                        .map(|field| -> Result<_> {
                            Ok(match (bits, field.wire_type()?) {
                                (32, WireType::Bits32(field_type)) => {
                                    Some(field_type.native_type())
                                }
                                (64, WireType::Bits64(field_type)) => {
                                    Some(field_type.native_type())
                                }
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
                    let wrong_wire_field_numbers_iter = self
                        .msg
                        .fields()
                        .map(|field| {
                            Ok(match (bits, field.wire_type()?) {
                                (32, WireType::Bits32(_)) | (64, WireType::Bits64(_)) => None,
                                _ => Some(field.number().to_string()),
                            })
                        })
                        .filter_map(|ro| ro.transpose());
                    let pattern = ::itertools::Itertools::intersperse_with(
                        wrong_wire_field_numbers_iter,
                        || Ok(" | ".to_string()),
                    )
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

    pub fn print_msg_puroro_deserializable<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (format!(
            "\
{cfg}
impl{gpb} ::puroro::deser::DeserializableFromBytes for {name}{gp} {{
    fn deserialize<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {{
        let mut bytes_iter = ::puroro::deser::BytesIter::new(iter);
        bytes_iter.deser_message(self)
    }}
}}\n",
            name = self.frag_gen.struct_name(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(),
            gpb = self.frag_gen.struct_generic_params_bounds(),
        ),)
            .write_into(output)
    }

    pub fn print_msg_ser_serializable<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro::serializer::Serializable for {name}{gpb} {{
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        use ::puroro::helpers::MaybeRepeatedField;\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                2,
                iter(self.msg.fields().map(|field| -> Result<_> {
                    Ok(match field.wire_type()? {
                        WireType::Variant(field_type) => format!(
                            "\
serializer.serialize_variants_twice::<{tag}, _>(
    {number}, 
    self.{name}.iter_for_ser()
        .cloned()
        .map(|v| Ok(v)))?;\n",
                            number = field.number(),
                            tag = field_type.native_tag_type(self.msg.path_to_root_mod()),
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

    pub fn print_msg_puroro_serializable<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (format!(
            "\
{cfg}
impl{gp} ::puroro::Serializable for {name}{gpb} {{
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {{
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }}
}}\n",
            name = self.frag_gen.struct_name(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(),
            gpb = self.frag_gen.struct_generic_params_bounds(),
        ),)
            .write_into(output)
    }

    fn print_msg_trait_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl {name}Trait for {struct_name} {{\n",
                struct_name = self.frag_gen.struct_name(self.msg)?,
                name = self.msg.native_bare_type_name(),
                cfg = self.frag_gen.cfg_condition(),
            ),
            indent(iter(self.msg.fields().map(|field| -> Result<_> {
                Ok(match (field.label()?, field.type_()?) {
                    (FieldLabel::Optional, FieldType::Message(_)) => {
                        format!(
                            "\
fn {name}(&self) -> ::std::option::Option<{reftype}> {{
    self.{name}.as_deref()
}}\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_ref_type_name("")?,
                        )
                    }
                    (FieldLabel::Required, FieldType::Message(_)) => {
                        format!(
                            "\
fn {name}(&self) -> {reftype} {{
    &self.{name}
}}\n",
                            name = field.native_name(),
                            reftype = field.native_scalar_ref_type_name("")?,
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
                            reftype = field.native_scalar_ref_type_name("")?,
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
}}
#[cfg(feature = \"puroro-nightly\")]
type {camel_name}Iter<'a> = impl Iterator<Item={reftype_lt_a}>;
#[cfg(feature = \"puroro-nightly\")]
fn {name}_iter(&self) -> Self::{camel_name}Iter<'_> {{
    self.{name}.iter(){process_iter}
}}\n",
                            name = field.native_name(),
                            camel_name = to_camel_case(field.native_name()),
                            reftype = field.native_scalar_ref_type_name("")?,
                            process_iter = process_iter,
                            reftype_lt_a = field.native_scalar_ref_type_name("'a")?,
                        )
                    }
                })
            }))),
            "}}\n",
        )
            .write_into(output)
    }
}
