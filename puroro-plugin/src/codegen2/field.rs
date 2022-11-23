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

use super::util::{StrExt, WeakExt};
use super::{FieldRule, FieldType, LengthDelimitedType, Message};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::{Ident, TokenStream};
use ::puroro_protobuf_compiled::google::protobuf::{field_descriptor_proto, FieldDescriptorProto};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait Field: Debug {
    fn message(&self) -> Result<Rc<dyn Message>>;
    fn number(&self) -> i32;

    fn gen_struct_field_decl(&self) -> Result<TokenStream>;
    fn gen_struct_field_methods(&self) -> Result<TokenStream>;
    fn gen_struct_field_clone_arm(&self) -> Result<TokenStream>;
    fn gen_struct_field_deser_arm(&self, field_data_ident: &TokenStream) -> Result<TokenStream>;

    // Message's bitfield allocation
    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>>;
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize>;
}

#[derive(Debug)]
pub(super) struct FieldImpl {
    name: String,
    message: Weak<dyn Message>,
    rule: OnceCell<FieldRule>,
    r#type: OnceCell<FieldType>,
    proto3_optional: bool,
    label_opt: Option<field_descriptor_proto::Label>,
    type_opt: Option<field_descriptor_proto::Type>,
    number: i32,
    type_name: String,

    // Message's bitfield allocation
    allocated_bitfield: OnceCell<FieldBitfieldAllocation>,

    // Generated tokens cache
    struct_field_ident: OnceCell<Rc<Ident>>,
    struct_field_type: OnceCell<Rc<TokenStream>>,
}

#[derive(Debug, Clone, Copy)]
struct FieldBitfieldAllocation {
    // Bit used by optional field.
    maybe_optional: Option<usize>,
    tail: usize,
}

impl Field for FieldImpl {
    fn message(&self) -> Result<Rc<dyn Message>> {
        Ok(self.message.try_upgrade()?)
    }
    fn number(&self) -> i32 {
        self.number
    }

    fn maybe_allocated_bitfield_tail(&self) -> Result<Option<usize>> {
        Ok(self.allocated_bitfield.get().map(|a| a.tail))
    }
    fn assign_and_get_bitfield_tail(&self, head: usize) -> Result<usize> {
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
        match self.allocated_bitfield.try_insert(alloc) {
            Ok(alloc) => Ok(alloc.tail),
            Err(_) => Err(ErrorKind::InternalError {
                detail: "Tried to assign the field's bitfield twice.".to_string(),
            })?,
        }
    }

    fn gen_struct_field_decl(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_field_ident()?;
        let r#type = self.gen_struct_field_type()?;
        Ok(quote! {
            #ident: #r#type,
        })
    }
    fn gen_struct_field_methods(&self) -> Result<TokenStream> {
        match self.rule()? {
            FieldRule::Repeated => self.gen_struct_field_methods_for_repeated(),
            _ => self.gen_struct_field_methods_for_non_repeated(),
        }
    }
    fn gen_struct_field_clone_arm(&self) -> Result<TokenStream> {
        let ident = self.gen_struct_field_ident()?;
        let r#type = self.gen_struct_field_type()?;
        Ok(quote! {
            #ident: <#r#type as ::std::clone::Clone>::clone(&self.#ident),
        })
    }
    fn gen_struct_field_deser_arm(&self, field_data_ident: &TokenStream) -> Result<TokenStream> {
        let ident = self.gen_struct_field_ident()?;
        let number = self.number();
        let r#type = self.gen_struct_field_type()?;
        Ok(quote! {
            #number => <#r#type as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                &mut self.#ident,
                &mut self._bitfield,
                #field_data_ident,
            )?,
        })
    }
}

impl FieldImpl {
    pub(super) fn new(proto: &FieldDescriptorProto, message: Weak<dyn Message>) -> Rc<Self> {
        Rc::new(FieldImpl {
            name: proto.name().to_string(),
            message,
            rule: OnceCell::new(),
            r#type: OnceCell::new(),
            proto3_optional: proto.proto3_optional(),
            label_opt: proto.label_opt(),
            type_opt: proto.type_opt(),
            number: proto.number(),
            type_name: proto.type_name().to_string(),
            allocated_bitfield: OnceCell::new(),
            struct_field_type: OnceCell::new(),
            struct_field_ident: OnceCell::new(),
        })
    }

    fn rule(&self) -> Result<FieldRule> {
        self.rule
            .get_or_try_init(|| {
                let syntax = self.message()?.input_file()?.syntax()?;
                Ok(FieldRule::try_new(
                    self.label_opt.clone(),
                    syntax,
                    self.proto3_optional,
                )?)
            })
            .cloned()
    }

    fn r#type(&self) -> Result<&FieldType> {
        self.r#type.get_or_try_init(|| {
            let syntax = self.message()?.input_file()?.syntax()?;
            Ok(FieldType::try_new(
                self.type_opt.clone(),
                &self.type_name,
                syntax,
                self,
            )?)
        })
    }

    fn bitfield_index_for_optional(&self) -> Result<Option<usize>> {
        let alloc = if let Some(alloc) = self.allocated_bitfield.get() {
            alloc
        } else {
            let _ = self.message()?.bitfield_size()?;
            let Some(alloc) = self.allocated_bitfield.get() else {
                Err(ErrorKind::InternalError { detail: "field bitfield is not set after the message's bitfield size is calculated.".to_string() })?
            };
            alloc
        };
        Ok(alloc.maybe_optional)
    }

    fn gen_struct_field_type(&self) -> Result<Rc<TokenStream>> {
        use FieldRule::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        self.struct_field_type
            .get_or_try_init(|| {
                let primitive_type = self.r#type()?.rust_type()?;
                let tag_type = self.r#type()?.tag_type()?;
                let bitfield_index = self.bitfield_index_for_optional()?.unwrap_or(usize::MAX);
                let type_name = match (self.rule()?, self.r#type()?) {
                    (Optional, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                        OptionalNumericalField::<#primitive_type, #tag_type, #bitfield_index>
                    },
                    (Singular, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                        SingularNumericalField::<#primitive_type, #tag_type>
                    },
                    (Repeated, Variant(_) | Bits32(_) | Bits64(_)) => quote! {
                        RepeatedNumericalField::<#primitive_type, #tag_type>
                    },
                    (Optional, LengthDelimited(String)) => quote! {
                        OptionalStringField::<#bitfield_index>
                    },
                    (Singular, LengthDelimited(String)) => quote! {
                        SingularStringField
                    },
                    (Repeated, LengthDelimited(String)) => quote! {
                        RepeatedStringField
                    },
                    (Optional, LengthDelimited(Bytes)) => quote! {
                        OptionalBytesField::<#bitfield_index>
                    },
                    (Singular, LengthDelimited(Bytes)) => quote! {
                        SingularBytesField
                    },
                    (Repeated, LengthDelimited(Bytes)) => quote! {
                        RepeatedBytesField
                    },
                    (Optional | Singular, LengthDelimited(Message(_))) => quote! {
                        SingularHeapMessageField::<#primitive_type>
                    },
                    (Repeated, LengthDelimited(Message(_))) => quote! {
                        RepeatedMessageField::<#primitive_type>
                    },
                };
                Ok(Rc::new(quote! {
                    self::_puroro::internal::field_type::#type_name
                }))
            })
            .cloned()
    }

    fn gen_struct_field_ident(&self) -> Result<Rc<Ident>> {
        self.struct_field_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }

    fn gen_struct_field_methods_for_repeated(&self) -> Result<TokenStream> {
        debug_assert!(matches!(self.rule(), Ok(FieldRule::Repeated)));
        let getter_ident =
            format_ident!("{}", self.name.to_lower_snake_case().escape_rust_keywords());
        let field_ident = self.gen_struct_field_ident()?;
        let field_type = self.gen_struct_field_type()?;
        let getter_item_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::String) => Rc::new(quote! {
                impl ::std::ops::Deref::<Target = str>
            }),
            FieldType::LengthDelimited(LengthDelimitedType::Bytes) => Rc::new(quote! {
                impl ::std::ops::Deref::<Target = [u8]>
            }),
            FieldType::LengthDelimited(LengthDelimitedType::Message(m)) => {
                m.gen_rust_struct_path()?
            }
            _ => self.r#type()?.rust_type()?,
        };
        Ok(quote! {
            pub fn #getter_ident(&self) -> &[#getter_item_type] {
                use self::_puroro::internal::field_type::RepeatedFieldType;
                <#field_type as RepeatedFieldType>::get_field(
                    &self.#field_ident, &self._bitfield,
                )
            }
        })
    }

    fn gen_struct_field_methods_for_non_repeated(&self) -> Result<TokenStream> {
        debug_assert!(matches!(
            self.rule(),
            Ok(FieldRule::Optional | FieldRule::Singular)
        ));
        let getter_ident =
            format_ident!("{}", self.name.to_lower_snake_case().escape_rust_keywords());
        let field_ident = self.gen_struct_field_ident()?;
        let field_type = self.gen_struct_field_type()?;
        let getter_type = match self.r#type()? {
            FieldType::LengthDelimited(LengthDelimitedType::Message(_)) => {
                let message_ref = self.r#type()?.rust_maybe_borrowed_type()?;
                Rc::new(quote! {
                    ::std::option::Option::< #message_ref >
                })
            }
            _ => self.r#type()?.rust_maybe_borrowed_type()?,
        };
        Ok(quote! {
            pub fn #getter_ident(&self) -> #getter_type {
               use self::_puroro::internal::field_type::NonRepeatedFieldType;
                <#field_type as NonRepeatedFieldType>::get_field(
                    &self.#field_ident, &self._bitfield, ::std::default::Default::default,
                )
            }
        })
    }
}
