use super::message_frags::MessageImplFragmentGenerator;
use super::message_traits::MessageTraitCodeGenerator;
use super::writer::{func, indent, indent_n, iter, seq, IntoFragment};
use crate::context::{AllocatorType, Context, ImplType};
use crate::generators::message_traits::{AssociatedType, FieldLabelType};
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
                func(|output| self.print_msg_merge_from_iter(output)),
                match self.context.impl_type() {
                    ImplType::Default => {
                        func(|output| self.print_msg_deser_from_slice_using_from_iter(output))
                    }
                    ImplType::SliceView => {
                        func(|output| self.print_msg_deser_from_slice_for_slice_view(output))
                    }
                },
                func(|output| self.print_msg_ser(output)),
            ),
            func(|output| self.print_impl_trait(output)),
            func(|output| self.print_impl_message(output)),
            func(|output| self.print_impl_map_entry(output)),
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
                gp = self.frag_gen.struct_generic_params(),
            ),
            indent((
                iter(self.msg.fields().map(|field| {
                    Ok(format!(
                        "{vis}{name}: {type_},\n",
                        vis = self.frag_gen.field_visibility(),
                        name = field.native_ident()?,
                        type_ = self.frag_gen.field_type_name(field)?,
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
        match (self.context.impl_type(), self.context.alloc_type()) {
            (ImplType::Default, _) => self.new_method_default_impl(output),
            (ImplType::SliceView { .. }, AllocatorType::Default) => {
                self.new_method_slice_view_impl(output)
            }
            _ => {
                unreachable!()
            }
        }
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
{cfg}
impl{gp} {ident}{gpb} {{
    pub {decl} {{
        Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
                decl = self.frag_gen.new_method_declaration()
            ),
            indent_n(
                3,
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
            "        \
        }}
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
            format!(
                "\
{cfg}
impl{gp} {ident}{gpb} {{
    fn new() -> Self {{
        Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                3,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        "\
::puroro_internal::InternalDataForSliceViewStruct::new()",
                    )
                }),
            ),
            "        \
        }}
    }}
}}",
            format!(
                "\
{cfg}
impl{gp} {ident}{gpb} {{
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {{
        let mut new_self = Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params().remove("S"),
                gpb = self
                    .frag_gen
                    .struct_generic_params_bounds()
                    .replace("S", "&'slice [u8]"),
            ),
            indent_n(
                3,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        "\
::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice)",
                    )
                }),
            ),
            "        \
        }};
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }}
}}",
            format!(
                "\
{cfg}
impl{gp} {ident}{gpb} 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {{
        let mut new_self = Self {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self
                    .frag_gen
                    .struct_generic_params()
                    .remove("S")
                    .push("'par")
                    .push("SS"),
                gpb = self
                    .frag_gen
                    .struct_generic_params_bounds()
                    .replace("S", "::puroro_internal::SourceLdSlices<'slice, 'par, SS>"),
            ),
            indent_n(
                3,
                func(|output| {
                    self.new_method_self_members(
                        output,
                        "\
::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
    field_in_parent, field_number_in_parent, parent_internal_data)",
                    )
                }),
            ),
            "        \
        }};
        for ld_slice in new_self.puroro_internal.source_ld_slices() {{
            ld_slice?.merge_into_message(&mut new_self)?;
        }}
        Ok(new_self)
    }}
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
                gp = self
                    .frag_gen
                    .struct_generic_params()
                    .replace("S", "S: ::std::clone::Clone"),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                3,
                iter(self.msg.fields().map(|field| {
                    Ok(format!(
                        "{ident}: {clone},\n",
                        ident = field.native_ident()?,
                        clone = self.frag_gen.field_clone(
                            field.native_ident()?,
                            &self.frag_gen.field_type_name(field)?
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
            gp = self.frag_gen.struct_generic_params(),
            gpb = self.frag_gen.struct_generic_params_bounds(),
        ),)
            .write_into(output)
    }

    fn print_msg_merge_from_iter<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        if !self.frag_gen.is_merge_from_iter_available() {
            return Ok(());
        }
        (
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::deser::MergeableMessageFromIter for {ident}{gpb} {{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {{
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                3,
                (
                    iter(self.msg.fields().map(|field| -> Result<_> {
                        Ok(format!(
                            "\
{number} => {{
    <{type_} as FieldMergeFromIter<
        tags::{type_tag}, 
        tags::{label_tag}>>
    ::merge(&mut self.{ident}, field, {default_func})?;
}}\n",
                            number = field.number(),
                            ident = field.native_ident()?,
                            type_ = self.frag_gen.field_type_name(field)?,
                            type_tag = self.frag_gen.type_tag_ident_gp(field, None)?,
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
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {{
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }}
}}\n",
                name = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
        )
            .write_into(output)
    }

    fn print_msg_deser_from_slice_using_from_iter<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        if !self.frag_gen.is_default_available() {
            return Ok(());
        }
        (format!(
            "\
{cfg}
impl{gp} ::puroro::DeserializableFromSlice<'slice> for {ident}{gpb} {{
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {{
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }}
}}\n",
            ident = self.frag_gen.struct_ident(self.msg)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params().push("'slice"),
            gpb = self.frag_gen.struct_generic_params_bounds(),
        ))
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
impl{gp} ::puroro::DeserializableFromSlice<'slice> for {ident}{gpb} {{
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {{
        Self::try_new_with_slice(slice)
    }}
}}
{cfg}
impl{gp} ::std::convert::TryFrom<&'slice [u8]> for {ident}{gpb} {{
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {{
        Self::try_new_with_slice(value)
    }}
}}
\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self
                    .frag_gen
                    .struct_generic_params()
                    .push("'slice")
                    .remove("S"),
                gpb = self
                    .frag_gen
                    .struct_generic_params_bounds()
                    .replace("S", "&'slice [u8]"),
            ),
            format!(
                "\
{cfg}
impl{gp} ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for {ident}{gpb} {{
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {{
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro_internal::tags;
        match field_number {{\n",
                ident = self.frag_gen.struct_ident(self.msg)?,
                cfg = self.frag_gen.cfg_condition(),
                gp = self.frag_gen.struct_generic_params(),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent_n(
                3,
                (
                    iter(self.msg.fields().map(|field| -> Result<_> {
                        Ok(format!(
                            "\
{number} => {{
    <{type_} as FieldMergeFromSlice<
        tags::{type_tag}, 
        tags::{label_tag}>>
    ::merge(&mut self.{ident}, field, slice_from_this_field, enclosing_slice)?;
}}\n",
                            number = field.number(),
                            ident = field.native_ident()?,
                            type_ = self.frag_gen.field_type_name(field)?,
                            type_tag = self.frag_gen.type_tag_ident_gp(field, None)?,
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
                        gp = self.frag_gen.struct_generic_params(),
                        gpb = self.frag_gen.struct_generic_params_bounds(),
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
                                type_ = self.frag_gen.field_type_name(field)?,
                                type_tag = self.frag_gen.type_tag_ident_gp(field, None)?,
                                label_tag = field.label_tag()?,
                            ))
                        })),
                    ),
                    "        \
        Ok(())
    }}
}}\n",
                )),
                ImplType::SliceView => format!(
                    "\
{cfg}
impl{gp} ::puroro_internal::ser::SerializableMessage for {ident}{gpb} {{
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {{
        for ld_slice in self.puroro_internal.source_ld_slices() {{
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }}
        Ok(())
    }}
}}\n",
                    ident = self.frag_gen.struct_ident(self.msg)?,
                    cfg = self.frag_gen.cfg_condition(),
                    gp = self
                        .frag_gen
                        .struct_generic_params()
                        .replace("S", "S: ::puroro_internal::SliceSource<'slice>"),
                    gpb = self.frag_gen.struct_generic_params_bounds(),
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
                gp = self
                    .frag_gen
                    .struct_generic_params()
                    .replace("S", "S: ::puroro_internal::SliceSource<'slice>"),
                gpb = self.frag_gen.struct_generic_params_bounds(),
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
                gp = self
                    .frag_gen
                    .struct_generic_params()
                    .replace("S", "S: ::puroro_internal::SliceSource<'slice>"),
                gpb = self.frag_gen.struct_generic_params_bounds(),
            ),
            indent((iter(self.msg.fields().map(|field| -> Result<_> {
                let getter_methods = self.traits_gen.generate_getter_method_decls(field)?;
                let maybe_message_associated_type = if let (
                    Some(AssociatedType {
                        ident,
                        gp,
                        where_clause,
                        ..
                    }),
                    FieldType::Message(m),
                ) =
                    (getter_methods.maybe_msg_type, field.type_()?)
                {
                    let slice_source = if matches!(field.label()?, FieldLabel::Repeated) {
                        "&'slice [u8]"
                    } else {
                        "::puroro_internal::SourceLdSlices<'slice, 'this, S>"
                    };
                    format!(
                        "type {ident}{gp} {where_clause} = {actual_type_name};\n",
                        ident = ident,
                        gp = gp,
                        where_clause = where_clause,
                        actual_type_name =
                            self.frag_gen.type_name_of_msg(m, &[("S", slice_source)])?,
                    )
                } else {
                    "".to_string()
                };
                let body = match (
                    self.context.impl_type(),
                    self.traits_gen
                        .generate_getter_method_decls(field)?
                        .field_label_type,
                    field.type_()?,
                ) {
                    (
                        ImplType::SliceView,
                        FieldLabelType::BareField { get_decl },
                        FieldType::Message(m),
                    ) => {
                        format!(
                            "\
{decl} {{
    ::std::borrow::Cow::Owned(
        {msg}::try_new_with_parent(
            self.{ident}.clone(),
            {field_number},
            &self.puroro_internal
        ).expect(\"Invalid input slice. Consider checking the slice content earlier (TBD).\")
    )
}}\n",
                            decl = get_decl,
                            msg = self.frag_gen.type_name_of_msg(
                                m,
                                &[("S", "::puroro_internal::SourceLdSlices<'slice, 'this, S>")]
                            )?,
                            ident = field.native_ident()?,
                            field_number = field.number(),
                        )
                    }
                    (
                        ImplType::SliceView,
                        FieldLabelType::OptionalField { get_decl },
                        FieldType::Message(m),
                    ) => format!(
                        "\
{decl} {{
    self.{ident}.as_ref().map(|field| {{
        ::std::borrow::Cow::Owned(
            {msg}::try_new_with_parent(
                ::std::option::Option::Some(field),
                {field_number},
                &self.puroro_internal
            ).expect(\"Invalid input slice. Consider checking the slice content earlier (TBD).\")
        )
    }})
}}\n",
                        decl = get_decl,
                        msg = self.frag_gen.type_name_of_msg(
                            m,
                            &[("S", "::puroro_internal::SourceLdSlices<'slice, 'this, S>")]
                        )?,
                        ident = field.native_ident()?,
                        field_number = field.number(),
                    ),
                    (
                        ImplType::SliceView,
                        FieldLabelType::BareField { get_decl },
                        FieldType::String | FieldType::Bytes,
                    ) => format!(
                        "{decl} {{
    ::std::borrow::Cow::Borrowed(self.{ident}.as_ref())
}}\n",
                        decl = get_decl,
                        ident = field.native_ident()?,
                    ),
                    (
                        ImplType::SliceView,
                        FieldLabelType::OptionalField { get_decl },
                        FieldType::String | FieldType::Bytes,
                    ) => format!(
                        "{decl} {{
    self.{ident}.as_ref().map(|x| {{
        ::std::borrow::Cow::Borrowed(x.as_ref())
    }})
}}\n",
                        decl = get_decl,
                        ident = field.native_ident()?,
                    ),
                    (
                        ImplType::SliceView,
                        FieldLabelType::RepeatedField {
                            repeated_type:
                                AssociatedType {
                                    ident: type_ident,
                                    gp: type_gp,
                                    where_clause: type_where,
                                    ..
                                },
                            get_decl,
                        }
                        | FieldLabelType::MapField {
                            repeated_type:
                                AssociatedType {
                                    ident: type_ident,
                                    gp: type_gp,
                                    where_clause: type_where,
                                    ..
                                },
                            get_decl,
                        },
                        _,
                    ) => format!(
                        "\
type {type_ident}{type_gp} {type_where} =
    ::puroro_internal::RepeatedSliceViewField::<
        'slice,
        'this,
        S,
        ::puroro_internal::tags::{type_tag}
    >;
{get_decl} {{
    ::puroro_internal::RepeatedSliceViewField::new(
        self.{ident}.as_ref(),
        {field_number},
        &self.puroro_internal,
    )
}}\n",
                        type_ident = type_ident,
                        type_gp = type_gp,
                        type_tag = self
                            .frag_gen
                            .type_tag_ident_gp(field, &[("S", "&'slice [u8]")])?,
                        type_where = type_where,
                        get_decl = get_decl,
                        ident = field.native_ident()?,
                        field_number = field.number(),
                    ),
                    (
                        ImplType::Default,
                        FieldLabelType::BareField { get_decl },
                        FieldType::Message(_) | FieldType::String | FieldType::Bytes,
                    ) => format!(
                        "\
{decl} {{
    ::std::borrow::Cow::Borrowed(self.{ident}.as_ref())
}}\n",
                        decl = get_decl,
                        ident = field.native_ident()?
                    ),
                    (
                        ImplType::Default,
                        FieldLabelType::OptionalField { get_decl },
                        FieldType::Message(_) | FieldType::String | FieldType::Bytes,
                    ) => format!(
                        "\
{decl} {{
    self.{ident}.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
}}\n",
                        decl = get_decl,
                        ident = field.native_ident()?
                    ),

                    (
                        _,
                        FieldLabelType::BareField { get_decl }
                        | FieldLabelType::OptionalField { get_decl },
                        _,
                    ) => {
                        format!(
                            "{decl} {{\n    self.{ident}.clone()\n}}\n",
                            decl = get_decl,
                            ident = field.native_ident()?
                        )
                    }

                    (
                        ImplType::Default,
                        FieldLabelType::RepeatedField {
                            repeated_type:
                                AssociatedType {
                                    ident: type_ident,
                                    gp: type_gp,
                                    where_clause: type_where_clause,
                                    ..
                                },
                            get_decl,
                        }
                        | FieldLabelType::MapField {
                            repeated_type:
                                AssociatedType {
                                    ident: type_ident,
                                    gp: type_gp,
                                    where_clause: type_where_clause,
                                    ..
                                },
                            get_decl,
                        },
                        _,
                    ) => format!(
                        "\
type {type_ident}{type_gp} {type_where} = &'this {type_name};
{get_decl} {{
    &self.{ident}
}}\n",
                        type_ident = type_ident,
                        type_gp = type_gp,
                        type_name = self.frag_gen.field_type_name(field)?,
                        type_where = type_where_clause,
                        get_decl = get_decl,
                        ident = field.native_ident()?,
                    ),
                };
                Ok(format!("{}{}", maybe_message_associated_type, body))
            })),)),
            "}}\n",
        )
            .write_into(output)
    }

    fn print_impl_message<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (format!(
            "\
{cfg}
impl{gp} ::puroro::Message for {struct_ident}{gpb} {{
    type InternalData = {internal_data_type};
    fn puroro_internal_data(&self) -> &Self::InternalData {{
        &self.puroro_internal
    }}
    type BoxedType = {boxed_type};
    fn into_boxed(self) -> Self::BoxedType {{
        {box_new}
    }}
}}\n",
            struct_ident = self.frag_gen.struct_ident(self.msg)?,
            internal_data_type = self.frag_gen.internal_data_type(),
            boxed_type = self.frag_gen.boxed_type("Self"),
            box_new = self.frag_gen.box_new("self"),
            cfg = self.frag_gen.cfg_condition(),
            gp = self
                .frag_gen
                .struct_generic_params()
                .replace("S", "S: ::puroro_internal::SliceSource<'slice>"),
            gpb = self.frag_gen.struct_generic_params_bounds(),
        ),)
            .write_into(output)
    }

    fn print_impl_map_entry<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        if !self.msg.is_map_entry() {
            return Ok(());
        } else {
            match self.context.impl_type() {
                ImplType::Default => self.print_impl_map_entry_normal(output),
                ImplType::SliceView => self.print_impl_map_entry_slice_view(output),
            }
        }
    }

    fn print_impl_map_entry_normal<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        let (key_field, value_field) = self.msg.key_value_of_map_entry()?;
        (format!(
            "\
{cfg}
impl{gp} ::puroro_internal::MapEntryForNormalImpl for {entry_type} {{
    type OwnedKeyType = {owned_key_type};
    type OwnedValueType = {owned_value_type};
    fn into_tuple(self) -> (Self::OwnedKeyType, Self::OwnedValueType) {{
        use ::puroro_internal::FieldTakeOrInit;
        use ::puroro::InternalData;
        (
            {take_key}, 
            {take_value},
        )
    }}
    fn ser_kv<T, Q, R>(key: &Q, value: &R, serializer: &mut T, ) -> ::puroro::Result<()> 
    where
        T: ::puroro_internal::ser::MessageSerializer,
        Self::OwnedKeyType: ::std::borrow::Borrow<Q>,
        Self::OwnedValueType: ::std::borrow::Borrow<R>,
    {{
        use ::puroro_internal::FieldSer;
        use ::puroro_internal::tags;
        <{owned_key_type} as FieldSer<
            tags::{key_type_tag}, 
            tags::Required>>
            ::ser(key, serializer, 1)?;
        <{owned_value_type} as FieldSer<
            tags::{value_type_tag}, 
            tags::Required>>
            ::ser(value, serializer, 2)?;
        Ok(())
    }}
}}\n",
            entry_type = self.frag_gen.type_name_of_msg(self.msg, None)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(),
            owned_key_type = self.frag_gen.map_owned_key_type_name(key_field)?,
            key_type_tag = self.frag_gen.type_tag_ident_gp(key_field, None)?,
            take_key = self.frag_gen.field_take_or_init(key_field)?,
            owned_value_type = self.frag_gen.map_owned_value_type_name(value_field)?,
            value_type_tag = self.frag_gen.type_tag_ident_gp(value_field, None)?,
            take_value = self.frag_gen.field_take_or_init(value_field)?,
        ),)
            .write_into(output)
    }

    fn print_impl_map_entry_slice_view<W: std::fmt::Write>(
        &self,
        output: &mut Indentor<W>,
    ) -> Result<()> {
        let (key_field, value_field) = self.msg.key_value_of_map_entry()?;
        (format!(
            "\
{cfg}
impl{gp} ::puroro_internal::MapEntryForSliceViewImpl<'slice> for {entry_type} {{
    type OwnedKeyType = {owned_key_type};
    type ValueGetterType = {value_getter_type};
    fn key_eq<Q>(&self, key: &Q)
    where
        Self::OwnedKeyType: ::std::borrow::Borrow<Q>,
        Q: Eq + ?Sized
    {{
        <Self as {message_trait_type}>::key(self).eq(key)
    }}
    fn value(&self) -> Self::ValueGetterType {{
        <Self as {message_trait_type}>::value(self)
    }}
}}\n",
            entry_type = self.frag_gen.type_name_of_msg(self.msg, None)?,
            message_trait_type = self
                .traits_gen
                .trait_ident_with_relative_path(self.msg, self.msg.package()?)?,
            cfg = self.frag_gen.cfg_condition(),
            gp = self.frag_gen.struct_generic_params(),
            owned_key_type = self.frag_gen.map_owned_key_type_name(key_field)?,
            value_getter_type = self.traits_gen.map_value_getter_type_name(value_field)?,
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
                        gp = self.frag_gen.struct_generic_params().push("'a"),
                        gpb = self.frag_gen.struct_generic_params_bounds(),
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
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {{
        Self::new_in(bump)
    }}
}}\n",
                        cfg = self.frag_gen.cfg_condition(),
                        name = self.frag_gen.struct_ident(self.msg)?,
                        gp = self.frag_gen.struct_generic_params(),
                        gpb = self.frag_gen.struct_generic_params_bounds(),
                    )
                }
                _ => "".to_string(),
            },
        )
            .write_into(output)
    }
}
