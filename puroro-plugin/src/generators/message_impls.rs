use super::message_frags::MessageImplFragmentGenerator;
use super::message_traits::{GetterMethods, MessageTraitCodeGenerator};
use super::writer::{func, indent, indent_n, iter, IntoFragment};
use crate::context::{AllocatorType, Context};
use crate::utils::Indentor;
use crate::wrappers::{FieldLabel, FieldType, MessageDescriptor};
use crate::{ErrorKind, Result};

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
            frag_gen: MessageImplFragmentGenerator::new(context, msg),
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
                func(|output| self.print_map_entry_impl(output)),
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
pub struct {ident}{gp} {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
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
impl{gp} {ident}{gpb} {{
    pub {new_decl} {{
        Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
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
impl{gp} ::std::default::Default for {ident}{gpb} {{
    fn default() -> Self {{
        Self::new()
    }}
}}\n",
                    ident = self.frag_gen.struct_ident(self.msg)?,
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
impl{gp} ::std::clone::Clone for {ident}{gpb} {{
    fn clone(&self) -> Self {{
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
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
            "            \
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
impl{gp} ::puroro_internal::deser::DeserializableMessageFromIter for {ident}{gpb} {{
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
        let puroro_internal = &self.puroro_internal;
        match field_number {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
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
    ::deser(&mut self.{ident}, field, {default_func})?;
}}\n",
                            number = field.number(),
                            ident = field.native_ident()?,
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
                name = self.frag_gen.struct_ident(self.msg)?,
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
impl{gp} ::puroro_internal::ser::Serializable for {ident}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
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
    ::ser(&self.{ident}, serializer, {number})?;\n",
                        number = field.number(),
                        ident = field.native_ident()?,
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
                name = self.frag_gen.struct_ident(self.msg)?,
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
                struct_ident = self.frag_gen.struct_ident(self.msg)?,
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
                        actual_type_name = self.frag_gen.struct_ident_with_relative_path(msg)?,
                        gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                    ))
                })),
                iter(self.msg.fields().map(|field| -> Result<_> {
                    Ok(match self.traits_gen.generate_getter_method_decls(field)? {
                        GetterMethods::ScalarField(decl) => {
                            let transform = match field.type_()? {
                                FieldType::String | FieldType::Bytes | FieldType::Message(_) => {
                                    ".as_ref()"
                                }
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
                                FieldType::String | FieldType::Bytes | FieldType::Message(_) => {
                                    ".as_deref()"
                                }
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
                            type_ident,
                            type_bound: _,
                            get_decl,
                        }
                        | GetterMethods::MapField {
                            type_ident,
                            type_bound: _,
                            get_decl,
                        } => {
                            format!(
                                "\
type {type_ident} = {type_name};
{get_decl} {{
    &self.{ident}
}}\n",
                                type_ident = type_ident,
                                type_name = self.frag_gen.field_type_for(field)?,
                                get_decl = get_decl,
                                ident = field.native_ident()?,
                            )
                        }
                    })
                })),
            )),
            "}}\n",
        )
            .write_into(output)
    }

    pub fn print_map_entry_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        if !self.msg.is_map_entry() {
            return Ok(());
        }
        let (key_field, value_field) = self.msg.key_value_of_map_entry()?;
        (format!(
            "\
{cfg}
impl{gp} ::puroro_internal::helpers::MapEntry for {entry_type} {{
    type KeyType = {key_type};
    type ValueType = {value_type};
    fn ser_kv<T: ::puroro_internal::ser::MessageSerializer>(
        key: &Self::KeyType,
        value: &Self::ValueType,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {{
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <{key_type} as FieldSer<
            tags::{key_type_tag}, 
            tags::Required>>
            ::ser(key, serializer, 1)?;
        <{value_type} as FieldSer<
            tags::{value_type_tag}, 
            tags::Required>>
            ::ser(value, serializer, 2)?;
        Ok(())
    }}
    fn into_tuple(self) -> (Self::KeyType, Self::ValueType) {{
        use ::puroro_internal::helpers::FieldTakeOrInit;
        (
            {take_key}, 
            {take_value},
        )
    }}
}}\n",
            entry_type = self.frag_gen.type_name_of_msg(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(&[]),
            key_type = self.frag_gen.field_scalar_item_type_for(key_field)?,
            key_type_tag = self.frag_gen.type_tag_for(key_field)?,
            take_key = self.frag_gen.field_take_or_init(key_field)?,
            value_type = self.frag_gen.field_scalar_item_type_for(value_field)?,
            value_type_tag = self.frag_gen.type_tag_for(value_field)?,
            take_value = self.frag_gen.field_take_or_init(value_field)?,
        ),)
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
                    name = self.frag_gen.struct_ident(self.msg)?,
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
                    name = self.frag_gen.struct_ident(self.msg)?,
                    gp = self.frag_gen.struct_generic_params(&[]),
                    gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                )
            }
        },)
            .write_into(output)
    }
}
