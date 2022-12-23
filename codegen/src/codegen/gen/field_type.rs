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
    Bits32Type, Bits64Type, EnumExt, FieldType, LengthDelimitedType, MessageExt, VariantType,
};
use crate::Result;
use ::proc_macro2::{Span, TokenStream};
use ::quote::quote;
use ::std::rc::Rc;
use ::syn::{Ident, Lifetime};

impl FieldType {
    pub fn rust_type(&self) -> Result<Rc<TokenStream>> {
        use FieldType::*;
        match self {
            Variant(v) => v.rust_type(),
            LengthDelimited(ld) => ld.rust_type(),
            Bits32(b) => b.rust_type(),
            Bits64(b) => b.rust_type(),
        }
    }
    pub fn rust_maybe_borrowed_type(&self, lt: Option<Ident>) -> Result<Rc<TokenStream>> {
        if let FieldType::LengthDelimited(ref ld) = self {
            ld.rust_maybe_borrowed_type(lt)
        } else {
            self.rust_type()
        }
    }
    pub fn rust_mut_ref_type(&self) -> Result<Rc<TokenStream>> {
        if let FieldType::LengthDelimited(ref ld) = self {
            ld.rust_mut_ref_type()
        } else {
            let raw_type = self.rust_type()?;
            Ok(Rc::new(quote! { &mut #raw_type }))
        }
    }
    pub fn tag_type(&self) -> Result<Rc<TokenStream>> {
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
            Enum2(e) | Enum3(e) => {
                let ty = e.try_upgrade()?.gen_rust_enum_type()?;
                Rc::new(quote! { #ty })
            }
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
                let enum_path = e.try_upgrade()?.gen_rust_enum_path()?;
                quote! { Enum2 :: <#enum_path> }
            }
            Enum3(e) => {
                let enum_path = e.try_upgrade()?.gen_rust_enum_path()?;
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
            Message(m) => {
                let ty = m.try_upgrade()?.gen_rust_struct_type()?;
                Rc::new(quote! { #ty })
            }
        })
    }
    fn rust_maybe_borrowed_type(&self, lt: Option<Ident>) -> Result<Rc<TokenStream>> {
        use LengthDelimitedType::*;
        let lt = lt.map(|ident| Lifetime {
            apostrophe: Span::call_site(),
            ident,
        });
        Ok(match self {
            String => Rc::new(quote! { &#lt str }),
            Bytes => Rc::new(quote! { &#lt [u8] }),
            Message(m) => {
                let path = m.try_upgrade()?.gen_rust_struct_type()?;
                Rc::new(quote! {
                    &#lt #path
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
                let path = m.try_upgrade()?.gen_rust_struct_type()?;
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
                let struct_path = m.try_upgrade()?.gen_rust_struct_type()?;
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
