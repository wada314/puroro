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
use super::super::{FieldType, LengthDelimitedType, MessageExt, OneofExt, OneofField};
use super::field::gen_default_fn;
use super::PackageOrMessageExt;
use crate::syn::{
    parse2, Expr, ExprMethodCall, Field, Ident, ImplItemMethod, Lifetime, NamedField, PathSegment,
    Type,
};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait OneofFieldExt {
    fn gen_union_field_ident(&self) -> Result<Rc<Ident>>;
    fn gen_union_getter_ident(&self) -> Result<Rc<Ident>>;
    fn gen_union_getter_opt_ident(&self) -> Result<Rc<Ident>>;
    fn gen_union_getter_mut_ident(&self) -> Result<Rc<Ident>>;
    fn gen_generic_type_param_ident(&self) -> Result<Ident>;
    fn gen_case_enum_value_ident(&self) -> Result<Ident>;

    fn gen_maybe_borrowed_type(&self, lt: Option<Lifetime>) -> Result<Rc<Type>>;

    fn gen_union_field(&self) -> Result<Rc<Field>>;
    fn gen_union_methods(&self) -> Result<Vec<ImplItemMethod>>;
    fn gen_struct_methods(&self) -> Result<Vec<ImplItemMethod>>;
    fn gen_struct_impl_debug_method_call(&self, receiver: Expr) -> Result<ExprMethodCall>;
}

#[derive(Debug, Default)]
struct Cache {
    union_field_ident: OnceCell<Rc<Ident>>,
    union_gettr_ident: OnceCell<Rc<Ident>>,
    union_gettr_opt_ident: OnceCell<Rc<Ident>>,
    union_gettr_mut_ident: OnceCell<Rc<Ident>>,
}

impl<T: ?Sized + OneofField> OneofFieldExt for T {
    fn gen_union_field_ident(&self) -> Result<Rc<Ident>> {
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
    fn gen_union_getter_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_gettr_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }
    fn gen_union_getter_opt_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_gettr_opt_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}_opt",
                    self.name()?.to_lower_snake_case()
                )))
            })
            .cloned()
    }
    fn gen_union_getter_mut_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .union_gettr_mut_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}_mut",
                    self.name()?.to_lower_snake_case()
                )))
            })
            .cloned()
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

    fn gen_maybe_borrowed_type(&self, lt: Option<Lifetime>) -> Result<Rc<Type>> {
        Ok(self.r#type()?.rust_maybe_borrowed_type(lt)?)
    }

    fn gen_union_field(&self) -> Result<Rc<Field>> {
        let ident = self.gen_union_field_ident()?;
        let inner_type_name_segment: PathSegment = {
            use FieldType::*;
            use LengthDelimitedType::*;
            let r#type = self.r#type()?;
            parse2(match r#type {
                LengthDelimited(Message(m)) => {
                    let message_path = m.try_upgrade()?.gen_rust_struct_type()?;
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
        let field_type: Type = parse2(quote! {
            ::std::mem::ManuallyDrop::<
                self::_puroro::internal::oneof_field_type:: #inner_type_name_segment
            >
        })?;

        Ok(Rc::new(
            parse2::<NamedField>(quote! {
                #ident: #field_type
            })?
            .into(),
        ))
    }

    fn gen_union_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let getter_ident = self.gen_union_getter_ident()?;
        let getter_opt_ident = self.gen_union_getter_opt_ident()?;
        let getter_mut_ident = self.gen_union_getter_mut_ident()?;
        let borrowed_type = self.r#type()?.rust_maybe_borrowed_type(None)?;
        let getter_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => {
                Rc::new(parse2(quote! {
                    ::std::option::Option::< #borrowed_type >
                })?)
            }
            _ => Rc::clone(&borrowed_type),
        };
        let getter_opt_type: Type = parse2(quote! {
            ::std::option::Option::< #borrowed_type >
        })?;
        let getter_mut_type = self.r#type()?.rust_mut_ref_type()?;
        let union_ident = self.oneof()?.gen_union_ident()?;
        let case_ident = format_ident!("{}Case", self.oneof()?.name()?.to_camel_case());
        let union_item_ident = self.gen_union_field_ident()?;
        let enum_item_ident = self.gen_case_enum_value_ident()?;
        let bitfield_begin = self.oneof()?.bitfield_index_for_oneof()?.0;
        let bitfield_end = self.oneof()?.bitfield_index_for_oneof()?.1;
        let default_fn = gen_default_fn(self)?;

        Ok(vec![
            parse2(quote! {
                pub(crate) fn #getter_ident<B: self::_puroro::internal::bitvec::BitSlice>(&self, bits: &B) -> #getter_type {
                    #[allow(unused)] use ::std::option::Option::{None, Some};
                    #[allow(unused)] use ::std::default::Default;
                    use self::_puroro::internal::oneof_field_type::OneofFieldType;
                    use ::std::ops::Deref as _;
                    use self::_puroro::internal::oneof_type::OneofCase as _;

                    let case_opt = self::#case_ident::from_bitslice(bits);
                    let item_opt = matches!(case_opt, Some(self::#case_ident::#enum_item_ident(()))).then(|| {
                        unsafe {
                            self.#union_item_ident.deref()
                        }
                    });
                    OneofFieldType::get_field_or_else(item_opt, #default_fn)
                }
            })?,
            parse2(quote! {
                pub(crate) fn #getter_opt_ident<B: self::_puroro::internal::bitvec::BitSlice>(&self, bits: &B) -> #getter_opt_type {
                    #[allow(unused)] use ::std::option::Option::{None, Some};
                    use self::_puroro::internal::oneof_field_type::OneofFieldType;
                    use ::std::ops::Deref as _;
                    use self::_puroro::internal::oneof_type::OneofCase as _;

                    let case_opt = self::#case_ident::from_bitslice(bits);
                    let item_opt = matches!(case_opt, Some(self::#case_ident::#enum_item_ident(()))).then(|| {
                        unsafe {
                            self.#union_item_ident.deref()
                        }
                    });
                    OneofFieldType::get_field_opt(item_opt)
                }
            })?,
            parse2(quote! {
                pub(crate) fn #getter_mut_ident<B: self::_puroro::internal::bitvec::BitSlice>(&mut self, bits: &mut B) -> #getter_mut_type {
                    #[allow(unused)] use ::std::option::Option::Some;
                    #[allow(unused)] use ::std::default::Default;
                    use ::std::mem::ManuallyDrop;
                    use self::_puroro::internal::oneof_type::{OneofCase as _, OneofUnion};
                    use self::_puroro::internal::oneof_field_type::OneofFieldType as _;

                    let case_opt = self::#case_ident::from_bitslice(bits);
                    if let Some(self::#case_ident::#enum_item_ident(())) = case_opt {
                        // The union is already set to the expected value. Do nothing.
                    } else {
                        // Need to reset the union value to the expected field type.
                        <Self as OneofUnion>::clear(self, bits);
                        let index = self::#case_ident::into_u32(self::#case_ident::#enum_item_ident(()));
                        bits.set_range(#bitfield_begin..#bitfield_end, index);
                        *self = self::#union_ident {
                            #union_item_ident: ManuallyDrop::new((#default_fn)())
                        };
                    }
                    unsafe {
                        &mut self.#union_item_ident
                    }.get_field_mut()
                }
            })?,
        ])
    }

    fn gen_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        let oneof_struct_field_ident = self.oneof()?.gen_struct_field_ident()?;
        let getter_ident = self.gen_union_getter_ident()?;
        let getter_opt_ident = self.gen_union_getter_opt_ident()?;
        let getter_mut_ident = self.gen_union_getter_mut_ident()?;
        let has_ident = format_ident!("has_{}", self.name()?.to_lower_snake_case());
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());

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
        let case_ident = format_ident!("{}Case", self.oneof()?.name()?.to_camel_case());
        let case_path = {
            let module_path = self.oneof()?.message()?.gen_rust_module_path()?;
            quote! { #module_path :: #case_ident }
        };
        let enum_item_ident = self.gen_case_enum_value_ident()?;

        Ok(vec![
            parse2(quote! {
                pub fn #getter_ident(&self) -> #getter_type {
                    self.#oneof_struct_field_ident.#getter_ident(&self._bitfield)
                }
            })?,
            parse2(quote! {
                pub fn #getter_opt_ident(&self) -> #getter_opt_type {
                    self.#oneof_struct_field_ident.#getter_opt_ident(&self._bitfield)
                }
            })?,
            parse2(quote! {
                pub fn #getter_mut_ident(&mut self) -> #getter_mut_type {
                    self.#oneof_struct_field_ident.#getter_mut_ident(&mut self._bitfield)
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
                    use self::_puroro::internal::oneof_type::OneofCase;
                    use self::_puroro::internal::oneof_type::OneofUnion;
                    if let Some(#case_path::#enum_item_ident(_)) = OneofCase::from_bitslice(&self._bitfield) {
                        self.#oneof_struct_field_ident.clear(&mut self._bitfield)
                    }
                }
            })?,
        ])
    }

    fn gen_struct_impl_debug_method_call(&self, receiver: Expr) -> Result<ExprMethodCall> {
        let ident = self.gen_union_field_ident()?;
        let getter_opt_ident = self.gen_union_getter_opt_ident()?;
        Ok(parse2(quote! {
            #receiver.field(stringify!(#ident), &self.#getter_opt_ident())
        })?)
    }
}
