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
    fn gen_union_item_ident(&self) -> Result<Rc<Ident>>;
    fn gen_union_item_decl(&self) -> Result<TokenStream>;
    fn gen_union_methods(&self) -> Result<TokenStream>;
    fn gen_generic_type_param_ident(&self) -> Result<Ident>;
    fn gen_case_enum_value_ident(&self) -> Result<Ident>;
    fn gen_maybe_borrowed_type(&self, lt: Option<Ident>) -> Result<Rc<TokenStream>>;
}

#[derive(Debug, Default)]
struct Cache {
    union_item_ident: OnceCell<Rc<Ident>>,
}

impl<T: ?Sized + OneofField> OneofFieldExt for T {
    fn gen_union_item_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_item_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }
    fn gen_union_item_decl(&self) -> Result<TokenStream> {
        let ident = self.gen_union_item_ident()?;
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

    fn gen_union_methods(&self) -> Result<TokenStream> {
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let borrowed_type = self.r#type()?.rust_maybe_borrowed_type(None)?;
        let getter_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => Rc::new(quote! {
                ::std::option::Option::< #borrowed_type >
            }),
            _ => Rc::clone(&borrowed_type),
        };
        let case_ident = format_ident!("{}Case", self.oneof()?.name()?.to_camel_case());
        let union_item_ident = self.gen_union_item_ident()?;
        let enum_item_ident = self.gen_case_enum_value_ident()?;
        Ok(quote! {
            pub(crate) fn #getter_ident<B: self::_puroro::bitvec::BitSlice>(&self, bits: &B) -> #getter_type {
                #[allow(unused)] use ::std::option::Option::{None, Some};
                #[allow(unused)] use ::std::default::Default;
                use self::_puroro::internal::oneof_field_type::OneofFieldTypeOpt;
                use ::std::ops::Deref as _;
                use self::_puroro::internal::oneof_type::OneofCase as _;

                let case_opt = self::#case_ident::from_bitslice(bits);
                let item_opt = matches!(case_opt, Some(self::#case_ident::#enum_item_ident(()))).then(|| {
                    unsafe {
                        self.#union_item_ident.deref()
                    }
                });
                OneofFieldTypeOpt::get_field(item_opt, Default::default)
            }
        })
    }

    fn gen_generic_type_param_ident(&self) -> Result<Ident> {
        Ok(format_ident!(
            "{}",
            self.name()?.to_camel_case().escape_rust_keywords()
        ))
    }
    fn gen_case_enum_value_ident(&self) -> Result<Ident> {
        Ok(format_ident!(
            "{}",
            self.name()?.to_camel_case().escape_rust_keywords()
        ))
    }
    fn gen_maybe_borrowed_type(&self, lt: Option<Ident>) -> Result<Rc<TokenStream>> {
        Ok(self.r#type()?.rust_maybe_borrowed_type(lt)?.clone())
    }
}
