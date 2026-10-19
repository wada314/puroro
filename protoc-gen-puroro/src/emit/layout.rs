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
    /// `utf8_validation=VERIFY` string. `try_str`.
    Str,
    /// `bytes` or `utf8_validation=NONE` string. `try_bytes`.
    Bytes,
}

/// How a lazy parent records one message LEN.
///
/// Singular and repeated differ in the slot method (`&mut` merge vs append),
/// so each shape is its own variant. An eager repeated child names the type
/// used to construct one value per LEN.
#[derive(Clone)]
pub(super) enum MessageApply {
    /// Singular or oneof child implements `LazyMessage`.
    SharedSingular,
    /// Repeated child implements `LazyMessage`.
    SharedRepeated,
    /// Singular or oneof child is the eager type. Decode the LEN into the slot.
    EagerSingular,
    /// Repeated child is the eager type. Construct one value per LEN and push it.
    EagerRepeated(Box<Type>),
}

/// One associated type on `mod layout::Layout`.
#[derive(Clone)]
pub(super) struct LayoutAssoc {
    pub assoc: Ident,
    pub kind: LayoutAssocKind,
}

#[derive(Clone)]
pub(super) enum LayoutAssocKind {
    /// Singular string / bytes: `InlineOrHeap` or `Inline` vs `WireOrSso`.
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

    parse::parse_items(quote! {
        pub mod layout {
            #root_alias

            /// Catalog slots that differ between the eager and lazy aliases.
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

            impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> Layout<A>
                for ::puroro_rt::Lazy<A>
            {
                type Bits = #lazy_bits;
                #(#lazy_tys)*

                #[inline]
                fn empty_bits() -> Self::Bits {
                    #lazy_empty
                }
            }
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
