//! Lazy alias inherent API and trait impls (`FooLazy` / `FooImpl<A, Lazy<A>>`).

use super::layout::{LayoutAssoc, LayoutAssocKind, LenAccess, MessageApply};
use super::message::{FieldEmit, MapEmit, RepeatedEmit, ScalarEmit};
use super::oneof::OneofEmit;
use crate::error::Result;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::quote;
use ::syn::{Item, Type};

pub(super) fn render_items(
    fields: &[FieldEmit],
    impl_name: &Ident,
    eager_alias: &Ident,
    lazy_alias: &Ident,
    name_str: &str,
    companion: &Ident,
    validate_body: &TokenStream,
) -> Result<Vec<Item>> {
    let lazy_ty = quote! { #impl_name<A, ::puroro_rt::Lazy<A>> };
    let getters = render_getters(fields, companion);
    let apply_arms = render_apply_arms(fields, companion);
    let clone_fields = render_clone_fields(fields);
    let new_fields = super::message::new_in_field_tokens(fields);
    let lazy_name_str = format!("{name_str}Lazy");
    let default_impl = render_default_impl(lazy_alias);
    let global_impl = render_global_ctor(impl_name);
    let inherent = render_inherent(&lazy_ty, &getters, &new_fields, eager_alias, companion);
    let lazy_message = render_lazy_message(lazy_alias, &apply_arms);
    let clone_impls = render_clone_impls(lazy_alias, &clone_fields);
    let eq_debug = render_eq_debug(lazy_alias, &lazy_name_str);
    let dealloc = render_dealloc(lazy_alias);
    let codec = render_codec_impls(lazy_alias, validate_body);

    super::parse::parse_items(quote! {
        #default_impl
        #global_impl
        #inherent
        #lazy_message
        #clone_impls
        #eq_debug
        #dealloc
        #codec
    })
}

fn render_default_impl(lazy_alias: &Ident) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone + ::core::default::Default>
            ::core::default::Default for #lazy_alias<A>
        {
            fn default() -> Self {
                Self::new_in(A::default())
            }
        }
    }
}

fn render_global_ctor(impl_name: &Ident) -> TokenStream {
    quote! {
        impl #impl_name<
            ::allocator_api2::alloc::Global,
            ::puroro_rt::Lazy<::allocator_api2::alloc::Global>,
        > {
            pub fn new() -> Self {
                Self::new_in(::allocator_api2::alloc::Global)
            }

            pub fn decode<B: ::bytes::Buf>(
                mut buf: B,
            ) -> ::core::result::Result<Self, ::puroro::DecodeError> {
                let mut msg = Self::new();
                ::puroro::Message::merge_from(&mut msg, &mut buf)?;
                ::core::result::Result::Ok(msg)
            }
        }
    }
}

fn render_inherent(
    lazy_ty: &TokenStream,
    getters: &[TokenStream],
    new_fields: &[TokenStream],
    eager_alias: &Ident,
    companion: &Ident,
) -> TokenStream {
    let reads = lazy_read_methods(getters);
    let ingest = lazy_ingest_methods(new_fields, eager_alias, companion);
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> #lazy_ty {
            #reads
            #ingest
        }
    }
}

fn lazy_read_methods(getters: &[TokenStream]) -> TokenStream {
    quote! {
        #(#getters)*

        pub fn encoded_len(&self) -> ::core::result::Result<usize, ::puroro::DecodeError> {
            self._common.lazy.scan.encoded_len()
        }

        pub fn encode<B: ::bytes::BufMut>(
            &self,
            buf: &mut B,
        ) -> ::core::result::Result<(), ::puroro::DecodeError> {
            self._common.lazy.scan.encode(buf)
        }

        pub fn encode_to_vec(
            &self,
        ) -> ::core::result::Result<::std::vec::Vec<u8>, ::puroro::DecodeError> {
            let mut out = ::std::vec::Vec::with_capacity(self.encoded_len()?);
            self.encode(&mut out)?;
            ::core::result::Result::Ok(out)
        }
    }
}

fn lazy_ingest_methods(
    new_fields: &[TokenStream],
    eager_alias: &Ident,
    companion: &Ident,
) -> TokenStream {
    quote! {
            pub fn new_in(alloc: A) -> Self {
                Self {
                    _common: ::puroro_rt::LazyMessageCommon::new_in(
                        <::puroro_rt::Lazy<A> as #companion::Layout<A>>::empty_bits(),
                        alloc.clone(),
                    ),
                    #(#new_fields)*
                }
            }

            pub fn push(&mut self, chunk: &[u8]) -> ::core::result::Result<(), ::puroro::DecodeError> {
                let (origin, records) = self._common.lazy.scan.push(chunk)?;
                for rec in records {
                    ::puroro_rt::decode::LazyMessage::apply_record(self, &rec, origin)?;
                }
                ::core::result::Result::Ok(())
            }

            pub fn finish(&mut self) -> ::core::result::Result<(), ::puroro::DecodeError> {
                self._common.lazy.scan.finish()
            }

            pub fn merge_from<B: ::bytes::Buf>(
                &mut self,
                buf: &mut B,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                self.ingest_buf(buf)
            }

            fn ingest_buf<B: ::bytes::Buf>(
                &mut self,
                buf: &mut B,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                while ::bytes::Buf::has_remaining(buf) {
                    let n = ::bytes::Buf::chunk(buf).len();
                    if n == 0 {
                        let rest = ::bytes::Buf::copy_to_bytes(buf, ::bytes::Buf::remaining(buf));
                        self.push(&rest)?;
                        break;
                    }
                    self.push(::bytes::Buf::chunk(buf))?;
                    ::bytes::Buf::advance(buf, n);
                }
                self.finish()
            }

            /// Decode `_wire` into the eager alias.
            pub fn into_eager(
                self,
            ) -> ::core::result::Result<#eager_alias<A>, ::puroro::DecodeError> {
                self._common.lazy.scan.require_finished()?;
                let mut eager = #eager_alias::new_in(self._common.alloc.clone());
                self._common.lazy.scan.for_each_body(|chunk| {
                    let mut buf = chunk;
                    ::puroro::Message::merge_from(&mut eager, &mut buf)
                })?;
                ::core::result::Result::Ok(eager)
            }
    }
}

fn render_lazy_message(lazy_alias: &Ident, apply_arms: &[TokenStream]) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::decode::LazyMessage<A>
            for #lazy_alias<A>
        {
            fn scan(&self) -> &::puroro_rt::decode::LazyScan<A> {
                &self._common.lazy.scan
            }

            fn scan_mut(&mut self) -> &mut ::puroro_rt::decode::LazyScan<A> {
                &mut self._common.lazy.scan
            }

            fn apply_record(
                &mut self,
                rec: &::puroro_rt::decode::ScannedRecord,
                origin: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                let _ = origin;
                match rec.field_number.as_u32() {
                    #(#apply_arms)*
                    _ => self._common.on_unrecognized_tag(rec.field_number.as_u32()),
                }
            }
        }
    }
}

fn render_clone_impls(lazy_alias: &Ident, clone_fields: &[TokenStream]) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::CloneIn<A>
            for #lazy_alias<A>
        {
            fn clone_in(&self, alloc: A) -> Self {
                Self {
                    _common: self._common.clone_in(alloc.clone()),
                    #(#clone_fields)*
                }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::clone::Clone
            for #lazy_alias<A>
        {
            fn clone(&self) -> Self {
                ::puroro_rt::CloneIn::clone_in(self, self._common.alloc.clone())
            }
        }
    }
}

fn render_eq_debug(lazy_alias: &Ident, lazy_name_str: &str) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::cmp::PartialEq
            for #lazy_alias<A>
        {
            fn eq(&self, other: &Self) -> bool {
                ::puroro_rt::decode::LazyMessage::eq_with_eager(self, other, |msg| {
                    msg.clone().into_eager()
                })
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::core::fmt::Debug
            for #lazy_alias<A>
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if ::puroro_rt::decode::LazyMessage::ensure_scanned(self).is_err() {
                    return f.debug_struct(#lazy_name_str).finish_non_exhaustive();
                }
                match self.clone().into_eager() {
                    ::core::result::Result::Ok(eager) => ::core::fmt::Debug::fmt(&eager, f),
                    ::core::result::Result::Err(_) => {
                        f.debug_struct(#lazy_name_str).finish_non_exhaustive()
                    }
                }
            }
        }
    }
}

fn render_dealloc(lazy_alias: &Ident) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::DeallocateIn<A>
            for #lazy_alias<A>
        {
            #[inline]
            unsafe fn deallocate_in(self, _alloc: &A) {
                ::core::mem::drop(self);
            }
        }

        ::puroro_rt::impl_owned_slot_bounds!(#lazy_alias);
    }
}

fn render_codec_impls(lazy_alias: &Ident, validate_body: &TokenStream) -> TokenStream {
    quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::MessageEncode
            for #lazy_alias<A>
        {
            fn encoded_len(&self, _ctx: &mut ::puroro_rt::EncodeCtx) -> usize {
                self._common.lazy.scan.body_len()
            }

            fn encode_raw<B: ::bytes::BufMut>(
                &self,
                _ctx: &mut ::puroro_rt::EncodeCtx,
                buf: &mut B,
            ) {
                self._common
                    .lazy
                    .scan
                    .write_bodies(buf)
                    .expect("lazy wire span out of range");
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::MessageMerge
            for #lazy_alias<A>
        {
            fn merge_from_with_depth<B: ::puroro::DecodeBuf>(
                &mut self,
                buf: &mut B,
                depth: usize,
            ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                if depth >= ::puroro::RECURSION_LIMIT {
                    return ::core::result::Result::Err(
                        ::puroro::DecodeError::RecursionLimitExceeded,
                    );
                }
                self.ingest_buf(buf)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro_rt::DefaultIn<A>
            for #lazy_alias<A>
        {
            #[inline]
            fn default_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> ::puroro::Message
            for #lazy_alias<A>
        {
            type Alloc = A;

            fn new_in(alloc: A) -> Self
            where
                A: ::core::clone::Clone,
            {
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
            ) -> ::core::result::Result<(), ::puroro::DecodeError>
            where
                A: ::core::clone::Clone,
            {
                self.ingest_buf(buf)
            }

            fn unknown_fields(
                &self,
            ) -> impl ::core::iter::Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
                ::core::iter::empty()
            }

            fn validate(&self) -> ::core::result::Result<(), ::puroro::DecodeError> {
                ::puroro_rt::decode::LazyMessage::ensure_scanned(self)?;
                #validate_body
            }
        }
    }
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

fn len_read(access: LenAccess, slot: TokenStream) -> TokenStream {
    match access {
        LenAccess::Str => quote! {
            #slot.try_str(&self._common, self._common.lazy.scan.wire())
        },
        LenAccess::Bytes => quote! {
            #slot.try_bytes(&self._common, self._common.lazy.scan.wire())
        },
    }
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
    let ensured = scan_guard();
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<::core::option::Option<&#view>, ::puroro::DecodeError> {
            #ensured
            ::core::result::Result::Ok(#expr)
        }
    }
}

fn singular_getter(field: &ScalarEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let ensured = scan_guard();
    if let Some(access) = field.len_access {
        let read = len_read(access, quote! { self.#name });
        let has = field.presence_bit.as_ref().map(|(bit, _)| {
            let has_name = Ident::new(&format!("has_{}", field.name_str), name.span());
            quote! {
                pub fn #has_name(&self) -> ::core::result::Result<bool, ::puroro::DecodeError> {
                    #ensured
                    ::core::result::Result::Ok(self._common.is_bit_set(#companion::#bit))
                }
            }
        });
        let method = optional_method(
            name,
            &field.optional_ty,
            quote! {
                #ensured
                #read
            },
        );
        return quote! {
            #method
            #has
        };
    }
    if field.message_apply.is_some() {
        let view = lazy_child_view(field.layout.as_ref(), &field.mut_target, companion);
        return message_method(name, &view, quote! { self.#name.bind(&self._common).get() });
    }
    if field.optional_getter {
        optional_method(
            name,
            &field.optional_ty,
            quote! {
                #ensured
                ::core::result::Result::Ok(self.#name.bind(&self._common).optional())
            },
        )
    } else {
        let implicit_ty = &field.implicit_ty;
        quote! {
            pub fn #name(&self) -> ::core::result::Result<#implicit_ty, ::puroro::DecodeError> {
                #ensured
                ::core::result::Result::Ok(self.#name.bind(&self._common).value())
            }
        }
    }
}

fn repeated_getter(field: &RepeatedEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let elem = lazy_child_view(field.layout.as_ref(), &field.slice_elem_ty, companion);
    let ensured = scan_guard();
    let read = if is_repeated_spans(field.layout.as_ref()) {
        quote! {
            self.#name
                .bind(self._common.lazy.scan.wire(), &self._common)?
                .as_slice()
        }
    } else {
        quote! { self.#name.bind(&self._common).as_slice() }
    };
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<&[#elem], ::puroro::DecodeError> {
            #ensured
            ::core::result::Result::Ok(#read)
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
    let ensured = scan_guard();
    quote! {
        pub fn #name(
            &self,
        ) -> ::core::result::Result<
            impl ::puroro::MapRef<#key, #value> + '_,
            ::puroro::DecodeError,
        > {
            #ensured
            self.#name
                .bind(self._common.lazy.scan.wire(), &self._common)
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
            ::core::result::Result::Ok(self.#name.as_ref().map(
                <#companion::#storage<A, ::puroro_rt::Lazy<A>> as ::puroro_rt::OneofGroup>::case,
            ))
        }
    }];
    for variant in &field.variants {
        let vname = &variant.name;
        let field_const = &variant.field_const;
        let variant_of = quote! {
            self.#name
                .bind(&self._common)
                .variant_of::<{ #companion::#field_const }>()
        };
        let body = if let Some(access) = variant.len_access {
            let read = len_read(access, quote! { f });
            optional_method(
                vname,
                &variant.optional_ty,
                quote! {
                    #ensured
                    match #variant_of.as_field() {
                        ::core::option::Option::Some(f) => #read,
                        ::core::option::Option::None => {
                            ::core::result::Result::Ok(::puroro::Optional::new(
                                ::core::option::Option::None,
                            ))
                        }
                    }
                },
            )
        } else if variant.message_apply.is_some() {
            let view = lazy_child_view(variant.layout.as_ref(), &variant.mut_target, companion);
            message_method(vname, &view, quote! { #variant_of.get() })
        } else {
            let optional_ty = &variant.optional_ty;
            quote! {
                pub fn #vname(
                    &self,
                ) -> ::core::result::Result<
                    ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>,
                    ::puroro::DecodeError,
                > {
                    #ensured
                    ::core::result::Result::Ok(#variant_of.optional())
                }
            }
        };
        out.push(body);
    }
    out
}

fn render_apply_arms(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => arms.push(singular_apply(field, companion)),
            FieldEmit::Repeated(field) => arms.push(repeated_apply(field, companion)),
            FieldEmit::Map(field) => {
                let name = &field.name;
                arms.push(apply_arm(
                    &field.field_const,
                    companion,
                    quote! { self.#name },
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
    let name = &field.name;
    let kind = if field.len_access.is_some() {
        SlotApply::Len
    } else if let Some(apply) = field.message_apply.as_ref() {
        SlotApply::Message(apply)
    } else {
        SlotApply::Numerical
    };
    apply_arm(&field.field_const, companion, quote! { self.#name }, kind)
}

fn repeated_apply(field: &RepeatedEmit, companion: &Ident) -> TokenStream {
    let name = &field.name;
    let kind = if is_repeated_spans(field.layout.as_ref()) {
        SlotApply::SpanList
    } else if let Some(apply) = field.message_apply.as_ref() {
        SlotApply::Message(apply)
    } else {
        SlotApply::RepeatedNumerical
    };
    apply_arm(&field.field_const, companion, quote! { self.#name }, kind)
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
                #slot.store_len_span(span, &mut self._common);
                ::core::result::Result::Ok(())
            }
        },
        SlotApply::SpanList => quote! {
            #companion::#field_const => {
                let span = ::puroro_rt::decode::scanned_len_span(origin, rec)?;
                #slot.store_span(
                    span,
                    self._common.lazy.scan.wire(),
                    &self._common,
                )
            }
        },
        SlotApply::Message(apply) => message_apply_arm(slot, field_const, companion, apply),
        SlotApply::Numerical => quote! {
            #companion::#field_const => #slot
                .bind_mut(&mut self._common)
                .merge_scanned(&rec.value),
        },
        SlotApply::RepeatedNumerical => quote! {
            #companion::#field_const => {
                let wire = self._common.lazy.scan.shared().clone_handle();
                #slot.bind_mut(&mut self._common).merge_scanned(
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
            let root = self._common.lazy.scan.shared().clone_handle();
            #slot.merge_shared(&root, span, &mut self._common)
        },
        MessageApply::SharedRepeated => quote! {
            let root = self._common.lazy.scan.shared().clone_handle();
            #slot.merge_shared(&root, span, &self._common)
        },
        MessageApply::EagerSingular => quote! {
            let payload = span.slice(self._common.lazy.scan.wire())?;
            let child = #slot.bind_mut(&mut self._common).get_mut();
            ::puroro::Message::merge_from(child, &mut &payload[..])?;
            ::core::result::Result::Ok(())
        },
        MessageApply::EagerRepeated(child_ty) => quote! {
            let payload = span.slice(self._common.lazy.scan.wire())?;
            let mut child = #child_ty::new_in(self._common.alloc.clone());
            ::puroro::Message::merge_from(&mut child, &mut &payload[..])?;
            #slot.push_in(child, self._common.alloc.clone());
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
    let group = &field.name;
    field
        .variants
        .iter()
        .map(|variant| {
            let field_const = &variant.field_const;
            let slot = quote! {
                self.#group
                    .bind_mut(&mut self._common)
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

fn render_clone_fields(fields: &[FieldEmit]) -> Vec<TokenStream> {
    // The last field moves `alloc`. Earlier fields clone it so each
    // `clone_field` call receives an owned allocator.
    let last = fields.len().saturating_sub(1);
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let name = field.name();
            let alloc = if i == last {
                quote! { alloc }
            } else {
                quote! { alloc.clone() }
            };
            quote! {
                #name: ::puroro_rt::FieldCloneIn::clone_field(&self.#name, &self._common, #alloc),
            }
        })
        .collect()
}
