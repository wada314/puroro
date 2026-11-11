//! Lazy getters on `Record<FooFields<A, Lazy<A>>, …>`.

use super::layout::{LayoutAssoc, LayoutAssocKind, MessageApply};
use super::message::{FieldEmit, MapEmit, RepeatedEmit, ScalarEmit};
use super::oneof::OneofEmit;
use crate::error::Result;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::quote;
use ::syn::{Item, Type};

/// Lazy getters on the generated lazy newtype. Shell traits live in `puroro-rt`.
pub(super) fn render_items(
    fields: &[FieldEmit],
    eager_alias: &Ident,
    lazy_alias: &Ident,
    companion: &Ident,
) -> Result<Vec<Item>> {
    let getters = render_getters(fields, companion);
    super::parse::parse_items(quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> #lazy_alias<A> {
            #(#getters)*

            /// Decode `_wire` into the eager message.
            pub fn into_eager(
                self,
            ) -> ::core::result::Result<#eager_alias<A>, ::puroro::DecodeError> {
                ::core::result::Result::Ok(#eager_alias {
                    record: ::puroro_rt::Record::decode_eager(self.record)?,
                })
            }
        }
    })
}

fn render_getters(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    let mut out = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => out.push(singular_getter(field, companion)),
            FieldEmit::Repeated(field) => out.push(repeated_getter(field, companion)),
            FieldEmit::Map(field) => out.push(map_getter(field, companion)),
            FieldEmit::Oneof(field) => out.extend(oneof_getters(field, companion)),
        }
    }
    out
}

fn scan_guard() -> TokenStream {
    quote! { ::puroro_rt::decode::LazyMessage::ensure_scanned(self)?; }
}

fn optional_method(name: &Ident, optional_ty: &Type, body: TokenStream) -> TokenStream {
    quote! {
        pub fn #name<'a>(
            &'a self,
        ) -> ::core::result::Result<
            ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>,
            ::puroro::DecodeError,
        >
        where
            A: 'a,
        {
            #body
        }
    }
}

fn message_method(name: &Ident, view: &TokenStream, expr: TokenStream) -> TokenStream {
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<::core::option::Option<&#view>, ::puroro::DecodeError> {
            #expr
        }
    }
}

fn singular_getter(field: &ScalarEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let get = super::message::record_get(companion, &field.field_const);
    if field.len_access.is_some() {
        let has = field.presence_bit.as_ref().map(|_| {
            let has_name = Ident::new(&format!("has_{}", field.name_str), name.span());
            let has_body = super::message::record_has(companion, &field.field_const);
            quote! {
                pub fn #has_name(&self) -> ::core::result::Result<bool, ::puroro::DecodeError> {
                    #has_body
                }
            }
        });
        let method = optional_method(name, &field.optional_ty, get);
        return quote! {
            #method
            #has
        };
    }
    if field.message_apply.is_some() {
        let view = lazy_child_view(field.layout.as_ref(), &field.mut_target, companion);
        return message_method(name, &view, get);
    }
    if field.optional_getter {
        optional_method(name, &field.optional_ty, get)
    } else {
        let implicit_ty = &field.implicit_ty;
        quote! {
            pub fn #name(&self) -> ::core::result::Result<#implicit_ty, ::puroro::DecodeError> {
                #get
            }
        }
    }
}

fn repeated_getter(field: &RepeatedEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let elem = lazy_child_view(field.layout.as_ref(), &field.slice_elem_ty, companion);
    let get = super::message::record_get(companion, &field.field_const);
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<&[#elem], ::puroro::DecodeError> {
            #get
        }
    }
}

fn map_getter(field: &MapEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let key = &field.key_view;
    let child = field
        .layouts
        .iter()
        .find(|assoc| matches!(assoc.kind, LayoutAssocKind::Child { .. }));
    let value = lazy_child_view(child, &field.value_view, companion);
    let get = super::message::record_get(companion, &field.field_const);
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<
            impl ::puroro::MapRef<#key, #value> + '_,
            ::puroro::DecodeError,
        > {
            #get
        }
    }
}

fn oneof_getters(field: &OneofEmit, companion: &Ident) -> Vec<TokenStream> {
    let name = &field.name;
    let case_name = &field.case_name;
    let storage = &field.storage_name;
    let ensured = scan_guard();
    let mut out = vec![quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<
            ::core::option::Option<#companion::#case_name>,
            ::puroro::DecodeError,
        > {
            #ensured
            ::core::result::Result::Ok(self.record.fields.#name.as_ref().map(
                <#companion::#storage<A, ::puroro_rt::Lazy<A>> as ::puroro_rt::OneofGroup>::case,
            ))
        }
    }];
    for variant in &field.variants {
        let vname = &variant.name;
        let field_const = &variant.field_const;
        let get = super::message::record_get(companion, field_const);
        let body = if variant.len_access.is_some() {
            optional_method(vname, &variant.optional_ty, get)
        } else if variant.message_apply.is_some() {
            let view = lazy_child_view(variant.layout.as_ref(), &variant.mut_target, companion);
            message_method(vname, &view, get)
        } else {
            let optional_ty = &variant.optional_ty;
            quote! {
                pub fn #vname(
                    &self,
                ) -> ::core::result::Result<
                    ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>,
                    ::puroro::DecodeError,
                > {
                    #get
                }
            }
        };
        out.push(body);
    }
    out
}

pub(super) fn render_apply_arms(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => arms.push(singular_apply(field, companion)),
            FieldEmit::Repeated(field) => arms.push(repeated_apply(field, companion)),
            FieldEmit::Map(field) => {
                arms.push(apply_arm(
                    &field.field_const,
                    companion,
                    super::message::slot_mut_expr(companion, &field.field_const),
                    SlotApply::SpanList,
                ));
            }
            FieldEmit::Oneof(field) => arms.extend(oneof_apply(field, companion)),
        }
    }
    arms
}

/// Which lazy apply arm a slot uses.
enum SlotApply<'a> {
    /// Singular or oneof string / bytes: `store_len_span`.
    Len,
    /// Repeated string / bytes, or a map: `store_span`.
    SpanList,
    Message(&'a MessageApply),
    Numerical,
    RepeatedNumerical,
}

fn singular_apply(field: &ScalarEmit, companion: &Ident) -> TokenStream {
    let kind = if field.len_access.is_some() {
        SlotApply::Len
    } else if let Some(apply) = field.message_apply.as_ref() {
        SlotApply::Message(apply)
    } else {
        SlotApply::Numerical
    };
    apply_arm(
        &field.field_const,
        companion,
        super::message::slot_mut_expr(companion, &field.field_const),
        kind,
    )
}

fn repeated_apply(field: &RepeatedEmit, companion: &Ident) -> TokenStream {
    let kind = if is_repeated_spans(field.layout.as_ref()) {
        SlotApply::SpanList
    } else if let Some(apply) = field.message_apply.as_ref() {
        SlotApply::Message(apply)
    } else {
        SlotApply::RepeatedNumerical
    };
    apply_arm(
        &field.field_const,
        companion,
        super::message::slot_mut_expr(companion, &field.field_const),
        kind,
    )
}

fn lazy_child_view(
    layout: Option<&LayoutAssoc>,
    fallback: &Type,
    companion: &Ident,
) -> TokenStream {
    if let Some(assoc) = layout
        && matches!(assoc.kind, LayoutAssocKind::Child { .. })
    {
        let name = &assoc.assoc;
        return quote! { <::puroro_rt::Lazy<A> as #companion::Layout<A>>::#name };
    }
    quote! { #fallback }
}

fn is_repeated_spans(layout: Option<&LayoutAssoc>) -> bool {
    matches!(
        layout.map(|assoc| &assoc.kind),
        Some(LayoutAssocKind::Repeated { .. })
    )
}

fn apply_arm(
    field_const: &Ident,
    companion: &Ident,
    slot: TokenStream,
    kind: SlotApply<'_>,
) -> TokenStream {
    match kind {
        SlotApply::Len => quote! {
            #companion::#field_const => {
                let span = ::puroro_rt::decode::scanned_len_span(origin, rec)?;
                #slot.store_len_span(span, common);
                ::core::result::Result::Ok(())
            }
        },
        SlotApply::SpanList => quote! {
            #companion::#field_const => {
                let span = ::puroro_rt::decode::scanned_len_span(origin, rec)?;
                #slot.store_span(
                    span,
                    common.lazy.scan.wire(),
                    common,
                )
            }
        },
        SlotApply::Message(apply) => message_apply_arm(slot, field_const, companion, apply),
        SlotApply::Numerical => quote! {
            #companion::#field_const => #slot
                .bind_mut(common)
                .merge_scanned(&rec.value),
        },
        SlotApply::RepeatedNumerical => quote! {
            #companion::#field_const => {
                let wire = common.lazy.scan.shared().clone_handle();
                #slot.bind_mut(common).merge_scanned(
                    rec,
                    wire.as_bytes(),
                    origin,
                )
            }
        },
    }
}

fn message_apply_arm(
    slot: TokenStream,
    field_const: &Ident,
    companion: &Ident,
    apply: &MessageApply,
) -> TokenStream {
    let body = match apply {
        MessageApply::SharedSingular => quote! {
            let root = common.lazy.scan.shared().clone_handle();
            #slot.merge_shared(&root, span, common)
        },
        MessageApply::SharedRepeated => quote! {
            let root = common.lazy.scan.shared().clone_handle();
            #slot.merge_shared(&root, span, common)
        },
        MessageApply::EagerSingular => quote! {
            let payload = span.slice(common.lazy.scan.wire())?;
            let child = #slot.bind_mut(common).get_mut();
            ::puroro::Message::merge_from(child, &mut &payload[..])?;
            ::core::result::Result::Ok(())
        },
        MessageApply::EagerRepeated(child_ty) => quote! {
            let payload = span.slice(common.lazy.scan.wire())?;
            let mut child = #child_ty::new_in(common.alloc.clone());
            ::puroro::Message::merge_from(&mut child, &mut &payload[..])?;
            #slot.push_in(child, common.alloc.clone());
            ::core::result::Result::Ok(())
        },
    };
    quote! {
        #companion::#field_const => {
            let span = ::puroro_rt::decode::scanned_len_span(origin, rec)?;
            #body
        }
    }
}

fn oneof_apply(field: &OneofEmit, companion: &Ident) -> Vec<TokenStream> {
    field
        .variants
        .iter()
        .map(|variant| {
            let field_const = &variant.field_const;
            let slot_mut = super::message::slot_mut_expr(companion, field_const);
            let slot = quote! {
                #slot_mut
                    .bind_mut(common)
                    .variant_mut::<{ #companion::#field_const }>()
            };
            let kind = if variant.len_access.is_some() {
                SlotApply::Len
            } else if let Some(apply) = variant.message_apply.as_ref() {
                SlotApply::Message(apply)
            } else {
                SlotApply::Numerical
            };
            apply_arm(field_const, companion, slot, kind)
        })
        .collect()
}
