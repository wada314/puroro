use std::borrow::Cow;

use super::writer::{func, indent, indent_n, iter, Fragment, IntoFragment};
use crate::context::Context;
use crate::utils::Indentor;
use crate::wrappers::{
    FieldDescriptor, FieldLabel, FieldType, LengthDelimitedFieldType, MessageDescriptor,
    NonTrivialFieldType, WireType,
};
use crate::{ErrorKind, Result};

const DESER_MOD: &'static str = "::puroro::deserializer::stream";

pub struct MessageImplCodeGenerator<'c, 'g, G>
where
    G: MessageImplFragmentGenerator<'c>,
{
    context: &'c Context<'c>,
    msg: &'c MessageDescriptor<'c>,
    field_gen: &'g G,
}

impl<'c, 'g, G> MessageImplCodeGenerator<'c, 'g, G>
where
    G: MessageImplFragmentGenerator<'c>,
{
    pub fn new(context: &'c Context<'c>, msg: &'c MessageDescriptor<'c>, field_gen: &'g G) -> Self {
        Self {
            context,
            msg,
            field_gen,
        }
    }

    pub fn print_msg<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            func(|output| self.print_msg_struct(output)),
            func(|output| self.print_msg_default(output)),
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
                name = self.field_gen.struct_name(self.msg)?,
                cfg = self.field_gen.cfg_condition(),
                gp = self.field_gen.struct_generic_params(""),
            ),
            indent(iter(self.msg.fields().map(|field| {
                Ok(format!(
                    "pub {name}: {type_},\n",
                    name = field.native_name(),
                    type_ = self.field_gen.field_type_for(field)?,
                ))
            }))),
            "\
}}\n",
        )
            .write_into(output)
    }

    pub fn print_msg_default<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        if !self.field_gen.is_default_available() {
            return Ok(());
        }
        (
            format!(
                "\
{cfg}
impl{gp} ::std::default::Default for {name}{gpb} {{
    fn default() -> Self {{
        use ::std::convert::TryInto;
        Self {{\n",
                name = self.field_gen.struct_name(self.msg)?,
                cfg = self.field_gen.cfg_condition(),
                gp = self.field_gen.struct_generic_params(""),
                gpb = self.field_gen.struct_generic_params_bounds(""),
            ),
            indent_n(
                3,
                iter(
                    self.msg
                        .fields()
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

    pub fn print_msg_deser_deserializable<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} {d}::MessageDeserializeEventHandler for &'a mut {name}{gpb} {{
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
                name = self.field_gen.struct_name(self.msg)?,
                cfg = self.field_gen.cfg_condition(),
                gp = self.field_gen.struct_generic_params("'a"),
                gpb = self.field_gen.struct_generic_params_bounds(""),
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
            format!(
                "{d}::Field::Variant(variant) => match field_number {{\n",
                d = DESER_MOD
            ),
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
            format!(
                "{d}::Field::LengthDelimited(ldd) => match field_number {{\n",
                d = DESER_MOD
            ),
            indent((
                iter(self.msg.fields().map(|field| -> Result<Fragment<_>> {
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
                                tag = field_type.native_tag_type(self.msg.path_to_root_mod()),
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

    pub fn print_msg_deser_deserializable_bitsxx_arm<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
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
impl{gpb} ::puroro::Deserializable for {name}{gp} {{
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {{
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }}
}}\n",
            name = self.field_gen.struct_name(self.msg)?,
            cfg = self.field_gen.cfg_condition(),
            gp = self.field_gen.struct_generic_params(""),
            gpb = self.field_gen.struct_generic_params_bounds(""),
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
                name = self.field_gen.struct_name(self.msg)?,
                cfg = self.field_gen.cfg_condition(),
                gp = self.field_gen.struct_generic_params(""),
                gpb = self.field_gen.struct_generic_params_bounds(""),
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
            name = self.field_gen.struct_name(self.msg)?,
            cfg = self.field_gen.cfg_condition(),
            gp = self.field_gen.struct_generic_params(""),
            gpb = self.field_gen.struct_generic_params_bounds(""),
        ),)
            .write_into(output)
    }

    fn print_msg_trait_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl {name}Trait for {struct_name} {{\n",
                struct_name = self.field_gen.struct_name(self.msg)?,
                name = self.msg.native_bare_type_name(),
                cfg = self.field_gen.cfg_condition(),
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
}

pub trait MessageImplFragmentGenerator<'c> {
    fn struct_name(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'c, str>>;
    fn cfg_condition(&self) -> &'static str;
    fn is_default_available(&self) -> bool;
    fn field_type_for(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>>;
    fn struct_generic_params(&self, extra_args: &'static str) -> Cow<'static, str>;
    fn struct_generic_params_bounds(&self, extra_args: &'static str) -> Cow<'static, str>;
}
pub struct MessageImplFragmentGeneratorForNormalStruct<'c> {
    context: &'c Context<'c>,
}

impl<'c> MessageImplFragmentGenerator<'c> for MessageImplFragmentGeneratorForNormalStruct<'c> {
    fn struct_name(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'c, str>> {
        Ok(msg.native_bare_type_name().into())
    }

    fn cfg_condition(&self) -> &'static str {
        ""
    }

    fn is_default_available(&self) -> bool {
        true
    }

    fn field_type_for(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>> {
        let scalar_type: Cow<'static, str> = match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => "::std::string::String".into(),
                NonTrivialFieldType::Bytes => "::std::vec::Vec<u8>".into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_fully_qualified_type_name(field.path_to_root_mod())
                )
                .into(),
                NonTrivialFieldType::Message(m) => m
                    .native_fully_qualified_type_name(field.path_to_root_mod())
                    .into(),
            },
        };
        Ok(match field.label()? {
            FieldLabel::Optional => {
                if matches!(field.type_()?, FieldType::Message(_)) {
                    format!(
                        "::std::option::Option<::std::boxed::Box<{type_}>>",
                        type_ = scalar_type,
                    )
                    .into()
                } else {
                    scalar_type.into()
                }
            }
            FieldLabel::Required => {
                if matches!(field.type_()?, FieldType::Message(_)) {
                    format!("::std::boxed::Box<{type_}>", type_ = scalar_type,).into()
                } else {
                    scalar_type.into()
                }
            }
            FieldLabel::Repeated => {
                format!("::std::vec::Vec<{type_}>", type_ = scalar_type,).into()
            }
        })
    }

    fn struct_generic_params(&self, extra_args: &'static str) -> Cow<'static, str> {
        if extra_args.is_empty() {
            "".into()
        } else {
            format!("<{args}>", args = extra_args).into()
        }
    }

    fn struct_generic_params_bounds(&self, extra_args: &'static str) -> Cow<'static, str> {
        if extra_args.is_empty() {
            "".into()
        } else {
            format!("<{args}>", args = extra_args).into()
        }
    }
}

impl<'c> MessageImplFragmentGeneratorForNormalStruct<'c> {
    pub fn new(context: &'c Context<'c>) -> Self {
        Self { context }
    }
}

pub struct MessageImplFragmentGeneratorForBumpaloStruct<'c> {
    context: &'c Context<'c>,
}

impl<'c> MessageImplFragmentGenerator<'c> for MessageImplFragmentGeneratorForBumpaloStruct<'c> {
    fn struct_name(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'c, str>> {
        Ok(format!("{name}Bumpalo", name = msg.native_bare_type_name()).into())
    }

    fn cfg_condition(&self) -> &'static str {
        "#[cfg(feature = \"puroro-bumpalo\")]"
    }

    fn is_default_available(&self) -> bool {
        false
    }

    fn field_type_for(&self, field: &'c FieldDescriptor<'c>) -> Result<Cow<'c, str>> {
        let scalar_type: Cow<'static, str> = match field.type_()?.native_trivial_type_name() {
            Ok(name) => name.into(),
            Err(nontrivial_type) => match nontrivial_type {
                NonTrivialFieldType::Group => Err(ErrorKind::GroupNotSupported)?,
                NonTrivialFieldType::String => "::bumpalo::collections::String".into(),
                NonTrivialFieldType::Bytes => "::bumpalo::collections::Vec<u8>".into(),
                NonTrivialFieldType::Enum(e) => format!(
                    "::std::result::Result<{type_}, i32>",
                    type_ = e.native_fully_qualified_type_name(field.path_to_root_mod())
                )
                .into(),
                NonTrivialFieldType::Message(m) => m
                    .native_fully_qualified_type_name(field.path_to_root_mod())
                    .into(),
            },
        };
        Ok(match field.label()? {
            FieldLabel::Optional => {
                if matches!(field.type_()?, FieldType::Message(_)) {
                    format!(
                        "::std::option::Option<::bumpalo::boxed::Box<{type_}>>",
                        type_ = scalar_type,
                    )
                    .into()
                } else {
                    scalar_type.into()
                }
            }
            FieldLabel::Required => {
                if matches!(field.type_()?, FieldType::Message(_)) {
                    format!("::bumpalo::boxed::Box<{type_}>", type_ = scalar_type,).into()
                } else {
                    scalar_type.into()
                }
            }
            FieldLabel::Repeated => {
                format!("::bumpalo::collections::Vec<{type_}>", type_ = scalar_type,).into()
            }
        })
    }

    fn struct_generic_params(&self, extra_args: &'static str) -> Cow<'static, str> {
        if extra_args.is_empty() {
            "<'b>".into()
        } else {
            format!("<'b, {args}>", args = extra_args).into()
        }
    }

    fn struct_generic_params_bounds(&self, extra_args: &'static str) -> Cow<'static, str> {
        if extra_args.is_empty() {
            "<'b>".into()
        } else {
            format!("<'b, {args}>", args = extra_args).into()
        }
    }
}

impl<'c> MessageImplFragmentGeneratorForBumpaloStruct<'c> {
    pub fn new(context: &'c Context<'c>) -> Self {
        Self { context }
    }
}
