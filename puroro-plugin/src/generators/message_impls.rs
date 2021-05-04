use super::message_frags::MessageImplFragmentGenerator;
use super::message_traits::{GetterMethods, MessageTraitCodeGenerator};
use super::writer::{func, indent, indent_n, iter, IntoFragment};
use crate::context::{AllocatorType, Context};
use crate::utils::Indentor;
use crate::wrappers::{FieldLabel, FieldType, MessageDescriptor};
use crate::Result;

pub struct MessageImplCodeGenerator<'a, 'c> {
    context: &'a Context<'c>,
    msg: &'c MessageDescriptor<'c>,
    frag_gen: MessageImplFragmentGenerator<'a, 'c>,
    traits_gen: MessageTraitCodeGenerator<'a, 'c>,
}

impl<'a, 'c> MessageImplCodeGenerator<'a, 'c> {
    pub fn new(context: &'a Context<'c>, msg: &'c MessageDescriptor<'c>) -> Self {
        Self {
            context,
            msg,
            frag_gen: MessageImplFragmentGenerator::new(context),
            traits_gen: MessageTraitCodeGenerator::new(context, msg),
        }
    }

    pub fn print_msg<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            func(|output| self.print_msg_struct(output)),
            func(|output| self.print_msg_new(output)),
            func(|output| self.print_msg_clone(output)),
            (
                func(|output| self.print_msg_deser_from_iter(output)),
                func(|output| self.print_msg_ser(output)),
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
#[derive(Debug)]
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
                        name = field.native_ident()?,
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
                                    name = field.native_ident()?
                                )
                            }
                            AllocatorType::Bumpalo => format!(
                                "{name}: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),\n",
                                name = field.native_ident()?
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

    pub fn print_msg_clone<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::std::clone::Clone for {name}{gpb} {{
    fn clone(&self) -> Self {{
        use ::puroro_internal::helpers::FieldClone;
        Self {{\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent_n(
                3,
                iter(self.msg.fields().map(|field| {
                    Ok(format!(
                        "{ident}: {clone},\n",
                        ident = field.native_ident()?,
                        clone = self.frag_gen.field_clone(
                            field.native_ident()?,
                            &self.frag_gen.field_type_for(field)?
                        ),
                    ))
                })),
            ),
            "        \
            puroro_internal: self.puroro_internal.clone(),
        }}
    }}
}}\n",
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
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
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
    <{type_} as FieldDeserFromIter<
        tags::{type_tag}, 
        tags::{label_tag}>>
    ::deser(&mut self.{name}, field, {default_func})?;
}}\n",
                            number = field.number(),
                            name = field.native_ident()?,
                            type_ = self.frag_gen.field_type_for(field)?,
                            type_tag = self.frag_gen.type_tag_for(field)?,
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

    pub fn print_msg_ser<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::ser::Serializable for {name}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;\n",
                name = self.frag_gen.struct_name(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent_n(
                2,
                iter(self.msg.fields().map(|field| -> Result<_> {
                    Ok(format!(
                        "\
<{type_} as FieldSer<
        tags::{type_tag}, 
        tags::{label_tag}>>
    ::ser(&self.{name}, serializer, {number})?;\n",
                        number = field.number(),
                        name = field.native_ident()?,
                        type_ = self.frag_gen.field_type_for(field)?,
                        type_tag = self.frag_gen.type_tag_for(field)?,
                        label_tag = field.label_tag()?,
                    ))
                })),
            ),
            "        \
        Ok(())
    }}
}}\n",
            format!(
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
            ),
        )
            .write_into(output)
    }

    fn print_msg_trait_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} {trait_ident} for {struct_ident}{gpb} {{\n",
                struct_ident = self.frag_gen.struct_name(self.msg)?,
                trait_ident = self.traits_gen.trait_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent((
                iter(self.msg.unique_msgs_from_fields()?.map(|msg| {
                    // typedefs for message types
                    Ok(format!(
                        "type {assoc_type_ident} = {actual_type_name}{gpb};\n",
                        assoc_type_ident = self.traits_gen.associated_msg_type_ident(msg)?,
                        actual_type_name = self
                            .frag_gen
                            .struct_name_with_relative_path(msg, self.msg.package()?)?,
                        gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                    ))
                })),
                iter(
                    // typedefs for repeated field iterators
                    self.msg
                        .fields()
                        .filter(|field| matches!(field.label(), Ok(FieldLabel::Repeated)))
                        .map(|field| {
                            Ok(format!(
                                "\
#[cfg(feature = \"puroro-nightly\")]
type {iter_ident}<'a> = impl ::std::iter::Iterator<Item = {reftype}>;\n",
                                iter_ident = self.traits_gen.associated_iter_type_ident(field)?,
                                reftype =
                                    self.traits_gen.scalar_maybe_ref_type_name(field, "'a")?
                            ))
                        }),
                ),
                iter(self.msg.fields().map(|field| -> Result<_> {
                    Ok(
                        match self.traits_gen.generate_getter_method_decls(field, true)? {
                            GetterMethods::ScalarField(decl) => {
                                let transform = match field.type_()? {
                                    FieldType::String
                                    | FieldType::Bytes
                                    | FieldType::Message(_) => ".as_ref()",
                                    _ => ".clone()",
                                };

                                format!(
                                    "{decl} {{\n    self.{ident}{transform}\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?,
                                    transform = transform,
                                )
                            }
                            GetterMethods::OptionalField(decl) => {
                                let transform = match field.type_()? {
                                    FieldType::String
                                    | FieldType::Bytes
                                    | FieldType::Message(_) => ".as_deref()",
                                    _ => ".clone()",
                                };
                                format!(
                                    "{decl} {{\n    self.{ident}{transform}\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?,
                                    transform = transform,
                                )
                            }
                            GetterMethods::RepeatedField {
                                for_each,
                                boxed_iter,
                                iter,
                            } => {
                                let transform_iter = match field.type_()? {
                                    FieldType::Message(_) => "",
                                    FieldType::String | FieldType::Bytes => ".map(|v| v.as_ref())",
                                    _ => ".cloned()",
                                };
                                format!(
                                    "\
{for_each} {{
    for item in (self.{ident}).iter(){transform_iter} {{
        (f)(item);
    }}
}}
{boxed_iter} {{
    ::std::boxed::Box::new(self.{ident}.iter(){transform_iter})
}}
#[cfg(feature = \"puroro-nightly\")]
{iter} {{
    self.{ident}.iter(){transform_iter}
}}\n",
                                    for_each = for_each,
                                    boxed_iter = boxed_iter,
                                    iter = iter,
                                    ident = field.native_ident()?,
                                    transform_iter = transform_iter
                                )
                            }
                        },
                    )
                })),
            )),
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
