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
use super::{
    MessageExt, Oneof, OneofField, OneofFieldExt, PackageOrMessageExt, PURORO_INTERNAL, PURORO_LIB,
};
use crate::syn::{
    parse2, Arm, Expr, Field, FieldValue, Ident, ImplItemMethod, Item, ItemImpl, NamedField, Stmt,
    Type,
};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::Rc;

pub trait OneofExt {
    // Message's bitfield allocation
    fn bitfield_index_for_oneof(&self) -> Result<(usize, usize)>;
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>>;
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize>;

    fn gen_union_ident(&self) -> Result<Rc<Ident>>;
    fn gen_struct_field_ident(&self) -> Result<Rc<Ident>>;
    fn gen_union_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>>;
    fn gen_case_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>>;

    fn gen_oneof_items(&self) -> Result<Vec<Item>>;
    fn gen_struct_field(&self) -> Result<Field>;
    fn gen_struct_methods(&self) -> Result<Vec<ImplItemMethod>>;
    fn gen_struct_impl_clone_field_value(&self) -> Result<FieldValue>;
    fn gen_struct_impl_message_deser_arms(&self, field_data_expr: &Expr) -> Result<Vec<Arm>>;
    fn gen_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt>;
    fn gen_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr>;
}

#[derive(Debug, Default)]
struct Cache {
    union_ident: OnceCell<Rc<Ident>>,
    case_ident: OnceCell<Rc<Ident>>,
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

    fn gen_union_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>> {
        let message_module = self.message()?.gen_rust_module_path()?;
        let union_ident = self.gen_union_ident()?;
        Ok(Rc::new(parse2(quote! {
            #message_module :: #union_ident :: < #(#generics),* >
        })?))
    }

    fn gen_case_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>> {
        let message_module = self.message()?.gen_rust_module_path()?;
        let case_ident = gen_case_ident(self)?;
        Ok(Rc::new(parse2(quote! {
            #message_module :: #case_ident :: < #(#generics),* >
        })?))
    }

    fn gen_oneof_items(&self) -> Result<Vec<Item>> {
        let union_ident = self.gen_union_ident()?;
        let case_ident = gen_case_ident(self)?;
        let union_fields = try_map_fields(self, |f| f.gen_union_field())?;
        let union_methods = try_map_fields(self, |f| Ok(f.gen_union_methods()?.into_iter()))?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let case_names = try_map_fields(self, |f| f.gen_case_enum_value_ident())?;
        let generic_params = try_map_fields(self, |f| f.gen_union_generic_param_ident())?;
        let generic_param_bounds =
            try_map_fields(self, |f| f.gen_union_generic_param_where_bounds())?;

        let oneof_union_impl = gen_oneof_union_impl(self)?;
        let oneof_case_impl = gen_oneof_case_impl(self)?;

        // Union includes none case, where the case enum does not.
        Ok(vec![
            parse2(quote! {
                pub union #union_ident < #(#generic_params),* > {
                    _none: (),
                    #(#union_fields),*,
                }
            })?,
            parse2(quote! {
                #[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
                pub enum #case_ident<
                    #(#generic_params = (),)*
                > {
                    #(#case_names(#generic_params),)*
                }
            })?,
            parse2(quote! {
                impl< #(#generic_params),* > #union_ident< #(#generic_params),* >
                where
                    #(#generic_param_bounds,)*
                {
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
                impl< #(#generic_params),* > ::std::default::Default for #union_ident< #(#generic_params),* >
                {
                    fn default() -> Self {
                        Self { _none: () }
                    }
                }
            })?,
        ])
    }

    fn gen_struct_field(&self) -> Result<Field> {
        let field_ident = self.gen_struct_field_ident()?;
        let generic_params = self
            .fields()?
            .map(|f| f.gen_union_field_type())
            .collect::<Result<Vec<_>>>()?;
        let field_type = self.gen_union_type(generic_params.iter().cloned())?;
        Ok(parse2::<NamedField>(quote! {
            #field_ident: #field_type
        })?
        .into())
    }

    fn gen_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_struct_field_ident()?;

        let getter_case_generic_params = try_map_fields(self, |f| f.gen_maybe_borrowed_type(None))?;
        let getter_type = self.gen_case_type(getter_case_generic_params.iter().cloned())?;

        Ok(vec![
            parse2(quote! {
                pub fn #getter_ident(&self) -> ::std::option::Option<#getter_type> {
                    use #PURORO_INTERNAL::oneof_type::OneofUnion as _;
                    self.#field_ident.case_ref(&self._bitfield)
                }
            })?,
            parse2(quote! {
                pub fn #clear_ident(&mut self) {
                    use #PURORO_INTERNAL::oneof_type::OneofUnion as _;
                    self.#field_ident.clear(&mut self._bitfield)
                }
            })?,
        ])
    }

    fn gen_struct_impl_clone_field_value(&self) -> Result<FieldValue> {
        let ident = self.gen_struct_field_ident()?;
        Ok(parse2(quote! {
            #ident: #PURORO_INTERNAL::oneof_type::OneofUnion::clone(&self.#ident, &self._bitfield)
        })?)
    }

    fn gen_struct_impl_message_deser_arms(&self, field_data_expr: &Expr) -> Result<Vec<Arm>> {
        let field_ident = self.gen_struct_field_ident()?;
        let field_numbers = try_map_fields(self, |f| f.number())?;

        let message_module = self.message()?.gen_rust_module_path()?;
        let case_ident = format_ident!("{}Case", self.name()?.to_camel_case());
        let case_names = try_map_fields(self, |f| f.gen_case_enum_value_ident())?;

        iter::zip(field_numbers.into_iter(), case_names.into_iter())
            .map(|(field_number, case_name)| {
                Ok(parse2(quote! {
                    #field_number => self.#field_ident.deser_from_iter(
                        &mut self._bitfield,
                        #field_data_expr,
                        #message_module::#case_ident::#case_name(()),
                    )?,
                })?)
            })
            .collect::<Result<Vec<_>>>()
    }

    fn gen_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt> {
        let field_ident = self.gen_struct_field_ident()?;
        Ok(parse2(quote! {
            self.#field_ident.ser_to_write(
                &self._bitfield,
                #out_expr
            )?;
        })?)
    }

    fn gen_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        Ok(parse2(quote! {
            self.#getter_ident() == #rhs_expr.#getter_ident()
        })?)
    }
}

fn try_map_fields<F, R>(this: &(impl ?Sized + Oneof), f: F) -> Result<Vec<R>>
where
    F: FnMut(Rc<dyn OneofField>) -> Result<R>,
{
    this.fields()?.map(f).collect::<Result<Vec<_>>>()
}

fn gen_case_ident(this: &(impl ?Sized + Oneof)) -> Result<Rc<Ident>> {
    this.cache()
        .get::<Cache>()?
        .case_ident
        .get_or_try_init(|| {
            Ok(Rc::new(format_ident!(
                "{}Case",
                this.name()?.to_camel_case()
            )))
        })
        .cloned()
}

fn gen_oneof_union_impl(this: &(impl ?Sized + Oneof)) -> Result<ItemImpl> {
    let union_ident = this.gen_union_ident()?;
    let case_ident = gen_case_ident(this)?;
    let union_field_idents = try_map_fields(this, |f| f.gen_union_field_ident())?;
    let getter_mut_idents = try_map_fields(this, |f| f.gen_union_getter_mut_ident())?;
    let field_numbers = try_map_fields(this, |f| f.number())?;
    let case_names = try_map_fields(this, |f| f.gen_case_enum_value_ident())?;
    let generic_params = try_map_fields(this, |f| f.gen_union_generic_param_ident())?;
    let generic_param_bounds = try_map_fields(this, |f| f.gen_union_generic_param_where_bounds())?;
    let bitfield_begin = this.bitfield_index_for_oneof()?.0;
    let bitfield_end = this.bitfield_index_for_oneof()?.1;

    Ok(parse2(quote! {
        impl< #(#generic_params),* > #PURORO_INTERNAL::oneof_type::OneofUnion
        for #union_ident< #(#generic_params),* >
        where #( #generic_param_bounds, )*
        {
            type Case = self::#case_ident;
            type CaseRef<'a> = self::#case_ident::<#(
                #generic_params::GetterType<'a>
            ),*>
            where Self: 'a;

            fn case_ref<B: #PURORO_INTERNAL::bitvec::BitSlice>(&self, bits: &B)
                -> ::std::option::Option<Self::CaseRef<'_>>
            {
                use #PURORO_INTERNAL::oneof_type::OneofCase;
                use ::std::mem::ManuallyDrop;
                use ::std::ops::Deref as _;
                let case_opt = <self::#case_ident as OneofCase>::from_bitslice(bits);
                case_opt.map(|case| {
                    match case {
                        #(self::#case_ident::#case_names(_) => self::#case_ident::#case_names(
                            ManuallyDrop::deref(unsafe { &self.#union_field_idents }).get_field()
                        ),)*
                    }
                })
            }

            fn clear<B: #PURORO_INTERNAL::bitvec::BitSlice>(&mut self, bits: &mut B) {
                use #PURORO_INTERNAL::oneof_type::OneofCase;
                use ::std::mem::ManuallyDrop;
                #[allow(unused)] use ::std::option::Option::Some;
                match <self::#case_ident as OneofCase>::from_bitslice(bits) {
                    #(Some(self::#case_ident::#case_names(())) => {
                        unsafe { ManuallyDrop::take(&mut self.#union_field_idents) };
                    })*
                    _ => ()
                }
                bits.set_range(#bitfield_begin..#bitfield_end, 0);
            }

            fn clone<B: #PURORO_INTERNAL::bitvec::BitSlice>(&self, bits: &B) -> Self {
                use #PURORO_INTERNAL::oneof_type::OneofCase;
                #[allow(unused)] use ::std::option::Option::Some;
                #[allow(unused)] use ::std::clone::Clone;
                match <self::#case_ident as OneofCase>::from_bitslice(bits) {
                    #(Some(self::#case_ident::#case_names(())) => Self {
                        #union_field_idents: Clone::clone(unsafe { &self.#union_field_idents }),
                    },)*
                    _ => Self { _none: (), },
                }
            }

            fn deser_from_iter<I, B>(
                &mut self,
                bitvec: &mut B,
                field_data: #PURORO_INTERNAL::ser::FieldData<I>,
                case: Self::Case,
            ) -> #PURORO_LIB::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                B: #PURORO_INTERNAL::bitvec::BitSlice,
            {
                #[allow(unused)] use ::std::result::Result::Ok;
                match case {
                    #(Self::Case::#case_names(_) => {
                        let _ = <Self>::#getter_mut_idents(self, bitvec);
                        unsafe { &mut self.#union_field_idents }.deser_from_iter(field_data)?;
                    })*
                }
                Ok(())
            }

            fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> #PURORO_LIB::Result<()>
            where
                W: ::std::io::Write,
                B: #PURORO_INTERNAL::bitvec::BitSlice
            {
                #[allow(unused)] use ::std::option::Option::Some;
                #[allow(unused)] use ::std::result::Result::Ok;
                use #PURORO_INTERNAL::oneof_type::OneofCase as _;
                match self::#case_ident::from_bitslice(bitvec) {
                    #(Some(self::#case_ident::#case_names(_)) => {
                        unsafe { &self.#union_field_idents }.ser_to_write(
                            #field_numbers, out,
                        )?;
                    })*
                    _ => (),
                }
                Ok(())
            }
        }
    })?)
}

fn gen_oneof_case_impl(this: &(impl ?Sized + Oneof)) -> Result<ItemImpl> {
    let case_ident = gen_case_ident(this)?;
    let union_fields = try_map_fields(this, |f| f.gen_union_field())?;
    let item_indices = (1..=(union_fields.len() as u32)).collect::<Vec<_>>();
    let case_names = try_map_fields(this, |f| f.gen_case_enum_value_ident())?;
    let bitfield_begin = this.bitfield_index_for_oneof()?.0;
    let bitfield_end = this.bitfield_index_for_oneof()?.1;

    Ok(parse2(quote! {
        impl #PURORO_INTERNAL::oneof_type::OneofCase for #case_ident {
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
    })?)
}
