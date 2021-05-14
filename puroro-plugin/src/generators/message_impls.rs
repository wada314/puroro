use super::message_frags::MessageImplFragmentGenerator;
use super::message_traits::{GetterMethods, MessageTraitCodeGenerator};
use super::writer::{func, indent, indent_n, iter, seq, IntoFragment};
use crate::context::{AllocatorType, Context, ImplType};
use crate::utils::Indentor;
use crate::wrappers::{FieldType, MessageDescriptor};
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
            frag_gen: MessageImplFragmentGenerator::new(context, msg),
            traits_gen: MessageTraitCodeGenerator::new(context, msg),
        }
    }

    pub fn print_msg<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            func(|output| self.print_msg_struct(output)),
            func(|output| self.print_new_methods(output)),
            func(|output| self.print_clone(output)),
            func(|output| self.print_default(output)),
            (
                func(|output| self.print_msg_deser_from_iter(output)),
                match self.context.impl_type() {
                    ImplType::Default => {
                        func(|output| self.print_msg_deser_from_slice_using_from_iter(output))
                    }
                    ImplType::SliceView { .. } => {
                        func(|output| self.print_msg_deser_from_slice_for_slice_view(output))
                    }
                },
                func(|output| self.print_msg_ser(output)),
                func(|output| self.print_impl_map_entry(output)),
            ),
            func(|output| self.print_impl_trait(output)),
            func(|output| self.print_impl_field_new(output)),
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

    fn print_new_methods<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "{cfg}\nimpl{gp} {ident}{gpb} {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(&[]),
                gpb = self.frag_gen.struct_generic_params_bounds(&[]),
            ),
            indent(
                match (self.context.impl_type(), self.context.alloc_type()) {
                    (ImplType::Default, _) => func(|output| self.new_method_default_impl(output)),
                    (ImplType::SliceView { .. }, AllocatorType::Default) => {
                        func(|output| self.new_method_slice_view_impl(output))
                    }
                    _ => {
                        unreachable!()
                    }
                },
            ),
            "}}\n",
        )
            .write_into(output)
    }

    fn new_method_self_members<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
        internal_init: &str,
    ) -> Result<()> {
        (
            iter(self.msg.fields().map(|field| {
                Ok(format!(
                    "{name}: {field_new},\n",
                    name = field.native_ident()?,
                    field_new = self.frag_gen.field_new(),
                ))
            })),
            format!("puroro_internal: {value},\n", value = internal_init),
        )
            .write_into(output)
    }

    fn new_method_default_impl<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
pub {decl} {{
    Self {{\n",
                decl = self.frag_gen.new_method_declaration()
            ),
            indent_n(
                2,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        match self.context.alloc_type() {
                            AllocatorType::Default => {
                                "::puroro_internal::InternalDataForNormalStruct::new()"
                            }
                            AllocatorType::Bumpalo => {
                                "::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump)"
                            }
                        },
                    )
                }),
            ),
            "    \
    }}
}}\n",
        )
            .write_into(output)
    }

    fn new_method_slice_view_impl<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            "\
fn try_new(slice: &'slice [u8]) -> ::puroro::Result<Self> {{
    let mut new_self = Self {{\n",
            indent_n(
                2,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        "\
::puroro_internal::InternalDataForSliceViewStruct::new(slice)",
                    )
                }),
            ),
            "    \
    }};
    for ld_slice in new_self.puroro_internal.slices() {
        ld_slice.deser_message(&mut new_self)?;
    }
    Ok(new_self)
}}

fn try_new_with_parent(
    parent_field: &'p ::std::option::Option<::puroro_internal::SliceViewFields<'slice>>,
    field_number_in_parent: usize,
    parent_internal_data: &'p ::puroro_internal::InternalDataForSliceViewStruct<'slice, 'p>,
) -> ::puroro::Result<Self> {{
    let mut new_self = Self {{\n",
            indent_n(
                2,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        "\
::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
    parent_field, field_number_in_parent, parent_internal_data)",
                    )
                }),
            ),
            "    \
    }};
    for ld_slice in new_self.puroro_internal.slices() {
        ld_slice.deser_message(&mut new_self)?;
    }
    Ok(new_self)
}}\n",
        )
            .write_into(output)
    }

    fn print_clone<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::std::clone::Clone for {ident}{gpb} {{
    fn clone(&self) -> Self {{
        use ::puroro_internal::FieldClone;
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

    fn print_default<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        if !self.frag_gen.is_default_available() {
            return Ok(());
        }
        (format!(
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
        ),)
            .write_into(output)
    }

    fn print_msg_deser_from_iter<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        if !self.frag_gen.is_deser_from_iter_available() {
            return Ok(());
        }
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::deser::DeserializableMessageFromIter for {ident}{gpb} {{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {{
        use ::puroro_internal::FieldDeserFromIter;
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
        Ok(true)
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

    fn print_msg_deser_from_slice_using_from_iter<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (format!(
            "\
{cfg}
impl{gp} ::puroro::DeserializableFromSlice for {ident}{gpb} {{
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {{
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }}
}}\n",
            ident = self.frag_gen.struct_ident(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(&[]),
            gpb = self.frag_gen.struct_generic_params_bounds(&[]),
        ),)
            .write_into(output)
    }

    fn print_msg_deser_from_slice_for_slice_view<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for {ident}{gpb} {{
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: &'slice [u8],
        enclosing_slice: &'slice [u8],
    ) -> ::puroro::Result<bool>
    {{
        use ::puroro_internal::FieldDeserFromSlice;
        use ::puroro_internal::tags;
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
    <{type_} as FieldDeserFromSlice<
        tags::{type_tag}, 
        tags::{label_tag}>>
    ::deser(&mut self.{ident}, field, slice_from_this_field, enclosing_slice)?;
}}\n",
                            number = field.number(),
                            ident = field.native_ident()?,
                            type_ = self.frag_gen.field_type_for(field)?,
                            type_tag = self.frag_gen.type_tag_for(field)?,
                            label_tag = field.label_tag()?,
                        ))
                    })),
                    "_ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,\n",
                ),
            ),
            "        \
        }}
        Ok(true)
    }}
}}\n",
        )
            .write_into(output)
    }

    fn print_msg_ser<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            match self.context.impl_type() {
                ImplType::Default => seq((
                    format!(
                        "\
{cfg}
impl{gp} ::puroro_internal::ser::SerializableMessage for {ident}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        use ::puroro_internal::FieldSer;
        use ::puroro_internal::tags;\n",
                        ident = self.frag_gen.struct_ident(self.msg)?,
                        cfg = self.frag_gen.cfg_condition(),
                        gp = self.frag_gen.struct_generic_params(&[]),
                        gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                    ),
                    indent_n::<_, W>(
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
                )),
                ImplType::SliceView { .. } => format!(
                    "\
{cfg}
impl{gp} ::puroro_internal::ser::SerializableMessage for {ident}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        for ld_slice in self.puroro_internal.slices() {{
            serializer.serialize_raw_fields(ld_slice.as_slice())?;
        }}
        Ok(())
    }}
}}\n",
                    ident = self.frag_gen.struct_ident(self.msg)?,
                    cfg = self.frag_gen.cfg_condition(),
                    gp = self.frag_gen.struct_generic_params(&[]),
                    gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                )
                .into(),
            },
            format!(
                "\
{cfg}
impl{gp} ::puroro::Serializable for {name}{gpb} {{
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {{
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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

    fn print_impl_trait<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
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
                        GetterMethods::BareField(decl) => {
                            // Getter method returns bare type `T`
                            match (self.context.impl_type(), field.type_()?) {
                                (ImplType::Default, FieldType::Message(_))
                                | (_, FieldType::String)
                                | (_, FieldType::Bytes) => format!(
                                    "{decl} {{\n    self.{ident}.as_ref()\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?
                                ),
                                (ImplType::SliceView { .. }, FieldType::Message(m)) => format!(
                                    "\
{decl} {{
    {msg}::try_new_with_parent(
        self.{ident}.as_ref(),
        {field_number},
        &self.puroro_internal
    ).expect(\"Invalid input slice. Consider checking the slice content earlier (TBD).\")
}}\n",
                                    decl = decl,
                                    msg = self.frag_gen.type_name_of_msg(m)?,
                                    ident = field.native_ident()?,
                                    field_number = field.number(),
                                ),
                                _ => format!(
                                    "{decl} {{\n    self.{ident}.clone()\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?
                                ),
                            }
                        }
                        GetterMethods::OptionalField(decl) => {
                            // Getter method returns type `Option<T>`
                            match (self.context.impl_type(), field.type_()?) {
                                (ImplType::Default, FieldType::Message(_))
                                | (_, FieldType::String)
                                | (_, FieldType::Bytes) => format!(
                                    "{decl} {{\n    self.{ident}.as_deref()\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?
                                ),
                                (ImplType::SliceView { .. }, FieldType::Message(m)) => format!(
                                    "\
{decl} {{
    if self.{ident}.is_some() {{
        {msg}::try_new_with_parent(
            self.{ident}.as_ref(),
            {field_number},
            &self.puroro_internal
        ).expect(\"Invalid input slice. Consider checking the slice content earlier (TBD).\")
    }} else {{
        None
    }}
}}\n",
                                    decl = decl,
                                    msg = self.frag_gen.type_name_of_msg(m)?,
                                    ident = field.native_ident()?,
                                    field_number = field.number(),
                                ),
                                _ => format!(
                                    "{decl} {{\n    self.{ident}.clone()\n}}\n",
                                    decl = decl,
                                    ident = field.native_ident()?
                                ),
                            }
                        }
                        GetterMethods::RepeatedField {
                            return_type_ident,
                            return_type_bound: _,
                            get_decl,
                        }
                        | GetterMethods::MapField {
                            return_type_ident,
                            return_type_bound: _,
                            get_decl,
                        } => {
                            format!(
                                "\
type {return_type_ident} = {type_name};
{get_decl} {{
    &self.{ident}
}}\n",
                                return_type_ident = return_type_ident,
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

    fn print_impl_map_entry<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        if !self.msg.is_map_entry() {
            return Ok(());
        }
        let (key_field, value_field) = self.msg.key_value_of_map_entry()?;
        (format!(
            "\
{cfg}
impl{gp} ::puroro_internal::MapEntry for {entry_type} {{
    type KeyType = {key_type};
    type ValueType = {value_type};
    fn ser_kv<T: ::puroro_internal::ser::MessageSerializer>(
        key: &Self::KeyType,
        value: &Self::ValueType,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {{
        use ::puroro_internal::FieldSer;
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
        use ::puroro_internal::FieldTakeOrInit;
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

    fn print_impl_field_new<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            match (self.context.impl_type(), self.context.alloc_type()) {
                (ImplType::Default, AllocatorType::Default) => {
                    format!(
                        "\
impl{gp} ::puroro_internal::FieldNew<'a> for {name}{gpb} {{
    fn new() -> Self {{
        Default::default()
    }}
}}\n",
                        name = self.frag_gen.struct_ident(self.msg)?,
                        gp = self.frag_gen.struct_generic_params(&["'a"]),
                        gpb = self.frag_gen.struct_generic_params_bounds(&[]),
                    )
                }
                (ImplType::Default, AllocatorType::Bumpalo) => {
                    format!(
                        "\
{cfg}
impl{gp} ::puroro_internal::FieldNew<'bump> for {name}{gpb} {{
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
                _ => "".to_string(),
            },
        )
            .write_into(output)
    }
}
