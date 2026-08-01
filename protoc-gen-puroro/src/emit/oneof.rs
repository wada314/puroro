//! Emit a oneof submodule (`shape` / `Case` / `Storage` + trait impls).

use crate::error::Result;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::quote;

/// Owned facts for one generated oneof group.
pub(super) struct OneofEmit {
    pub name: Ident,
    pub name_str: String,
    pub mod_name: Ident,
    pub shape_name: Ident,
    pub case_name: Ident,
    pub storage_name: Ident,
    pub variants: Vec<OneofVariantEmit>,
}

/// One variant inside a oneof group.
pub(super) struct OneofVariantEmit {
    pub name: Ident,
    pub name_str: String,
    pub variant_name: Ident,
    pub type_param: Ident,
    pub field_alias: Ident,
    pub field_const: Ident,
    pub number: u32,
    pub marker: TokenStream,
    pub layout_ty: Option<TokenStream>,
    pub value_bit: Option<(Ident, usize)>,
    pub is_message: bool,
    pub is_bool: bool,
    pub mut_target: TokenStream,
    pub optional_ty: TokenStream,
}

/// Render `mod <oneof> { … }` plus `use` / `pub use` for the parent message module.
pub(super) fn render_module_and_exports(oneof: &OneofEmit) -> Result<TokenStream> {
    let mod_name = &oneof.mod_name;
    let shape_name = &oneof.shape_name;
    let case_name = &oneof.case_name;
    let storage_name = &oneof.storage_name;

    let module_body = render_module_body(oneof)?;
    Ok(quote! {
        mod #mod_name {
            #module_body
        }

        use #mod_name::#storage_name;
        pub use #mod_name::{#shape_name, #case_name};
    })
}

fn render_module_body(oneof: &OneofEmit) -> Result<TokenStream> {
    let shape_name = &oneof.shape_name;
    let case_name = &oneof.case_name;
    let storage_name = &oneof.storage_name;

    let type_params: Vec<&Ident> = oneof.variants.iter().map(|v| &v.type_param).collect();

    let shape_variants: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            let tp = &v.type_param;
            quote! { #vn(#tp), }
        })
        .collect();

    let case_variants: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! { #vn, }
        })
        .collect();

    let field_alias_defs: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let alias = &v.field_alias;
            let marker = &v.marker;
            let field_const = &v.field_const;
            match &v.layout_ty {
                None => quote! {
                    type #alias<A> = ::puroro_rt::SingularField<
                        #marker,
                        ::puroro_rt::Oneof,
                        { super::#field_const },
                        A,
                    >;
                },
                Some(layout_ty) => quote! {
                    type #alias<A> = ::puroro_rt::SingularField<
                        #marker,
                        ::puroro_rt::Oneof,
                        { super::#field_const },
                        A,
                        #layout_ty,
                    >;
                },
            }
        })
        .collect();

    let storage_args: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let alias = &v.field_alias;
            quote! { #alias<A> }
        })
        .collect();

    let case_match_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! {
                Self::#vn(_) => #case_name::#vn,
            }
        })
        .collect();

    let to_ref_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! {
                Self::#vn(f) => #shape_name::#vn(f.value(common)),
            }
        })
        .collect();

    let to_mut_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            if v.is_bool {
                let bit = v
                    .value_bit
                    .as_ref()
                    .map(|(ident, _)| ident)
                    .expect("oneof bool has value bit");
                quote! {
                    Self::#vn(_) => #shape_name::#vn(common.bit_mut(super::#bit)),
                }
            } else {
                quote! {
                    Self::#vn(f) => #shape_name::#vn(f.value_mut(common)),
                }
            }
        })
        .collect();

    let clone_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! {
                Self::#vn(f) => Self::#vn(::puroro_rt::FieldCloneIn::clone_field(
                    f, common, alloc,
                )),
            }
        })
        .collect();

    let ref_tys: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            if v.is_message {
                let mut_target = &v.mut_target;
                quote! { &'a #mut_target }
            } else {
                let optional_ty = &v.optional_ty;
                quote! { #optional_ty }
            }
        })
        .collect();

    let mut_tys: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let marker = &v.marker;
            quote! { <#marker as ::puroro_rt::SingularType>::Mut<'a, A> }
        })
        .collect();

    let oneof_variant_impls: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            let alias = &v.field_alias;
            let field_const = &v.field_const;
            quote! {
                impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone>
                    ::puroro_rt::OneofVariant<{ super::#field_const }> for #storage_name<A>
                {
                    type Value = #alias<A>;

                    fn variant_ref(&self) -> ::core::option::Option<&Self::Value> {
                        match self {
                            Self::#vn(f) => ::core::option::Option::Some(f),
                            _ => ::core::option::Option::None,
                        }
                    }

                    fn variant_mut(&mut self) -> ::core::option::Option<&mut Self::Value> {
                        match self {
                            Self::#vn(f) => ::core::option::Option::Some(f),
                            _ => ::core::option::Option::None,
                        }
                    }

                    fn from_variant(value: Self::Value) -> Self {
                        Self::#vn(value)
                    }
                }
            }
        })
        .collect();

    let encode_len_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! { Self::#vn(f) => ::puroro_rt::FieldEncode::encoded_len(f, common), }
        })
        .collect();

    let encode_raw_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! { Self::#vn(f) => ::puroro_rt::FieldEncode::encode_raw(f, common, buf), }
        })
        .collect();

    let dealloc_arms: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            let vn = &v.variant_name;
            quote! { Self::#vn(mut f) => f.deallocate(common), }
        })
        .collect();

    Ok(quote! {
        // Same `_root` chain as forest modules so `self::_root::…` type paths work.
        mod _root {
            pub(super) use super::super::_root::*;
        }

        use ::allocator_api2::alloc::Allocator;
        use ::bytes::BufMut;
        use ::puroro_rt::{
            FieldDeallocate, MessageCommon, MessageCommonAlloc, MessageCommonBits, OneofDeallocate,
            OneofEncodable, OneofGroup,
        };

        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq)]
        pub enum #shape_name<#(#type_params),*> {
            #(#shape_variants)*
        }

        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
        )]
        pub enum #case_name {
            #(#case_variants)*
        }

        #(#field_alias_defs)*

        pub(crate) type #storage_name<A> = #shape_name<#(#storage_args),*>;

        impl<A: Allocator + ::core::clone::Clone> OneofGroup for #storage_name<A> {
            type Case = #case_name;
            type Ref<'a>
                = #shape_name<#(#ref_tys),*>
            where
                A: 'a;
            type Mut<'a>
                = #shape_name<#(#mut_tys),*>
            where
                A: 'a;
            type Presence = super::__Presence;
            type Alloc = A;

            fn case(storage: &Self) -> Self::Case {
                match storage {
                    #(#case_match_arms)*
                }
            }

            fn to_ref<'a>(
                storage: &'a Self,
                common: &'a MessageCommon<Self::Presence, Self::Alloc>,
            ) -> Self::Ref<'a> {
                match storage {
                    #(#to_ref_arms)*
                }
            }

            fn to_mut<'a>(
                storage: &'a mut Self,
                common: &'a mut MessageCommon<Self::Presence, Self::Alloc>,
            ) -> Self::Mut<'a> {
                match storage {
                    #(#to_mut_arms)*
                }
            }

            fn clone_storage_in(
                storage: &Self,
                common: &MessageCommon<Self::Presence, Self::Alloc>,
                alloc: Self::Alloc,
            ) -> Self {
                match storage {
                    #(#clone_arms)*
                }
            }
        }

        #(#oneof_variant_impls)*

        impl<A: Allocator + ::core::clone::Clone> OneofEncodable<A> for #storage_name<A> {
            fn encoded_len<P>(&self, common: &MessageCommon<P, A>) -> usize
            where
                MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>,
            {
                match self {
                    #(#encode_len_arms)*
                }
            }

            fn encode_raw<P, B: BufMut>(&self, common: &MessageCommon<P, A>, buf: &mut B)
            where
                MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>,
            {
                match self {
                    #(#encode_raw_arms)*
                }
            }
        }

        impl<A: Allocator + ::core::clone::Clone, P> OneofDeallocate<MessageCommon<P, A>>
            for #storage_name<A>
        {
            unsafe fn deallocate(self, common: &MessageCommon<P, A>) {
                match self {
                    #(#dealloc_arms)*
                }
            }
        }
    })
}

/// Group getter / clear + per-variant accessors for the parent message.
pub(super) fn render_accessors(oneof: &OneofEmit) -> TokenStream {
    let name = &oneof.name;
    let name_str = &oneof.name_str;
    let name_mut = Ident::new(&format!("{name_str}_mut"), name.span());
    let clear_name = Ident::new(&format!("clear_{name_str}"), name.span());
    let case_name = &oneof.case_name;
    let shape_name = &oneof.shape_name;
    let storage_name = &oneof.storage_name;

    let ref_tys: Vec<_> = oneof
        .variants
        .iter()
        .map(|v| {
            if v.is_message {
                let mut_target = &v.mut_target;
                quote! { &'a #mut_target }
            } else {
                let optional_ty = &v.optional_ty;
                quote! { #optional_ty }
            }
        })
        .collect();

    let mut variant_accessors = Vec::new();
    for v in &oneof.variants {
        let vname = &v.name;
        let vname_str = &v.name_str;
        let vname_mut = Ident::new(&format!("{vname_str}_mut"), vname.span());
        let field_const = &v.field_const;
        let mut_target = &v.mut_target;
        let optional_ty = &v.optional_ty;
        let group = name;

        if v.is_message {
            variant_accessors.push(quote! {
                pub fn #vname(&self) -> ::core::option::Option<&#mut_target> {
                    self.#group
                        .bind(&self._common)
                        .variant_of::<#field_const>()
                        .get()
                }

                pub fn #vname_mut(&mut self) -> &mut #mut_target {
                    self.#group
                        .bind_mut(&mut self._common)
                        .variant_mut::<#field_const>()
                        .bind_mut(&mut self._common)
                        .value_mut()
                }
            });
        } else {
            variant_accessors.push(quote! {
                pub fn #vname<'a>(
                    &'a self,
                ) -> ::puroro::Optional<#optional_ty, impl ::puroro::HasDefault<#optional_ty>>
                where
                    A: 'a,
                {
                    self.#group
                        .bind(&self._common)
                        .variant_of::<#field_const>()
                        .optional()
                }

                pub fn #vname_mut<'s>(
                    &'s mut self,
                ) -> impl ::core::ops::DerefMut<Target = #mut_target> + 's {
                    self.#group
                        .bind_mut(&mut self._common)
                        .variant_mut::<#field_const>()
                        .bind_mut(&mut self._common)
                        .value_mut()
                }
            });
        }
    }

    quote! {
        pub fn #name<'a>(
            &'a self,
        ) -> impl ::puroro::OneofView<
            Case = #case_name,
            Ref = #shape_name<#(#ref_tys),*>,
        > + 'a {
            ::puroro_rt::OneofView::<#storage_name<A>>::new(&self.#name, &self._common)
        }

        pub fn #name_mut<'a>(&'a mut self) -> impl ::puroro::OneofViewMut<Case = #case_name> + 'a {
            ::puroro_rt::OneofViewMut::<#storage_name<A>>::new(
                &mut self.#name,
                &mut self._common,
            )
        }

        pub fn #clear_name(&mut self) {
            ::puroro::OneofViewMut::clear(self.#name_mut());
        }

        #(#variant_accessors)*
    }
}

/// Merge arms for every variant field number.
pub(super) fn render_merge_arms(oneof: &OneofEmit) -> Vec<TokenStream> {
    let group = &oneof.name;
    oneof
        .variants
        .iter()
        .map(|v| {
            let field_const = &v.field_const;
            quote! {
                #field_const => {
                    self.#group
                        .bind_mut(&mut self._common)
                        .variant_mut::<#field_const>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
            }
        })
        .collect()
}

/// `FIELD_*` constants for each variant.
pub(super) fn render_field_consts(oneof: &OneofEmit) -> Vec<TokenStream> {
    oneof
        .variants
        .iter()
        .map(|v| {
            let ident = &v.field_const;
            let number = v.number;
            quote! {
                pub const #ident: u32 = #number;
            }
        })
        .collect()
}

/// Bool value-bit constants for oneof variants.
pub(super) fn render_bit_consts(oneof: &OneofEmit) -> Vec<TokenStream> {
    oneof
        .variants
        .iter()
        .filter_map(|v| {
            let (ident, bit) = v.value_bit.as_ref()?;
            Some(quote! {
                pub const #ident: usize = #bit;
            })
        })
        .collect()
}
