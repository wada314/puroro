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
    DataTypeBase, FieldBase, FieldOrOneofExt, Oneof, OneofField, PackageOrMessageExt,
    PURORO_INTERNAL, PURORO_LIB,
};
use crate::syn::{
    parse2, Arm, Attribute, Expr, FieldValue, Ident, ImplItemMethod, Item, ItemImpl, Stmt, Type,
};
use crate::{FatalErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::Rc;

#[derive(Debug, Default)]
struct Cache {
    union_ident: OnceCell<Rc<Ident>>,
    case_ident: OnceCell<Rc<Ident>>,
    allocated_bitfield: OnceCell<OneofBitfieldAllocation>,
}

#[derive(Debug, Clone, Copy)]
struct OneofBitfieldAllocation {
    oneof_bits_range: (usize, usize),
    tail: usize,
}

impl Oneof {
    pub(crate) fn bitfield_index_for_oneof(&self) -> Result<(usize, usize)> {
        let alloc = if let Some(alloc) = self.cache().get::<Cache>()?.allocated_bitfield.get() {
            alloc
        } else {
            // Force allocate the bitfields
            let _ = self.message()?.bitfield_size()?;
            let Some(alloc) = self.cache().get::<Cache>()?.allocated_bitfield.get() else {
            Err(FatalErrorKind::InternalError { detail: "Oneof bitfield is not allocated".to_string() })?
        };
            alloc
        };
        Ok(alloc.oneof_bits_range)
    }

    pub(crate) fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>> {
        Ok(self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .get()
            .map(|a| a.tail))
    }

    pub(crate) fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize> {
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

    pub(crate) fn gen_oneof_union_type(
        &self,
        generics: impl Iterator<Item = Rc<Type>>,
    ) -> Result<Rc<Type>> {
        let message_module = self.message()?.gen_rust_module_path()?;
        let union_ident = self.gen_union_ident()?;
        Ok(Rc::new(parse2(quote! {
            #message_module :: #union_ident :: < #(#generics),* >
        })?))
    }

    pub(crate) fn gen_oneof_case_type(
        &self,
        generics: impl Iterator<Item = Rc<Type>>,
    ) -> Result<Rc<Type>> {
        let message_module = self.message()?.gen_rust_module_path()?;
        let case_ident = self.gen_case_ident()?;
        Ok(Rc::new(parse2(quote! {
            #message_module :: _case :: #case_ident :: < #(#generics),* >
        })?))
    }

    pub(crate) fn gen_oneof_union_items(&self) -> Result<Vec<Item>> {
        let union_ident = self.gen_union_ident()?;
        let union_fields = self.try_map_fields(|f| f.gen_oneof_union_field())?;
        let union_methods = self
            .try_map_fields(|f| Ok(f.gen_oneof_union_methods()?.into_iter()))?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let generic_params = self.try_map_fields(|f| f.gen_oneof_union_generic_param_ident())?;
        let generic_param_bounds =
            self.try_map_fields(|f| f.gen_oneof_union_generic_param_where_bounds())?;

        let oneof_union_impl = self.gen_oneof_union_impl()?;

        // Union includes none case, where the case enum does not.
        Ok(vec![
            parse2(quote! {
                pub union #union_ident < #(#generic_params),* > {
                    _none: (),
                    #(#union_fields),*,
                }
            })?,
            parse2(quote! {
                impl< #(#generic_params),* > #union_ident< #(#generic_params),* >
                where
                    Self: #PURORO_INTERNAL::OneofUnion,
                    #(#generic_param_bounds,)*
                {
                    #(#union_methods)*
                }
            })?,
            parse2(quote! {
                #oneof_union_impl
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

    pub(crate) fn gen_oneof_case_items(&self) -> Result<Vec<Item>> {
        let case_ident = self.gen_case_ident()?;
        let case_names = self.try_map_fields(|f| f.gen_oneof_case_value_ident())?;
        let generic_params = self.try_map_fields(|f| f.gen_oneof_union_generic_param_ident())?;

        let oneof_case_impl = self.gen_oneof_case_impl()?;
        let docs = self.gen_oneof_case_doc_attrs()?;

        // Union includes none case, where the case enum does not.
        Ok(vec![
            parse2(quote! {
                #[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
                #(#docs)*
                pub enum #case_ident<
                    #(#generic_params = (),)*
                > {
                    #(#case_names(#generic_params),)*
                }
            })?,
            parse2(quote! {
                #oneof_case_impl
            })?,
        ])
    }

    pub(crate) fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>> {
        let generic_params = self
            .fields()?
            .map(|f| f.gen_oneof_union_field_type())
            .collect::<Result<Vec<_>>>()?;
        self.gen_oneof_union_type(generic_params.iter().cloned())
    }

    pub(crate) fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_fields_struct_field_ident()?;
        let view_type = self.message()?.gen_view_struct_type()?;

        Ok(vec![parse2(quote! {
            pub fn #clear_ident(&mut self) {
                let view_ref: &mut #view_type = &mut self.0;
                use #PURORO_INTERNAL::{SharedItems as _, OneofUnion as _};
                view_ref.fields.#field_ident.clear(view_ref.shared.bitfield_mut())
            }
        })?])
    }

    pub(crate) fn gen_view_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let field_ident = self.gen_fields_struct_field_ident()?;

        let getter_case_generic_params =
            self.try_map_fields(|f| f.gen_maybe_borrowed_type(None))?;
        let getter_type = self.gen_oneof_case_type(getter_case_generic_params.iter().cloned())?;

        Ok(vec![parse2(quote! {
            pub fn #getter_ident(&self) -> ::std::option::Option<#getter_type> {
                use #PURORO_INTERNAL::{SharedItems as _, OneofUnion as _};
                self.fields.#field_ident.case_ref(self.shared.bitfield())
            }
        })?])
    }

    pub(crate) fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue> {
        let ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(quote! {
            #ident: #PURORO_INTERNAL::OneofUnion::clone(&self.fields.#ident, self.shared.bitfield())
        })?)
    }

    pub(crate) fn gen_message_struct_impl_message_deser_arms(
        &self,
        field_data_expr: &Expr,
    ) -> Result<Vec<Arm>> {
        let field_ident = self.gen_fields_struct_field_ident()?;
        let field_numbers = self.try_map_fields(|f| f.number())?;

        let case_type = self.gen_oneof_case_type(iter::empty())?;
        let case_names = self.try_map_fields(|f| f.gen_oneof_case_value_ident())?;

        let view_type = self.message()?.gen_view_struct_type()?;

        iter::zip(field_numbers.into_iter(), case_names.into_iter())
            .map(|(field_number, case_name)| {
                Ok(parse2(quote! {
                    #field_number => {
                        let view_ref: &mut #view_type = &mut self.0;
                        view_ref.fields.#field_ident.deser_from_field_data(
                            view_ref.shared.bitfield_mut(),
                            #field_data_expr,
                            #case_type::#case_name(()),
                        )?
                    }
                })?)
            })
            .collect::<Result<Vec<_>>>()
    }

    pub(crate) fn gen_view_struct_impl_message_view_ser_stmt(
        &self,
        out_expr: &Expr,
    ) -> Result<Stmt> {
        let field_ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(quote! {
            self.fields.#field_ident.ser_to_write(
                self.shared.bitfield(),
                #out_expr
            )?;
        })?)
    }

    pub(crate) fn gen_view_struct_impl_debug_method_call(&self, receiver: &mut Expr) -> Result<()> {
        for field in self.fields()? {
            field.gen_message_struct_impl_debug_method_call(receiver)?;
        }
        Ok(())
    }

    pub(crate) fn gen_view_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        Ok(parse2(quote! {
            self.#getter_ident() == #rhs_expr.#getter_ident()
        })?)
    }

    pub(crate) fn gen_view_struct_impl_message_view_internal_new_boxed_field_value(
        &self,
        bitvec_mut_expr: &Expr,
    ) -> Result<FieldValue> {
        let ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(quote! {
            #ident: #PURORO_INTERNAL::OneofUnion::new(#bitvec_mut_expr)
        })?)
    }

    fn try_map_fields<F, R>(&self, f: F) -> Result<Vec<R>>
    where
        F: FnMut(&Rc<OneofField>) -> Result<R>,
    {
        self.fields()?.map(f).collect::<Result<Vec<_>>>()
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

    fn gen_case_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .case_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}Case",
                    self.name()?.to_camel_case()
                )))
            })
            .cloned()
    }

    fn gen_oneof_union_impl(&self) -> Result<ItemImpl> {
        let union_ident = self.gen_union_ident()?;
        let union_field_idents = self.try_map_fields(|f| f.gen_oneof_union_field_ident())?;
        let getter_mut_idents = self.try_map_fields(|f| f.gen_oneof_union_getter_mut_ident())?;
        let field_numbers = self.try_map_fields(|f| f.number())?;
        let case_names = self.try_map_fields(|f| f.gen_oneof_case_value_ident())?;
        let generic_params = self.try_map_fields(|f| f.gen_oneof_union_generic_param_ident())?;
        let generic_param_bounds =
            self.try_map_fields(|f| f.gen_oneof_union_generic_param_where_bounds())?;

        let case_ref_generic_params = generic_params
            .iter()
            .map(|param| {
                Ok(Rc::new(parse2(quote! {
                    <#param as #PURORO_INTERNAL::OneofFieldType>::GetterType::<'a>
                })?))
            })
            .collect::<Result<Vec<_>>>()?;
        let case_type = self.gen_oneof_case_type(iter::empty())?;
        let case_ref_type = self.gen_oneof_case_type(case_ref_generic_params.into_iter())?;

        let bitfield_begin = self.bitfield_index_for_oneof()?.0;
        let bitfield_end = self.bitfield_index_for_oneof()?.1;

        Ok(parse2(quote! {
            impl< #(#generic_params),* > #PURORO_INTERNAL::OneofUnion
            for #union_ident< #(#generic_params),* >
            where #( #generic_param_bounds, )*
            {
                fn new<B: #PURORO_INTERNAL::BitSlice>(bits: &mut B) -> Self {
                    bits.set_range(#bitfield_begin..#bitfield_end, 0);
                    Self { _none: (), }
                }

                type Case = #case_type;
                type CaseRef<'a> = #case_ref_type
                where Self: 'a;

                fn case_ref<B: #PURORO_INTERNAL::BitSlice>(&self, bits: &B)
                    -> ::std::option::Option<Self::CaseRef<'_>>
                {
                    use #PURORO_INTERNAL::OneofCase;
                    use ::std::mem::ManuallyDrop;
                    use ::std::ops::Deref as _;
                    let case_opt = OneofCase::from_bitslice(bits);
                    case_opt.map(|case| {
                        match case {
                            #(Self::Case::#case_names(_) => Self::CaseRef::#case_names(
                                ManuallyDrop::deref(unsafe { &self.#union_field_idents }).get_field()
                            ),)*
                        }
                    })
                }

                fn clear<B: #PURORO_INTERNAL::BitSlice>(&mut self, bits: &mut B) {
                    use #PURORO_INTERNAL::OneofCase;
                    use ::std::mem::ManuallyDrop;
                    #[allow(unused)] use ::std::option::Option::Some;
                    match OneofCase::from_bitslice(bits) {
                        #(Some(Self::Case::#case_names(())) => {
                            unsafe { ManuallyDrop::take(&mut self.#union_field_idents) };
                        })*
                        _ => ()
                    }
                    bits.set_range(#bitfield_begin..#bitfield_end, 0);
                }

                fn clone<B: #PURORO_INTERNAL::BitSlice>(&self, bits: &B) -> Self {
                    use #PURORO_INTERNAL::OneofCase;
                    #[allow(unused)] use ::std::option::Option::Some;
                    #[allow(unused)] use ::std::clone::Clone;
                    match OneofCase::from_bitslice(bits) {
                        #(Some(Self::Case::#case_names(())) => Self {
                            #union_field_idents: Clone::clone(unsafe { &self.#union_field_idents }),
                        },)*
                        _ => Self { _none: (), },
                    }
                }

                fn deser_from_field_data<'a, I, B>(
                    &mut self,
                    bitvec: &mut B,
                    field_data: #PURORO_INTERNAL::ser::FieldData<#PURORO_INTERNAL::ScopedIter<'a, I>>,
                    case: Self::Case,
                ) -> #PURORO_LIB::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    B: #PURORO_INTERNAL::BitSlice,
                {
                    #[allow(unused)] use ::std::result::Result::Ok;
                    match case {
                        #(Self::Case::#case_names(_) => {
                            let _ = <Self>::#getter_mut_idents(self, bitvec);
                            unsafe { &mut self.#union_field_idents }.deser_from_field_data(field_data)?;
                        })*
                    }
                    Ok(())
                }

                fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> #PURORO_LIB::Result<()>
                where
                    W: ::std::io::Write,
                    B: #PURORO_INTERNAL::BitSlice
                {
                    #[allow(unused)] use ::std::option::Option::Some;
                    #[allow(unused)] use ::std::result::Result::Ok;
                    use #PURORO_INTERNAL::OneofCase;
                    match OneofCase::from_bitslice(bitvec) {
                        #(Some(Self::Case::#case_names(_)) => {
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

    fn gen_oneof_case_impl(&self) -> Result<ItemImpl> {
        let case_ident = self.gen_case_ident()?;
        let union_fields = self.try_map_fields(|f| f.gen_oneof_union_field())?;
        let item_indices = (1..=(union_fields.len() as u32)).collect::<Vec<_>>();
        let case_names = self.try_map_fields(|f| f.gen_oneof_case_value_ident())?;
        let bitfield_begin = self.bitfield_index_for_oneof()?.0;
        let bitfield_end = self.bitfield_index_for_oneof()?.1;

        Ok(parse2(quote! {
            impl #PURORO_INTERNAL::OneofCase for #case_ident {
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

    fn gen_oneof_case_doc_attrs(&self) -> Result<Vec<Attribute>> {
        let input_file = self.message()?.input_file()?;
        let Some(sci) = input_file.source_code_info(self.location_path()?)? else {
            return Ok(Vec::new());
        };
        Ok(sci.gen_doc_attributes()?)
    }
}
