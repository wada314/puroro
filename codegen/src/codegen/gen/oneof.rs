// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::super::util::*;
use super::super::{MessageExt, Oneof, OneofField};
use super::{OneofFieldExt, PackageOrMessageExt};
use crate::syn::{parse2, Item};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait OneofExt {
    // Message's bitfield allocation
    fn bitfield_index_for_oneof(&self) -> Result<(usize, usize)>;
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>>;
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize>;

    fn gen_union_ident(&self) -> Result<Rc<Ident>>;
    fn gen_struct_field_ident(&self) -> Result<Rc<Ident>>;

    fn gen_union(&self) -> Result<Vec<Item>>;
    fn gen_struct_field_decl(&self) -> Result<TokenStream>;
    fn gen_struct_field_methods(&self) -> Result<TokenStream>;
    fn gen_struct_field_clone_arm(&self) -> Result<TokenStream>;
    fn gen_struct_field_deser_arms(&self, field_data_ident: &TokenStream) -> Result<TokenStream>;
    fn gen_struct_field_ser(&self, out_ident: &TokenStream) -> Result<TokenStream>;
    fn gen_struct_field_partial_eq_cmp(&self, rhs_ident: &TokenStream) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    union_ident: OnceCell<Rc<Ident>>,
    struct_field_ident: OnceCell<Rc<Ident>>,
    allocated_bitfield: OnceCell<OneofBitfieldAllocation>,
}

#[derive(Debug, Clone, Copy)]
struct OneofBitfieldAllocation {
    oneof_bits_range: (usize, usize),
    tail: usize,
}

impl<T: ?Sized + Oneof> OneofExt for T {
    fn bitfield_index_for_oneof(&self) -> Result<(usize, usize)> {
        let alloc = if let Some(alloc) = self.cache().get::<Cache>()?.allocated_bitfield.get() {
            alloc
        } else {
            // Force allocate the bitfields
            let _ = self.message()?.bitfield_size()?;
            let Some(alloc) = self.cache().get::<Cache>()?.allocated_bitfield.get() else {
            Err(ErrorKind::InternalError { detail: "Oneof bitfield is not allocated".to_string() })?
        };
            alloc
        };
        Ok(alloc.oneof_bits_range)
    }

    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>> {
        Ok(self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .get()
            .map(|a| a.tail))
    }

    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize> {
        let case_num = self.fields()?.count() + 1 /* 1 for none case */;
        let required_bits = (usize::leading_zeros(0) - usize::leading_zeros(case_num)) as usize;
        Ok(self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .get_or_init(|| OneofBitfieldAllocation {
                oneof_bits_range: (head, head + required_bits),
                tail: required_bits,
            })
            .tail)
    }

    fn gen_union_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_camel_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }

    fn gen_struct_field_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .struct_field_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }

    fn gen_union(&self) -> Result<Vec<Item>> {
        let union_ident = self.gen_union_ident()?;
        let case_ident = format_ident!("{}Case", self.name()?.to_camel_case());
        let union_items = try_map_fields(self, |f| f.gen_union_item_field())?;
        let item_type_names = try_map_fields(self, |f| f.gen_generic_type_param_ident())?;
        let union_methods = try_map_fields(self, |f| Ok(f.gen_union_methods()?.into_iter()))?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let case_names = try_map_fields(self, |f| f.gen_case_enum_value_ident())?;

        let oneof_union_impl = gen_oneof_union_impl(self)?;
        let oneof_case_impl = gen_oneof_case_impl(self)?;

        // Union includes none case, where the case enum does not.
        Ok(vec![
            parse2(quote! {
                pub union #union_ident {
                    _none: (),
                    #(#union_items),*,
                }
            })?,
            parse2(quote! {
                #[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
                pub enum #case_ident<
                    #(#item_type_names = (),)*
                > {
                    #(#case_names(#item_type_names),)*
                }
            })?,
            parse2(quote! {
                impl #union_ident {
                    #(#union_methods)*
                }
            })?,
            parse2(quote! {
                #oneof_union_impl
            })?,
            parse2(quote! {
                #oneof_case_impl
            })?,
            parse2(quote! {
                impl ::std::default::Default for #union_ident {
                    fn default() -> Self {
                        Self { _none: () }
                    }
                }
            })?,
        ])
    }

    fn gen_struct_field_decl(&self) -> Result<TokenStream> {
        let field_ident = self.gen_struct_field_ident()?;
        let message_module = self.message()?.gen_rust_module_path()?;
        let union_ident = self.gen_union_ident()?;
        Ok(quote! {
            #field_ident: #message_module :: #union_ident,
        })
    }

    fn gen_struct_field_methods(&self) -> Result<TokenStream> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_struct_field_ident()?;
        let message_module = self.message()?.gen_rust_module_path()?;
        let union_ident = self.gen_union_ident()?;

        Ok(quote! {
            pub fn #getter_ident(&self) -> ::std::option::Option<
                <#message_module::#union_ident as self::_puroro::internal::oneof_type::OneofUnion>::CaseRef<'_>
            >
            {
                use self::_puroro::internal::oneof_type::OneofUnion as _;
                self.#field_ident.case_ref(&self._bitfield)
            }

            pub fn #clear_ident(&mut self) {
                use self::_puroro::internal::oneof_type::OneofUnion as _;
                self.#field_ident.clear(&mut self._bitfield)
            }
        })
    }

    fn gen_struct_field_clone_arm(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_field_ident()?;
        let message_module = self.message()?.gen_rust_module_path()?;
        let union_ident = self.gen_union_ident()?;

        Ok(quote! {
            #ident: <#message_module :: #union_ident as self::_puroro::internal::oneof_type::OneofUnion>
                ::clone(&self.#ident, &self._bitfield),
        })
    }

    fn gen_struct_field_deser_arms(&self, field_data_ident: &TokenStream) -> Result<TokenStream> {
        let field_ident = self.gen_struct_field_ident()?;
        let field_numbers = try_map_fields(self, |f| f.number())?;

        let message_module = self.message()?.gen_rust_module_path()?;
        let case_ident = format_ident!("{}Case", self.name()?.to_camel_case());
        let case_names = try_map_fields(self, |f| f.gen_case_enum_value_ident())?;

        Ok(quote! {
            #(#field_numbers => self.#field_ident.deser_from_iter(
                &mut self._bitfield,
                #field_data_ident,
                #message_module::#case_ident::#case_names(()),
            )?,)*
        })
    }

    fn gen_struct_field_ser(&self, out_ident: &TokenStream) -> Result<TokenStream> {
        let field_ident = self.gen_struct_field_ident()?;
        Ok(quote! {
            self.#field_ident.ser_to_write(
                &self._bitfield,
                #out_ident
            )?;
        })
    }

    fn gen_struct_field_partial_eq_cmp(&self, rhs_ident: &TokenStream) -> Result<TokenStream> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        Ok(quote! {
            && self.#getter_ident() == #rhs_ident.#getter_ident()
        })
    }
}

fn try_map_fields<F, R>(this: &(impl ?Sized + Oneof), f: F) -> Result<Vec<R>>
where
    F: FnMut(Rc<dyn OneofField>) -> Result<R>,
{
    this.fields()?.map(f).collect::<Result<Vec<_>>>()
}

fn gen_oneof_union_impl(this: &(impl ?Sized + Oneof)) -> Result<TokenStream> {
    let union_ident = this.gen_union_ident()?;
    let case_ident = format_ident!("{}Case", this.name()?.to_camel_case());
    let union_item_idents = try_map_fields(this, |f| f.gen_union_item_ident())?;
    let getter_mut_idents = try_map_fields(this, |f| f.gen_union_getter_mut_ident())?;
    let field_numbers = try_map_fields(this, |f| f.number())?;
    let case_names = try_map_fields(this, |f| f.gen_case_enum_value_ident())?;
    let borrowed_types_a = try_map_fields(this, |f| {
        f.gen_maybe_borrowed_type(Some(parse2(quote! { 'a })?))
    })?;
    let bitfield_begin = this.bitfield_index_for_oneof()?.0;
    let bitfield_end = this.bitfield_index_for_oneof()?.1;

    Ok(quote! {
        impl self::_puroro::internal::oneof_type::OneofUnion for #union_ident {
            type Case = self::#case_ident;
            type CaseRef<'a> = self::#case_ident::<#(#borrowed_types_a,)*>;

            fn case_ref<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B)
                -> ::std::option::Option<Self::CaseRef<'_>>
            {
                use self::_puroro::internal::oneof_type::OneofCase;
                use ::std::mem::ManuallyDrop;
                use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
                use ::std::ops::Deref as _;
                let case_opt = <self::#case_ident as OneofCase>::from_bitslice(bits);
                case_opt.map(|case| {
                    match case {
                        #(self::#case_ident::#case_names(_) => self::#case_ident::#case_names(
                            ManuallyDrop::deref(unsafe { &self.#union_item_idents }).get_field()
                        ),)*
                    }
                })
            }

            fn clear<B: self::_puroro::bitvec::BitSlice>(&mut self, bits: &mut B) {
                use self::_puroro::internal::oneof_type::OneofCase;
                use ::std::mem::ManuallyDrop;
                #[allow(unused)] use ::std::option::Option::Some;
                match <self::#case_ident as OneofCase>::from_bitslice(bits) {
                    #(Some(self::#case_ident::#case_names(())) => {
                        unsafe { ManuallyDrop::take(&mut self.#union_item_idents) };
                    })*
                    _ => ()
                }
                bits.set_range(#bitfield_begin..#bitfield_end, 0);
            }

            fn clone<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> Self {
                use self::_puroro::internal::oneof_type::OneofCase;
                #[allow(unused)] use ::std::option::Option::Some;
                #[allow(unused)] use ::std::clone::Clone;
                match <self::#case_ident as OneofCase>::from_bitslice(bits) {
                    #(Some(self::#case_ident::#case_names(())) => Self {
                        #union_item_idents: Clone::clone(unsafe { &self.#union_item_idents }),
                    },)*
                    _ => Self { _none: (), },
                }
            }

            fn deser_from_iter<I, B>(
                &mut self,
                bitvec: &mut B,
                field_data: self::_puroro::internal::ser::FieldData<I>,
                case: Self::Case,
            ) -> self::_puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                B: self::_puroro::bitvec::BitSlice,
            {
                use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
                #[allow(unused)] use ::std::result::Result::Ok;
                match case {
                    #(Self::Case::#case_names(_) => {
                        let _ = <Self>::#getter_mut_idents(self, bitvec);
                        unsafe { &mut self.#union_item_idents }.deser_from_iter(field_data)?;
                    })*
                }
                Ok(())
            }

            fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
            where
                W: ::std::io::Write,
                B: self::_puroro::bitvec::BitSlice
            {
                #[allow(unused)] use ::std::option::Option::Some;
                #[allow(unused)] use ::std::result::Result::Ok;
                use self::_puroro::internal::oneof_type::OneofCase as _;
                use self::_puroro::internal::oneof_field_type::OneofFieldType as _;
                match self::#case_ident::from_bitslice(bitvec) {
                    #(Some(self::#case_ident::#case_names(_)) => {
                        unsafe { &self.#union_item_idents }.ser_to_write(
                            #field_numbers, out,
                        )?;
                    })*
                    _ => (),
                }
                Ok(())
            }
        }
    })
}

fn gen_oneof_case_impl(this: &(impl ?Sized + Oneof)) -> Result<TokenStream> {
    let case_ident = format_ident!("{}Case", this.name()?.to_camel_case());
    let union_items = try_map_fields(this, |f| f.gen_union_item_field())?;
    let item_indices = (1..=(union_items.len() as u32)).collect::<Vec<_>>();
    let case_names = try_map_fields(this, |f| f.gen_case_enum_value_ident())?;
    let bitfield_begin = this.bitfield_index_for_oneof()?.0;
    let bitfield_end = this.bitfield_index_for_oneof()?.1;

    Ok(quote! {
        impl self::_puroro::internal::oneof_type::OneofCase for #case_ident {
            const BITFIELD_BEGIN: usize = #bitfield_begin;
            const BITFIELD_END: usize = #bitfield_end;
            fn from_u32(x: u32) -> ::std::option::Option<Self> {
                match x {
                    #(#item_indices => ::std::option::Option::Some(Self::#case_names(())),)*
                    _ => ::std::option::Option::None,
                }
            }
            fn into_u32(self) -> u32 {
                match self {
                    #(Self::#case_names(_) => #item_indices,)*
                }
            }
        }
    })
}
