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
use super::super::{
    Field, FieldRule, FieldType, LengthDelimitedType, MessageExt, Oneof, OneofField,
};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait OneofFieldExt {
    fn gen_union_item_decl(&self) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    union_item_ident: OnceCell<Rc<Ident>>,
}

impl<T: ?Sized + OneofField> OneofFieldExt for T {
    fn gen_union_item_decl(&self) -> Result<TokenStream> {
        let ident = gen_union_item_ident(self)?;
        let inner_type_name = {
            use FieldType::*;
            use LengthDelimitedType::*;
            let r#type = self.r#type()?;
            match r#type {
                Variant(_) | Bits32(_) | Bits64(_) => {
                    let primitive = r#type.rust_type()?;
                    let tag = r#type.tag_type()?;
                    quote! {
                        NumericalField::<#primitive, #tag>
                    }
                }
                LengthDelimited(Bytes) => quote! { BytesField },
                LengthDelimited(String) => quote! { StringField },
                LengthDelimited(Message(m)) => {
                    let message_path = m.try_upgrade()?.gen_rust_struct_path()?;
                    quote! {
                        HeapMessageField::< #message_path >
                    }
                }
            }
        };
        let field_type = quote! {
            ::std::mem::ManuallyDrop::<
                self::_puroro::internal::oneof_field_type:: #inner_type_name
            >
        };

        Ok(quote! {
            #ident: #field_type,
        })
    }
}

fn gen_union_item_ident(this: &(impl ?Sized + OneofField)) -> Result<Rc<Ident>> {
    this.cache()
        .get::<Cache>()?
        .union_item_ident
        .get_or_try_init(|| Ok(Rc::new(format_ident!("{}", this.name()?))))
        .cloned()
}
