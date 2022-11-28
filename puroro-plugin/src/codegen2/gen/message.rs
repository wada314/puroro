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
use super::super::{FieldExt, Message, PackageOrMessageExt};
use super::OneofExt;
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait MessageExt: Debug {
    fn bitfield_size(&self) -> Result<usize>;
    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>>;
    fn gen_struct(&self) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    rust_struct_path: OnceCell<Rc<TokenStream>>,
    bitfield_size: OnceCell<usize>,
}
impl<T: ?Sized + Message> MessageExt for T {
    fn bitfield_size(&self) -> Result<usize> {
        <Self as Message>::cache(self)
            .get::<Cache>()?
            .bitfield_size
            .get_or_try_init(|| {
                let mut tail = 0;

                // bits for each field
                for field in self.fields()? {
                    if let Some(next_tail) = field.maybe_allocated_bitfield_tail()? {
                        tail = next_tail;
                    } else {
                        tail = field.assign_and_get_bitfield_tail(tail)?;
                    }
                }

                // bits for oneofs
                for oneof in self.oneofs()? {
                    if let Some(next_tail) = oneof.maybe_allocated_bitfield_tail()? {
                        tail = next_tail;
                    } else {
                        tail = oneof.assign_and_get_bitfield_tail(tail)?;
                    }
                }

                Ok(tail)
            })
            .cloned()
    }

    fn gen_rust_struct_path(&self) -> Result<Rc<TokenStream>> {
        <Self as Message>::cache(self)
            .get::<Cache>()?
            .rust_struct_path
            .get_or_try_init(|| {
                let parent = <Self as Message>::parent(self)?.gen_rust_module_path()?;
                let ident =
                    format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }

    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = gen_struct_ident(self)?;
        let fields = self
            .fields()?
            .map(|f| f.gen_struct_field_decl())
            .collect::<Result<Vec<_>>>()?;
        let methods = self
            .fields()?
            .map(|f| f.gen_struct_field_methods())
            .collect::<Result<Vec<_>>>()?;
        let bitfield_size_in_u32_array = (self.bitfield_size()? + 31) / 32;
        let message_impl = gen_struct_message_impl(self)?;
        let clone_impl = gen_struct_clone_impl(self)?;
        Ok(quote! {
            #[derive(::std::default::Default)]
            pub struct #ident {
                #(#fields)*
                _bitfield: self::_puroro::bitvec::BitArray<#bitfield_size_in_u32_array>,
            }

            impl #ident {
                #(#methods)*
            }

            #message_impl
            #clone_impl
        })
    }
}

fn gen_struct_ident(this: &(impl ?Sized + Message)) -> Result<TokenStream> {
    let ident = format_ident!(
        "{}",
        this.name()?
            .to_camel_case()
            .escape_rust_keywords()
            .to_string()
    );
    Ok(quote! { #ident })
}

fn gen_struct_message_impl(this: &(impl ?Sized + Message)) -> Result<TokenStream> {
    let ident = gen_struct_ident(this)?;
    let field_data_ident = quote! { field_data };
    let out_ident = quote! { out };
    let deser_arms = this
        .fields()?
        .map(|f| f.gen_struct_field_deser_arm(&field_data_ident))
        .collect::<Result<Vec<_>>>()?;
    let ser_fields = this
        .fields()?
        .map(|f| f.gen_struct_field_ser(&out_ident))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl self::_puroro::Message for #ident {
            fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> self::_puroro::Result<Self> {
                let mut msg = <Self as ::std::default::Default>::default();
                msg.merge_from_bytes_iter(iter)?;
                ::std::result::Result::Ok(msg)
            }

            fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item =::std::io::Result<u8>>>(&mut self, mut iter: I) -> self::_puroro::Result<()> {
                use self::_puroro::internal::ser::FieldData;
                while let Some((number, #field_data_ident)) = FieldData::from_bytes_iter(iter.by_ref())? {
                    match number {
                        #(#deser_arms)*
                        _ => todo!(), // Unknown field number
                    }
                }
                ::std::result::Result::Ok(())
            }

            fn to_bytes<W: ::std::io::Write>(&self, #[allow(unused)] #out_ident: &mut W) -> self::_puroro::Result<()> {
                #(#ser_fields)*
                ::std::result::Result::Ok(())
            }
        }
    })
}

fn gen_struct_clone_impl(this: &(impl ?Sized + Message)) -> Result<TokenStream> {
    let ident = gen_struct_ident(this)?;
    let field_clones = this
        .fields()?
        .map(|f| f.gen_struct_field_clone_arm())
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! {
        impl ::std::clone::Clone for #ident {
            fn clone(&self) -> Self {
                Self {
                    #(#field_clones)*
                    _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                }
            }
        }
    })
}
