//! Companion `Layout` trait: catalog slots that differ between eager and lazy.

use super::parse;
use crate::error::Result;
use crate::module_tree::nested_root_alias;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::quote;
use ::syn::{Item, Type};

/// How a lazy singular / oneof LEN getter reads the slot.
#[derive(Clone, Copy)]
pub(super) enum LenAccess {
    /// `utf8_validation=VERIFY` string. `profile_str`.
    Str,
    /// `bytes` or `utf8_validation=NONE` string. `profile_bytes`.
    Bytes,
}

/// Length-prefixed vs group framing for a nested message apply arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MessageFraming {
    LengthPrefixed,
    Group,
}

/// How a lazy parent records one nested message.
///
/// A child that implements `LazyMessage` stores the span with `merge_shared`,
/// whether the slot is singular, repeated, or a oneof variant. Eager singular
/// and eager repeated still differ: the latter names the type used to
/// construct one value per occurrence. Nesting is not counted here; an eager
/// child receives `depth + 1` from the apply arm. Group framing uses the
/// scanned group body (no length prefix); length-prefixed uses the LEN payload.
#[derive(Clone)]
pub(super) enum MessageApply {
    /// Child implements `LazyMessage`. Singular, repeated, and oneof call `merge_shared`.
    Shared(MessageFraming),
    /// Singular or oneof child is the eager type. Decode the body into the slot.
    EagerSingular(MessageFraming),
    /// Repeated child is the eager type. Construct one value per occurrence and push it.
    EagerRepeated {
        child: Box<Type>,
        framing: MessageFraming,
    },
}

/// One associated type on `mod layout::Layout`.
#[derive(Clone)]
pub(super) struct LayoutAssoc {
    pub assoc: Ident,
    pub kind: LayoutAssocKind,
}

#[derive(Clone)]
pub(super) enum LayoutAssocKind {
    /// Singular string / bytes: `InlineOrHeap` or `Inline` vs `P::StringLen` /
    /// `P::BytesLen` (`WireOrSso` or `SpanOnly`).
    SingularLen {
        marker: Type,
        eager: Type,
        lazy: Type,
    },
    /// Repeated string / bytes: `RepeatedReady` vs `RepeatedSpans`.
    Repeated {
        marker: Type,
        encoding: Type,
        field_const: Ident,
    },
    /// Map: `MapReady` vs `MapSpans`.
    Map {
        key: Type,
        value: Type,
        field_const: Ident,
    },
    /// Nested message that follows the parent layout.
    Child { eager: Type, lazy: Type },
}

pub(super) fn render_module(
    assocs: &[LayoutAssoc],
    eager_bits: TokenStream,
    lazy_bits: TokenStream,
    lazy_interior: bool,
    emit_lazy: bool,
) -> Result<Vec<Item>> {
    let root_alias = nested_root_alias();
    let assoc_defs: Vec<TokenStream> = assocs.iter().map(assoc_def).collect();
    let eager_tys: Vec<TokenStream> = assocs.iter().map(eager_ty).collect();
    let lazy_tys: Vec<TokenStream> = assocs.iter().map(lazy_ty).collect();
    let lazy_empty = if lazy_interior {
        quote! { ::puroro_rt::InteriorBitArray::zero() }
    } else {
        quote! { ::bitvec::array::BitArray::ZERO }
    };

    let layout_doc = if emit_lazy {
        "Catalog slots that differ between the eager and lazy aliases."
    } else {
        "Catalog slots for the eager message (lazy aliases need `--puroro_opt=lazy=true`)."
    };

    let lazy_impl = if emit_lazy {
        quote! {
            impl<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                P: ::puroro_rt::decode::LazyProfile,
            > Layout<A> for ::puroro_rt::Lazy<A, P>
            {
                type Bits = #lazy_bits;
                #(#lazy_tys)*

                #[inline]
                fn empty_bits() -> Self::Bits {
                    #lazy_empty
                }
            }
        }
    } else {
        quote! {}
    };

    parse::parse_items(quote! {
        pub mod layout {
            #root_alias

            #[doc = #layout_doc]
            ///
            /// Numericals, enums, and bools stay on the struct. This trait names
            /// string / bytes slots, repeated LEN storage, maps, and nested
            /// messages that follow the parent.
            pub trait Layout<A: ::allocator_api2::alloc::Allocator>: ::core::marker::Sized {
                type Bits: ::core::default::Default + ::core::clone::Clone + ::puroro_rt::BitStorage;

                #(#assoc_defs)*

                fn empty_bits() -> Self::Bits;
            }

            impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> Layout<A>
                for ::puroro_rt::Eager
            {
                type Bits = #eager_bits;
                #(#eager_tys)*

                #[inline]
                fn empty_bits() -> Self::Bits {
                    ::bitvec::array::BitArray::ZERO
                }
            }

            #lazy_impl
        }

        pub use layout::Layout;
    })
}

fn assoc_def(assoc: &LayoutAssoc) -> TokenStream {
    let name = &assoc.assoc;
    match &assoc.kind {
        LayoutAssocKind::SingularLen { marker, .. } => quote! {
            type #name: ::puroro_rt::ValueLayout<#marker, A>;
        },
        LayoutAssocKind::Repeated {
            marker,
            encoding,
            field_const,
        } => quote! {
            type #name: ::puroro_rt::RepeatedLayout<
                #marker,
                #encoding,
                { super::#field_const },
                A,
            >;
        },
        LayoutAssocKind::Map {
            key,
            value,
            field_const,
        } => quote! {
            type #name: ::puroro_rt::MapLayout<#key, #value, { super::#field_const }, A>;
        },
        LayoutAssocKind::Child { .. } => quote! {
            type #name: ::puroro_rt::MessageEncode
                + ::puroro_rt::DeallocateBound<A>
                + ::puroro_rt::DeallocateIn<A>;
        },
    }
}

fn eager_ty(assoc: &LayoutAssoc) -> TokenStream {
    let name = &assoc.assoc;
    match &assoc.kind {
        LayoutAssocKind::SingularLen { eager, .. } => quote! { type #name = #eager; },
        LayoutAssocKind::Repeated { .. } => {
            quote! { type #name = ::puroro_rt::RepeatedReady; }
        }
        LayoutAssocKind::Map { .. } => quote! { type #name = ::puroro_rt::MapReady; },
        LayoutAssocKind::Child { eager, .. } => quote! { type #name = #eager; },
    }
}

fn lazy_ty(assoc: &LayoutAssoc) -> TokenStream {
    let name = &assoc.assoc;
    match &assoc.kind {
        LayoutAssocKind::SingularLen { lazy, .. } => quote! { type #name = #lazy; },
        LayoutAssocKind::Repeated { .. } => {
            quote! { type #name = ::puroro_rt::RepeatedSpans; }
        }
        LayoutAssocKind::Map { .. } => quote! { type #name = ::puroro_rt::MapSpans; },
        LayoutAssocKind::Child { lazy, .. } => quote! { type #name = #lazy; },
    }
}
