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
use super::super::{Field, FieldRule, FieldType, LengthDelimitedType, MessageExt, Oneof};
use super::OneofFieldExt;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait OneofExt {
    // Message's bitfield allocation
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>>;
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize>;

    fn gen_union(&self) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    enum_ident: OnceCell<Rc<Ident>>,
    allocated_bitfield: OnceCell<OneofBitfieldAllocation>,
}

#[derive(Debug, Clone, Copy)]
struct OneofBitfieldAllocation {
    oneof_bits_range: (usize, usize),
    tail: usize,
}

impl<T: ?Sized + Oneof> OneofExt for T {
    fn gen_union(&self) -> Result<TokenStream> {
        let ident = gen_union_ident(self)?;
        let ident_case = format_ident!("{}Case", self.name()?.to_camel_case());
        let items = self
            .fields()?
            .map(|f| f.gen_union_item_decl())
            .collect::<Result<Vec<_>>>()?;
        let item_type_names = self
            .fields()?
            .map(|f| f.gen_generic_type_param_ident())
            .collect::<Result<Vec<_>>>()?;
        let case_names = self
            .fields()?
            .map(|f| f.gen_case_enum_value_ident())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            pub(super) union #ident {
                _none: (),
                #(#items)*
            }

            pub enum #ident_case<
                #(#item_type_names = (),)*
            > {
                #(#case_names(#item_type_names),)*
            }
        })
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
}

fn bitfield_index_for_oneof(this: &(impl ?Sized + Field)) -> Result<(usize, usize)> {
    let alloc = if let Some(alloc) = this.cache().get::<Cache>()?.allocated_bitfield.get() {
        alloc
    } else {
        // Force allocate the bitfields
        let _ = this.message()?.bitfield_size()?;
        let Some(alloc) = this.cache().get::<Cache>()?.allocated_bitfield.get() else {
            Err(ErrorKind::InternalError { detail: "Oneof bitfield is not allocated".to_string() })?
        };
        alloc
    };
    Ok(alloc.oneof_bits_range)
}

fn gen_union_ident(this: &(impl ?Sized + Oneof)) -> Result<Rc<Ident>> {
    this.cache()
        .get::<Cache>()?
        .enum_ident
        .get_or_try_init(|| {
            Ok(Rc::new(format_ident!(
                "{}",
                this.name()?.to_camel_case().escape_rust_keywords()
            )))
        })
        .cloned()
}
