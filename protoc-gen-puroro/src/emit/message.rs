//! Emit a message module body from a [`MessageFieldPlan`].
//!
//! Singular and repeated scalar / string / bytes / bool / enum / message
//! fields (including IMPLICIT / EXPLICIT / LEGACY_REQUIRED and bool
//! `BitPacked`), real oneof groups, and maps (legal keys × Copy scalar /
//! bool values).

use super::defaults;
use super::ident::{escape_ident, is_simple_ident, to_pascal_case};
use super::oneof::{self, OneofEmit, OneofVariantEmit};
use super::type_path::{fqn_to_enum_root_path, fqn_to_message_root_path};
use crate::default_value::{CustomDefault, DefaultLit};
use crate::descriptor::features::EnumType;
use crate::error::{Error, Result};
use crate::field_kind::{
    FieldKind, MessageFieldPlan, MessageMember, PlannedField, PlannedLayout, PlannedPresence,
    RepeatedEncodingKind, WireTypeKind, bit_array_byte_len,
};
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::syn::{Item, Type, parse_quote};

enum FieldEmit {
    Singular(Box<ScalarEmit>),
    Repeated(Box<RepeatedEmit>),
    Map(Box<MapEmit>),
    Oneof(Box<OneofEmit>),
}

/// Owned per-field facts needed for `quote!` (avoids borrowing `MessageFieldPlan`).
struct ScalarEmit {
    name: Ident,
    name_str: String,
    field_const: Ident,
    number: u32,
    marker: Type,
    presence_ty: Type,
    /// `L` type arg to emit. `None` omits it (`Inline` default). `Some(Inline)`
    /// when a custom `D` must follow (positional type args).
    layout_ty: Option<Type>,
    presence_bit: Option<(Ident, usize)>,
    value_bit: Option<(Ident, usize)>,
    style: AccessorStyle,
    /// Enums always use `.optional()` even when presence is IMPLICIT (DESIGN §4.6).
    optional_getter: bool,
    mut_target: Type,
    /// Return type for IMPLICIT value getters (`i32`, `&str`, …).
    implicit_ty: Type,
    /// Payload type inside `Optional<…>` (`i32`, `&'a str`, enum path, …).
    optional_ty: Type,
    /// Non-type-zero `[default = …]` marker (emitted into `mod defaults`).
    custom_default: Option<(CustomDefault, Vec<Item>)>,
    /// Shape of the generated `_mut` accessor return type.
    mut_style: SingularMutStyle,
}

struct RepeatedEmit {
    name: Ident,
    name_str: String,
    field_const: Ident,
    number: u32,
    marker: Type,
    encoding_ty: Type,
    style: RepeatedAccessorStyle,
    /// `as_slice` element type for packable / message repeated fields.
    slice_elem_ty: Type,
}

struct MapEmit {
    name: Ident,
    name_str: String,
    field_const: Ident,
    number: u32,
    key_marker: Type,
    value_marker: Type,
    /// User-facing key type in `MapRef` / mut traits (`str`, `i32`, …).
    key_view: Type,
    /// Shared view type for `MapRef` (`i32`, `str`, `Address<A>`, …).
    value_view: Type,
    /// `map<string, …>` — key view is `str`.
    string_key: bool,
    /// When set, pin `MutTarget` on the mutator return type (string / bytes).
    mut_target: Option<Type>,
}

#[derive(Clone, Copy)]
enum RepeatedAccessorStyle {
    /// `&[T]` + `values_mut()` → `Vec<T, A>`.
    Slice,
    /// `repeated string` — `RepeatedStringMut`.
    String,
    /// `repeated bytes` — `RepeatedBytesMut`.
    Bytes,
}

/// How a singular (non-message) `_mut` accessor is typed in generated code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SingularMutStyle {
    /// `impl DerefMut<Target = …>` using [`ScalarEmit::mut_target`] / variant `mut_target`.
    DerefMut,
    /// `impl ::puroro::StringMut<A>` (SSO layout; hides `puroro-rt` mutator).
    SsoString,
    /// `impl ::puroro::BytesMut<A>` (SSO layout; hides `puroro-rt` mutator).
    SsoBytes,
}

impl SingularMutStyle {
    pub(super) fn from_layout(wire: &WireTypeKind<'_>, layout: &PlannedLayout) -> Self {
        match (wire, layout) {
            (WireTypeKind::String { .. }, PlannedLayout::InlineOrHeap { .. }) => Self::SsoString,
            (WireTypeKind::Bytes { .. }, PlannedLayout::InlineOrHeap { .. }) => Self::SsoBytes,
            _ => Self::DerefMut,
        }
    }

    /// Return type of `_mut` for lifetime `lt` (e.g. `'s`) and `mut_target`.
    pub(super) fn return_ty(self, lt: &TokenStream, mut_target: &Type) -> Type {
        match self {
            Self::SsoString => parse_quote! { impl ::puroro::StringMut<A> + #lt },
            Self::SsoBytes => parse_quote! { impl ::puroro::BytesMut<A> + #lt },
            Self::DerefMut => {
                parse_quote! { impl ::core::ops::DerefMut<Target = #mut_target> + #lt }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum AccessorStyle {
    Implicit,
    Explicit,
    LegacyRequired,
    /// Nested message — `Option<&M>` / `&mut M` via pointer presence.
    Message,
}

impl FieldEmit {
    fn name(&self) -> &Ident {
        match self {
            Self::Singular(f) => &f.name,
            Self::Repeated(f) => &f.name,
            Self::Map(f) => &f.name,
            Self::Oneof(o) => &o.name,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            Self::Singular(f) => &f.name_str,
            Self::Repeated(f) => &f.name_str,
            Self::Map(f) => &f.name_str,
            Self::Oneof(o) => &o.name_str,
        }
    }

    fn field_const(&self) -> Option<&Ident> {
        match self {
            Self::Singular(f) => Some(&f.field_const),
            Self::Repeated(f) => Some(&f.field_const),
            Self::Map(f) => Some(&f.field_const),
            Self::Oneof(_) => None,
        }
    }

    fn number(&self) -> Option<u32> {
        match self {
            Self::Singular(f) => Some(f.number),
            Self::Repeated(f) => Some(f.number),
            Self::Map(f) => Some(f.number),
            Self::Oneof(_) => None,
        }
    }

    fn new_in_ctor(&self) -> TokenStream {
        match self {
            Self::Singular(_) => quote! { ::puroro_rt::SingularField::new_in },
            Self::Repeated(_) => quote! { ::puroro_rt::RepeatedField::new_in },
            Self::Map(_) => quote! { ::puroro_rt::MapField::new_in },
            Self::Oneof(_) => quote! { ::puroro_rt::OneofSlot::new_in },
        }
    }
}

/// Items for the parent module (struct + impls) and the snake_case companion.
pub(super) struct RenderedMessageItems {
    pub type_items: Vec<Item>,
    pub companion_items: Vec<Item>,
}

/// Render the message struct (parent) and companion (`FIELD_*`, defaults, oneofs).
pub(super) fn render_items(
    field_plan: &MessageFieldPlan<'_>,
    name: &Ident,
    name_str: &str,
    companion: &Ident,
) -> Result<RenderedMessageItems> {
    let fields = collect_fields(field_plan, companion)?;
    let bits_bytes = bit_array_byte_len(field_plan.bit_count());
    let bits_ty = quote! {
        ::bitvec::array::BitArray<[u8; #bits_bytes], ::bitvec::order::Lsb0>
    };

    let bit_consts = render_bit_consts(&fields);
    let field_consts = render_field_consts(&fields);
    let defaults_module = render_defaults_module(&fields)?;
    let struct_fields = render_struct_fields(&fields, companion);
    let new_in_fields = render_new_in_fields(&fields);
    let accessors = render_accessors(&fields, companion);
    let visit_shared = render_visit_calls(&fields, VisitKind::Shared);
    let visit_pair = render_visit_calls(&fields, VisitKind::Pair);
    let visit_pair_mut = render_visit_calls(&fields, VisitKind::PairMut);
    let visit_mut = render_visit_calls(&fields, VisitKind::Mut);
    let merge_arms = render_merge_arms(&fields, companion);
    let validate_body = render_validate(&fields);
    let oneof_modules = render_oneof_modules(&fields, &bits_ty)?;
    let empty_visit_sink = if fields.is_empty() {
        quote! { let _ = v; }
    } else {
        TokenStream::new()
    };
    let empty_pair_sink = if fields.is_empty() {
        quote! { let _ = (other, v); }
    } else {
        TokenStream::new()
    };
    let empty_pair_mut_sink = if fields.is_empty() {
        quote! { let _ = (dst, v); }
    } else {
        TokenStream::new()
    };

    let common_init = if fields.is_empty() {
        quote! {
            _common: ::puroro_rt::MessageCommon::new_in(
                ::bitvec::array::BitArray::ZERO,
                alloc,
            ),
        }
    } else {
        quote! {
            _common: ::puroro_rt::MessageCommon::new_in(
                ::bitvec::array::BitArray::ZERO,
                alloc.clone(),
            ),
        }
    };

    let companion_items = super::parse::parse_items(quote! {
        #bit_consts
        #field_consts
        #defaults_module
        #(#oneof_modules)*
    })?;

    let type_items = super::parse::parse_items(quote! {
        // @generated message body from protoc-gen-puroro.

        pub struct #name<
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone =
                ::allocator_api2::alloc::Global,
        > {
            _common: ::puroro_rt::MessageCommon<#bits_ty, A>,
            #(#struct_fields)*
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> #name<A> {
            pub fn new_in(alloc: A) -> Self {
                Self {
                    #common_init
                    #(#new_in_fields)*
                }
            }

            #(#accessors)*

            // Internal field walks for codec / Clone / Eq / Drop — not part of the
            // public message API (must not surface `puroro_rt` in pub signatures).
            fn visit_fields<V: ::puroro_rt::FieldVisitor<::puroro_rt::MessageCommon<#bits_ty, A>>>(
                &self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_visit_sink
                #(#visit_shared)*
                ::core::ops::ControlFlow::Continue(())
            }

            fn visit_field_pairs<
                V: ::puroro_rt::FieldPairVisitor<::puroro_rt::MessageCommon<#bits_ty, A>>,
            >(
                &self,
                other: &Self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_pair_sink
                #(#visit_pair)*
                ::core::ops::ControlFlow::Continue(())
            }

            fn visit_field_pairs_mut<
                V: ::puroro_rt::FieldPairVisitorMut<::puroro_rt::MessageCommon<#bits_ty, A>>,
            >(
                &self,
                dst: &mut Self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_pair_mut_sink
                #(#visit_pair_mut)*
                ::core::ops::ControlFlow::Continue(())
            }

            fn visit_fields_mut<
                V: ::puroro_rt::FieldVisitorMut<::puroro_rt::MessageCommon<#bits_ty, A>>,
            >(
                &mut self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_visit_sink
                #(#visit_mut)*
                ::core::ops::ControlFlow::Continue(())
            }
        }

        impl #name<::allocator_api2::alloc::Global> {
            pub fn new() -> Self {
                Self::new_in(::allocator_api2::alloc::Global)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone + ::core::default::Default>
            ::core::default::Default for #name<A>
        {
            fn default() -> Self {
                Self::new_in(A::default())
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::CloneIn<A>
            for #name<A>
        {
            fn clone_in(&self, alloc: A) -> Self {
                let mut dst = Self::new_in(alloc.clone());
                let mut v = ::puroro_rt::CloneFieldsVisitor::new(&self._common, &dst._common);
                let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
                let mut old = ::core::mem::replace(&mut dst._common, self._common.clone_in(alloc));
                old.deallocate();
                dst
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::clone::Clone
            for #name<A>
        {
            #[inline]
            fn clone(&self) -> Self {
                ::puroro_rt::CloneIn::clone_in(self, self._common.alloc.clone())
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::cmp::PartialEq
            for #name<A>
        {
            fn eq(&self, other: &Self) -> bool {
                matches!(
                    self.visit_field_pairs(
                        other,
                        &mut ::puroro_rt::FieldEqVisitor::new(&self._common, &other._common),
                    ),
                    ::core::ops::ControlFlow::Continue(())
                ) && self._common.unknown_fields_eq(&other._common)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::fmt::Debug
            for #name<A>
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut v =
                    ::puroro_rt::DebugStructVisitor::new(f.debug_struct(#name_str), &self._common);
                let _ = self.visit_fields(&mut v);
                v.finish()
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::ops::Drop
            for #name<A>
        {
            fn drop(&mut self) {
                let mut v = ::puroro_rt::FieldDeallocVisitor::new(&self._common);
                let _ = self.visit_fields_mut(&mut v);
                self._common.deallocate();
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::DeallocateIn<A>
            for #name<A>
        {
            #[inline]
            unsafe fn deallocate_in(self, _alloc: A) {
                ::core::mem::drop(self);
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::MessageEncode
            for #name<A>
        {
            fn encoded_len(&self, ctx: &mut ::puroro_rt::EncodeCtx) -> usize {
                let mut v = ::puroro_rt::EncodedLenVisitor::new(&self._common, ctx);
                let _ = self.visit_fields(&mut v);
                v.len + self._common.unknown_fields.len()
            }

            fn encode_raw<B: ::bytes::BufMut>(&self, ctx: &mut ::puroro_rt::EncodeCtx, buf: &mut B) {
                let _ = self.visit_fields(&mut ::puroro_rt::EncodeRawVisitor::new(
                    &self._common,
                    ctx,
                    buf,
                ));
                let unknown: &[u8] = &self._common.unknown_fields;
                ::bytes::BufMut::put_slice(buf, unknown);
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::MessageMerge
            for #name<A>
        {
            fn merge_from_with_depth<B: ::puroro::DecodeBuf>(
                &mut self,
                buf: &mut B,
                depth: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                if depth >= ::puroro::RECURSION_LIMIT {
                    return ::core::result::Result::Err(::puroro::DecodeError::RecursionLimitExceeded);
                }
                while ::bytes::Buf::has_remaining(buf) {
                    let (field_number, wire_type) = ::puroro_rt::decode::decode_tag(buf)?;
                    match field_number.as_u32() {
                        #(#merge_arms)*
                        _ => {
                            ::puroro_rt::decode::skip_field_and_save(
                                field_number,
                                wire_type,
                                buf,
                                &mut self._common.unknown_fields,
                                self._common.alloc.clone(),
                            )?;
                        }
                    }
                }
                ::core::result::Result::Ok(())
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::DefaultIn<A>
            for #name<A>
        {
            #[inline]
            fn default_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro::Message
            for #name<A>
        {
            type Alloc = A;

            fn new_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }

            fn encode<B: ::bytes::BufMut>(&self, buf: &mut B) {
                ::puroro_rt::encode_message(self, buf)
            }

            fn encode_to_vec(&self) -> ::std::vec::Vec<u8> {
                ::puroro_rt::encode_message_to_vec(self)
            }

            fn merge_from<B: ::bytes::Buf>(
                &mut self,
                buf: &mut B,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                ::puroro_rt::merge_message(self, buf)
            }

            fn unknown_fields(
                &self,
            ) -> impl ::core::iter::Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
                self._common.iter_unknown_fields()
            }

            fn validate(&self) -> ::core::result::Result<(), ::puroro::DecodeError> {
                #validate_body
            }
        }
    })?;

    Ok(RenderedMessageItems {
        type_items,
        companion_items,
    })
}

fn collect_fields(field_plan: &MessageFieldPlan<'_>, companion: &Ident) -> Result<Vec<FieldEmit>> {
    let mut out = Vec::new();
    for member in field_plan.members() {
        match member {
            MessageMember::Oneof(o) => {
                if !is_simple_ident(o.name()) {
                    return Err(Error::Codegen(format!(
                        "oneof name `{}` is not a simple Rust identifier",
                        o.name()
                    )));
                }
                if o.variants().is_empty() {
                    return Err(Error::Codegen(format!(
                        "oneof `{}` has no variants",
                        o.name()
                    )));
                }
                let mut variants = Vec::with_capacity(o.variants().len());
                for (i, field) in o.variants().iter().enumerate() {
                    variants.push(oneof_variant_emit(field, i)?);
                }
                let pascal = to_pascal_case(o.name());
                out.push(FieldEmit::Oneof(Box::new(OneofEmit {
                    name: escape_ident(o.name()),
                    name_str: o.name().to_owned(),
                    mod_name: escape_ident(o.name()),
                    shape_name: Ident::new(&pascal, Span::call_site()),
                    case_name: Ident::new(&format!("{pascal}Case"), Span::call_site()),
                    storage_name: Ident::new(&format!("{pascal}Storage"), Span::call_site()),
                    variants,
                })));
            }
            MessageMember::Field(field) => {
                if !is_simple_ident(field.name()) {
                    return Err(Error::Codegen(format!(
                        "field name `{}` is not a simple Rust identifier",
                        field.name()
                    )));
                }
                match field.kind() {
                    FieldKind::Map { key, value } => {
                        out.push(FieldEmit::Map(Box::new(map_emit(
                            field.name(),
                            field.field_const(),
                            field.number(),
                            key,
                            value,
                        )?)));
                    }
                    FieldKind::Repeated { wire, encoding } => {
                        out.push(FieldEmit::Repeated(Box::new(repeated_emit(
                            field.name(),
                            field.field_const(),
                            field.number(),
                            wire,
                            *encoding,
                        )?)));
                    }
                    FieldKind::Singular { presence, .. } => {
                        if matches!(presence, PlannedPresence::Oneof) {
                            return Err(Error::Codegen(format!(
                                "internal error: oneof field `{}` escaped as a top-level member",
                                field.name()
                            )));
                        }
                        out.push(FieldEmit::Singular(Box::new(scalar_emit(
                            field, companion,
                        )?)));
                    }
                }
            }
        }
    }
    Ok(out)
}

fn oneof_variant_emit(field: &PlannedField<'_>, index: usize) -> Result<OneofVariantEmit> {
    if !is_simple_ident(field.name()) {
        return Err(Error::Codegen(format!(
            "oneof variant name `{}` is not a simple Rust identifier",
            field.name()
        )));
    }
    let FieldKind::Singular {
        wire,
        presence,
        layout,
        custom_default,
    } = field.kind()
    else {
        return Err(Error::Codegen(format!(
            "oneof variant `{}` must be singular",
            field.name()
        )));
    };
    if !matches!(presence, PlannedPresence::Oneof) {
        return Err(Error::Codegen(format!(
            "internal error: oneof variant `{}` has presence {presence:?}",
            field.name()
        )));
    }

    let (layout_ty, value_bit) = match layout {
        PlannedLayout::Inline => (
            custom_default
                .is_some()
                .then(|| parse_quote! { ::puroro_rt::Inline }),
            None,
        ),
        PlannedLayout::BitPacked {
            value_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                // Const lives on the parent message module.
                Some(parse_quote! { ::puroro_rt::BitPacked<{ super::#ident }> }),
                Some((ident, *value_bit)),
            )
        }
        PlannedLayout::InlineOrHeap {
            heap_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                Some(parse_quote! { ::puroro_rt::InlineOrHeap<{ super::#ident }> }),
                Some((ident, *heap_bit)),
            )
        }
    };

    let views = wire_views(wire)?;
    let is_message = matches!(wire, WireTypeKind::Message(_));
    let is_bool = matches!(wire, WireTypeKind::Bool);
    let mut_style = SingularMutStyle::from_layout(wire, layout);
    let variant_pascal = to_pascal_case(field.name());
    Ok(OneofVariantEmit {
        name: escape_ident(field.name()),
        name_str: field.name().to_owned(),
        variant_name: Ident::new(&variant_pascal, Span::call_site()),
        type_param: Ident::new(&format!("T{index}"), Span::call_site()),
        field_alias: Ident::new(&format!("{variant_pascal}Field"), Span::call_site()),
        field_const: Ident::new(field.field_const(), Span::call_site()),
        number: field.number() as u32,
        marker: wire_marker_path(wire)?,
        layout_ty,
        value_bit,
        is_message,
        is_bool,
        mut_style,
        mut_target: views.mut_target,
        optional_ty: views.optional,
        custom_default: match custom_default {
            Some(custom) => Some((custom.clone(), defaults::render_marker_item(custom, wire)?)),
            None => None,
        },
    })
}

fn repeated_emit(
    name: &str,
    field_const: &str,
    number: i32,
    wire: &WireTypeKind<'_>,
    encoding: RepeatedEncodingKind,
) -> Result<RepeatedEmit> {
    let encoding_ty = match encoding {
        RepeatedEncodingKind::Packed => parse_quote! { ::puroro_rt::Packed },
        RepeatedEncodingKind::Expanded => parse_quote! { ::puroro_rt::Expanded },
    };
    let views = wire_views(wire)?;
    let (style, slice_elem_ty) = match wire {
        WireTypeKind::String { .. } => (RepeatedAccessorStyle::String, parse_quote! { () }),
        WireTypeKind::Bytes { .. } => (RepeatedAccessorStyle::Bytes, parse_quote! { () }),
        _ => {
            let Some(elem) = views.slice_elem else {
                return Err(Error::Codegen(format!(
                    "internal error: missing slice elem for repeated `{name}`"
                )));
            };
            (RepeatedAccessorStyle::Slice, elem)
        }
    };
    Ok(RepeatedEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        marker: wire_marker_path(wire)?,
        encoding_ty,
        style,
        slice_elem_ty,
    })
}

fn map_emit(
    name: &str,
    field_const: &str,
    number: i32,
    key: &WireTypeKind<'_>,
    value: &WireTypeKind<'_>,
) -> Result<MapEmit> {
    let (key_view, string_key) = map_key_view(key, name)?;
    let value_views = wire_views(value)?;
    Ok(MapEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        key_marker: wire_marker_path(key)?,
        value_marker: wire_marker_path(value)?,
        key_view,
        value_view: value_views.map_value,
        string_key,
        mut_target: value_views.map_value_mut,
    })
}

fn map_key_view(key: &WireTypeKind<'_>, field_name: &str) -> Result<(Type, bool)> {
    match key {
        WireTypeKind::String { .. } => Ok((parse_quote! { str }, true)),
        WireTypeKind::Bool
        | WireTypeKind::Int32
        | WireTypeKind::SInt32
        | WireTypeKind::SFixed32
        | WireTypeKind::Int64
        | WireTypeKind::SInt64
        | WireTypeKind::SFixed64
        | WireTypeKind::UInt32
        | WireTypeKind::Fixed32
        | WireTypeKind::UInt64
        | WireTypeKind::Fixed64 => Ok((wire_views(key)?.map_value, false)),
        other => Err(Error::Codegen(format!(
            "map field `{field_name}`: invalid map key type {other:?}"
        ))),
    }
}

fn scalar_emit(field: &PlannedField<'_>, companion: &Ident) -> Result<ScalarEmit> {
    let FieldKind::Singular {
        wire,
        presence,
        layout,
        custom_default,
    } = field.kind()
    else {
        return Err(Error::Codegen(format!(
            "internal error: scalar_emit on non-singular field `{}`",
            field.name()
        )));
    };
    let name = field.name();
    let field_const = field.field_const();
    let number = field.number();
    let custom_default = custom_default.as_ref();
    let (style, presence_ty, presence_bit) = match presence {
        PlannedPresence::Implicit => (
            AccessorStyle::Implicit,
            parse_quote! { ::puroro_rt::Implicit },
            None,
        ),
        PlannedPresence::Explicit { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                AccessorStyle::Explicit,
                parse_quote! { ::puroro_rt::Explicit<{ #companion::#ident }> },
                Some((ident, *bit)),
            )
        }
        PlannedPresence::LegacyRequired { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                AccessorStyle::LegacyRequired,
                parse_quote! { ::puroro_rt::LegacyRequired<{ #companion::#ident }> },
                Some((ident, *bit)),
            )
        }
        PlannedPresence::Message => (
            AccessorStyle::Message,
            parse_quote! { ::puroro_rt::Message },
            None,
        ),
        PlannedPresence::Oneof => {
            return Err(Error::Codegen(
                "internal error: oneof presence must use oneof_variant_emit".into(),
            ));
        }
    };

    let (layout_ty, value_bit) = match layout {
        PlannedLayout::Inline => (
            custom_default
                .is_some()
                .then(|| parse_quote! { ::puroro_rt::Inline }),
            None,
        ),
        PlannedLayout::BitPacked {
            value_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                Some(parse_quote! { ::puroro_rt::BitPacked<{ #companion::#ident }> }),
                Some((ident, *value_bit)),
            )
        }
        PlannedLayout::InlineOrHeap {
            heap_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                Some(parse_quote! { ::puroro_rt::InlineOrHeap<{ #companion::#ident }> }),
                Some((ident, *heap_bit)),
            )
        }
    };

    let views = wire_views(wire)?;
    let is_enum = matches!(wire, WireTypeKind::Enum { .. });
    let is_message = matches!(style, AccessorStyle::Message);
    Ok(ScalarEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        marker: wire_marker_path(wire)?,
        presence_ty,
        layout_ty,
        presence_bit,
        value_bit,
        style,
        // DESIGN §4.6: enum getters always project through Optional.
        optional_getter: !is_message
            && (is_enum
                || matches!(
                    style,
                    AccessorStyle::Explicit | AccessorStyle::LegacyRequired
                )),
        mut_target: views.mut_target,
        implicit_ty: views.implicit,
        optional_ty: views.optional,
        custom_default: match custom_default {
            Some(custom) => Some((custom.clone(), defaults::render_marker_item(custom, wire)?)),
            None => None,
        },
        mut_style: SingularMutStyle::from_layout(wire, layout),
    })
}

fn render_defaults_module(fields: &[FieldEmit]) -> Result<TokenStream> {
    let mut items: Vec<Item> = Vec::new();
    let mut needs_root = false;
    for field in fields {
        match field {
            FieldEmit::Singular(f) => {
                if let Some((custom, marker_items)) = &f.custom_default {
                    if matches!(custom.lit, DefaultLit::Enum { .. }) {
                        needs_root = true;
                    }
                    items.extend(marker_items.iter().cloned());
                }
            }
            FieldEmit::Oneof(o) => {
                for v in &o.variants {
                    if let Some((custom, marker_items)) = &v.custom_default {
                        if matches!(custom.lit, DefaultLit::Enum { .. }) {
                            needs_root = true;
                        }
                        items.extend(marker_items.iter().cloned());
                    }
                }
            }
            FieldEmit::Repeated(_) | FieldEmit::Map(_) => {}
        }
    }
    if items.is_empty() {
        return Ok(TokenStream::new());
    }
    let root_shim = if needs_root {
        quote! {
            // Same `_root` chain as oneof submodules so `self::_root::…` enum paths work.
            mod _root {
                pub(super) use super::super::_root::*;
            }
        }
    } else {
        TokenStream::new()
    };
    Ok(quote! {
        pub(crate) mod defaults {
            #root_shim
            #(#items)*
        }
    })
}

fn render_bit_consts(fields: &[FieldEmit]) -> TokenStream {
    let mut items = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => {
                if let Some((ident, bit)) = &field.presence_bit {
                    items.push(quote! {
                        pub const #ident: usize = #bit;
                    });
                }
                if let Some((ident, bit)) = &field.value_bit {
                    items.push(quote! {
                        pub const #ident: usize = #bit;
                    });
                }
            }
            FieldEmit::Oneof(o) => items.extend(oneof::render_bit_consts(o)),
            FieldEmit::Repeated(_) | FieldEmit::Map(_) => {}
        }
    }
    if items.is_empty() {
        TokenStream::new()
    } else {
        quote! {
            // Bit indices — presence, string SSO heap bits (1 = heap), then bool
            // value bits (field-number order).
            #(#items)*
        }
    }
}

fn render_field_consts(fields: &[FieldEmit]) -> TokenStream {
    let mut items = Vec::new();
    for field in fields {
        if let FieldEmit::Oneof(o) = field {
            items.extend(oneof::render_field_consts(o));
            continue;
        }
        let ident = field.field_const();
        let number = field.number();
        items.push(quote! {
            pub const #ident: u32 = #number;
        });
    }
    if items.is_empty() {
        TokenStream::new()
    } else {
        quote! {
            // Proto field numbers.
            #(#items)*
        }
    }
}

fn render_struct_fields(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| match field {
            FieldEmit::Singular(field) => {
                let name = &field.name;
                let marker = &field.marker;
                let presence_ty = &field.presence_ty;
                let field_const = &field.field_const;
                let layout_ty = field.layout_ty.iter();
                let default_ty: Option<Type> = field.custom_default.as_ref().map(|(c, _)| {
                    let marker = Ident::new(&c.marker_name, Span::call_site());
                    parse_quote! { #companion::defaults::#marker }
                });
                let default_ty = default_ty.iter();
                quote! {
                    #name: ::puroro_rt::SingularField<#marker, #presence_ty, { #companion::#field_const }, A #(, #layout_ty)* #(, #default_ty)*>,
                }
            }
            FieldEmit::Repeated(field) => {
                let name = &field.name;
                let marker = &field.marker;
                let encoding_ty = &field.encoding_ty;
                let field_const = &field.field_const;
                quote! {
                    #name: ::puroro_rt::RepeatedField<#marker, #encoding_ty, { #companion::#field_const }, A>,
                }
            }
            FieldEmit::Map(field) => {
                let name = &field.name;
                let key = &field.key_marker;
                let value = &field.value_marker;
                let field_const = &field.field_const;
                quote! {
                    #name: ::puroro_rt::MapField<#key, #value, { #companion::#field_const }, A>,
                }
            }
            FieldEmit::Oneof(o) => {
                let name = &o.name;
                let storage = &o.storage_name;
                quote! {
                    #name: ::puroro_rt::OneofSlot<#companion::#storage<A>>,
                }
            }
        })
        .collect()
}

fn render_oneof_modules(fields: &[FieldEmit], bits_ty: &TokenStream) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for field in fields {
        if let FieldEmit::Oneof(o) = field {
            out.extend(oneof::render_module_and_exports(o, bits_ty)?);
        }
    }
    Ok(out)
}

fn wire_marker_path(wire: &WireTypeKind<'_>) -> Result<Type> {
    Ok(match wire {
        WireTypeKind::Double => parse_quote! { ::puroro_rt::ProtoDouble },
        WireTypeKind::Float => parse_quote! { ::puroro_rt::ProtoFloat },
        WireTypeKind::Int64 => parse_quote! { ::puroro_rt::ProtoInt64 },
        WireTypeKind::UInt64 => parse_quote! { ::puroro_rt::ProtoUInt64 },
        WireTypeKind::Int32 => parse_quote! { ::puroro_rt::ProtoInt32 },
        WireTypeKind::Fixed64 => parse_quote! { ::puroro_rt::ProtoFixed64 },
        WireTypeKind::Fixed32 => parse_quote! { ::puroro_rt::ProtoFixed32 },
        WireTypeKind::Bool => parse_quote! { ::puroro_rt::ProtoBool },
        WireTypeKind::String { .. } => parse_quote! { ::puroro_rt::ProtoString },
        WireTypeKind::Bytes { .. } => parse_quote! { ::puroro_rt::ProtoBytes },
        WireTypeKind::UInt32 => parse_quote! { ::puroro_rt::ProtoUInt32 },
        WireTypeKind::SFixed32 => parse_quote! { ::puroro_rt::ProtoSFixed32 },
        WireTypeKind::SFixed64 => parse_quote! { ::puroro_rt::ProtoSFixed64 },
        WireTypeKind::SInt32 => parse_quote! { ::puroro_rt::ProtoSInt32 },
        WireTypeKind::SInt64 => parse_quote! { ::puroro_rt::ProtoSInt64 },
        WireTypeKind::Enum { ty, openness } => {
            let path = fqn_to_enum_root_path(ty)?;
            let kind: Type = match openness {
                EnumType::Open => parse_quote! { ::puroro_rt::Open },
                EnumType::Closed => parse_quote! { ::puroro_rt::Closed },
            };
            parse_quote! { ::puroro_rt::ProtoEnum<#path, #kind> }
        }
        WireTypeKind::Message(m) => {
            let path = fqn_to_message_root_path(m)?;
            parse_quote! { ::puroro_rt::ProtoMessage<#path<A>> }
        }
    })
}

fn render_new_in_fields(fields: &[FieldEmit]) -> Vec<TokenStream> {
    let last = fields.len().saturating_sub(1);
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let name = field.name();
            let ctor = field.new_in_ctor();
            if i == last {
                quote! { #name: #ctor(alloc), }
            } else {
                quote! { #name: #ctor(alloc.clone()), }
            }
        })
        .collect()
}

fn render_accessors(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| match field {
            FieldEmit::Repeated(field) => render_repeated_accessors(field),
            FieldEmit::Singular(field) => render_singular_accessors(field),
            FieldEmit::Map(field) => render_map_accessors(field),
            FieldEmit::Oneof(o) => oneof::render_accessors(o, companion),
        })
        .collect()
}

fn render_map_accessors(field: &MapEmit) -> TokenStream {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    let key_view = &field.key_view;
    let value_view = &field.value_view;
    // Pin MutTarget so `*entry_mut(…) = …` / `push_str` resolve through `impl Trait`.
    let mut_target = field.mut_target.as_ref().unwrap_or(value_view);
    if field.string_key {
        quote! {
            pub fn #name(&self) -> impl ::puroro::MapRef<str, #value_view> + '_ {
                self.#name.bind(&self._common)
            }

            pub fn #name_mut(
                &mut self,
            ) -> impl ::puroro::MapMut<str, #value_view, MutTarget = #mut_target> + '_ {
                self.#name.bind_mut(&mut self._common)
            }

            pub fn #clear_name(&mut self) {
                ::puroro::MapMut::clear(&mut self.#name_mut());
            }
        }
    } else {
        quote! {
            pub fn #name(&self) -> impl ::puroro::MapRef<#key_view, #value_view> + '_ {
                self.#name.bind(&self._common)
            }

            pub fn #name_mut(
                &mut self,
            ) -> impl ::puroro::MapMut<#key_view, #value_view, MutTarget = #mut_target> + '_
            {
                self.#name.bind_mut(&mut self._common)
            }

            pub fn #clear_name(&mut self) {
                ::puroro::MapMut::clear(&mut self.#name_mut());
            }
        }
    }
}

fn render_repeated_accessors(field: &RepeatedEmit) -> TokenStream {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    match field.style {
        RepeatedAccessorStyle::String => quote! {
            pub fn #name(&self) -> &[impl ::core::ops::Deref<Target = str>] {
                self.#name.bind(&self._common).as_slice()
            }

            pub fn #name_mut(&mut self) -> impl ::puroro::RepeatedStringMut<A> + '_ {
                self.#name.bind_mut(&mut self._common).container_mut()
            }

            pub fn #clear_name(&mut self) {
                self.#name.bind_mut(&mut self._common).clear();
            }
        },
        RepeatedAccessorStyle::Bytes => quote! {
            pub fn #name(&self) -> &[impl ::core::ops::Deref<Target = [u8]>] {
                self.#name.bind(&self._common).as_slice()
            }

            pub fn #name_mut(&mut self) -> impl ::puroro::RepeatedBytesMut<A> + '_ {
                self.#name.bind_mut(&mut self._common).container_mut()
            }

            pub fn #clear_name(&mut self) {
                self.#name.bind_mut(&mut self._common).clear();
            }
        },
        RepeatedAccessorStyle::Slice => {
            let elem = &field.slice_elem_ty;
            quote! {
                pub fn #name(&self) -> &[#elem] {
                    self.#name.bind(&self._common).as_slice()
                }

                pub fn #name_mut<'s>(
                    &'s mut self,
                ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<#elem, A>> + 's {
                    self.#name.bind_mut(&mut self._common).values_mut()
                }

                pub fn #clear_name(&mut self) {
                    self.#name.bind_mut(&mut self._common).clear();
                }
            }
        }
    }
}

fn render_singular_accessors(field: &ScalarEmit) -> TokenStream {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    let mut_target = &field.mut_target;
    let implicit_ty = &field.implicit_ty;
    let optional_ty = &field.optional_ty;

    if matches!(field.style, AccessorStyle::Message) {
        return quote! {
            pub fn #name(&self) -> ::core::option::Option<&#mut_target> {
                self.#name.bind(&self._common).get()
            }

            pub fn #name_mut(&mut self) -> &mut #mut_target {
                self.#name.bind_mut(&mut self._common).get_mut()
            }

            pub fn #clear_name(&mut self) {
                self.#name.bind_mut(&mut self._common).clear();
            }
        };
    }

    let getter = if field.optional_getter {
        quote! {
            pub fn #name<'a>(&'a self) -> ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>
            where
                A: 'a,
            {
                self.#name.bind(&self._common).optional()
            }
        }
    } else {
        quote! {
            pub fn #name(&self) -> #implicit_ty {
                self.#name.bind(&self._common).value()
            }
        }
    };

    // RPIT `StringMut` so public signatures do not name `puroro-rt`.
    let mut_ret = field.mut_style.return_ty(&quote! { 's }, mut_target);
    let mutator = quote! {
        pub fn #name_mut<'s>(&'s mut self) -> #mut_ret {
            self.#name.bind_mut(&mut self._common).value_mut()
        }
    };

    quote! {
        #getter

        #mutator

        pub fn #clear_name(&mut self) {
            self.#name.bind_mut(&mut self._common).clear();
        }
    }
}

/// Rust types projected from a proto wire kind for generated signatures.
struct WireViews {
    implicit: Type,
    optional: Type,
    mut_target: Type,
    slice_elem: Option<Type>,
    /// Map value view (`i32`, `str`, `Address<A>`, …).
    map_value: Type,
    /// `MutTarget` pin for map string / bytes values.
    map_value_mut: Option<Type>,
}

fn copy_views(ty: Type) -> WireViews {
    WireViews {
        implicit: ty.clone(),
        optional: ty.clone(),
        mut_target: ty.clone(),
        slice_elem: Some(ty.clone()),
        map_value: ty,
        map_value_mut: None,
    }
}

fn wire_views(wire: &WireTypeKind<'_>) -> Result<WireViews> {
    Ok(match wire {
        WireTypeKind::Double => copy_views(parse_quote! { f64 }),
        WireTypeKind::Float => copy_views(parse_quote! { f32 }),
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => {
            copy_views(parse_quote! { i64 })
        }
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => copy_views(parse_quote! { u64 }),
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => {
            copy_views(parse_quote! { i32 })
        }
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => copy_views(parse_quote! { u32 }),
        WireTypeKind::Bool => copy_views(parse_quote! { bool }),
        WireTypeKind::String { .. } => {
            let owned: Type = parse_quote! { ::puroro::String<A> };
            WireViews {
                implicit: parse_quote! { &str },
                optional: parse_quote! { &'a str },
                mut_target: owned.clone(),
                slice_elem: None,
                map_value: parse_quote! { str },
                map_value_mut: Some(owned),
            }
        }
        WireTypeKind::Bytes { .. } => {
            let owned: Type = parse_quote! { ::allocator_api2::vec::Vec<u8, A> };
            WireViews {
                implicit: parse_quote! { &[u8] },
                optional: parse_quote! { &'a [u8] },
                mut_target: owned.clone(),
                slice_elem: None,
                map_value: parse_quote! { [u8] },
                map_value_mut: Some(owned),
            }
        }
        WireTypeKind::Enum { ty, .. } => {
            let path = fqn_to_enum_root_path(ty)?;
            copy_views(parse_quote! { #path })
        }
        WireTypeKind::Message(m) => {
            let path = fqn_to_message_root_path(m)?;
            let ty: Type = parse_quote! { #path<A> };
            WireViews {
                implicit: parse_quote! { () },
                optional: parse_quote! { () },
                mut_target: ty.clone(),
                slice_elem: Some(ty.clone()),
                map_value: ty,
                map_value_mut: None,
            }
        }
    })
}

enum VisitKind {
    Shared,
    Pair,
    PairMut,
    Mut,
}

fn render_visit_calls(fields: &[FieldEmit], kind: VisitKind) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let name = field.name();
            let name_str = field.name_str();
            match kind {
                VisitKind::Shared => quote! {
                    v.visit(#name_str, &self.#name)?;
                },
                VisitKind::Pair => quote! {
                    v.visit(#name_str, &self.#name, &other.#name)?;
                },
                VisitKind::PairMut => quote! {
                    v.visit(#name_str, &self.#name, &mut dst.#name)?;
                },
                VisitKind::Mut => quote! {
                    v.visit(#name_str, &mut self.#name)?;
                },
            }
        })
        .collect()
}

fn render_merge_arms(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    for field in fields {
        if let FieldEmit::Oneof(o) = field {
            arms.extend(oneof::render_merge_arms(o, companion));
            continue;
        }
        let name = field.name();
        let field_const = field.field_const();
        arms.push(quote! {
            #companion::#field_const => {
                self.#name
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?;
            }
        });
    }
    arms
}

fn render_validate(fields: &[FieldEmit]) -> TokenStream {
    let required: Vec<_> = fields
        .iter()
        .filter_map(|f| match f {
            FieldEmit::Singular(f) if matches!(f.style, AccessorStyle::LegacyRequired) => {
                let name = &f.name;
                Some(quote! {
                    self.#name.validate_required(&self._common)?;
                })
            }
            _ => None,
        })
        .collect();
    if required.is_empty() {
        quote! { ::core::result::Result::Ok(()) }
    } else {
        quote! {
            #(#required)*
            ::core::result::Result::Ok(())
        }
    }
}
