// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use self file except in compliance with the License.
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
    DataTypeBase, FieldOrOneofExt, Message, PackageOrMessage, PackageOrMessageExt, PURORO_INTERNAL,
    PURORO_LIB,
};
use crate::syn::{parse2, Attribute, Expr, Ident, Item, ItemImpl, Type};
use crate::Result;
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

#[derive(Debug, Default)]
struct Cache {
    message_struct_type: OnceCell<Rc<Type>>,
    bitfield_size: OnceCell<usize>,
}
impl Message {
    pub(crate) fn bitfield_size(&self) -> Result<usize> {
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

    pub(crate) fn gen_message_struct_type(&self) -> Result<Rc<Type>> {
        self.cache()
            .get::<Cache>()?
            .message_struct_type
            .get_or_try_init(|| {
                let parent = self.parent()?.gen_rust_module_path()?;
                let ident = self.gen_message_struct_ident()?;
                Ok(Rc::new(parse2(quote! { #parent :: #ident })?))
            })
            .cloned()
    }

    pub(crate) fn gen_fields_struct_type(
        &self,
        generics: impl Iterator<Item = Rc<Type>>,
    ) -> Result<Rc<Type>> {
        let parent = self.parent()?.gen_rust_module_path()?;
        let ident = self.gen_fields_struct_ident()?;
        let generics = generics.collect::<Vec<_>>();
        Ok(Rc::new(parse2(
            quote! { #parent :: _fields :: #ident < #(#generics,)* > },
        )?))
    }

    pub(crate) fn gen_message_struct_items(&self) -> Result<Vec<Item>> {
        let ident = self.gen_message_struct_ident()?;
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
        let message_impl = self.gen_message_struct_message_impl()?;
        let message_internal_impl = self.gen_message_struct_message_internal_impl()?;
        let clone_impl = self.gen_message_struct_impl_clone()?;
        let drop_impl = self.gen_message_struct_impl_drop()?;
        let debug_impl = self.gen_message_struct_impl_debug()?;
        let partial_eq_impl = self.gen_message_struct_impl_partial_eq()?;
        let docs = self.gen_message_struct_doc_attrs()?;

        let item_struct = parse2(quote! {
            #[derive(::std::default::Default)]
            #(#docs)*
            pub struct #ident {
                fields: #fields_struct_type,
                shared: #PURORO_INTERNAL::SharedItemsImpl<#bitfield_size_in_u32_array>,
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
            message_internal_impl.into(),
            clone_impl.into(),
            drop_impl.into(),
            debug_impl.into(),
            partial_eq_impl.into(),
        ])
    }

    pub(crate) fn gen_fields_struct_items(&self) -> Result<Vec<Item>> {
        let ident = self.gen_fields_struct_ident()?;
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

    fn gen_message_struct_ident(&self) -> Result<Ident> {
        Ok(format_ident!(
            "{}",
            self.name()?
                .to_camel_case()
                .escape_rust_keywords()
                .to_string()
        ))
    }

    fn gen_fields_struct_ident(&self) -> Result<Ident> {
        Ok(format_ident!(
            "{}Fields",
            self.name()?.to_camel_case().to_string()
        ))
    }

    fn gen_message_struct_message_impl(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let out_ident = quote! { out };
        let out_expr = parse2(quote! { out })?;
        let ser_stmts = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_message_struct_impl_message_ser_stmt(&out_expr))
            .collect::<Result<Vec<_>>>()?;

        Ok(parse2(quote! {
            impl #PURORO_LIB::Message for #ident {
                fn from_bytes_iter<I: ::std::iter::Iterator<Item=::std::io::Result<u8>>>(iter: I) -> #PURORO_LIB::Result<Self> {
                    let mut msg = <Self as ::std::default::Default>::default();
                    msg.merge_from_bytes_iter(iter)?;
                    ::std::result::Result::Ok(msg)
                }

                fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item =::std::io::Result<u8>>>(
                    &mut self,
                    iter: I
                ) -> #PURORO_LIB::Result<()> {
                    let mut pos_iter = #PURORO_INTERNAL::PosIter::new(iter);
                    let mut scoped_iter = #PURORO_INTERNAL::ScopedIter::from_mut_pos_iter(&mut pos_iter);
                    <Self as #PURORO_INTERNAL::MessageInternal>::merge_from_scoped_bytes_iter(self, &mut scoped_iter)?;
                    scoped_iter.drop_and_check_scope_completed()?;
                    Ok(())
                }

                fn to_bytes<W: ::std::io::Write>(&self, #[allow(unused)] #out_ident: &mut W) -> #PURORO_LIB::Result<()> {
                    #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;
                    use #PURORO_INTERNAL::{SharedItems as _, UnknownFields as _};
                    #(#ser_stmts)*
                    self.shared.unknown_fields().ser_to_write(#out_ident)?;
                    ::std::result::Result::Ok(())
                }
            }
        })?)
    }

    fn gen_message_struct_message_internal_impl(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let field_data_ident: Ident = parse2(quote! { field_data })?;
        let field_data_expr = parse2(quote! { field_data })?;
        let deser_arms = self
            .fields_or_oneofs()?
            .map(|fo| {
                Ok(fo
                    .gen_message_struct_impl_message_deser_arms(&field_data_expr)?
                    .into_iter())
            })
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;

        Ok(parse2(quote! {
            impl #PURORO_INTERNAL::MessageInternal for #ident {
                fn merge_from_scoped_bytes_iter<'a, I: ::std::iter::Iterator<Item =::std::io::Result<u8>>>(
                    &mut self,
                    iter: &mut #PURORO_INTERNAL::ScopedIter<'a, I>,
                ) -> #PURORO_LIB::Result<()> {
                    use #PURORO_INTERNAL::ser::FieldData;
                    #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;
                    use #PURORO_INTERNAL::{SharedItems as _, UnknownFields as _};
                    #[allow(unused)] use ::std::result::Result;
                    #[allow(unused)] use ::std::result::Result::{Ok, Err};
                    #[allow(unused)] use ::std::vec::Vec;
                    use #PURORO_LIB::PuroroError;
                    while let Some((number, #field_data_ident)) = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
                        let result: #PURORO_LIB::Result<()> = (|| {
                            match number {
                                #(#deser_arms)*
                                _ => {
                                    let field_data = #field_data_ident.map(|iter| {
                                        iter.collect::<Result<Vec<_>, _>>()
                                    }).transpose()?;
                                    Err(PuroroError::UnknownFieldNumber(field_data))?
                                }
                            }
                            Ok(())
                        })();
                        match result {
                            Ok(_) => (),
                            Err(PuroroError::UnknownFieldNumber(field_data)) => {
                                // Recoverable error. Store the field into unknown_fields.
                                self.shared.unknown_fields_mut().push(number, field_data)?;
                            }
                            Err(e) => Err(e)?,
                        }
                    }
                    Ok(())
                }
            }
        })?)
    }

    fn gen_message_struct_impl_clone(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let fields_ident = self.gen_fields_struct_ident()?;
        let field_values = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_message_struct_impl_clone_field_value())
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            impl ::std::clone::Clone for #ident {
                fn clone(&self) -> Self {
                    #[allow(unused)] use #PURORO_INTERNAL::SharedItems as _;
                    Self {
                        fields: self::_fields::#fields_ident {
                            #(#field_values,)*
                        },
                        shared: ::std::clone::Clone::clone(&self.shared),
                    }
                }
            }
        })?)
    }

    fn gen_message_struct_impl_drop(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let oneof_idents = self
            .oneofs()?
            .map(|o| o.gen_fields_struct_field_ident())
            .collect::<Result<Vec<_>>>()?;
        // We need to explicitly clear the oneof unions.
        Ok(parse2(quote! {
            impl ::std::ops::Drop for #ident {
                fn drop(&mut self) {
                    #[allow(unused)] use #PURORO_INTERNAL::{OneofUnion as _, SharedItems as _};

                    #(self.fields.#oneof_idents.clear(self.shared.bitfield_mut());)*
                }
            }
        })?)
    }

    fn gen_message_struct_impl_debug(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let mut debug_fields: Expr = parse2(quote! { debug_struct })?;
        for field_or_oneof in self.fields_or_oneofs()? {
            field_or_oneof.gen_message_struct_impl_debug_method_call(&mut debug_fields)?;
        }

        Ok(parse2(quote! {
            impl ::std::fmt::Debug for #ident {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                    use #PURORO_INTERNAL::{SharedItems as _, UnknownFields as _};
                    let mut debug_struct = fmt.debug_struct(stringify!(#ident));
                    #debug_fields;
                    self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
                    debug_struct.finish()
                }
            }
        })?)
    }

    fn gen_message_struct_impl_partial_eq(&self) -> Result<ItemImpl> {
        let ident = self.gen_message_struct_ident()?;
        let rhs_expr = parse2(quote! { rhs })?;
        let cmp_exprs = self
            .fields_or_oneofs()?
            .map(|fo| fo.gen_message_struct_impl_partial_eq_cmp(&rhs_expr))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            impl ::std::cmp::PartialEq for #ident {
                fn eq(&self, rhs: &Self) -> bool {
                    #[allow(unused)] use #PURORO_INTERNAL::OneofUnion as _;
                    use #PURORO_INTERNAL::SharedItems as _;

                    true
                        #( && #cmp_exprs)*
                        && self.shared.unknown_fields() == rhs.shared.unknown_fields()
                }
            }
        })?)
    }

    fn gen_message_struct_doc_attrs(&self) -> Result<Vec<Attribute>> {
        let input_file = self.input_file()?;
        let Some(sci) = input_file.source_code_info(self.location_path()?)? else {
            return Ok(Vec::new());
        };
        Ok(sci.gen_doc_attributes()?)
    }
}
