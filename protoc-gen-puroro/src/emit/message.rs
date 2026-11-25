//! Emit a message module body from a [`MessageFieldPlan`].
//!
//! Singular and repeated scalar / string / bytes / bool / enum / message
//! fields (including IMPLICIT / EXPLICIT / LEGACY_REQUIRED and bool
//! `BitPacked`), real oneof groups, and maps (legal keys × Copy scalar /
//! bool values).

use super::docs;
use super::field_emit::{
    EagerGetter, FieldEmit, MapEmit, RepeatedAccessorStyle, RepeatedEmit, ScalarEmit,
};
use super::ident::to_pascal_case;
use super::layout::{self, LayoutAssoc, LayoutAssocKind};
use super::oneof::{self, OneofEmit};
use crate::descriptor::{Comments, UnknownFieldsPolicy};
use crate::error::Result;
use crate::field_kind::{MessageFieldPlan, bit_array_byte_len};
use crate::module_tree::nested_root_alias;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::syn::{Item, Type, parse_quote};

/// Items for the parent module (struct + impls) and the snake_case companion.
pub(super) struct RenderedMessageItems {
    pub type_items: Vec<Item>,
    pub companion_items: Vec<Item>,
}

/// Render the message struct (parent) and companion (`FIELD_*`, defaults, oneofs).
///
/// `full_name` is the protobuf type name without a leading `.` (`example.Task`).
/// When `emit_lazy` is false, only the eager alias is emitted (default plugin
/// mode). Pass `--puroro_opt=lazy=true` to also emit `NameLazy`.
pub(super) fn render_items(
    field_plan: &MessageFieldPlan<'_>,
    name: &Ident,
    name_str: &str,
    companion: &Ident,
    full_name: &str,
    comments: &Comments,
    emit_lazy: bool,
) -> Result<RenderedMessageItems> {
    let fields = field_plan
        .members()
        .iter()
        .map(|member| super::field_emit::emit_member(member, companion))
        .collect::<Result<Vec<_>>>()?;
    let bits_bytes = bit_array_byte_len(field_plan.bit_count());
    let bits_ty = quote! {
        ::bitvec::array::BitArray<[u8; #bits_bytes], ::bitvec::order::Lsb0>
    };
    let lazy_interior = field_plan.lazy_bit_count() > field_plan.bit_count();
    let lazy_bytes = bit_array_byte_len(field_plan.lazy_bit_count());
    let lazy_bits_ty = if lazy_interior {
        quote! { ::puroro_rt::InteriorBitArray<#lazy_bytes> }
    } else {
        bits_ty.clone()
    };
    let unknown_ty = super::unknown_store_ty(field_plan.unknown_fields());
    // Field struct is generic over `L`. User-facing names are `Record` aliases.
    let fields_name = Ident::new(&format!("{name_str}Fields"), Span::call_site());
    let lazy_name_str = format!("{name_str}Lazy");
    let lazy_alias = Ident::new(&format!("{name_str}Lazy"), Span::call_site());

    let bit_consts = render_bit_consts(&fields);
    let field_consts = render_field_consts(&fields);
    let defaults_module = render_defaults_module(&fields);
    let slot_aliases = render_slot_aliases(&fields, name_str, companion);
    let struct_fields = render_struct_fields(&fields, name_str, companion);
    let has_slot_impls = render_has_slot_impls(&fields, &fields_name, companion, name_str);
    let new_in_fields = render_new_in_fields(&fields);
    let visit_shared = render_visit_calls(&fields, VisitKind::Shared);
    let visit_pair = render_visit_calls(&fields, VisitKind::Pair);
    let visit_mut = render_visit_calls(&fields, VisitKind::Mut);
    let accessors = render_accessors(&fields, companion);
    let getters = &accessors.getters;
    let mutators = &accessors.mutators;
    let clone_stmts = fields
        .iter()
        .map(|field| {
            let name = field.name();
            quote! {
                {
                    let new = ::puroro_rt::FieldCloneIn::clone_field(
                        &self.#name,
                        src_common,
                        alloc.clone(),
                    );
                    let mut old = ::core::mem::replace(&mut dst.#name, new);
                    ::puroro_rt::FieldDeallocate::deallocate(&mut old, dst_common);
                }
            }
        })
        .collect::<Vec<_>>();
    let merge_arms = render_merge_arms(&fields, companion);
    let validate_body = render_validate(&fields, companion);
    let layout = quote! { L };
    let validate_method = record_validate_method(validate_body.as_ref(), &layout);
    let record_fields_impl = quote! {
            impl<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                L: #companion::Layout<A> + ::puroro_rt::decode::MessageScan<A>,
            > ::puroro_rt::RecordFields<A, L> for #fields_name<A, L>
            {
                type Bits = <#layout as #companion::Layout<A>>::Bits;
                type Unknown = #unknown_ty;
                type EagerMessage = #name<A>;

                const EAGER_NAME: &'static str = #name_str;
                const LAZY_NAME: &'static str = #lazy_name_str;
                const FULL_NAME: &'static str = #full_name;

                fn empty_bits() -> Self::Bits {
                    <#layout as #companion::Layout<A>>::empty_bits()
                }

                fn visit_mut<
                    V: ::puroro_rt::FieldVisitorMut<
                        ::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, #layout>,
                    >,
                >(
                    &mut self,
                    #[allow(unused_variables)] v: &mut V,
                ) -> ::core::ops::ControlFlow<V::Break> {
                    #(#visit_mut)*
                    ::core::ops::ControlFlow::Continue(())
                }

                #validate_method
            }
    };
    let construct_eager = quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone>
            ::puroro_rt::ConstructRecordFields<A, ::puroro_rt::Eager> for #fields_name<A, ::puroro_rt::Eager>
        {
            #[allow(unused_variables)]
            fn new(alloc: A) -> Self {
                Self {
                    #(#new_in_fields)*
                    _layout: ::core::marker::PhantomData,
                }
            }
        }
    };
    let construct_lazy = if emit_lazy {
        quote! {
            impl<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                P: ::puroro_rt::decode::LazyProfile,
            > ::puroro_rt::ConstructRecordFields<A, ::puroro_rt::Lazy<A, P>>
                for #fields_name<A, ::puroro_rt::Lazy<A, P>>
            {
                #[allow(unused_variables)]
                fn new(alloc: A) -> Self {
                    Self {
                        #(#new_in_fields)*
                        _layout: ::core::marker::PhantomData,
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    let construct_record_impls = [construct_eager, construct_lazy];
    let clone_eager = quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone>
            ::puroro_rt::CloneRecordFields<A, ::puroro_rt::Eager> for #fields_name<A, ::puroro_rt::Eager>
        {
            #[allow(unused_variables)]
            fn clone_fields(
                &self,
                dst: &mut Self,
                src_common: &::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, ::puroro_rt::Eager>,
                dst_common: &::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, ::puroro_rt::Eager>,
                alloc: A,
            ) {
                #(#clone_stmts)*
            }
        }
    };
    let clone_lazy = if emit_lazy {
        quote! {
            impl<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                P: ::puroro_rt::decode::LazyProfile,
            > ::puroro_rt::CloneRecordFields<A, ::puroro_rt::Lazy<A, P>>
                for #fields_name<A, ::puroro_rt::Lazy<A, P>>
            {
                #[allow(unused_variables)]
                fn clone_fields(
                    &self,
                    dst: &mut Self,
                    src_common: &::puroro_rt::MessageCommon<
                        Self::Bits,
                        A,
                        Self::Unknown,
                        ::puroro_rt::Lazy<A, P>,
                    >,
                    dst_common: &::puroro_rt::MessageCommon<
                        Self::Bits,
                        A,
                        Self::Unknown,
                        ::puroro_rt::Lazy<A, P>,
                    >,
                    alloc: A,
                ) {
                    #(#clone_stmts)*
                }
            }
        }
    } else {
        quote! {}
    };
    let clone_record_impls = [clone_eager, clone_lazy];
    let eager_record_impl = quote! {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone>
            ::puroro_rt::EagerRecordFields<A> for #fields_name<A, ::puroro_rt::Eager>
        {
            fn visit_fields<
                V: ::puroro_rt::FieldVisitor<
                    ::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, ::puroro_rt::Eager>,
                >,
            >(
                &self,
                #[allow(unused_variables)] v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #(#visit_shared)*
                ::core::ops::ControlFlow::Continue(())
            }

            fn visit_pairs<
                V: ::puroro_rt::FieldPairVisitor<
                    ::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, ::puroro_rt::Eager>,
                >,
            >(
                &self,
                #[allow(unused_variables)] other: &Self,
                #[allow(unused_variables)] v: &mut V,
            ) -> ::core::ops::ControlFlow<V::Break> {
                #(#visit_pair)*
                ::core::ops::ControlFlow::Continue(())
            }

            #[allow(unused_variables)]
            fn merge_known<B: ::puroro::DecodeBuf>(
                &mut self,
                common: &mut ::puroro_rt::MessageCommon<
                    Self::Bits,
                    A,
                    Self::Unknown,
                    ::puroro_rt::Eager,
                >,
                field_number: u32,
                wire_type: ::puroro::WireType,
                buf: &mut B,
                depth: usize,
            ) -> ::core::result::Result<bool, ::puroro::DecodeError> {
                match field_number {
                    #(#merge_arms)*
                    _ => ::core::result::Result::Ok(false),
                }
            }
        }
    };
    let oneof_modules = render_oneof_modules(
        &fields,
        &bits_ty,
        &lazy_bits_ty,
        field_plan.unknown_fields(),
        emit_lazy,
    )?;
    let layout_assocs = collect_layout_assocs(&fields);
    let layout_items = layout::render_module(
        &layout_assocs,
        bits_ty.clone(),
        lazy_bits_ty,
        lazy_interior,
        emit_lazy,
    )?;

    let companion_items = bit_consts
        .into_iter()
        .chain(field_consts)
        .chain(defaults_module)
        .chain(layout_items)
        .chain(oneof_modules)
        .collect();

    let message_doc = docs::doc_attr(comments);
    let lazy_alias_items = if emit_lazy {
        let apply_arms = super::lazy::render_apply_arms(&fields, companion);
        let lazy_doc = quote! {
            #message_doc
            /// Experimental lazy alias. Emitted only with `--puroro_opt=lazy=true`.
            ///
            /// Design and behaviour may change; see DESIGN.md §8.
        };
        quote! {
            #lazy_doc
            pub struct #lazy_alias<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone = ::allocator_api2::alloc::Global,
                P: ::puroro_rt::decode::LazyProfile = ::puroro_rt::decode::Owned,
            > {
                #[allow(clippy::type_complexity)]
                record: ::puroro_rt::Record<
                    #fields_name<A, ::puroro_rt::Lazy<A, P>>,
                    A,
                    ::puroro_rt::Lazy<A, P>,
                    #unknown_ty,
                >,
            }

            ::puroro_rt::forward_record!(lazy #lazy_alias, #fields_name);

            impl<
                A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                P: ::puroro_rt::decode::LazyProfile,
            > ::puroro_rt::LazyRecordApply<A, P> for #fields_name<A, ::puroro_rt::Lazy<A, P>>
            {
                #[allow(unused_variables)]
                fn apply_record(
                    &mut self,
                    common: &mut ::puroro_rt::MessageCommon<
                        Self::Bits,
                        A,
                        Self::Unknown,
                        ::puroro_rt::Lazy<A, P>,
                    >,
                    rec: &::puroro_rt::decode::ScannedRecord,
                    origin: usize,
                    depth: usize,
                ) -> ::core::result::Result<(), ::puroro::DecodeError> {
                    match rec.field_number.as_u32() {
                        #(#apply_arms)*
                        _ => common.note_unrecognized_record(rec, origin),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let into_lazy_method = if emit_lazy {
        quote! {
            /// Encodes `self` and rebuilds it as the experimental lazy alias.
            pub fn into_lazy(self) -> #lazy_alias<A> {
                #lazy_alias {
                    record: ::puroro_rt::Record::encode_lazy(self.record),
                }
            }
        }
    } else {
        quote! {}
    };

    let mut type_items = super::parse::parse_items(quote! {
        // @generated message body from protoc-gen-puroro.

        #(#slot_aliases)*

        pub(crate) struct #fields_name<
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone = ::allocator_api2::alloc::Global,
            L: #companion::Layout<A> = ::puroro_rt::Eager,
        > {
            #(#struct_fields)*
            _layout: ::core::marker::PhantomData<(A, L)>,
        }

        #(#has_slot_impls)*

        #message_doc
        pub struct #name<
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone = ::allocator_api2::alloc::Global,
        > {
            record: ::puroro_rt::Record<
                #fields_name<A, ::puroro_rt::Eager>,
                A,
                ::puroro_rt::Eager,
                #unknown_ty,
            >,
        }

        #lazy_alias_items

        ::puroro_rt::forward_record!(eager #name, #fields_name);

        #record_fields_impl

        #(#construct_record_impls)*

        #(#clone_record_impls)*

        #eager_record_impl

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> #name<A> {
            #(#getters)*

            #(#mutators)*

            #into_lazy_method
        }
    })?;
    if emit_lazy {
        type_items.extend(super::lazy::render_items(
            &fields,
            name,
            &fields_name,
            &lazy_alias,
            companion,
        )?);
    }

    Ok(RenderedMessageItems {
        type_items,
        companion_items,
    })
}

fn render_defaults_module(fields: &[FieldEmit]) -> Option<Item> {
    let mut items: Vec<Item> = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(f) => {
                if let Some((_, marker_items)) = &f.custom_default {
                    items.extend(marker_items.iter().cloned());
                }
            }
            FieldEmit::Oneof(o) => {
                for v in &o.variants {
                    if let Some((_, marker_items)) = &v.custom_default {
                        items.extend(marker_items.iter().cloned());
                    }
                }
            }
            FieldEmit::Repeated(_) | FieldEmit::Map(_) => {}
        }
    }
    if items.is_empty() {
        return None;
    }
    let root_alias = nested_root_alias();
    Some(parse_quote! {
        pub(crate) mod defaults {
            #root_alias
            #(#items)*
        }
    })
}

fn render_bit_consts(fields: &[FieldEmit]) -> Vec<Item> {
    let mut items = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => {
                if let Some((ident, bit)) = &field.presence_bit {
                    items.push(parse_quote! {
                        pub const #ident: usize = #bit;
                    });
                }
                if let Some((ident, bit)) = &field.value_bit {
                    items.push(parse_quote! {
                        pub const #ident: usize = #bit;
                    });
                }
                if let Some((ident, bit)) = &field.lazy_kind_const {
                    items.push(parse_quote! {
                        pub const #ident: usize = #bit;
                    });
                }
            }
            FieldEmit::Oneof(o) => items.extend(oneof::render_bit_consts(o)),
            FieldEmit::Repeated(_) | FieldEmit::Map(_) => {}
        }
    }
    items
}

/// Companion `FIELD_*` consts (`pub const FIELD_TITLE: u32 = 1`).
///
/// A oneof is one struct member but each variant is its own proto field, so
/// those consts are emitted here (the oneof module names them `super::FIELD_*`).
fn render_field_consts(fields: &[FieldEmit]) -> Vec<Item> {
    let mut items = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(f) => items.push(field_const_item(&f.field_const, f.number)),
            FieldEmit::Repeated(f) => items.push(field_const_item(&f.field_const, f.number)),
            FieldEmit::Map(f) => items.push(field_const_item(&f.field_const, f.number)),
            FieldEmit::Oneof(o) => items.extend(oneof::render_field_consts(o)),
        }
    }
    items
}

fn field_const_item(ident: &Ident, number: u32) -> Item {
    parse_quote! {
        pub const #ident: u32 = #number;
    }
}

fn slot_alias(message: &str, field_name: &str) -> Ident {
    Ident::new(
        &format!("{message}{}Slot", to_pascal_case(field_name)),
        Span::call_site(),
    )
}

/// `L` is a parameter only when the catalog type names the layout (`L::TitleLen`, …).
fn layout_param_used(ty: &TokenStream) -> bool {
    ty.clone().into_iter().any(|tree| match tree {
        ::proc_macro2::TokenTree::Ident(ident) => ident == "L",
        ::proc_macro2::TokenTree::Group(group) => layout_param_used(&group.stream()),
        ::proc_macro2::TokenTree::Punct(_) | ::proc_macro2::TokenTree::Literal(_) => false,
    })
}

fn slot_params(ty: &TokenStream) -> TokenStream {
    if layout_param_used(ty) {
        quote! { A, L }
    } else {
        quote! { A }
    }
}

fn slot_applied(message: &str, field: &FieldEmit, companion: &Ident) -> TokenStream {
    let alias = slot_alias(message, field.name_str());
    let ty = field_slot_ty(field, companion);
    if layout_param_used(&ty) {
        quote! { #alias<A, L> }
    } else {
        quote! { #alias<A> }
    }
}

/// One catalog type per slot. The struct and `HasField` name this alias.
fn render_slot_aliases(fields: &[FieldEmit], message: &str, companion: &Ident) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let alias = slot_alias(message, field.name_str());
            let ty = field_slot_ty(field, companion);
            let params = slot_params(&ty);
            quote! {
                pub(crate) type #alias<#params> = #ty;
            }
        })
        .collect()
}

fn render_struct_fields(
    fields: &[FieldEmit],
    message: &str,
    companion: &Ident,
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let name = field.name();
            let applied = slot_applied(message, field, companion);
            quote! { #name: #applied, }
        })
        .collect()
}

fn render_has_slot_impls(
    fields: &[FieldEmit],
    fields_name: &Ident,
    companion: &Ident,
    message: &str,
) -> Vec<TokenStream> {
    let mut impls = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Oneof(oneof) => {
                let name = &oneof.name;
                let storage = &oneof.storage_name;
                for variant in &oneof.variants {
                    let field_const = &variant.field_const;
                    impls.push(quote! {
                        impl<
                            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                            L: #companion::Layout<A>,
                        > ::puroro_rt::HasField<{ #companion::#field_const }>
                            for #fields_name<A, L>
                        {
                            type Ref<'a> = ::puroro_rt::OneofFieldRef<
                                'a,
                                #companion::#storage<A, L>,
                                { #companion::#field_const },
                            > where Self: 'a;
                            type Mut<'a> = ::puroro_rt::OneofFieldMut<
                                'a,
                                #companion::#storage<A, L>,
                                { #companion::#field_const },
                            > where Self: 'a;

                            #[inline]
                            fn field<'a>(&'a self) -> Self::Ref<'a> {
                                ::puroro_rt::OneofFieldRef::new(&self.#name)
                            }

                            #[inline]
                            fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                                ::puroro_rt::OneofFieldMut::new(&mut self.#name)
                            }
                        }
                    });
                }
            }
            other => {
                let field_const = other
                    .field_const()
                    .expect("non-oneof field has a FIELD_* const");
                let name = other.name();
                let applied = slot_applied(message, other, companion);
                impls.push(quote! {
                    impl<
                        A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
                        L: #companion::Layout<A>,
                    > ::puroro_rt::HasField<{ #companion::#field_const }>
                        for #fields_name<A, L>
                    {
                        type Ref<'a> = &'a #applied where Self: 'a;
                        type Mut<'a> = &'a mut #applied where Self: 'a;

                        #[inline]
                        fn field<'a>(&'a self) -> Self::Ref<'a> {
                            &self.#name
                        }

                        #[inline]
                        fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                            &mut self.#name
                        }
                    }
                });
            }
        }
    }
    impls
}

enum VisitKind {
    Shared,
    Pair,
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
                VisitKind::Mut => quote! {
                    v.visit(#name_str, &mut self.#name)?;
                },
            }
        })
        .collect()
}

/// `HasField::field_mut` on the fields struct (`self` in merge / apply).
pub(super) fn field_mut_expr(companion: &Ident, field_const: &Ident) -> TokenStream {
    quote! {
        <Self as ::puroro_rt::HasField<{ #companion::#field_const }>>::field_mut(self)
    }
}

fn field_expr(companion: &Ident, field_const: &Ident) -> TokenStream {
    quote! {
        <Self as ::puroro_rt::HasField<{ #companion::#field_const }>>::field(self)
    }
}

/// Bound view from `self.record.field::<N>()`. `Record` does not implement `HasField`.
pub(super) fn eager_field(companion: &Ident, field_const: &Ident) -> TokenStream {
    bound_record_field(companion, field_const, false)
}

/// Mutable bound view of field `N` on `&mut self.record`.
pub(super) fn eager_field_mut(companion: &Ident, field_const: &Ident) -> TokenStream {
    bound_record_field(companion, field_const, true)
}

/// `self.record.field::<N>()` / `field_mut::<N>()`, already bound to `common`.
pub(super) fn bound_record_field(
    companion: &Ident,
    field_const: &Ident,
    mutable: bool,
) -> TokenStream {
    let method = if mutable {
        quote! { field_mut }
    } else {
        quote! { field }
    };
    quote! {
        self.record.#method::<{ #companion::#field_const }>()
    }
}

/// Layout type argument for [`lazy_field`].
#[derive(Clone, Copy)]
pub(super) enum LazyLayout {
    /// `Lazy<A, P>` on `impl<A, P: LazyProfile>`.
    ParamP,
}

/// Borrowed projection inside a lazy message method.
pub(super) fn lazy_field(
    fields_name: &Ident,
    companion: &Ident,
    field_const: &Ident,
    layout: LazyLayout,
) -> TokenStream {
    let layout_ty = match layout {
        LazyLayout::ParamP => quote! { ::puroro_rt::Lazy<A, P> },
    };
    message_field(fields_name, &layout_ty, companion, field_const, false)
}

fn message_field(
    fields_name: &Ident,
    layout: &TokenStream,
    companion: &Ident,
    field_const: &Ident,
    mutable: bool,
) -> TokenStream {
    let method = if mutable {
        quote! { field_mut }
    } else {
        quote! { field }
    };
    let receiver = if mutable {
        quote! { &mut self.record.fields }
    } else {
        quote! { &self.record.fields }
    };
    quote! {
        <#fields_name<A, #layout> as ::puroro_rt::HasField<{ #companion::#field_const }>>::#method(#receiver)
    }
}

fn field_slot_ty(field: &FieldEmit, companion: &Ident) -> TokenStream {
    match field {
        FieldEmit::Singular(field) => singular_slot_ty(field, companion),
        FieldEmit::Repeated(field) => repeated_slot_ty(field, companion),
        FieldEmit::Map(field) => map_slot_ty(field, companion),
        FieldEmit::Oneof(field) => oneof_slot_ty(field, companion),
    }
}

fn singular_slot_ty(field: &ScalarEmit, companion: &Ident) -> TokenStream {
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
        ::puroro_rt::SingularField<#marker, #presence_ty, { #companion::#field_const }, A #(, #layout_ty)* #(, #default_ty)*>
    }
}

fn repeated_slot_ty(field: &RepeatedEmit, companion: &Ident) -> TokenStream {
    let marker = &field.marker;
    let encoding_ty = &field.encoding_ty;
    let field_const = &field.field_const;
    let span_layout = field.layout.as_ref().and_then(|assoc| {
        matches!(assoc.kind, LayoutAssocKind::Repeated { .. }).then_some(&assoc.assoc)
    });
    let span_layout = span_layout.iter();
    quote! {
        ::puroro_rt::RepeatedField<
            #marker,
            #encoding_ty,
            { #companion::#field_const },
            A
            #(, <L as #companion::Layout<A>>::#span_layout)*
        >
    }
}

fn map_slot_ty(field: &MapEmit, companion: &Ident) -> TokenStream {
    let key_marker = &field.key_marker;
    let value_marker = &field.value_marker;
    let field_const = &field.field_const;
    let map_assoc = field
        .layouts
        .iter()
        .find(|assoc| matches!(assoc.kind, LayoutAssocKind::Map { .. }))
        .map(|assoc| &assoc.assoc)
        .expect("map field has a Map layout assoc");
    quote! {
        ::puroro_rt::MapField<
            #key_marker,
            #value_marker,
            { #companion::#field_const },
            A,
            <L as #companion::Layout<A>>::#map_assoc,
        >
    }
}

fn oneof_slot_ty(field: &OneofEmit, companion: &Ident) -> TokenStream {
    let storage_name = &field.storage_name;
    quote! { ::puroro_rt::OneofSlot<#companion::#storage_name<A, L>> }
}

fn collect_layout_assocs(fields: &[FieldEmit]) -> Vec<LayoutAssoc> {
    let mut out = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Singular(field) => {
                if let Some(assoc) = &field.layout {
                    out.push(assoc.clone());
                }
            }
            FieldEmit::Repeated(field) => {
                if let Some(assoc) = &field.layout {
                    out.push(assoc.clone());
                }
            }
            FieldEmit::Map(field) => out.extend(field.layouts.iter().cloned()),
            FieldEmit::Oneof(field) => {
                for variant in &field.variants {
                    if let Some(assoc) = &variant.layout {
                        out.push(assoc.clone());
                    }
                }
            }
        }
    }
    out
}

fn render_oneof_modules(
    fields: &[FieldEmit],
    bits_ty: &TokenStream,
    lazy_bits_ty: &TokenStream,
    unknown_fields: UnknownFieldsPolicy,
    emit_lazy: bool,
) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for field in fields {
        if let FieldEmit::Oneof(o) = field {
            out.extend(oneof::render_module_and_exports(
                o,
                bits_ty,
                lazy_bits_ty,
                unknown_fields,
                emit_lazy,
            )?);
        }
    }
    Ok(out)
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

struct FieldAccessors {
    getters: Vec<TokenStream>,
    mutators: Vec<TokenStream>,
}

fn render_accessors(fields: &[FieldEmit], companion: &Ident) -> FieldAccessors {
    let mut getters = Vec::new();
    let mut mutators = Vec::new();
    for field in fields {
        match field {
            FieldEmit::Repeated(field) => {
                let (g, m) = render_repeated_accessors(field, companion);
                getters.push(g);
                mutators.push(m);
            }
            FieldEmit::Singular(field) => {
                let (g, m) = render_singular_accessors(field, companion);
                getters.push(g);
                mutators.push(m);
            }
            FieldEmit::Map(field) => {
                let (g, m) = render_map_accessors(field, companion);
                getters.push(g);
                mutators.push(m);
            }
            FieldEmit::Oneof(o) => {
                let (g, m) = oneof::render_accessors(o, companion);
                getters.push(g);
                mutators.push(m);
            }
        }
    }
    FieldAccessors { getters, mutators }
}

fn render_map_accessors(field: &MapEmit, companion: &Ident) -> (TokenStream, TokenStream) {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    let key_view = &field.key_view;
    let value_view = &field.value_view;
    // Pin MutTarget so `*entry_mut(…) = …` / `push_str` resolve through `impl Trait`.
    let mut_target = field.mut_target.as_ref().unwrap_or(value_view);
    let get = eager_field(companion, &field.field_const);
    let get_mut = eager_field_mut(companion, &field.field_const);
    let clear = eager_field_mut(companion, &field.field_const);
    let doc = &field.doc;
    let getter = quote! {
        #doc
        pub fn #name(&self) -> impl ::puroro::MapRef<#key_view, #value_view> + '_ {
            #get
        }
    };
    let mutator = quote! {
        pub fn #name_mut(
            &mut self,
        ) -> impl ::puroro::MapMut<#key_view, #value_view, MutTarget = #mut_target> + '_ {
            #get_mut
        }

        pub fn #clear_name(&mut self) {
            #clear.clear();
        }
    };
    (getter, mutator)
}

fn render_repeated_accessors(
    field: &RepeatedEmit,
    companion: &Ident,
) -> (TokenStream, TokenStream) {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    let elem = &field.slice_elem_ty;
    let get = eager_field(companion, &field.field_const);
    let get_mut = eager_field_mut(companion, &field.field_const);
    let clear = eager_field_mut(companion, &field.field_const);
    let doc = &field.doc;
    let getter = quote! {
        #doc
        pub fn #name(&self) -> &[#elem] {
            #get.as_slice()
        }
    };
    let mutator_body = match field.style {
        RepeatedAccessorStyle::String => quote! {
            pub fn #name_mut(&mut self) -> impl ::puroro::RepeatedStringMut<A> + '_ {
                #get_mut.container_mut()
            }
        },
        RepeatedAccessorStyle::Bytes => quote! {
            pub fn #name_mut(&mut self) -> impl ::puroro::RepeatedBytesMut<A> + '_ {
                #get_mut.container_mut()
            }
        },
        RepeatedAccessorStyle::Slice => quote! {
            pub fn #name_mut<'s>(
                &'s mut self,
            ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<#elem, A>> + 's {
                #get_mut.values_mut()
            }
        },
    };
    let mutator = quote! {
        #mutator_body

        pub fn #clear_name(&mut self) {
            #clear.clear();
        }
    };
    (getter, mutator)
}

fn render_singular_accessors(field: &ScalarEmit, companion: &Ident) -> (TokenStream, TokenStream) {
    let name = &field.name;
    let name_mut = Ident::new(&format!("{}_mut", field.name_str), Span::call_site());
    let clear_name = Ident::new(&format!("clear_{}", field.name_str), Span::call_site());
    let mut_target = &field.mut_target;
    let implicit_ty = &field.implicit_ty;
    let optional_ty = &field.optional_ty;
    let get = eager_field(companion, &field.field_const);
    let get_mut = eager_field_mut(companion, &field.field_const);
    let clear = eager_field_mut(companion, &field.field_const);
    let doc = &field.doc;

    if matches!(field.getter_shape.eager(), EagerGetter::Message) {
        let getter = quote! {
            #doc
            pub fn #name(&self) -> ::core::option::Option<&#mut_target> {
                #get.get()
            }
        };
        let set_method = if let Some(eager) = &field.set_eager {
            let set_name = Ident::new(&format!("set_{}", field.name_str), Span::call_site());
            quote! {
                pub fn #set_name(&mut self, src: #eager) {
                    *self.#name_mut() = src.into_lazy();
                }
            }
        } else {
            // Follow-parent (and pinned-eager) singular messages move-replace
            // the stored child. A lazy pin takes the branch above.
            let set_name = Ident::new(&format!("set_{}", field.name_str), Span::call_site());
            quote! {
                pub fn #set_name(&mut self, src: #mut_target) {
                    *self.#name_mut() = src;
                }
            }
        };
        let mutator = quote! {
            pub fn #name_mut(&mut self) -> &mut #mut_target {
                #get_mut.get_mut()
            }

            #set_method

            pub fn #clear_name(&mut self) {
                #clear.clear();
            }
        };
        return (getter, mutator);
    }

    let getter = if matches!(field.getter_shape.eager(), EagerGetter::Optional) {
        quote! {
            #doc
            pub fn #name<'a>(&'a self) -> ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>
            where
                A: 'a,
            {
                #get.optional()
            }
        }
    } else {
        quote! {
            #doc
            pub fn #name(&self) -> #implicit_ty {
                #get.value()
            }
        }
    };

    // RPIT `StringMut` so public signatures do not name `puroro-rt`.
    let mut_ret = field.mut_style.return_ty(&parse_quote! { 's }, mut_target);
    let mutator = quote! {
        pub fn #name_mut<'s>(&'s mut self) -> #mut_ret {
            #get_mut.value_mut()
        }

        pub fn #clear_name(&mut self) {
            #clear.clear();
        }
    };
    (getter, mutator)
}

fn render_merge_arms(fields: &[FieldEmit], companion: &Ident) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    for field in fields {
        if let FieldEmit::Oneof(o) = field {
            arms.extend(oneof::render_merge_arms(o, companion));
            continue;
        }
        let field_const = field
            .field_const()
            .expect("non-oneof field has a FIELD_* const");
        let slot = field_mut_expr(companion, field_const);
        arms.push(quote! {
            #companion::#field_const => {
                #slot
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                ::core::result::Result::Ok(true)
            }
        });
    }
    arms
}

fn render_validate(fields: &[FieldEmit], companion: &Ident) -> Option<TokenStream> {
    let required: Vec<_> = fields
        .iter()
        .filter_map(|f| match f {
            FieldEmit::Singular(f) if f.required => {
                let slot = field_expr(companion, &f.field_const);
                Some(quote! {
                    #slot.validate_required(common)?;
                })
            }
            _ => None,
        })
        .collect();
    if required.is_empty() {
        None
    } else {
        Some(quote! {
            #(#required)*
            ::core::result::Result::Ok(())
        })
    }
}

fn record_validate_method(body: Option<&TokenStream>, layout: &TokenStream) -> TokenStream {
    let Some(body) = body else {
        return quote! {};
    };
    quote! {
        const CHECKS_REQUIRED: bool = true;

        fn validate(
            &self,
            common: &::puroro_rt::MessageCommon<Self::Bits, A, Self::Unknown, #layout>,
        ) -> ::core::result::Result<(), ::puroro::DecodeError> {
            #body
        }
    }
}
