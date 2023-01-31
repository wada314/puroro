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
    DataTypeBase, FieldBase, FieldBaseExt, FieldOrOneofExt, FieldType, LengthDelimitedType,
    OneofField, PURORO_INTERNAL,
};
use crate::syn::{
    parse2, Expr, ExprMethodCall, Field, Ident, ImplItemMethod, Lifetime, NamedField, PathSegment,
    Type,
};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::Rc;

#[derive(Debug, Default)]
struct Cache {
    union_field_ident: OnceCell<Rc<Ident>>,
    union_field_type: OnceCell<Rc<Type>>,
    union_getter_ident: OnceCell<Rc<Ident>>,
    union_getter_opt_ident: OnceCell<Rc<Ident>>,
    union_getter_mut_ident: OnceCell<Rc<Ident>>,
    union_generic_param_ident: OnceCell<Rc<Ident>>,
    union_generic_param_where_bounds: OnceCell<Rc<TokenStream>>,
}

impl OneofField {
    pub(crate) fn gen_oneof_union_field_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_field_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_union_getter_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_getter_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_union_getter_opt_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_getter_opt_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}_opt",
                    self.name()?.to_lower_snake_case()
                )))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_union_getter_mut_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_getter_mut_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}_mut",
                    self.name()?.to_lower_snake_case()
                )))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_union_generic_param_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_generic_param_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "T{}",
                    self.name()?.to_camel_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_union_generic_param_where_bounds(&self) -> Result<Rc<TokenStream>> {
        self.cache()
            .get::<Cache>()?
            .union_generic_param_where_bounds
            .get_or_try_init(|| {
                let ident = self.gen_oneof_union_generic_param_ident()?;
                Ok(Rc::new(quote! {
                    #ident: #PURORO_INTERNAL::OneofFieldType
                }))
            })
            .cloned()
    }
    pub(crate) fn gen_oneof_case_value_ident(&self) -> Result<Ident> {
        Ok(format_ident!(
            "{}",
            self.name()?.to_camel_case().escape_rust_keywords()
        ))
    }

    pub(crate) fn gen_oneof_union_field_type(&self) -> Result<Rc<Type>> {
        self.cache()
            .get::<Cache>()?
            .union_field_type
            .get_or_try_init(|| {
                let inner_type_name_segment: PathSegment = {
                    use FieldType::*;
                    use LengthDelimitedType::*;
                    let r#type = self.r#type()?;
                    parse2(match r#type {
                        LengthDelimited(Message(m)) => {
                            let message_path = m.try_upgrade()?.gen_message_struct_type()?;
                            quote! {
                                HeapMessageField::< #message_path >
                            }
                        }
                        Variant(_) | Bits32(_) | Bits64(_) => {
                            let primitive = r#type.rust_type()?;
                            let tag = r#type.tag_type()?;
                            quote! {
                                NumericalField::<#primitive, #tag>
                            }
                        }
                        LengthDelimited(_) => {
                            let owned_type = r#type.rust_type()?;
                            let tag = r#type.tag_type()?;
                            quote! {
                                UnsizedField::<#owned_type, #tag>
                            }
                        }
                    })?
                };
                Ok(Rc::new(parse2(quote! {
                    #PURORO_INTERNAL:: #inner_type_name_segment
                })?))
            })
            .cloned()
    }

    pub(crate) fn gen_maybe_borrowed_type(&self, lt: Option<Lifetime>) -> Result<Rc<Type>> {
        Ok(self.r#type()?.rust_maybe_borrowed_type(lt)?)
    }

    pub(crate) fn gen_oneof_union_field(&self) -> Result<Rc<Field>> {
        let ident = self.gen_oneof_union_field_ident()?;
        let field_inner_type_ident = self.gen_oneof_union_generic_param_ident()?;
        let field_type: Type = parse2(quote! {
            ::std::mem::ManuallyDrop::< #field_inner_type_ident >
        })?;

        Ok(Rc::new(
            parse2::<NamedField>(quote! {
                #ident: #field_type
            })?
            .into(),
        ))
    }

    pub(crate) fn gen_oneof_union_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let getter_ident = self.gen_oneof_union_getter_ident()?;
        let getter_opt_ident = self.gen_oneof_union_getter_opt_ident()?;
        let getter_mut_ident = self.gen_oneof_union_getter_mut_ident()?;

        let generic_param_type_ident = self.gen_oneof_union_generic_param_ident()?;
        let getter_type: Type = parse2(quote! {
            <#generic_param_type_ident as #PURORO_INTERNAL::OneofFieldType>
                ::GetterOrElseType<'_>
        })?;
        let getter_opt_type: Type = parse2(quote! {
            <#generic_param_type_ident as #PURORO_INTERNAL::OneofFieldType>
                ::GetterOptType<'_>
        })?;
        let getter_mut_type: Type = parse2(quote! {
            <#generic_param_type_ident as #PURORO_INTERNAL::OneofFieldType>
                ::GetterMutType<'_>
        })?;

        let case_type = self.oneof()?.gen_oneof_case_type(iter::empty())?;
        let union_field_ident = self.gen_oneof_union_field_ident()?;
        let enum_item_ident = self.gen_oneof_case_value_ident()?;
        let bitfield_begin = self.oneof()?.bitfield_index_for_oneof()?.0;
        let bitfield_end = self.oneof()?.bitfield_index_for_oneof()?.1;
        let default_fn = self.gen_default_fn()?;

        Ok(vec![
            parse2(quote! {
                pub(crate) fn #getter_ident<B: #PURORO_INTERNAL::BitSlice>(&self, bits: &B) -> #getter_type {
                    #[allow(unused)] use ::std::option::Option::{None, Some};
                    #[allow(unused)] use ::std::default::Default;
                    use #PURORO_INTERNAL::OneofFieldType;
                    use ::std::ops::Deref as _;
                    use #PURORO_INTERNAL::OneofCase;

                    let case_opt = OneofCase::from_bitslice(bits);
                    let field_opt = matches!(case_opt, Some(#case_type::#enum_item_ident(()))).then(|| {
                        unsafe {
                            self.#union_field_ident.deref()
                        }
                    });
                    OneofFieldType::get_field_or_else(field_opt, #default_fn)
                }
            })?,
            parse2(quote! {
                pub(crate) fn #getter_opt_ident<B: #PURORO_INTERNAL::BitSlice>(&self, bits: &B) -> #getter_opt_type {
                    #[allow(unused)] use ::std::option::Option::{None, Some};
                    use #PURORO_INTERNAL::OneofFieldType;
                    use ::std::ops::Deref as _;
                    use #PURORO_INTERNAL::OneofCase;

                    let case_opt = OneofCase::from_bitslice(bits);
                    let field_opt = matches!(case_opt, Some(#case_type::#enum_item_ident(()))).then(|| {
                        unsafe {
                            self.#union_field_ident.deref()
                        }
                    });
                    OneofFieldType::get_field_opt(field_opt)
                }
            })?,
            parse2(quote! {
                pub(crate) fn #getter_mut_ident<B: #PURORO_INTERNAL::BitSlice>(&mut self, bits: &mut B) -> #getter_mut_type {
                    #[allow(unused)] use ::std::option::Option::Some;
                    #[allow(unused)] use ::std::default::Default;
                    use ::std::mem::ManuallyDrop;
                    use ::std::ops::DerefMut as _;
                    use #PURORO_INTERNAL::{OneofCase, OneofUnion};
                    use #PURORO_INTERNAL::OneofFieldType;

                    let case_opt = OneofCase::from_bitslice(bits);
                    if let Some(#case_type::#enum_item_ident(())) = case_opt {
                        // The union is already set to the expected value. Do nothing.
                    } else {
                        // Need to reset the union value to the expected field type.
                        <Self as OneofUnion>::clear(self, bits);
                        let index = OneofCase::into_u32(#case_type::#enum_item_ident(()));
                        bits.set_range(#bitfield_begin..#bitfield_end, index);
                        *self = Self {
                            #union_field_ident: ManuallyDrop::new((#default_fn)())
                        };
                    }
                    let field_mut = unsafe {
                        self.#union_field_ident.deref_mut()
                    };
                    OneofFieldType::get_field_mut(field_mut)
                }
            })?,
        ])
    }

    pub(crate) fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let oneof_struct_field_ident = self.oneof()?.gen_fields_struct_field_ident()?;
        let getter_ident = self.gen_oneof_union_getter_ident()?;
        let getter_opt_ident = self.gen_oneof_union_getter_opt_ident()?;
        let getter_mut_ident = self.gen_oneof_union_getter_mut_ident()?;
        let has_ident = format_ident!("has_{}", self.name()?.to_lower_snake_case());
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());

        let case_type = self.oneof()?.gen_oneof_case_type(iter::empty())?;
        let borrowed_type = self.r#type()?.rust_maybe_borrowed_type(None)?;
        let getter_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => {
                Rc::new(parse2(quote! {
                    ::std::option::Option::< #borrowed_type >
                })?)
            }
            _ => Rc::clone(&borrowed_type),
        };
        let getter_opt_type = quote! {
            ::std::option::Option::< #borrowed_type >
        };
        let getter_mut_type = self.r#type()?.rust_mut_ref_type()?;
        let enum_item_ident = self.gen_oneof_case_value_ident()?;
        let maybe_method_doc = self.gen_message_struct_field_method_doc()?.map(|doc| {
            quote! {
                #[doc=#doc]
            }
        });

        Ok(vec![
            parse2(quote! {
                pub fn #getter_ident(&self) -> #getter_type {
                    use #PURORO_INTERNAL::SharedItems as _;
                    self.fields.#oneof_struct_field_ident.#getter_ident(self.shared.bitfield())
                }
            })?,
            parse2(quote! {
                #maybe_method_doc
                pub fn #getter_opt_ident(&self) -> #getter_opt_type {
                    use #PURORO_INTERNAL::SharedItems as _;
                    self.fields.#oneof_struct_field_ident.#getter_opt_ident(self.shared.bitfield())
                }
            })?,
            parse2(quote! {
                pub fn #getter_mut_ident(&mut self) -> #getter_mut_type {
                    use #PURORO_INTERNAL::SharedItems as _;
                    self.fields.#oneof_struct_field_ident.#getter_mut_ident(self.shared.bitfield_mut())
                }
            })?,
            parse2(quote! {
                pub fn #has_ident(&self) -> bool {
                    self.#getter_opt_ident().is_some()
                }
            })?,
            parse2(quote! {
                pub fn #clear_ident(&mut self) {
                    #[allow(unused)] use ::std::option::Option::Some;
                    use #PURORO_INTERNAL::{OneofCase, OneofUnion as _, SharedItems as _};
                    if let Some(#case_type::#enum_item_ident(_)) = OneofCase::from_bitslice(self.shared.bitfield()) {
                        self.fields.#oneof_struct_field_ident.clear(self.shared.bitfield_mut())
                    }
                }
            })?,
        ])
    }

    pub(crate) fn gen_message_struct_impl_debug_method_call(
        &self,
        receiver: &mut Expr,
    ) -> Result<()> {
        let ident = self.gen_oneof_union_field_ident()?;
        let getter_opt_ident = self.gen_oneof_union_getter_opt_ident()?;
        let new_expr: ExprMethodCall = parse2(quote! {
            #receiver.field(stringify!(#ident), &self.#getter_opt_ident())
        })?;
        *receiver = new_expr.into();
        Ok(())
    }

    fn gen_message_struct_field_method_doc(&self) -> Result<Option<String>> {
        let input_file = self.message()?.input_file()?;
        let Some(sci) = input_file.source_code_info(self.location_path()?)? else {
            return Ok(None);
        };
        Ok(sci.into())
    }
}
