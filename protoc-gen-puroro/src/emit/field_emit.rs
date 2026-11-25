//! Field facts quoted by the message, lazy, and oneof renderers.
//!
//! [`emit_member`] turns a [`crate::field_kind::MessageMember`] into owned
//! `quote!` inputs. Rendering those inputs into items lives in
//! [`super::message`].

use super::defaults;
use super::docs;
use super::ident::{escape_ident, is_simple_ident, to_pascal_case};
use super::layout::{LayoutAssoc, LayoutAssocKind, LenAccess, MessageApply, MessageFraming};
use super::oneof::{OneofEmit, OneofVariantEmit};
use super::type_path::{fqn_to_enum_root_path, fqn_to_message_lazy_path, fqn_to_message_root_path};
use crate::default_value::CustomDefault;
use crate::descriptor::features::EnumType;
use crate::error::{Error, Result};
use crate::field_kind::{
    FieldKind, MessageMember, PlannedField, PlannedLayout, PlannedOneof, PlannedPresence,
    RepeatedEncodingKind, ResolvedChildParse, StringCatalogMarker, WireTypeKind,
    lazy_kind_bit_const,
};
use crate::resolved::Message as ResolvedMessage;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::syn::{Item, Lifetime, Type, parse_quote};

pub(super) enum FieldEmit {
    Singular(Box<ScalarEmit>),
    Repeated(Box<RepeatedEmit>),
    Map(Box<MapEmit>),
    Oneof(Box<OneofEmit>),
}

/// Owned per-field facts needed for `quote!` (avoids borrowing `MessageFieldPlan`).
pub(super) struct ScalarEmit {
    /// Struct / accessor ident — `title`, `r#type`.
    pub(super) name: Ident,
    /// Raw proto name — `"title"` (for `title_mut`, visit keys).
    pub(super) name_str: String,
    /// Proto comments as `#[doc = …]` (empty when absent).
    pub(super) doc: TokenStream,
    /// Companion const ident — `FIELD_TITLE`.
    pub(super) field_const: Ident,
    /// Proto field number — `1`.
    pub(super) number: u32,
    /// `T` type arg — `::puroro_rt::ProtoInt32`, `ProtoString`, `ProtoEnum<Status, Open>`.
    pub(super) marker: Type,
    /// `P` type arg — `Implicit`, `Explicit<{ task::BIT_TITLE }>`, `Message`.
    pub(super) presence_ty: Type,
    /// `L` type arg. `None` omits it (`Inline` default). `Some(Inline)` when a
    /// custom `D` must follow. Also `BitPacked<{ task::BIT_DONE_VALUE }>`,
    /// `InlineOrHeap<{ task::BIT_TITLE_SSO }>`.
    pub(super) layout_ty: Option<Type>,
    /// Presence-bit const and index — `Some((BIT_TITLE, 0))`; `None` if Implicit / Message.
    pub(super) presence_bit: Option<(Ident, usize)>,
    /// Value / SSO-heap bit — `Some((BIT_DONE_VALUE, 1))`, `Some((BIT_TITLE_SSO, 2))`;
    /// `None` if the payload is a normal `Inline` field.
    pub(super) value_bit: Option<(Ident, usize)>,
    /// Eager, lazy, and (for messages) oneof getters share this shape.
    pub(super) getter_shape: GetterShape,
    /// `validate_required` — `LegacyRequired` scalars and inlined required messages.
    pub(super) required: bool,
    /// `_mut` `DerefMut` target — `i32`, `::puroro::String<A>`, `Address<A>`.
    pub(super) mut_target: Type,
    /// IMPLICIT `.value()` return — `i32`, `&str`, `&[u8]`.
    pub(super) implicit_ty: Type,
    /// Payload inside `Optional<…>` — `i32`, `&'a str`, `Status`.
    pub(super) optional_ty: Type,
    /// Non-type-zero `[default = …]` (`MaxRetriesDefault` + `HasDefault` items).
    pub(super) custom_default: Option<(CustomDefault, Vec<Item>)>,
    /// `_mut` return: `DerefMut` vs `StringMut` / `BytesMut` (SSO).
    pub(super) mut_style: SingularMutStyle,
    /// `mod layout` associated type, when the slot differs by ingest.
    pub(super) layout: Option<LayoutAssoc>,
    /// Lazy apply for a singular message. `None` when this is not a message.
    pub(super) message_apply: Option<MessageApply>,
    /// Eager type accepted by `set_*` when the field is a `LAZY` pin.
    pub(super) set_eager: Option<Type>,
    /// `BIT_*_LAZY_KIND` and its index, when this slot has a `WireOrSso` arm.
    pub(super) lazy_kind_const: Option<(Ident, usize)>,
}

pub(super) struct RepeatedEmit {
    /// Struct / accessor ident — `tag_ids`, `labels`.
    pub(super) name: Ident,
    /// Raw proto name — `"tag_ids"`.
    pub(super) name_str: String,
    /// Proto comments as `#[doc = …]` (empty when absent).
    pub(super) doc: TokenStream,
    /// Companion const ident — `FIELD_TAG_IDS`.
    pub(super) field_const: Ident,
    /// Proto field number — `6`.
    pub(super) number: u32,
    /// Element marker — `::puroro_rt::ProtoInt32`, `ProtoString`, `ProtoMessage<Address<A>>`.
    pub(super) marker: Type,
    /// Encoding param — `::puroro_rt::Packed` or `Expanded`.
    pub(super) encoding_ty: Type,
    /// Slice vs `RepeatedStringMut` / `RepeatedBytesMut`.
    pub(super) style: RepeatedAccessorStyle,
    /// `as_slice` element — `i32`, `Address<A>`, `impl Deref<Target = str>`.
    pub(super) slice_elem_ty: Type,
    /// Repeated string / bytes (`RepeatedReady` vs `RepeatedSpans`).
    pub(super) layout: Option<LayoutAssoc>,
    /// Lazy apply for a repeated message. `None` for non-messages.
    pub(super) message_apply: Option<MessageApply>,
}

pub(super) struct MapEmit {
    /// Struct / accessor ident — `attrs`.
    pub(super) name: Ident,
    /// Raw proto name — `"attrs"`.
    pub(super) name_str: String,
    /// Proto comments as `#[doc = …]` (empty when absent).
    pub(super) doc: TokenStream,
    /// Companion const ident — `FIELD_ATTRS`.
    pub(super) field_const: Ident,
    /// Proto field number — `10`.
    pub(super) number: u32,
    /// Key marker — `::puroro_rt::ProtoString`, `ProtoInt32`.
    pub(super) key_marker: Type,
    /// Value marker — `::puroro_rt::ProtoInt32`, `ProtoMessage<Address<A>>`.
    pub(super) value_marker: Type,
    /// `MapRef` / `MapMut` key — `str`, `i32`, `bool`.
    pub(super) key_view: Type,
    /// `MapRef` value view — `i32`, `str`, `Address<A>`.
    pub(super) value_view: Type,
    /// Pin `MutTarget` for string / bytes values — `::puroro::String<A>`, `Vec<u8, A>`.
    pub(super) mut_target: Option<Type>,
    /// `MapReady` vs `MapSpans`, plus a follow-parent value message when there is one.
    pub(super) layouts: Vec<LayoutAssoc>,
}

#[derive(Clone, Copy)]
pub(super) enum RepeatedAccessorStyle {
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
            (WireTypeKind::String { utf8 }, PlannedLayout::InlineOrHeap { .. }) => {
                match StringCatalogMarker::from_utf8(*utf8) {
                    StringCatalogMarker::ProtoString => Self::SsoString,
                    StringCatalogMarker::ProtoStringUnchecked => Self::SsoBytes,
                }
            }
            (WireTypeKind::Bytes { .. }, PlannedLayout::InlineOrHeap { .. }) => Self::SsoBytes,
            _ => Self::DerefMut,
        }
    }

    /// Return type of `_mut` for lifetime `lt` (e.g. `'s`) and `mut_target`.
    pub(super) fn return_ty(self, lt: &Lifetime, mut_target: &Type) -> Type {
        match self {
            Self::SsoString => parse_quote! { impl ::puroro::StringMut<A> + #lt },
            Self::SsoBytes => parse_quote! { impl ::puroro::BytesMut<A> + #lt },
            Self::DerefMut => {
                parse_quote! { impl ::core::ops::DerefMut<Target = #mut_target> + #lt }
            }
        }
    }
}

/// How a singular field or oneof variant is read.
///
/// A len slot (string / bytes) still has an eager arm: lazy getters call
/// `profile_str` / `profile_bytes`, and eager getters use [`LenEagerRead`].
#[derive(Clone, Copy)]
pub(super) enum GetterShape {
    /// `.value()` → `implicit_ty`.
    Value,
    /// `.optional()`.
    Optional,
    /// `.get()` / `.get_mut()`.
    Message,
    /// Lazy len read. Eager side is `eager`.
    Len {
        eager: LenEagerRead,
        access: LenAccess,
    },
}

/// Eager getter used by a [`GetterShape::Len`] slot.
#[derive(Clone, Copy)]
pub(super) enum LenEagerRead {
    Value,
    Optional,
}

/// Eager projection of [`GetterShape`], with the len arm collapsed.
#[derive(Clone, Copy)]
pub(super) enum EagerGetter {
    Value,
    Optional,
    Message,
}

impl GetterShape {
    pub(super) fn eager(self) -> EagerGetter {
        match self {
            Self::Value => EagerGetter::Value,
            Self::Optional => EagerGetter::Optional,
            Self::Message => EagerGetter::Message,
            Self::Len { eager, .. } => match eager {
                LenEagerRead::Value => EagerGetter::Value,
                LenEagerRead::Optional => EagerGetter::Optional,
            },
        }
    }

    pub(super) fn len_access(self) -> Option<LenAccess> {
        match self {
            Self::Len { access, .. } => Some(access),
            Self::Value | Self::Optional | Self::Message => None,
        }
    }

    pub(super) fn is_message(self) -> bool {
        matches!(self, Self::Message)
    }
}

/// `L` type argument for a planned layout. `bit_path` is `companion::BIT_*` or `super::BIT_*`.
fn planned_layout_ty(
    layout: &PlannedLayout,
    bit_path: impl Fn(&Ident) -> TokenStream,
    emit_inline: bool,
) -> (Option<Type>, Option<(Ident, usize)>) {
    match layout {
        PlannedLayout::Inline => (
            emit_inline.then(|| parse_quote! { ::puroro_rt::Inline }),
            None,
        ),
        PlannedLayout::Boxed => (Some(parse_quote! { ::puroro_rt::Boxed }), None),
        PlannedLayout::BitPacked {
            value_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            let path = bit_path(&ident);
            (
                Some(parse_quote! { ::puroro_rt::BitPacked<{ #path }> }),
                Some((ident, *value_bit)),
            )
        }
        PlannedLayout::InlineOrHeap {
            heap_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            let path = bit_path(&ident);
            (
                Some(parse_quote! { ::puroro_rt::InlineOrHeap<{ #path }> }),
                Some((ident, *heap_bit)),
            )
        }
    }
}

fn scalar_getter_shape(wire: &WireTypeKind<'_>, presence: &PlannedPresence) -> GetterShape {
    let explicit = matches!(
        presence,
        PlannedPresence::Explicit { .. } | PlannedPresence::LegacyRequired { .. }
    );
    let is_enum = matches!(wire, WireTypeKind::Enum { .. });
    if let Some(access) = len_access(wire) {
        let eager = if explicit || is_enum {
            LenEagerRead::Optional
        } else {
            LenEagerRead::Value
        };
        return GetterShape::Len { eager, access };
    }
    if wire.is_nested_message() {
        return GetterShape::Message;
    }
    if explicit || is_enum {
        GetterShape::Optional
    } else {
        GetterShape::Value
    }
}

fn oneof_getter_shape(wire: &WireTypeKind<'_>) -> GetterShape {
    if let Some(access) = len_access(wire) {
        GetterShape::Len {
            eager: LenEagerRead::Optional,
            access,
        }
    } else if wire.is_nested_message() {
        GetterShape::Message
    } else {
        GetterShape::Optional
    }
}

impl FieldEmit {
    pub(super) fn name(&self) -> &Ident {
        match self {
            Self::Singular(f) => &f.name,
            Self::Repeated(f) => &f.name,
            Self::Map(f) => &f.name,
            Self::Oneof(o) => &o.name,
        }
    }

    pub(super) fn name_str(&self) -> &str {
        match self {
            Self::Singular(f) => &f.name_str,
            Self::Repeated(f) => &f.name_str,
            Self::Map(f) => &f.name_str,
            Self::Oneof(o) => &o.name_str,
        }
    }

    /// Companion `FIELD_*` ident. `None` for a oneof slot — the slot has no
    /// field number; each variant has its own const (see [`render_field_consts`]).
    pub(super) fn field_const(&self) -> Option<&Ident> {
        match self {
            Self::Singular(f) => Some(&f.field_const),
            Self::Repeated(f) => Some(&f.field_const),
            Self::Map(f) => Some(&f.field_const),
            Self::Oneof(_) => None,
        }
    }

    pub(super) fn new_in_ctor(&self) -> TokenStream {
        match self {
            Self::Singular(_) => quote! { ::puroro_rt::SingularField::new_in },
            Self::Repeated(_) => quote! { ::puroro_rt::RepeatedField::new_in },
            Self::Map(_) => quote! { ::puroro_rt::MapField::new_in },
            Self::Oneof(_) => quote! { ::puroro_rt::OneofSlot::new_in },
        }
    }
}

pub(super) fn emit_member(member: &MessageMember<'_>, companion: &Ident) -> Result<FieldEmit> {
    match member {
        MessageMember::Oneof(o) => Ok(FieldEmit::Oneof(Box::new(emit_oneof(o)?))),
        MessageMember::Field(field) => emit_field(field, companion),
    }
}

fn emit_oneof(o: &PlannedOneof<'_>) -> Result<OneofEmit> {
    if !is_simple_ident(o.name()) {
        return Err(Error::Codegen(format!(
            "cannot use oneof name `{}` as a Rust identifier",
            o.name()
        )));
    }
    if o.variants().is_empty() {
        return Err(Error::Codegen(format!(
            "oneof `{}` has no variants",
            o.name()
        )));
    }
    let variants = o
        .variants()
        .iter()
        .enumerate()
        .map(|(i, field)| emit_oneof_variant(field, i))
        .collect::<Result<Vec<_>>>()?;
    let pascal = to_pascal_case(o.name());
    Ok(OneofEmit {
        name: escape_ident(o.name()),
        name_str: o.name().to_owned(),
        doc: docs::doc_attr(o.comments()),
        mod_name: escape_ident(o.name()),
        shape_name: Ident::new(&pascal, Span::call_site()),
        case_name: Ident::new(&format!("{pascal}Case"), Span::call_site()),
        storage_name: Ident::new(&format!("{pascal}Storage"), Span::call_site()),
        variants,
    })
}

fn emit_field(field: &PlannedField<'_>, companion: &Ident) -> Result<FieldEmit> {
    if !is_simple_ident(field.name()) {
        return Err(Error::Codegen(format!(
            "cannot use field name `{}` as a Rust identifier",
            field.name()
        )));
    }
    Ok(match field.kind() {
        FieldKind::Map { .. } => FieldEmit::Map(Box::new(emit_map(field, companion)?)),
        FieldKind::Repeated { .. } => {
            FieldEmit::Repeated(Box::new(emit_repeated(field, companion)?))
        }
        FieldKind::Singular { .. } => FieldEmit::Singular(Box::new(emit_scalar(field, companion)?)),
    })
}

fn emit_oneof_variant(field: &PlannedField<'_>, index: usize) -> Result<OneofVariantEmit> {
    if !is_simple_ident(field.name()) {
        return Err(Error::Codegen(format!(
            "cannot use oneof variant name `{}` as a Rust identifier",
            field.name()
        )));
    }
    let FieldKind::Singular {
        wire,
        presence: PlannedPresence::Oneof,
        layout,
        custom_default,
    } = field.kind()
    else {
        return Err(Error::internal(format!(
            "oneof variant `{}` is not a oneof singular",
            field.name()
        )));
    };

    let plan = plan_singular(
        field,
        wire,
        layout,
        custom_default.as_ref(),
        SingularSite::Oneof,
    )?;
    let variant_pascal = to_pascal_case(field.name());
    // A len slot or a follow-parent child mentions `Layout`. A concrete pin does not.
    let generic_layout = plan.layout.is_some();
    Ok(OneofVariantEmit {
        name: escape_ident(field.name()),
        name_str: field.name().to_owned(),
        doc: docs::doc_attr(field.field().comments()),
        variant_name: Ident::new(&variant_pascal, Span::call_site()),
        type_param: Ident::new(&format!("T{index}"), Span::call_site()),
        field_alias: Ident::new(&format!("{variant_pascal}Field"), Span::call_site()),
        field_const: Ident::new(field.field_const(), Span::call_site()),
        number: field.number() as u32,
        marker: plan.marker,
        layout_ty: plan.layout_ty,
        value_bit: plan.value_bit,
        is_bool: matches!(wire, WireTypeKind::Bool),
        mut_style: plan.mut_style,
        mut_target: plan.mut_target,
        optional_ty: plan.optional_ty,
        custom_default: plan.custom_default,
        getter_shape: oneof_getter_shape(wire),
        message_apply: plan.message_apply,
        layout: plan.layout,
        lazy_kind: plan.lazy_kind,
        generic_layout,
    })
}

fn emit_repeated(field: &PlannedField<'_>, companion: &Ident) -> Result<RepeatedEmit> {
    let FieldKind::Repeated { wire, encoding } = field.kind() else {
        return Err(Error::internal(format!(
            "field `{}` is not repeated",
            field.name()
        )));
    };
    let name = field.name();
    let field_const = field.field_const();
    let number = field.number();
    let encoding_ty: Type = match encoding {
        RepeatedEncodingKind::Packed => parse_quote! { ::puroro_rt::Packed },
        RepeatedEncodingKind::Expanded => parse_quote! { ::puroro_rt::Expanded },
    };
    let catalog = wire_catalog(wire)?;
    let style = match wire {
        WireTypeKind::String { utf8 } => match StringCatalogMarker::from_utf8(*utf8) {
            StringCatalogMarker::ProtoString => RepeatedAccessorStyle::String,
            StringCatalogMarker::ProtoStringUnchecked => RepeatedAccessorStyle::Bytes,
        },
        WireTypeKind::Bytes { .. } => RepeatedAccessorStyle::Bytes,
        _ => RepeatedAccessorStyle::Slice,
    };
    let mut marker = catalog.marker;
    let mut slice_elem_ty = catalog.views.slice_elem;
    let mut layout = None;
    let mut message_apply = None;
    if let Some(child) = wire.as_nested_message() {
        // A message element is not a span list. Follow-parent only changes
        // the element type; storage stays `RepeatedReady`.
        let framing = message_framing(wire);
        let pin = resolve_child_pin(name, child, field.child_parse(), FollowLazyBuf::ShareParent)?;
        message_apply = Some(pin.repeated_apply(framing));
        if let Some(assoc) = pin.follow() {
            layout = Some(assoc.clone());
        }
        if let Some(ty) = pin.field_marker(companion, framing) {
            marker = ty;
        }
        if let Some((_, lazy)) = pin.lazy_pin() {
            slice_elem_ty = lazy.clone();
        }
    } else if len_access(wire).is_some() {
        let assoc = assoc_ident(name, "Repeated");
        let field_ident = Ident::new(field_const, Span::call_site());
        layout = Some(LayoutAssoc {
            assoc,
            kind: LayoutAssocKind::Repeated {
                marker: marker.clone(),
                encoding: encoding_ty.clone(),
                field_const: field_ident,
            },
        });
    }

    Ok(RepeatedEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        doc: docs::doc_attr(field.field().comments()),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        marker,
        encoding_ty,
        style,
        slice_elem_ty,
        layout,
        message_apply,
    })
}

fn emit_map(field: &PlannedField<'_>, companion: &Ident) -> Result<MapEmit> {
    let FieldKind::Map { key, value } = field.kind() else {
        return Err(Error::internal(format!(
            "field `{}` is not a map",
            field.name()
        )));
    };
    let name = field.name();
    let field_const = field.field_const();
    let number = field.number();
    // `plan_map_field` already rejected illegal keys via `is_legal_map_key()`.
    let key_wire = wire_catalog(key)?;
    let key_view = key_wire.views.map_value;
    let key_marker = key_wire.marker;
    let value_wire = wire_catalog(value)?;
    let mut value_marker = value_wire.marker;
    let mut map_value_bound = value_marker.clone();
    let mut value_view = value_wire.views.map_value;
    let mut layouts = Vec::new();
    if let Some(child) = value.as_nested_message() {
        // Map values materialise through `Message`, so the lazy alias stays
        // on the Bytes default rather than sharing the parent's `P`. Group
        // map values are rejected in `plan_map_field`.
        let framing = message_framing(value);
        let pin = resolve_child_pin(name, child, field.child_parse(), FollowLazyBuf::OwnedBytes)?;
        if let Some(assoc) = pin.follow() {
            let assoc_name = &assoc.assoc;
            // The `Layout` bound names the assoc on `Self`. The field
            // marker names it on the message's `L` parameter.
            let marker_path = nested_marker_path(framing);
            map_value_bound = parse_quote! { #marker_path<Self::#assoc_name> };
            layouts.push(assoc.clone());
        }
        if let Some((_, lazy)) = pin.lazy_pin() {
            value_view = lazy.clone();
        }
        if let Some(ty) = pin.field_marker(companion, framing) {
            value_marker = ty;
        }
        if pin.lazy_pin().is_some() {
            map_value_bound = value_marker.clone();
        }
    }
    let map_assoc = assoc_ident(name, "Map");
    let field_ident = Ident::new(field_const, Span::call_site());
    layouts.push(LayoutAssoc {
        assoc: map_assoc,
        kind: LayoutAssocKind::Map {
            key: key_marker.clone(),
            value: map_value_bound,
            field_const: field_ident,
        },
    });
    Ok(MapEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        doc: docs::doc_attr(field.field().comments()),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        key_marker,
        value_marker,
        key_view,
        value_view,
        mut_target: value_wire.views.map_value_mut,
        layouts,
    })
}

fn emit_scalar(field: &PlannedField<'_>, companion: &Ident) -> Result<ScalarEmit> {
    let FieldKind::Singular {
        wire,
        presence:
            presence @ (PlannedPresence::Implicit
            | PlannedPresence::Explicit { .. }
            | PlannedPresence::LegacyRequired { .. }
            | PlannedPresence::Message),
        layout,
        custom_default,
    } = field.kind()
    else {
        return Err(Error::internal(format!(
            "emit_scalar on oneof or non-singular field `{}`",
            field.name()
        )));
    };
    let name = field.name();
    let field_const = field.field_const();
    let number = field.number();
    let custom_default = custom_default.as_ref();
    let required = matches!(presence, PlannedPresence::LegacyRequired { .. })
        || (matches!(presence, PlannedPresence::Message) && field.field().is_legacy_required());
    let (presence_ty, presence_bit) = match presence {
        PlannedPresence::Implicit => (parse_quote! { ::puroro_rt::Implicit }, None),
        PlannedPresence::Explicit { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                parse_quote! { ::puroro_rt::Explicit<{ #companion::#ident }> },
                Some((ident, *bit)),
            )
        }
        PlannedPresence::LegacyRequired { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                parse_quote! { ::puroro_rt::LegacyRequired<{ #companion::#ident }> },
                Some((ident, *bit)),
            )
        }
        PlannedPresence::Message => (parse_quote! { ::puroro_rt::Message }, None),
        PlannedPresence::Oneof => unreachable!("rejected above"),
    };

    let plan = plan_singular(
        field,
        wire,
        layout,
        custom_default,
        SingularSite::Field { companion },
    )?;

    Ok(ScalarEmit {
        name: escape_ident(name),
        name_str: name.to_owned(),
        doc: docs::doc_attr(field.field().comments()),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        marker: plan.marker,
        presence_ty,
        layout_ty: plan.layout_ty,
        presence_bit,
        value_bit: plan.value_bit,
        getter_shape: scalar_getter_shape(wire, presence),
        required,
        mut_target: plan.mut_target,
        implicit_ty: plan.implicit_ty,
        optional_ty: plan.optional_ty,
        custom_default: plan.custom_default,
        mut_style: plan.mut_style,
        layout: plan.layout,
        message_apply: plan.message_apply,
        set_eager: plan.set_eager,
        lazy_kind_const: plan.lazy_kind,
    })
}

/// Message field versus oneof variant. The planning steps match; the module
/// that owns `BIT_*` and `Layout` does not.
#[derive(Clone, Copy)]
enum SingularSite<'a> {
    /// Field on the message impl. Paths are `#companion::BIT_*`.
    Field { companion: &'a Ident },
    /// Variant inside `mod <oneof>`. Paths are `super::BIT_*`.
    Oneof,
}

/// Facts shared by [`emit_scalar`] and [`emit_oneof_variant`] after wire,
/// layout, lazy-kind, and child-pin resolution.
struct SingularPlan {
    layout_ty: Option<Type>,
    value_bit: Option<(Ident, usize)>,
    marker: Type,
    mut_target: Type,
    implicit_ty: Type,
    optional_ty: Type,
    custom_default: Option<(CustomDefault, Vec<Item>)>,
    mut_style: SingularMutStyle,
    layout: Option<LayoutAssoc>,
    message_apply: Option<MessageApply>,
    /// Eager alias for `set_*`. Only a message field pinned to the lazy child.
    set_eager: Option<Type>,
    lazy_kind: Option<(Ident, usize)>,
}

fn plan_singular(
    field: &PlannedField<'_>,
    wire: &WireTypeKind<'_>,
    layout: &PlannedLayout,
    custom_default: Option<&CustomDefault>,
    site: SingularSite<'_>,
) -> Result<SingularPlan> {
    let name = field.name();
    let (mut layout_ty, value_bit) = planned_layout_ty(
        layout,
        |ident| match site {
            // Oneof modules sit under the companion, so bits are `super::`.
            SingularSite::Field { companion } => quote! { #companion::#ident },
            SingularSite::Oneof => quote! { super::#ident },
        },
        custom_default.is_some(),
    );

    let catalog = wire_catalog(wire)?;
    let mut marker = catalog.marker;
    let mut mut_target = catalog.views.mut_target;
    let mut set_eager = None;
    let mut layout_assoc = None;
    let mut message_apply = None;
    let mut lazy_kind = None;

    if let Some(kind_bit) = field.lazy_kind_bit() {
        let kind_ident = Ident::new(&lazy_kind_bit_const(name), Span::call_site());
        lazy_kind = Some((kind_ident.clone(), kind_bit));
        // A message field's eager arm is written inside `mod layout`, which
        // already sits under the companion, so that arm names `super::BIT_*`.
        // A oneof variant's `layout_ty` was built with `super::` above.
        let eager_layout = match site {
            SingularSite::Field { .. } => singular_len_eager(name, layout)?,
            SingularSite::Oneof => layout_ty
                .clone()
                .unwrap_or_else(|| parse_quote! { ::puroro_rt::Inline }),
        };
        let assoc = wire_or_sso_assoc(name, &marker, &kind_ident, eager_layout, len_access(wire));
        if let SingularSite::Field { companion } = site {
            let assoc_name = assoc.assoc.clone();
            layout_ty = Some(parse_quote! { <L as #companion::Layout<A>>::#assoc_name });
        }
        layout_assoc = Some(assoc);
    }

    if let Some(child) = wire.as_nested_message() {
        let framing = message_framing(wire);
        let pin = resolve_child_pin(name, child, field.child_parse(), FollowLazyBuf::ShareParent)?;
        message_apply = Some(pin.singular_apply(framing));
        if let Some(assoc) = pin.follow() {
            layout_assoc = Some(assoc.clone());
        }
        let child_marker = match site {
            SingularSite::Field { companion } => pin.field_marker(companion, framing),
            SingularSite::Oneof => pin.oneof_marker(framing),
        };
        if let Some(ty) = child_marker {
            marker = ty;
        }
        if let Some((eager, lazy)) = pin.lazy_pin() {
            if matches!(site, SingularSite::Field { .. }) {
                set_eager = Some(eager.clone());
            }
            mut_target = lazy.clone();
        }
    }

    Ok(SingularPlan {
        layout_ty,
        value_bit,
        marker,
        mut_target,
        implicit_ty: catalog.views.implicit,
        optional_ty: catalog.views.optional,
        custom_default: match custom_default {
            Some(custom) => Some((custom.clone(), defaults::render_marker_item(custom, wire)?)),
            None => None,
        },
        mut_style: SingularMutStyle::from_layout(wire, layout),
        layout: layout_assoc,
        message_apply,
        set_eager,
        lazy_kind,
    })
}

fn len_access(wire: &WireTypeKind<'_>) -> Option<LenAccess> {
    match wire {
        WireTypeKind::String { utf8 } => match StringCatalogMarker::from_utf8(*utf8) {
            StringCatalogMarker::ProtoString => Some(LenAccess::Str),
            StringCatalogMarker::ProtoStringUnchecked => Some(LenAccess::Bytes),
        },
        WireTypeKind::Bytes { .. } => Some(LenAccess::Bytes),
        _ => None,
    }
}

/// `Inline` or `InlineOrHeap<{ super::BIT_* }>`, the eager arm of a `WireOrSso` slot.
fn singular_len_eager(field_name: &str, layout: &PlannedLayout) -> Result<Type> {
    match layout {
        PlannedLayout::Inline => Ok(parse_quote! { ::puroro_rt::Inline }),
        PlannedLayout::InlineOrHeap { bit_const, .. } => {
            let ident = Ident::new(bit_const, Span::call_site());
            Ok(parse_quote! { ::puroro_rt::InlineOrHeap<{ super::#ident }> })
        }
        PlannedLayout::Boxed | PlannedLayout::BitPacked { .. } => Err(Error::internal(format!(
            "field `{field_name}` has a lazy kind bit but is not a string or bytes slot"
        ))),
    }
}

fn wire_or_sso_assoc(
    field_name: &str,
    marker: &Type,
    kind_ident: &Ident,
    eager: Type,
    access: Option<LenAccess>,
) -> LayoutAssoc {
    let lazy = match access {
        Some(LenAccess::Str) => {
            parse_quote! { <P as ::puroro_rt::decode::LazyProfile>::StringLen<A, { super::#kind_ident }> }
        }
        Some(LenAccess::Bytes) | None => {
            parse_quote! { <P as ::puroro_rt::decode::LazyProfile>::BytesLen<A, { super::#kind_ident }> }
        }
    };
    LayoutAssoc {
        assoc: assoc_ident(field_name, "Len"),
        kind: LayoutAssocKind::SingularLen {
            marker: marker.clone(),
            eager,
            lazy,
        },
    }
}

fn assoc_ident(name: &str, suffix: &str) -> Ident {
    Ident::new(
        &format!("{}{suffix}", to_pascal_case(name)),
        Span::call_site(),
    )
}

/// How a follow-parent lazy child stores its island buffer.
enum FollowLazyBuf {
    /// Share the parent's `P` (`NameLazy<A, P>`): singular / repeated / oneof.
    ShareParent,
    /// Own a `Bytes` island (`NameLazy<A>`): map values materialise via `Message`.
    OwnedBytes,
}

fn child_type_pair(
    message: &ResolvedMessage<'_>,
    follow_lazy: FollowLazyBuf,
) -> Result<(Type, Type)> {
    let eager = fqn_to_message_root_path(message)?;
    let lazy = fqn_to_message_lazy_path(message)?;
    let lazy_ty = match follow_lazy {
        FollowLazyBuf::ShareParent => parse_quote! { #lazy<A, P> },
        FollowLazyBuf::OwnedBytes => parse_quote! { #lazy<A> },
    };
    Ok((parse_quote! { #eager<A> }, lazy_ty))
}

/// Nested message pin, resolved once for every singular / repeated / map / oneof use.
enum ChildPin {
    /// Child type is a `Layout` associated type and follows the parent alias.
    Follow(Box<LayoutAssoc>),
    /// Pinned to the lazy alias on both parents. Slot type is still a `Layout`
    /// associated type so the lazy parent can use `NameLazy<A, P>`.
    Lazy {
        /// Eager child accepted by `set_*` (`Name` → `into_lazy`).
        set_from: Box<Type>,
        /// Eager parent's mut/get view (`NameLazy<A>` / Bytes default).
        view: Box<Type>,
        layout: Box<LayoutAssoc>,
    },
    /// Pinned to the eager alias. The wire marker from [`wire_catalog`] stays.
    Eager { eager: Box<Type> },
}

fn resolve_child_pin(
    field_name: &str,
    child: &ResolvedMessage<'_>,
    pin: ResolvedChildParse,
    follow_lazy: FollowLazyBuf,
) -> Result<ChildPin> {
    let (eager, lazy) = child_type_pair(child, follow_lazy)?;
    Ok(match pin {
        ResolvedChildParse::Follow => {
            let assoc = assoc_ident(field_name, "Msg");
            ChildPin::Follow(Box::new(LayoutAssoc {
                assoc,
                kind: LayoutAssocKind::Child { eager, lazy },
            }))
        }
        ResolvedChildParse::Lazy => {
            let eager_path = fqn_to_message_root_path(child)?;
            let lazy_path = fqn_to_message_lazy_path(child)?;
            let assoc = assoc_ident(field_name, "Msg");
            ChildPin::Lazy {
                set_from: Box::new(parse_quote! { #eager_path<A> }),
                view: Box::new(parse_quote! { #lazy_path<A> }),
                layout: Box::new(LayoutAssoc {
                    assoc,
                    kind: LayoutAssocKind::Child {
                        // Eager parent stores `NameLazy<A>` (Bytes).
                        eager: parse_quote! { #lazy_path<A> },
                        // Lazy parent stores `NameLazy<A, P>`.
                        lazy: parse_quote! { #lazy_path<A, P> },
                    },
                }),
            }
        }
        ResolvedChildParse::Eager => ChildPin::Eager {
            eager: Box::new(eager),
        },
    })
}

impl ChildPin {
    fn singular_apply(&self, framing: MessageFraming) -> MessageApply {
        match self {
            Self::Follow(_) | Self::Lazy { .. } => MessageApply::Shared(framing),
            Self::Eager { .. } => MessageApply::EagerSingular(framing),
        }
    }

    fn repeated_apply(&self, framing: MessageFraming) -> MessageApply {
        match self {
            Self::Follow(_) | Self::Lazy { .. } => MessageApply::Shared(framing),
            Self::Eager { eager } => MessageApply::EagerRepeated {
                child: eager.clone(),
                framing,
            },
        }
    }

    /// Marker on the message impl. `layout` is `#companion::Layout<A>`.
    /// `None` for an eager pin: keep the marker from [`wire_catalog`].
    fn field_marker(&self, layout: &Ident, framing: MessageFraming) -> Option<Type> {
        match self {
            Self::Follow(assoc) | Self::Lazy { layout: assoc, .. } => {
                let name = &assoc.assoc;
                let marker = nested_marker_path(framing);
                Some(parse_quote! { #marker<<L as #layout::Layout<A>>::#name> })
            }
            Self::Eager { .. } => None,
        }
    }

    /// Marker inside a oneof module. The follow-parent path names `super::Layout`
    /// because the type alias has no `L: Layout<A>` bound.
    fn oneof_marker(&self, framing: MessageFraming) -> Option<Type> {
        match self {
            Self::Follow(assoc) | Self::Lazy { layout: assoc, .. } => {
                let name = &assoc.assoc;
                let marker = nested_marker_path(framing);
                Some(parse_quote! {
                    #marker<<L as super::Layout<A>>::#name>
                })
            }
            Self::Eager { .. } => None,
        }
    }

    fn follow(&self) -> Option<&LayoutAssoc> {
        match self {
            Self::Follow(assoc) | Self::Lazy { layout: assoc, .. } => Some(assoc),
            Self::Eager { .. } => None,
        }
    }

    /// Eager `set_*` source and mut/get view when the field is a lazy pin.
    fn lazy_pin(&self) -> Option<(&Type, &Type)> {
        match self {
            Self::Lazy { set_from, view, .. } => Some((set_from.as_ref(), view.as_ref())),
            Self::Follow(_) | Self::Eager { .. } => None,
        }
    }
}

/// Catalog marker plus the Rust types quoted into generated signatures.
///
/// One arm owns both so a new [`WireTypeKind`] cannot update the marker and
/// leave the signature types behind (or the other way around). Copyable
/// scalars share one Rust type across several markers via [`copy_catalog`].
struct WireCatalog {
    marker: Type,
    views: WireViews,
}

fn copy_catalog(marker: Type, ty: Type) -> WireCatalog {
    WireCatalog {
        marker,
        views: copy_views(ty),
    }
}

fn wire_catalog(wire: &WireTypeKind<'_>) -> Result<WireCatalog> {
    Ok(match wire {
        WireTypeKind::Double => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoDouble },
            parse_quote! { f64 },
        ),
        WireTypeKind::Float => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoFloat },
            parse_quote! { f32 },
        ),
        WireTypeKind::Int64 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoInt64 },
            parse_quote! { i64 },
        ),
        WireTypeKind::SInt64 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoSInt64 },
            parse_quote! { i64 },
        ),
        WireTypeKind::SFixed64 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoSFixed64 },
            parse_quote! { i64 },
        ),
        WireTypeKind::UInt64 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoUInt64 },
            parse_quote! { u64 },
        ),
        WireTypeKind::Fixed64 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoFixed64 },
            parse_quote! { u64 },
        ),
        WireTypeKind::Int32 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoInt32 },
            parse_quote! { i32 },
        ),
        WireTypeKind::SInt32 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoSInt32 },
            parse_quote! { i32 },
        ),
        WireTypeKind::SFixed32 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoSFixed32 },
            parse_quote! { i32 },
        ),
        WireTypeKind::UInt32 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoUInt32 },
            parse_quote! { u32 },
        ),
        WireTypeKind::Fixed32 => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoFixed32 },
            parse_quote! { u32 },
        ),
        WireTypeKind::Bool => copy_catalog(
            parse_quote! { ::puroro_rt::ProtoBool },
            parse_quote! { bool },
        ),
        WireTypeKind::String { utf8 } => match StringCatalogMarker::from_utf8(*utf8) {
            StringCatalogMarker::ProtoString => WireCatalog {
                marker: parse_quote! { ::puroro_rt::ProtoString },
                views: string_views(),
            },
            StringCatalogMarker::ProtoStringUnchecked => WireCatalog {
                marker: parse_quote! { ::puroro_rt::ProtoStringUnchecked },
                views: len_bytes_views(),
            },
        },
        WireTypeKind::Bytes { .. } => WireCatalog {
            marker: parse_quote! { ::puroro_rt::ProtoBytes },
            views: len_bytes_views(),
        },
        WireTypeKind::Enum { ty, openness } => {
            let path = fqn_to_enum_root_path(ty)?;
            let kind: Type = match openness {
                EnumType::Open => parse_quote! { ::puroro_rt::Open },
                EnumType::Closed => parse_quote! { ::puroro_rt::Closed },
            };
            WireCatalog {
                marker: parse_quote! { ::puroro_rt::ProtoEnum<#path, #kind> },
                views: copy_views(parse_quote! { #path }),
            }
        }
        WireTypeKind::Message(m) | WireTypeKind::Group(m) => {
            let path = fqn_to_message_root_path(m)?;
            let ty: Type = parse_quote! { #path<A> };
            let marker_path = nested_marker_path(message_framing(wire));
            WireCatalog {
                marker: parse_quote! { #marker_path<#ty> },
                views: WireViews {
                    implicit: parse_quote! { () },
                    optional: parse_quote! { () },
                    mut_target: ty.clone(),
                    slice_elem: ty.clone(),
                    map_value: ty,
                    map_value_mut: None,
                },
            }
        }
    })
}

fn message_framing(wire: &WireTypeKind<'_>) -> MessageFraming {
    if wire.is_group() {
        MessageFraming::Group
    } else {
        MessageFraming::LengthPrefixed
    }
}

fn nested_marker_path(framing: MessageFraming) -> Type {
    match framing {
        MessageFraming::LengthPrefixed => parse_quote! { ::puroro_rt::ProtoMessage },
        MessageFraming::Group => parse_quote! { ::puroro_rt::ProtoGroup },
    }
}

/// Rust types projected from a proto wire kind for generated signatures.
struct WireViews {
    implicit: Type,
    optional: Type,
    mut_target: Type,
    /// Repeated `as_slice` element (`i32`, `impl Deref<Target = str>`, …).
    slice_elem: Type,
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
        slice_elem: ty.clone(),
        map_value: ty,
        map_value_mut: None,
    }
}

fn string_views() -> WireViews {
    let owned: Type = parse_quote! { ::puroro::String<A> };
    WireViews {
        implicit: parse_quote! { &str },
        optional: parse_quote! { &'a str },
        mut_target: owned.clone(),
        slice_elem: parse_quote! { impl ::core::ops::Deref<Target = str> },
        map_value: parse_quote! { str },
        map_value_mut: Some(owned),
    }
}

fn len_bytes_views() -> WireViews {
    let owned: Type = parse_quote! { ::allocator_api2::vec::Vec<u8, A> };
    WireViews {
        implicit: parse_quote! { &[u8] },
        optional: parse_quote! { &'a [u8] },
        mut_target: owned.clone(),
        slice_elem: parse_quote! { impl ::core::ops::Deref<Target = [u8]> },
        map_value: parse_quote! { [u8] },
        map_value_mut: Some(owned),
    }
}
