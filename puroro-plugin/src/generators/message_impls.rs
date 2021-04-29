use super::message_frags::MessageImplFragmentGenerator;
use super::writer::{func, indent, indent_n, iter, IntoFragment};
use crate::context::{AllocatorType, Context};
use crate::utils::{to_camel_case, Indentor};
use crate::wrappers::{
    FieldLabel, FieldType, LengthDelimitedFieldType, MessageDescriptor, WireType,
};
use crate::Result;

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
                func(|output| self.print_msg_deser_from_iter(output)),
                func(|output| self.print_msg_ser_serializable(output)),
                func(|output| self.print_msg_puroro_serializable(output)),
            ),
            func(|output| self.print_msg_trait_impl(output)),
            func(|output| self.print_msg_field_new_impl(output)),
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
                gp = self.frag_gen.struct_generic_params(&[]),
            ),
            indent((
                iter(self.msg.fields().map(|field| {
                    Ok(format!(
                        "{vis}{name}: {type_},\n",
                        vis = self.frag_gen.field_visibility(),
                        name = field.native_name()?,
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
        Self {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                new_decl = self.frag_gen.new_method_declaration(),
            ),
            indent_n(
                3,
                (
                    iter(self.msg.fields().map(|field| {
                        Ok(match self.context.alloc_type() {
                            AllocatorType::Default => {
                                format!(
                                    "{name}: ::puroro_internal::helpers::FieldNew::new(),\n",
                                    name = field.native_name()?
                                )
                            }
                            AllocatorType::Bumpalo => format!(
                                "{name}: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),\n",
                                name = field.native_name()?
                            ),
                        })
                    })),
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
                    gp = self.frag_gen.struct_generic_params(&[]),
                    gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                )
            } else {
                "".to_string()
            },
        )
            .write_into(output)
    }

    pub fn print_msg_deser_from_iter<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::deser::DeserializableMessageFromIter for {name}{gpb} {{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {{
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent_n(
                3,
                (
                    iter(self.msg.fields().map(|field| -> Result<_> {
                        Ok(format!(
                            "\
{number} => {{
    <{type_} as DeserializableFieldFromIter<(
        tags::{type_tag}, 
        tags::{label_tag})>>
    ::deser(&mut self.{name}, field, {default_func})?;
}}\n",
                            number = field.number(),
                            name = field.native_name()?,
                            type_ = self.frag_gen.field_type_for(field)?,
                            type_tag = field.type_tag()?,
                            label_tag = field.label_tag()?,
                            default_func = self.frag_gen.default_func_for(field)?,
                        ))
                    })),
                    "_ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,\n",
                ),
            ),
            "        \
        }}
        Ok(())
    }}
}}\n",
            format!(
                "\
{cfg}
impl{gp} ::puroro::DeserializableFromIter for {name}{gpb} {{
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {{
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }}
}}\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
        )
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
impl{gp} ::puroro_internal::ser::Serializable for {name}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        use ::puroro_internal::helpers::MaybeRepeatedField;\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
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
                            tag = field_type.native_tag_type(self.msg.path_to_root_mod()?)?,
                            name = field.native_name()?,
                        ),

                        WireType::LengthDelimited(field_type) => match field_type {
                            LengthDelimitedFieldType::String => format!(
                                "\
for string in self.{name}.iter_for_ser() {{
    serializer.serialize_bytes_twice({number}, string.bytes().map(|b| Ok(b)))?;
}}\n",
                                name = field.native_name()?,
                                number = field.number()
                            ),
                            LengthDelimitedFieldType::Bytes => format!(
                                "\
for bytes in self.{name}.iter_for_ser() {{
    serializer.serialize_bytes_twice({number}, bytes.iter().map(|b| Ok(*b)))?;
}}\n",
                                name = field.native_name()?,
                                number = field.number()
                            ),
                            LengthDelimitedFieldType::Message(_) => format!(
                                "\
for msg in self.{name}.iter_for_ser() {{
    serializer.serialize_message_twice({number}, msg)?;
}}\n",
                                number = field.number(),
                                name = field.native_name()?,
                            ),
                        },
                        WireType::Bits32(_) | WireType::Bits64(_) => format!(
                            "\
for item in self.{name}.iter_for_ser() {{
    serializer.serialize_fixed_bits({number}, item.to_le_bytes())?;
}}\n",
                            name = field.native_name()?,
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
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }}
}}\n",
            name = self.frag_gen.struct_name(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(&[]),
            gpb = self.frag_gen.struct_generic_params_bounds(&[]),
        ),)
            .write_into(output)
    }

    fn print_msg_trait_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} {name}Trait for {struct_name}{gpb} {{\n",
                struct_name = self.frag_gen.struct_name(self.msg)?,
                name = self.msg.native_bare_type_name()?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent(iter(self.msg.fields().map(|field| -> Result<_> {
                Ok((
                    if let FieldType::Message(m) = field.type_()? {
                        // Associated Type for the message type
                        format!(
                            "type {camel_name}Type = {submsg_type};\n",
                            camel_name = to_camel_case(field.native_name()?),
                            submsg_type = self.frag_gen.type_name_of_msg(m, field.package()?)?,
                        )
                    } else {
                        "".to_string()
                    },
                    match (field.label()?, field.type_()?) {
                        (FieldLabel::Optional2, FieldType::Message(_))
                        | (FieldLabel::Optional3, FieldType::Message(_)) => {
                            format!(
                                "\
fn {name}(&self) -> ::std::option::Option<{reftype}> {{
    self.{name}.as_deref()
}}\n",
                                name = field.native_name()?,
                                reftype = field.native_maybe_ref_type("'_")?,
                            )
                        }
                        (FieldLabel::Required, FieldType::Message(_)) => {
                            format!(
                                "\
fn {name}(&self) -> {reftype} {{
    &self.{name}
}}\n",
                                name = field.native_name()?,
                                reftype = field.native_maybe_ref_type("'_")?,
                            )
                        }
                        (FieldLabel::Required, _) | (FieldLabel::Optional3, _) => {
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
                                name = field.native_name()?,
                                reftype = field.native_maybe_ref_type("'_")?,
                                process_ref = process_ref,
                            )
                        }
                        (FieldLabel::Optional2, _) => {
                            // getter function with Option.
                            let process_ref = match field.type_()? {
                                FieldType::String | FieldType::Bytes => ".as_deref()",
                                FieldType::Message(_) => "", // This should be catched by the arm above
                                _ => ".clone()",
                            };
                            format!(
                                "\
fn {name}(&self) -> ::std::option::Option<{reftype}> {{
    self.{name}{process_ref}
}}\n",
                                name = field.native_name()?,
                                reftype = field.native_maybe_ref_type("'_")?,
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
    F: FnMut({reftype})
{{
    for item in (self.{name}).iter(){process_iter} {{
        (f)(item);
    }}
}}
fn {name}_boxed_iter(&self) ->
    ::std::boxed::Box<dyn '_ + Iterator<Item={reftype}>>
{{
    ::std::boxed::Box::new(self.{name}.iter(){process_iter})
}}
#[cfg(feature = \"puroro-nightly\")]
type {camel_name}Iter<'a> = impl Iterator<Item={reftype_lt_a}>;
#[cfg(feature = \"puroro-nightly\")]
fn {name}_iter(&self) -> Self::{camel_name}Iter<'_> {{
    self.{name}.iter(){process_iter}
}}\n",
                                name = field.native_name()?,
                                camel_name = to_camel_case(field.native_name()?),
                                reftype = field.native_maybe_ref_type("'_")?,
                                process_iter = process_iter,
                                reftype_lt_a = field.native_maybe_ref_type("'a")?,
                            )
                        }
                    },
                ))
            }))),
            "}}\n",
        )
            .write_into(output)
    }

    fn print_msg_field_new_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (match self.context.alloc_type() {
            crate::context::AllocatorType::Default => {
                format!(
                    "\
impl{gp} ::puroro_internal::helpers::FieldNew<'a> for {name}{gpb} {{
    fn new() -> Self {{
        Default::default()
    }}
}}\n",
                    name = self.frag_gen.struct_name(self.msg)?,
                    gp = self.frag_gen.struct_generic_params(&["'a"]),
                    gpb = self.frag_gen.struct_generic_params_bounds(&[""]),
                )
            }
            crate::context::AllocatorType::Bumpalo => {
                format!(
                    "\
{cfg}
impl{gp} ::puroro_internal::helpers::FieldNew<'bump> for {name}{gpb} {{
    fn new() -> Self {{
        unimplemented!()
    }}
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {{
        Self::new_in(bump)
    }}
}}\n",
                    cfg = self.frag_gen.cfg_condition(),
                    name = self.frag_gen.struct_name(self.msg)?,
                    gp = self.frag_gen.struct_generic_params(&[]),
                    gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                )
            }
        },)
            .write_into(output)
    }
}
