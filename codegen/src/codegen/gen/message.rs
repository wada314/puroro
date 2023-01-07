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
    FieldExt, FieldOrOneofExt, Message, OneofExt, OneofFieldExt, PackageOrMessageExt,
    PURORO_INTERNAL, PURORO_LIB,
};
use crate::syn::{parse2, Expr, Ident, Item, ItemImpl, Type};
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait MessageExt: Debug {
    fn bitfield_size(&self) -> Result<usize>;

    fn gen_message_struct_type(&self) -> Result<Rc<Type>>;
    fn gen_fields_struct_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>>;

    fn gen_message_struct_items(&self) -> Result<Vec<Item>>;
    fn gen_fields_struct_items(&self) -> Result<Vec<Item>>;
}

#[derive(Debug, Default)]
struct Cache {
    message_struct_type: OnceCell<Rc<Type>>,
    bitfield_size: OnceCell<usize>,
}
impl<T: ?Sized + Message> MessageExt for T {
    fn bitfield_size(&self) -> Result<usize> {
        self.cache()
            .get::<Cache>()?
            .bitfield_size
            .get_or_try_init(|| {
                let mut tail = 0;

                // bits for each (non-oneof) fields
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

    fn gen_message_struct_type(&self) -> Result<Rc<Type>> {
        self.cache()
            .get::<Cache>()?
            .message_struct_type
            .get_or_try_init(|| {
                let parent = <Self as Message>::parent(self)?.gen_rust_module_path()?;
                let ident = gen_message_struct_ident(self)?;
                Ok(Rc::new(parse2(quote! { #parent :: #ident })?))
            })
            .cloned()
    }

    fn gen_fields_struct_type(&self, generics: impl Iterator<Item = Rc<Type>>) -> Result<Rc<Type>> {
        let parent = <Self as Message>::parent(self)?.gen_rust_module_path()?;
        let ident = gen_fields_struct_ident(self)?;
        let generics = generics.collect::<Vec<_>>();
        Ok(Rc::new(parse2(
            quote! { #parent :: _fields :: #ident < #(#generics,)* > },
        )?))
    }

    fn gen_message_struct_items(&self) -> Result<Vec<Item>> {
        let ident = gen_message_struct_ident(self)?;
        let fields_types = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_fields_struct_field_type())
            .collect::<Result<Vec<_>>>()?;
        let fields_struct_type = self.gen_fields_struct_type(fields_types.into_iter())?;

        let fields_or_oneofs_methods = self
            .fields_or_oneofs()?
            .map(|fo| (Ok(fo.gen_message_struct_methods()?.into_iter())))
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;
        let oneofs = self.oneofs()?.collect::<Vec<_>>();
        let oneof_field_methods = oneofs
            .iter()
            .map(|o| o.fields())
            .flatten_ok()
            .map(|f| Ok(f?.gen_message_struct_methods()?.into_iter()))
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;
        let bitfield_size_in_u32_array = (self.bitfield_size()? + 31) / 32;
        let message_impl = gen_message_struct_message_impl(self)?;
        let clone_impl = gen_message_struct_impl_clone(self)?;
        let drop_impl = gen_message_struct_impl_drop(self)?;
        let debug_impl = gen_message_struct_impl_debug(self)?;
        let partial_eq_impl = gen_message_struct_impl_partial_eq(self)?;

        let item_struct = parse2(quote! {
            #[derive(::std::default::Default)]
            pub struct #ident {
                fields: #fields_struct_type,
                bitfield: #PURORO_INTERNAL::BitArray<#bitfield_size_in_u32_array>,
            }
        })?;
        let impl_struct = parse2(quote! {
            impl #ident {
                #(#fields_or_oneofs_methods)*
                #(#oneof_field_methods)*
            }
        })?;
        Ok(vec![
            item_struct,
            impl_struct,
            message_impl.into(),
            clone_impl.into(),
            drop_impl.into(),
            debug_impl.into(),
            partial_eq_impl.into(),
        ])
    }

    fn gen_fields_struct_items(&self) -> Result<Vec<Item>> {
        let ident = gen_fields_struct_ident(self)?;
        let generics = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_fields_struct_generic_param_ident())
            .collect::<Result<Vec<_>>>()?;
        let fields = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_fields_struct_field())
            .collect::<Result<Vec<_>>>()?;

        Ok(vec![parse2(quote! {
            #[derive(::std::default::Default)]
            pub struct #ident <#(#generics),*> {
                #(#fields,)*
            }
        })?])
    }
}

fn gen_message_struct_ident(this: &(impl ?Sized + Message)) -> Result<Ident> {
    Ok(format_ident!(
        "{}",
        this.name()?
            .to_camel_case()
            .escape_rust_keywords()
            .to_string()
    ))
}

fn gen_fields_struct_ident(this: &(impl ?Sized + Message)) -> Result<Ident> {
    Ok(format_ident!(
        "{}Fields",
        this.name()?.to_camel_case().to_string()
    ))
}

fn gen_message_struct_message_impl(this: &(impl ?Sized + Message)) -> Result<ItemImpl> {
    let ident = gen_message_struct_ident(this)?;
    let field_data_ident: Ident = parse2(quote! { field_data })?;
    let field_data_expr = parse2(quote! { field_data })?;
    let out_ident = quote! { out };
    let out_expr = parse2(quote! { out })?;
    let deser_arms = this
        .fields_or_oneofs()?
        .map(|fo| {
            Ok(fo
                .gen_message_struct_impl_message_deser_arms(&field_data_expr)?
                .into_iter())
        })
        .flatten_ok()
        .collect::<Result<Vec<_>>>()?;
    let ser_fields = this
        .fields()?
        .map(|f| f.gen_message_struct_impl_message_ser_stmt(&out_expr))
        .collect::<Result<Vec<_>>>()?;
    let ser_oneof_stmts = this
        .oneofs()?
        .map(|o| o.gen_message_struct_impl_message_ser_stmt(&out_expr))
        .collect::<Result<Vec<_>>>()?;

    Ok(parse2(quote! {
        impl #PURORO_LIB::Message for #ident {
            fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> #PURORO_LIB::Result<Self> {
                let mut msg = <Self as ::std::default::Default>::default();
                msg.merge_from_bytes_iter(iter)?;
                ::std::result::Result::Ok(msg)
            }

            fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item =::std::io::Result<u8>>>(&mut self, mut iter: I) -> #PURORO_LIB::Result<()> {
                use #PURORO_INTERNAL::ser::FieldData;
                #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;
                while let Some((number, #field_data_ident)) = FieldData::from_bytes_iter(iter.by_ref())? {
                    match number {
                        #(#deser_arms)*
                        _ => todo!(), // Unknown field number
                    }
                }
                ::std::result::Result::Ok(())
            }

            fn to_bytes<W: ::std::io::Write>(&self, #[allow(unused)] #out_ident: &mut W) -> #PURORO_LIB::Result<()> {
                #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;
                #(#ser_fields)*
                #(#ser_oneof_stmts)*
                ::std::result::Result::Ok(())
            }
        }
    })?)
}

fn gen_message_struct_impl_clone(this: &(impl ?Sized + Message)) -> Result<ItemImpl> {
    let ident = gen_message_struct_ident(this)?;
    let fields_ident = gen_fields_struct_ident(this)?;
    let field_clones = this
        .fields()?
        .map(|f| f.gen_message_struct_impl_clone_field_value())
        .collect::<Result<Vec<_>>>()?;
    let oneof_clones = this
        .oneofs()?
        .map(|o| o.gen_message_struct_impl_clone_field_value())
        .collect::<Result<Vec<_>>>()?;
    Ok(parse2(quote! {
        impl ::std::clone::Clone for #ident {
            fn clone(&self) -> Self {
                Self {
                    fields: self::_fields::#fields_ident {
                        #(#field_clones,)*
                        #(#oneof_clones,)*
                    },
                    bitfield: ::std::clone::Clone::clone(&self.bitfield),
                }
            }
        }
    })?)
}

fn gen_message_struct_impl_drop(this: &(impl ?Sized + Message)) -> Result<ItemImpl> {
    let ident = gen_message_struct_ident(this)?;
    let oneof_idents = this
        .oneofs()?
        .map(|o| o.gen_fields_struct_field_ident())
        .collect::<Result<Vec<_>>>()?;
    // We need to explicitly clear the oneof unions.
    Ok(parse2(quote! {
        impl ::std::ops::Drop for #ident {
            fn drop(&mut self) {
                #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;

                #(self.fields.#oneof_idents.clear(&mut self.bitfield);)*
            }
        }
    })?)
}

fn gen_message_struct_impl_debug(this: &(impl ?Sized + Message)) -> Result<ItemImpl> {
    let ident = gen_message_struct_ident(this)?;
    let mut fmt_body: Expr = parse2(quote! { fmt.debug_struct(stringify!(#ident)) })?;

    for field in this.fields()? {
        fmt_body = field
            .gen_message_struct_impl_debug_method_call(fmt_body)?
            .into();
    }
    for oneof in this.oneofs()? {
        for field in oneof.fields()? {
            fmt_body = field
                .gen_message_struct_impl_debug_method_call(fmt_body)?
                .into();
        }
    }

    Ok(parse2(quote! {
        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                #fmt_body.finish()
            }
        }
    })?)
}

fn gen_message_struct_impl_partial_eq(this: &(impl ?Sized + Message)) -> Result<ItemImpl> {
    let ident = gen_message_struct_ident(this)?;
    let rhs_expr = parse2(quote! { rhs })?;
    let field_cmps = this
        .fields()?
        .map(|f| f.gen_message_struct_impl_partial_eq_cmp(&rhs_expr))
        .collect::<Result<Vec<_>>>()?;
    let oneof_cmps = this
        .oneofs()?
        .map(|o| o.gen_message_struct_impl_partial_eq_cmp(&rhs_expr))
        .collect::<Result<Vec<_>>>()?;
    Ok(parse2(quote! {
        impl ::std::cmp::PartialEq for #ident {
            fn eq(&self, rhs: &Self) -> bool {
                #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;

                true
                    #( && #field_cmps)*
                    #( && #oneof_cmps)*
            }
        }
    })?)
}
