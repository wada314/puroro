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
    DataTypeBase, Field, FieldBase, FieldBaseExt, FieldOrOneofExt, FieldRule, FieldType,
    LengthDelimitedType, PURORO_INTERNAL,
};
use crate::syn::{
    parse2, Arm, Attribute, Expr, ExprMethodCall, FieldValue, ImplItemMethod, PathSegment, Stmt,
    Type,
};
use crate::{FatalErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

#[derive(Debug, Clone, Copy)]
struct FieldBitfieldAllocation {
    // Bit used by optional field.
    maybe_optional: Option<usize>,
    tail: usize,
}

#[derive(Debug, Default)]
struct Cache {
    // Message's bitfield allocation
    allocated_bitfield: OnceCell<FieldBitfieldAllocation>,
}

impl Field {
    pub(crate) fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>> {
        Ok(self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .get()
            .map(|a| a.tail))
    }
    pub(crate) fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize> {
        // We need to allocate the bit for optional bit.
        let mut alloc = FieldBitfieldAllocation {
            maybe_optional: None,
            tail: head,
        };
        match (self.rule()?, self.r#type()?) {
            (_, FieldType::LengthDelimited(LengthDelimitedType::Message(_))) => {
                // Optional bit not needed for Message field.
                // Do nothing
            }
            (FieldRule::Optional, _) => {
                alloc.maybe_optional = Some(alloc.tail);
                alloc.tail += 1;
            }
            (FieldRule::Singular | FieldRule::Repeated, _) => {
                // Optional bit not needed for singular / repeated field.
                // Do nothing
            }
        }
        match self
            .cache()
            .get::<Cache>()?
            .allocated_bitfield
            .try_insert(alloc)
        {
            Ok(alloc) => Ok(alloc.tail),
            Err(_) => Err(FatalErrorKind::InternalError {
                detail: "Tried to assign the field's bitfield twice.".to_string(),
            })?,
        }
    }

    pub(crate) fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>> {
        use FieldRule::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        let primitive_type = self.r#type()?.rust_type()?;
        let tag_type = self.r#type()?.tag_type()?;
        let bitfield_index = self.bitfield_index_for_optional()?.unwrap_or(usize::MAX);
        let type_name_segment: PathSegment = parse2(match (self.rule()?, self.r#type()?) {
            (Optional | Singular, LengthDelimited(Message(_))) => quote! {
                SingularHeapMessageField::<#primitive_type>
            },
            (Repeated, LengthDelimited(Message(_))) => quote! {
                RepeatedMessageField::<#primitive_type>
            },
            (Optional, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                OptionalNumericalField::<#primitive_type, #tag_type, #bitfield_index>
            },
            (Singular, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                SingularNumericalField::<#primitive_type, #tag_type>
            },
            (Repeated, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                RepeatedNumericalField::<#primitive_type, #tag_type>
            },
            (Optional, LengthDelimited(_)) => quote! {
                OptionalUnsizedField::<#primitive_type, #tag_type, #bitfield_index>
            },
            (Singular, LengthDelimited(_)) => quote! {
                SingularUnsizedField::<#primitive_type, #tag_type>
            },
            (Repeated, LengthDelimited(_)) => quote! {
                RepeatedUnsizedField::<#primitive_type, #tag_type>
            },
        })?;
        Ok(Rc::new(parse2(quote! {
            #PURORO_INTERNAL::#type_name_segment
        })?))
    }

    pub(crate) fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        match self.rule()? {
            FieldRule::Repeated => self.gen_message_struct_field_methods_for_repeated(),
            _ => self.gen_message_struct_field_methods_for_non_repeated(),
        }
    }
    pub(crate) fn gen_view_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        match self.rule()? {
            FieldRule::Repeated => self.gen_view_struct_field_methods_for_repeated(),
            _ => self.gen_view_struct_field_methods_for_non_repeated(),
        }
    }
    pub(crate) fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue> {
        let ident = self.gen_fields_struct_field_ident()?;
        Ok(parse2(quote! {
            #ident: ::std::clone::Clone::clone(&self.fields.#ident)
        })?)
    }
    pub(crate) fn gen_message_struct_impl_message_deser_arm(
        &self,
        field_data_expr: &Expr,
    ) -> Result<Arm> {
        let ident = self.gen_fields_struct_field_ident()?;
        let number = self.number()?;
        Ok(parse2(quote! {
            #number => #PURORO_INTERNAL::FieldType::deser_from_field_data(
                &mut self.view.fields.#ident,
                self.view.shared.bitfield_mut(),
                #field_data_expr,
            )?,
        })?)
    }
    pub(crate) fn gen_message_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt> {
        let ident = self.gen_fields_struct_field_ident()?;
        let number = self.number()?;
        Ok(parse2(quote! {
            #PURORO_INTERNAL::FieldType::ser_to_write(
                &self.view.fields.#ident,
                self.view.shared.bitfield(),
                #number,
                #out_expr,
            )?;
        })?)
    }
    pub(crate) fn gen_view_struct_impl_debug_method_call(&self, receiver: &mut Expr) -> Result<()> {
        let ident = self.gen_fields_struct_field_ident()?;
        let new_expr: ExprMethodCall = parse2(match self.rule()? {
            FieldRule::Repeated => {
                let getter_ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                quote! {
                    #receiver.field(
                        stringify!(#ident),
                        &self.#getter_ident().into_iter().collect::<::std::vec::Vec<_>>().as_slice()
                    )
                }
            }
            _ => {
                let getter_opt_ident = format_ident!("{}_opt", self.name()?.to_lower_snake_case());
                quote! {
                    #receiver.field(stringify!(#ident), &self.#getter_opt_ident())
                }
            }
        })?;
        *receiver = new_expr.into();
        Ok(())
    }
    pub(crate) fn gen_view_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr> {
        Ok(parse2(match self.rule()? {
            FieldRule::Repeated => {
                let getter_ident = format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                );
                quote! { self.#getter_ident().into_iter().eq(#rhs_expr.#getter_ident()) }
            }
            _ => {
                let getter_opt_ident = format_ident!("{}_opt", self.name()?.to_lower_snake_case());
                quote! { self.#getter_opt_ident() == #rhs_expr.#getter_opt_ident() }
            }
        })?)
    }

    fn bitfield_index_for_optional(&self) -> Result<Option<usize>> {
        let alloc = if let Some(alloc) = self.cache().get::<Cache>()?.allocated_bitfield.get() {
            alloc
        } else {
            let _ = self.message()?.bitfield_size()?;
            let Some(alloc) = self
            .cache()
            .get::<Cache>()?.allocated_bitfield.get() else {
                Err(FatalErrorKind::InternalError { detail: "field bitfield is not set after the message's bitfield size is calculated.".to_string() })?
            };
            alloc
        };
        Ok(alloc.maybe_optional)
    }

    fn gen_message_struct_field_methods_for_repeated(&self) -> Result<Vec<ImplItemMethod>> {
        debug_assert!(matches!(self.rule(), Ok(FieldRule::Repeated)));
        let getter_mut_ident = format_ident!("{}_mut", self.name()?.to_lower_snake_case());
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_fields_struct_field_ident()?;
        let mut_item_type = self.r#type()?.rust_type()?;
        Ok(vec![
            parse2(quote! {
                pub fn #getter_mut_ident(&mut self) -> &mut ::std::vec::Vec::<#mut_item_type> {
                    use #PURORO_INTERNAL::{RepeatedFieldType, SharedItems as _};
                    RepeatedFieldType::get_field_mut(
                        &mut self.view.fields.#field_ident, self.view.shared.bitfield_mut(),
                    )
                }
            })?,
            parse2(quote! {
                pub fn #clear_ident(&mut self) {
                    use #PURORO_INTERNAL::{RepeatedFieldType, SharedItems as _};
                    RepeatedFieldType::clear(
                        &mut self.view.fields.#field_ident, self.view.shared.bitfield_mut(),
                    )
                }
            })?,
        ])
    }

    fn gen_message_struct_field_methods_for_non_repeated(&self) -> Result<Vec<ImplItemMethod>> {
        debug_assert!(matches!(
            self.rule(),
            Ok(FieldRule::Optional | FieldRule::Singular)
        ));
        let getter_mut_ident = format_ident!("{}_mut", self.name()?.to_lower_snake_case());
        let clear_ident = format_ident!("clear_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_fields_struct_field_ident()?;
        let getter_mut_type = self.r#type()?.rust_mut_ref_type()?;
        let default_fn = self.gen_default_fn()?;

        Ok(vec![
            parse2(quote! {
                pub fn #getter_mut_ident(&mut self) -> #getter_mut_type {
                    use #PURORO_INTERNAL::{NonRepeatedFieldType, SharedItems as _};
                    NonRepeatedFieldType::get_field_mut(
                        &mut self.view.fields.#field_ident, self.view.shared.bitfield_mut(), #default_fn,
                    )
                }
            })?,
            parse2(quote! {
                pub fn #clear_ident(&mut self) {
                    use #PURORO_INTERNAL::{NonRepeatedFieldType, SharedItems as _};
                    NonRepeatedFieldType::clear(
                        &mut self.view.fields.#field_ident, self.view.shared.bitfield_mut(),
                    )
                }
            })?,
        ])
    }

    fn gen_message_struct_field_method_doc_attrs(&self) -> Result<Vec<Attribute>> {
        let input_file = self.message()?.input_file()?;
        let Some(sci) = input_file.source_code_info(self.location_path()?)? else {
            return Ok(Vec::new());
        };
        Ok(sci.gen_doc_attributes()?)
    }

    fn gen_view_struct_field_methods_for_repeated(&self) -> Result<Vec<ImplItemMethod>> {
        debug_assert!(matches!(self.rule(), Ok(FieldRule::Repeated)));
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let field_ident = self.gen_fields_struct_field_ident()?;
        let index_output_type = match self.r#type()? {
            FieldType::LengthDelimited(ld_type) => match ld_type {
                LengthDelimitedType::String => Rc::new(parse2(quote! { str })?),
                LengthDelimitedType::Bytes => Rc::new(parse2(quote! { [u8] })?),
                LengthDelimitedType::Message(m) => m.try_upgrade()?.gen_view_struct_type()?,
            },
            _ => self.r#type()?.rust_type()?,
        };
        let into_iter_item_type = match self.r#type()? {
            FieldType::LengthDelimited(_) => Rc::new(parse2(quote! { & #index_output_type })?),
            _ => self.r#type()?.rust_type()?,
        };
        let docs = self.gen_message_struct_field_method_doc_attrs()?;
        Ok(vec![parse2(quote! {
            #(#docs)*
            pub fn #getter_ident(&self) -> impl
                '_ +
                ::std::iter::IntoIterator<Item = #into_iter_item_type> +
                ::std::ops::Index<usize, Output = #index_output_type>
            {
                use #PURORO_INTERNAL::{RepeatedFieldType, SharedItems as _};
                RepeatedFieldType::get_field2(
                    &self.fields.#field_ident, self.shared.bitfield(),
                )
            }
        })?])
    }

    fn gen_view_struct_field_methods_for_non_repeated(&self) -> Result<Vec<ImplItemMethod>> {
        debug_assert!(matches!(
            self.rule(),
            Ok(FieldRule::Optional | FieldRule::Singular)
        ));
        let getter_ident = format_ident!(
            "{}",
            self.name()?.to_lower_snake_case().escape_rust_keywords()
        );
        let getter_opt_ident = format_ident!("{}_opt", self.name()?.to_lower_snake_case());
        let getter_has_ident = format_ident!("has_{}", self.name()?.to_lower_snake_case());
        let field_ident = self.gen_fields_struct_field_ident()?;
        let borrowed_type = self.r#type()?.rust_maybe_borrowed_type(None)?;
        let getter_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => {
                Rc::new(parse2(quote! {
                    ::std::option::Option::< #borrowed_type >
                })?)
            }
            _ => Rc::clone(&borrowed_type),
        };
        let getter_opt_type = Rc::new(quote! {
            ::std::option::Option::< #borrowed_type >
        });
        let default_fn = self.gen_default_fn()?;
        let docs = self.gen_message_struct_field_method_doc_attrs()?;
        let (getter_docs, getter_opt_docs) = if matches!(self.rule()?, FieldRule::Optional) {
            (Vec::new(), docs)
        } else {
            (docs, Vec::new())
        };

        Ok(vec![
            parse2(quote! {
                #(#getter_docs)*
                pub fn #getter_ident(&self) -> #getter_type {
                   use #PURORO_INTERNAL::{NonRepeatedFieldType, SharedItems as _};
                    NonRepeatedFieldType::get_field_or_else(
                        &self.fields.#field_ident, self.shared.bitfield(), #default_fn,
                    )
                }
            })?,
            parse2(quote! {
                #(#getter_opt_docs)*
                pub fn #getter_opt_ident(&self) -> #getter_opt_type {
                    use #PURORO_INTERNAL::{NonRepeatedFieldType, SharedItems as _};
                    NonRepeatedFieldType::get_field_opt(
                        &self.fields.#field_ident, self.shared.bitfield(),
                    )
                }
            })?,
            parse2(quote! {
                pub fn #getter_has_ident(&self) -> bool {
                    use #PURORO_INTERNAL::{NonRepeatedFieldType, SharedItems as _};
                    NonRepeatedFieldType::get_field_opt(
                        &self.fields.#field_ident, self.shared.bitfield(),
                    ).is_some()
                }
            })?,
        ])
    }
}
