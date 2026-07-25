//! Emit a message module body from a [`MessagePlan`].
//!
//! Singular and repeated scalar / string / bytes / bool / enum / message
//! fields (including IMPLICIT / EXPLICIT / LEGACY_REQUIRED and bool
//! `BitPacked`). Real oneof fields are rejected.

use super::ident::{is_simple_ident, rust_ident};
use super::type_path::{fqn_to_enum_root_path, fqn_to_message_root_path};
use crate::descriptor::features::EnumType;
use crate::error::{Error, Result};
use crate::field_kind::{
    CatalogLayout, CatalogPresence, FieldKind, MessageMember, MessagePlan, RepeatedEncodingKind,
    WireTypeKind, presence_byte_len,
};
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;

enum FieldEmit {
    Singular(ScalarEmit),
    Repeated(RepeatedEmit),
}

/// Owned per-field facts needed for `quote!` (avoids borrowing `MessagePlan`).
struct ScalarEmit {
    name: Ident,
    name_str: String,
    field_const: Ident,
    number: u32,
    marker: TokenStream,
    presence_ty: TokenStream,
    layout_ty: Option<TokenStream>,
    presence_bit: Option<(Ident, usize)>,
    value_bit: Option<(Ident, usize)>,
    style: AccessorStyle,
    /// Enums always use `.optional()` even when presence is IMPLICIT (DESIGN §4.6).
    optional_getter: bool,
    mut_target: TokenStream,
    /// Return type for IMPLICIT value getters (`i32`, `&str`, …).
    implicit_ty: TokenStream,
    /// Payload type inside `Optional<…>` (`i32`, `&'a str`, enum path, …).
    optional_ty: TokenStream,
}

struct RepeatedEmit {
    name: Ident,
    name_str: String,
    field_const: Ident,
    number: u32,
    marker: TokenStream,
    encoding_ty: TokenStream,
    style: RepeatedAccessorStyle,
    /// `as_slice` element type for packable / message repeated fields.
    slice_elem_ty: TokenStream,
}

#[derive(Clone, Copy)]
enum RepeatedAccessorStyle {
    /// `&[T]` + `values_mut()` → `Vec<T, A>`.
    Slice,
    /// `repeated string` — `RepeatedStringMut`.
    String,
    /// `repeated bytes` — `RepeatedContainerMut`.
    Bytes,
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
        }
    }

    fn name_str(&self) -> &str {
        match self {
            Self::Singular(f) => &f.name_str,
            Self::Repeated(f) => &f.name_str,
        }
    }

    fn field_const(&self) -> &Ident {
        match self {
            Self::Singular(f) => &f.field_const,
            Self::Repeated(f) => &f.field_const,
        }
    }

    fn number(&self) -> u32 {
        match self {
            Self::Singular(f) => f.number,
            Self::Repeated(f) => f.number,
        }
    }
}

/// Render items that belong inside the message's implementation module.
pub(super) fn render_items(plan: &MessagePlan<'_>) -> Result<TokenStream> {
    let fields = collect_fields(plan)?;
    let name = rust_ident(plan.message().name());
    let name_str = plan.message().name();
    let presence_bytes = presence_byte_len(plan.bit_count());

    let bit_consts = render_bit_consts(&fields);
    let field_consts = render_field_consts(&fields);
    let struct_fields = render_struct_fields(&fields);
    let new_in_fields = render_new_in_fields(&fields);
    let accessors = render_accessors(&fields);
    let visit_shared = render_visit_calls(&fields, VisitKind::Shared);
    let visit_pair = render_visit_calls(&fields, VisitKind::Pair);
    let visit_pair_mut = render_visit_calls(&fields, VisitKind::PairMut);
    let visit_mut = render_visit_calls(&fields, VisitKind::Mut);
    let merge_arms = render_merge_arms(&fields);
    let validate_body = render_validate(&fields);
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

    Ok(quote! {
        // @generated message body from protoc-gen-puroro.

        #bit_consts

        #field_consts

        type __Presence = ::bitvec::array::BitArray<[u8; #presence_bytes], ::bitvec::order::Lsb0>;

        pub struct #name<
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone =
                ::allocator_api2::alloc::Global,
        > {
            _common: ::puroro_rt::MessageCommon<__Presence, A>,
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

            pub fn visit_fields<V: ::puroro_rt::FieldVisitor<__Presence, A>>(
                &self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_visit_sink
                #(#visit_shared)*
                ::core::ops::ControlFlow::Continue(())
            }

            pub fn visit_field_pairs<V: ::puroro_rt::FieldPairVisitor<__Presence, A>>(
                &self,
                other: &Self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_pair_sink
                #(#visit_pair)*
                ::core::ops::ControlFlow::Continue(())
            }

            pub fn visit_field_pairs_mut<V: ::puroro_rt::FieldPairVisitorMut<__Presence, A>>(
                &self,
                dst: &mut Self,
                v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #empty_pair_mut_sink
                #(#visit_pair_mut)*
                ::core::ops::ControlFlow::Continue(())
            }

            pub fn visit_fields_mut<V: ::puroro_rt::FieldVisitorMut<__Presence, A>>(
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

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro::Message
            for #name<A>
        {
            type Alloc = A;

            fn new_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }

            fn encoded_len(&self) -> usize {
                let mut v = ::puroro_rt::EncodedLenVisitor::new(&self._common);
                let _ = self.visit_fields(&mut v);
                v.len + self._common.unknown_fields.len()
            }

            fn encode_raw<B: ::bytes::BufMut>(&self, buf: &mut B) {
                let _ = self.visit_fields(&mut ::puroro_rt::EncodeRawVisitor::new(&self._common, buf));
                let unknown: &[u8] = &self._common.unknown_fields;
                ::bytes::BufMut::put_slice(buf, unknown);
            }

            fn merge_from_with_depth<B: ::bytes::Buf>(
                &mut self,
                buf: &mut B,
                depth: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                if depth >= ::puroro::RECURSION_LIMIT {
                    return ::core::result::Result::Err(::puroro::DecodeError::RecursionLimitExceeded);
                }
                while ::bytes::Buf::has_remaining(buf) {
                    let (field_number, wire_type) = ::puroro_rt::decode::decode_tag(buf)?;
                    match field_number {
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

            fn unknown_fields(
                &self,
            ) -> impl ::core::iter::Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
                self._common.iter_unknown_fields()
            }

            fn validate(&self) -> ::core::result::Result<(), ::puroro::DecodeError> {
                #validate_body
            }
        }
    })
}

fn collect_fields(plan: &MessagePlan<'_>) -> Result<Vec<FieldEmit>> {
    let mut out = Vec::new();
    for member in plan.members() {
        match member {
            MessageMember::Oneof(o) => {
                return Err(Error::Codegen(format!(
                    "oneof `{}` is not supported by the field emitter yet",
                    o.name()
                )));
            }
            MessageMember::Field(field) => {
                if !is_simple_ident(field.name()) {
                    return Err(Error::Codegen(format!(
                        "field name `{}` is not a simple Rust identifier",
                        field.name()
                    )));
                }
                match field.kind() {
                    FieldKind::Repeated { wire, encoding } => {
                        out.push(FieldEmit::Repeated(repeated_emit(
                            field.name(),
                            field.field_const(),
                            field.number(),
                            wire,
                            *encoding,
                        )?));
                    }
                    FieldKind::Singular {
                        wire,
                        presence,
                        layout,
                    } => {
                        if matches!(presence, CatalogPresence::Oneof) {
                            return Err(Error::Codegen(format!(
                                "field `{}` presence {:?} is not supported by the field emitter yet",
                                field.name(),
                                presence
                            )));
                        }
                        out.push(FieldEmit::Singular(scalar_emit(
                            field.name(),
                            field.field_const(),
                            field.number(),
                            wire,
                            presence,
                            layout,
                        )?));
                    }
                }
            }
        }
    }
    Ok(out)
}

fn repeated_emit(
    name: &str,
    field_const: &str,
    number: i32,
    wire: &WireTypeKind<'_>,
    encoding: RepeatedEncodingKind,
) -> Result<RepeatedEmit> {
    let encoding_ty = match encoding {
        RepeatedEncodingKind::Packed => quote! { ::puroro_rt::Packed },
        RepeatedEncodingKind::Expanded => quote! { ::puroro_rt::Expanded },
    };
    let (style, slice_elem_ty) = match wire {
        WireTypeKind::String { .. } => (RepeatedAccessorStyle::String, TokenStream::new()),
        WireTypeKind::Bytes { .. } => (RepeatedAccessorStyle::Bytes, TokenStream::new()),
        _ => (
            RepeatedAccessorStyle::Slice,
            repeated_slice_elem_type(wire)?,
        ),
    };
    Ok(RepeatedEmit {
        name: rust_ident(name),
        name_str: name.to_owned(),
        field_const: Ident::new(field_const, Span::call_site()),
        number: number as u32,
        marker: wire_marker_path(wire)?,
        encoding_ty,
        style,
        slice_elem_ty,
    })
}

fn scalar_emit(
    name: &str,
    field_const: &str,
    number: i32,
    wire: &WireTypeKind<'_>,
    presence: &CatalogPresence,
    layout: &CatalogLayout,
) -> Result<ScalarEmit> {
    let (style, presence_ty, presence_bit) = match presence {
        CatalogPresence::Implicit => (
            AccessorStyle::Implicit,
            quote! { ::puroro_rt::Implicit },
            None,
        ),
        CatalogPresence::Explicit { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                AccessorStyle::Explicit,
                quote! { ::puroro_rt::Explicit<{ #ident }> },
                Some((ident, *bit)),
            )
        }
        CatalogPresence::LegacyRequired { bit, bit_const } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                AccessorStyle::LegacyRequired,
                quote! { ::puroro_rt::LegacyRequired<{ #ident }> },
                Some((ident, *bit)),
            )
        }
        CatalogPresence::Message => (
            AccessorStyle::Message,
            quote! { ::puroro_rt::Message },
            None,
        ),
        CatalogPresence::Oneof => {
            return Err(Error::Codegen(
                "internal error: unsupported presence in scalar_emit".into(),
            ));
        }
    };

    let (layout_ty, value_bit) = match layout {
        CatalogLayout::Inline => (None, None),
        CatalogLayout::BitPacked {
            value_bit,
            bit_const,
        } => {
            let ident = Ident::new(bit_const, Span::call_site());
            (
                Some(quote! { ::puroro_rt::BitPacked<{ #ident }> }),
                Some((ident, *value_bit)),
            )
        }
    };

    let is_enum = matches!(wire, WireTypeKind::Enum { .. });
    let is_message = matches!(style, AccessorStyle::Message);
    Ok(ScalarEmit {
        name: rust_ident(name),
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
        mut_target: mut_target_type(wire)?,
        implicit_ty: if is_message {
            TokenStream::new()
        } else {
            implicit_value_type(wire)?
        },
        optional_ty: if is_message {
            TokenStream::new()
        } else {
            optional_value_type(wire)?
        },
    })
}

fn render_bit_consts(fields: &[FieldEmit]) -> TokenStream {
    let mut items = Vec::new();
    for field in fields {
        let FieldEmit::Singular(field) = field else {
            continue;
        };
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
    if items.is_empty() {
        TokenStream::new()
    } else {
        quote! {
            // Bit indices — presence then bool value bits (field-number order).
            #(#items)*
        }
    }
}

fn render_field_consts(fields: &[FieldEmit]) -> TokenStream {
    if fields.is_empty() {
        return TokenStream::new();
    }
    let items: Vec<_> = fields
        .iter()
        .map(|field| {
            let ident = field.field_const();
            let number = field.number();
            quote! {
                pub const #ident: u32 = #number;
            }
        })
        .collect();
    quote! {
        // Proto field numbers.
        #(#items)*
    }
}

fn render_struct_fields(fields: &[FieldEmit]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| match field {
            FieldEmit::Singular(field) => {
                let name = &field.name;
                let marker = &field.marker;
                let presence_ty = &field.presence_ty;
                let field_const = &field.field_const;
                match &field.layout_ty {
                    None => quote! {
                        #name: ::puroro_rt::SingularField<#marker, #presence_ty, { #field_const }, A>,
                    },
                    Some(layout_ty) => quote! {
                        #name: ::puroro_rt::SingularField<#marker, #presence_ty, { #field_const }, A, #layout_ty>,
                    },
                }
            }
            FieldEmit::Repeated(field) => {
                let name = &field.name;
                let marker = &field.marker;
                let encoding_ty = &field.encoding_ty;
                let field_const = &field.field_const;
                quote! {
                    #name: ::puroro_rt::RepeatedField<#marker, #encoding_ty, { #field_const }, A>,
                }
            }
        })
        .collect()
}

fn wire_marker_path(wire: &WireTypeKind<'_>) -> Result<TokenStream> {
    Ok(match wire {
        WireTypeKind::Double => quote! { ::puroro_rt::ProtoDouble },
        WireTypeKind::Float => quote! { ::puroro_rt::ProtoFloat },
        WireTypeKind::Int64 => quote! { ::puroro_rt::ProtoInt64 },
        WireTypeKind::UInt64 => quote! { ::puroro_rt::ProtoUInt64 },
        WireTypeKind::Int32 => quote! { ::puroro_rt::ProtoInt32 },
        WireTypeKind::Fixed64 => quote! { ::puroro_rt::ProtoFixed64 },
        WireTypeKind::Fixed32 => quote! { ::puroro_rt::ProtoFixed32 },
        WireTypeKind::Bool => quote! { ::puroro_rt::ProtoBool },
        WireTypeKind::String { .. } => quote! { ::puroro_rt::ProtoString },
        WireTypeKind::Bytes { .. } => quote! { ::puroro_rt::ProtoBytes },
        WireTypeKind::UInt32 => quote! { ::puroro_rt::ProtoUInt32 },
        WireTypeKind::SFixed32 => quote! { ::puroro_rt::ProtoSFixed32 },
        WireTypeKind::SFixed64 => quote! { ::puroro_rt::ProtoSFixed64 },
        WireTypeKind::SInt32 => quote! { ::puroro_rt::ProtoSint32 },
        WireTypeKind::SInt64 => quote! { ::puroro_rt::ProtoSint64 },
        WireTypeKind::Enum { ty, openness } => {
            let path = fqn_to_enum_root_path(ty)?;
            let kind = match openness {
                EnumType::Open => quote! { ::puroro_rt::Open },
                EnumType::Closed => quote! { ::puroro_rt::Closed },
            };
            quote! { ::puroro_rt::ProtoEnum<#path, #kind> }
        }
        WireTypeKind::Message(m) => {
            let path = fqn_to_message_root_path(m)?;
            quote! { ::puroro_rt::ProtoMessage<#path<A>> }
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
            let ctor = match field {
                FieldEmit::Singular(_) => quote! { ::puroro_rt::SingularField::new_in },
                FieldEmit::Repeated(_) => quote! { ::puroro_rt::RepeatedField::new_in },
            };
            if i == last {
                quote! { #name: #ctor(alloc), }
            } else {
                quote! { #name: #ctor(alloc.clone()), }
            }
        })
        .collect()
}

fn render_accessors(fields: &[FieldEmit]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| match field {
            FieldEmit::Repeated(field) => render_repeated_accessors(field),
            FieldEmit::Singular(field) => render_singular_accessors(field),
        })
        .collect()
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

            pub fn #name_mut(&mut self) -> impl ::puroro::RepeatedContainerMut + '_ {
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
        match field.style {
            AccessorStyle::Implicit => quote! {
                pub fn #name(&self) -> #implicit_ty {
                    self.#name.bind(&self._common).value()
                }
            },
            AccessorStyle::Explicit | AccessorStyle::LegacyRequired => quote! {
                pub fn #name<'a>(&'a self) -> ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>
                where
                    A: 'a,
                {
                    self.#name.bind(&self._common).optional()
                }
            },
            AccessorStyle::Message => unreachable!("handled above"),
        }
    };

    quote! {
        #getter

        pub fn #name_mut<'s>(&'s mut self) -> impl ::core::ops::DerefMut<Target = #mut_target> + 's {
            self.#name.bind_mut(&mut self._common).value_mut()
        }

        pub fn #clear_name(&mut self) {
            self.#name.bind_mut(&mut self._common).clear();
        }
    }
}

fn implicit_value_type(wire: &WireTypeKind<'_>) -> Result<TokenStream> {
    Ok(match wire {
        WireTypeKind::Double => quote! { f64 },
        WireTypeKind::Float => quote! { f32 },
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => quote! { i64 },
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => quote! { u64 },
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => quote! { i32 },
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => quote! { u32 },
        WireTypeKind::Bool => quote! { bool },
        WireTypeKind::String { .. } => quote! { &str },
        WireTypeKind::Bytes { .. } => quote! { &[u8] },
        WireTypeKind::Enum { ty, .. } => fqn_to_enum_root_path(ty)?,
        WireTypeKind::Message(_) => {
            return Err(Error::Codegen(
                "internal error: message in implicit_value_type".into(),
            ));
        }
    })
}

fn optional_value_type(wire: &WireTypeKind<'_>) -> Result<TokenStream> {
    Ok(match wire {
        WireTypeKind::Double => quote! { f64 },
        WireTypeKind::Float => quote! { f32 },
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => quote! { i64 },
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => quote! { u64 },
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => quote! { i32 },
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => quote! { u32 },
        WireTypeKind::Bool => quote! { bool },
        WireTypeKind::String { .. } => quote! { &'a str },
        WireTypeKind::Bytes { .. } => quote! { &'a [u8] },
        WireTypeKind::Enum { ty, .. } => fqn_to_enum_root_path(ty)?,
        WireTypeKind::Message(_) => {
            return Err(Error::Codegen(
                "internal error: message in optional_value_type".into(),
            ));
        }
    })
}

fn mut_target_type(wire: &WireTypeKind<'_>) -> Result<TokenStream> {
    Ok(match wire {
        WireTypeKind::Double => quote! { f64 },
        WireTypeKind::Float => quote! { f32 },
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => quote! { i64 },
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => quote! { u64 },
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => quote! { i32 },
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => quote! { u32 },
        WireTypeKind::Bool => quote! { bool },
        WireTypeKind::String { .. } => quote! { ::puroro::String<A> },
        WireTypeKind::Bytes { .. } => quote! { ::allocator_api2::vec::Vec<u8, A> },
        WireTypeKind::Enum { ty, .. } => fqn_to_enum_root_path(ty)?,
        WireTypeKind::Message(m) => {
            let path = fqn_to_message_root_path(m)?;
            quote! { #path<A> }
        }
    })
}

fn repeated_slice_elem_type(wire: &WireTypeKind<'_>) -> Result<TokenStream> {
    Ok(match wire {
        WireTypeKind::Double => quote! { f64 },
        WireTypeKind::Float => quote! { f32 },
        WireTypeKind::Int64 | WireTypeKind::SInt64 | WireTypeKind::SFixed64 => quote! { i64 },
        WireTypeKind::UInt64 | WireTypeKind::Fixed64 => quote! { u64 },
        WireTypeKind::Int32 | WireTypeKind::SInt32 | WireTypeKind::SFixed32 => quote! { i32 },
        WireTypeKind::UInt32 | WireTypeKind::Fixed32 => quote! { u32 },
        WireTypeKind::Bool => quote! { bool },
        WireTypeKind::Enum { ty, .. } => fqn_to_enum_root_path(ty)?,
        WireTypeKind::Message(m) => {
            let path = fqn_to_message_root_path(m)?;
            quote! { #path<A> }
        }
        WireTypeKind::String { .. } | WireTypeKind::Bytes { .. } => {
            return Err(Error::Codegen(
                "internal error: string/bytes in repeated_slice_elem_type".into(),
            ));
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

fn render_merge_arms(fields: &[FieldEmit]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let name = field.name();
            let field_const = field.field_const();
            quote! {
                #field_const => {
                    self.#name
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
            }
        })
        .collect()
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
