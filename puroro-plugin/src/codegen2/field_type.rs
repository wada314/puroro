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

use super::r#enum::Enum;
use super::field::Field;
use super::message::Message;
use super::{MessageOrEnum, Syntax};
use crate::{ErrorKind, Result};
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::field_descriptor_proto;
use ::quote::quote;
use ::std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) enum FieldType {
    Variant(VariantType),
    LengthDelimited(LengthDelimitedType),
    Bits32(Bits32Type),
    Bits64(Bits64Type),
}
#[derive(Debug, Clone)]
pub(super) enum VariantType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Bool,
    Enum2(Rc<dyn Enum>),
    Enum3(Rc<dyn Enum>),
}
#[derive(Debug, Clone)]
pub(super) enum LengthDelimitedType {
    String,
    Bytes,
    Message(Rc<dyn Message>),
}
#[derive(Debug, Clone)]
pub(super) enum Bits32Type {
    Fixed32,
    SFixed32,
    Float,
}
#[derive(Debug, Clone)]
pub(super) enum Bits64Type {
    Fixed64,
    SFixed64,
    Double,
}

impl FieldType {
    pub(super) fn try_new(
        type_opt: Option<field_descriptor_proto::Type>,
        type_name: &str,
        syntax: Syntax,
        field: &dyn Field,
    ) -> Result<Self> {
        use field_descriptor_proto::Type::*;
        use Bits32Type::*;
        use Bits64Type::*;
        use FieldType::*;
        use LengthDelimitedType::*;
        use VariantType::*;

        let maybe_m_or_e = (!type_name.is_empty())
            .then(|| -> Result<_> { Ok(field.message()?.resolve_type_name(type_name)?) })
            .transpose()?;

        if let Some(r#type) = type_opt {
            Ok(match r#type {
                TypeInt32 => Variant(Int32),
                TypeUint32 => Variant(UInt32),
                TypeSint32 => Variant(SInt32),
                TypeInt64 => Variant(Int64),
                TypeUint64 => Variant(UInt64),
                TypeSint64 => Variant(SInt64),
                TypeBool => Variant(Bool),
                TypeEnum => {
                    if let Some(MessageOrEnum::Enum(e)) = maybe_m_or_e {
                        match syntax {
                            Syntax::Proto2 => Variant(Enum2(e)),
                            Syntax::Proto3 => Variant(Enum3(e)),
                        }
                    } else {
                        Err(ErrorKind::UnknownTypeName {
                            name: type_name.to_string(),
                        })?
                    }
                }
                TypeFixed32 => Bits32(Fixed32),
                TypeSfixed32 => Bits32(SFixed32),
                TypeFloat => Bits32(Float),
                TypeFixed64 => Bits64(Fixed64),
                TypeSfixed64 => Bits64(SFixed64),
                TypeDouble => Bits64(Double),
                TypeString => LengthDelimited(String),
                TypeBytes => LengthDelimited(Bytes),
                TypeGroup => Err(ErrorKind::GroupNotSupported)?,
                TypeMessage => {
                    if let Some(MessageOrEnum::Message(m)) = maybe_m_or_e {
                        LengthDelimited(Message(m))
                    } else {
                        Err(ErrorKind::UnknownTypeName {
                            name: type_name.to_string(),
                        })?
                    }
                }
            })
        } else if let Some(m_or_e) = maybe_m_or_e {
            Ok(match m_or_e {
                MessageOrEnum::Message(m) => LengthDelimited(Message(m)),
                MessageOrEnum::Enum(e) => match syntax {
                    Syntax::Proto2 => Variant(Enum2(e)),
                    Syntax::Proto3 => Variant(Enum3(e)),
                },
            })
        } else {
            Err(ErrorKind::MissingTypeName)?
        }
    }

    pub(super) fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use FieldType::*;
        match self {
            Variant(v) => v.rust_type(),
            LengthDelimited(ld) => ld.rust_type(),
            Bits32(b) => b.rust_type(),
            Bits64(b) => b.rust_type(),
        }
    }
    pub(super) fn rust_maybe_borrowed_type(&self) -> Result<Rc<TokenStream>> {
        if let FieldType::LengthDelimited(ref ld) = self {
            ld.rust_maybe_borrowed_type()
        } else {
            self.rust_type()
        }
    }
    pub(super) fn rust_mut_ref_type(&self) -> Result<Rc<TokenStream>> {
        if let FieldType::LengthDelimited(ref ld) = self {
            ld.rust_mut_ref_type()
        } else {
            let raw_type = self.rust_type()?;
            Ok(Rc::new(quote! { &mut #raw_type }))
        }
    }
    pub(super) fn tag_type(&self) -> Result<Rc<TokenStream>> {
        use FieldType::*;
        match self {
            Variant(v) => v.tag_type(),
            LengthDelimited(ld) => ld.tag_type(),
            Bits32(b) => b.tag_type(),
            Bits64(b) => b.tag_type(),
        }
    }
}

impl VariantType {
    fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use VariantType::*;
        Ok(match self {
            Int32 | SInt32 => Rc::new(quote! { i32 }),
            UInt32 => Rc::new(quote! { u32 }),
            Int64 | SInt64 => Rc::new(quote! { i64 }),
            UInt64 => Rc::new(quote! { u64 }),
            Bool => Rc::new(quote! { bool }),
            Enum2(e) => e.gen_rust_enum_path()?,
            Enum3(e) => e.gen_rust_enum_path()?,
        })
    }
    fn tag_type(&self) -> Result<Rc<TokenStream>> {
        use VariantType::*;
        let tag_name = match self {
            Int32 => quote! { Int32 },
            SInt32 => quote! { SInt32 },
            UInt32 => quote! { UInt32 },
            Int64 => quote! { Int64 },
            SInt64 => quote! { SInt64 },
            UInt64 => quote! { UInt64 },
            Bool => quote! { Bool },
            Enum2(e) => {
                let enum_path = e.gen_rust_enum_path()?;
                quote! { Enum2 :: <#enum_path> }
            }
            Enum3(e) => {
                let enum_path = e.gen_rust_enum_path()?;
                quote! { Enum3 :: <#enum_path> }
            }
        };
        Ok(Rc::new(quote! {
            self::_puroro::tags::#tag_name
        }))
    }
}
impl LengthDelimitedType {
    fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use LengthDelimitedType::*;
        Ok(match self {
            String => Rc::new(quote! { ::std::string::String }),
            Bytes => Rc::new(quote! { ::std::vec::Vec<u8> }),
            Message(m) => m.gen_rust_struct_path()?,
        })
    }
    fn rust_maybe_borrowed_type(&self) -> Result<Rc<TokenStream>> {
        use LengthDelimitedType::*;
        Ok(match self {
            String => Rc::new(quote! { &str }),
            Bytes => Rc::new(quote! { &[u8] }),
            Message(m) => {
                let path = m.gen_rust_struct_path()?;
                Rc::new(quote! {
                    & #path
                })
            }
        })
    }
    fn rust_mut_ref_type(&self) -> Result<Rc<TokenStream>> {
        use LengthDelimitedType::*;
        Ok(match self {
            String => Rc::new(quote! { &mut ::std::string::String }),
            Bytes => Rc::new(quote! { &mut ::std::vec::Vec::<u8> }),
            Message(m) => {
                let path = m.gen_rust_struct_path()?;
                Rc::new(quote! {
                    &mut #path
                })
            }
        })
    }
    fn tag_type(&self) -> Result<Rc<TokenStream>> {
        use LengthDelimitedType::*;
        let tag_name = match self {
            String => quote! { String },
            Bytes => quote! { Bytes },
            Message(m) => {
                let struct_path = m.gen_rust_struct_path()?;
                quote! { Message :: <#struct_path> }
            }
        };
        Ok(Rc::new(quote! {
            self::_puroro::tags::#tag_name
        }))
    }
}
impl Bits32Type {
    fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use Bits32Type::*;
        Ok(Rc::new(match self {
            Fixed32 => quote! { u32 },
            SFixed32 => quote! { i32 },
            Float => quote! { f32 },
        }))
    }
    fn tag_type(&self) -> Result<Rc<TokenStream>> {
        use Bits32Type::*;
        let tag_name = match self {
            Fixed32 => quote! { Fixed32 },
            SFixed32 => quote! { SFixed32 },
            Float => quote! { Float },
        };
        Ok(Rc::new(quote! {
            self::_puroro::tags::#tag_name
        }))
    }
}
impl Bits64Type {
    fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use Bits64Type::*;
        Ok(Rc::new(match self {
            Fixed64 => quote! { u64 },
            SFixed64 => quote! { i64 },
            Double => quote! { f64 },
        }))
    }
    fn tag_type(&self) -> Result<Rc<TokenStream>> {
        use Bits64Type::*;
        let tag_name = match self {
            Fixed64 => quote! { Fixed64 },
            SFixed64 => quote! { SFixed64 },
            Double => quote! { Double },
        };
        Ok(Rc::new(quote! {
            self::_puroro::tags::#tag_name
        }))
    }
}
