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

pub mod compile;
pub mod gen_enum_items;
pub mod gen_message_items;
pub mod module;

use crate::descriptor::{FieldType, I32Type, I64Type, LenType, VariantType, WireType};
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::collections::HashSet;
use ::syn::{parse2, parse_str, Ident, ItemUse, Path, Type, TypePath};

pub use compile::*;

#[derive(Clone)]
pub struct CodeGeneratorOptions {
    /// Should the generated code's type name be fully-qualified type name?
    /// e.g. should we just use `i32` or `::std::primitive::i32` ?
    /// Default to true.
    pub strict_type_path: bool,

    /// If true, in the generated code modules, we `use` the common types which
    /// are not included in the prelude: e.g. `::std::ops::Deref`.
    pub allow_import_common_types: bool,
}
impl Default for CodeGeneratorOptions {
    fn default() -> Self {
        Self {
            strict_type_path: true,
            allow_import_common_types: false,
        }
    }
}
impl CodeGeneratorOptions {
    pub fn primitive_type(&self, ty: &str) -> Result<Type> {
        let ident: Ident = parse_str(ty)?;
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::primitive::#ident }
        } else {
            quote! { #ident }
        })?)
    }
    pub fn imports(&self) -> Result<Vec<ItemUse>> {
        if self.allow_import_common_types {
            Ok(vec![
                parse2(quote! { #[allow(unused)] use ::std::ops::Deref; })?,
                parse2(quote! { #[allow(unused)] use ::std::ops::DerefMut; })?,
            ])
        } else {
            Ok(vec![])
        }
    }
    pub fn clone_trait(&self) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::clone::Clone }
        } else {
            quote! { Clone }
        })?)
    }
    pub fn vec_type(&self, elem_type: &Type, alloc: Option<&Type>) -> Result<Type> {
        let generic_params = if let Some(alloc) = alloc {
            quote! { #elem_type, #alloc }
        } else {
            quote! { #elem_type }
        };
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::vec::Vec<#generic_params> }
        } else {
            quote! { Vec<#generic_params> }
        })?)
    }
    pub fn option_type(&self, elem_type: &Type) -> Result<Type> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::option::Option<#elem_type> }
        } else {
            quote! { Option<#elem_type> }
        })?)
    }
    pub fn iter_trait(&self, elem_type: &Type) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { ::std::iter::Iterator<Item=#elem_type> }
        } else {
            quote! { Iterator<Item=#elem_type> }
        })?)
    }
    pub fn deref_mut_trait(&self, target: &Type) -> Result<Path> {
        Ok(parse2(
            if !self.strict_type_path && self.allow_import_common_types {
                quote! { DerefMut<Target=#target> }
            } else {
                quote! { ::std::ops::DerefMut<Target=#target> }
            },
        )?)
    }
    pub fn path_in_self_module(&self, path: &Path) -> Result<Path> {
        Ok(parse2(if self.strict_type_path {
            quote! { self::#path }
        } else {
            quote! { #path }
        })?)
    }
}

// List of the Rust "strict" and "reserved" keywords except not-"r#" prefixed ones.
const KEYWORDS_LIST: &[&str] = &[
    "abstract", "alignof", "as", "become", "box", "break", "const", "continue",
    /*"crate",*/ "do", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv",
    "proc", "pub", "pure", "ref", "return", /*"Self",*/ /*"self",*/ "sizeof", "static",
    "struct", /*"super",*/ "trait", "true", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while", "yield",
];
const KEYWORDS: LazyCell<HashSet<&'static str>> =
    LazyCell::new(|| KEYWORDS_LIST.iter().copied().collect());
const NOT_RAWNIZEABLE_KEYWORDS: LazyCell<HashSet<&'static str>> =
    LazyCell::new(|| ["crate", "Self", "self", "super"].iter().copied().collect());

pub fn avoid_reserved_keywords(s: &str) -> Cow<str> {
    if KEYWORDS.contains(&s) {
        return Cow::Owned(format!("r#{}", s));
    }
    if NOT_RAWNIZEABLE_KEYWORDS.contains(&s) {
        return Cow::Owned(format!("_{}", s));
    }
    return Cow::Borrowed(s);
}

pub fn to_ident(s: &str) -> Ident {
    format_ident!("{}", avoid_reserved_keywords(s))
}
pub fn to_ident_without_keyword_check(s: &str) -> Ident {
    format_ident!("{}", s)
}

impl<M, E: AsRef<ProtoPath>> FieldType<M, E> {
    fn maybe_into_primitive_type(
        self,
        current_path: impl AsRef<ProtoPath>,
        options: &CodeGeneratorOptions,
    ) -> Result<::std::result::Result<Type, LenType<M>>> {
        let wire_type = self.into_wire_type();
        Ok(match wire_type {
            WireType::Variant(v) => Ok(v.to_primitive_type(current_path, options)?),
            WireType::I32(i) => Ok(i.to_primitive_type(options)?),
            WireType::I64(i) => Ok(i.to_primitive_type(options)?),
            WireType::Len(l) => Err(l),
            _ => Err(format!("Group field is not supported"))?,
        })
    }
}
impl<E: AsRef<ProtoPath>> VariantType<E> {
    pub fn to_primitive_type(
        self,
        current_path: impl AsRef<ProtoPath>,
        options: &CodeGeneratorOptions,
    ) -> Result<Type> {
        Ok(match self {
            VariantType::Int32 => options.primitive_type("i32")?,
            VariantType::Int64 => options.primitive_type("i64")?,
            VariantType::UInt32 => options.primitive_type("u32")?,
            VariantType::UInt64 => options.primitive_type("u64")?,
            VariantType::SInt32 => options.primitive_type("i32")?,
            VariantType::SInt64 => options.primitive_type("i64")?,
            VariantType::Bool => options.primitive_type("bool")?,
            VariantType::Enum(path) => {
                let path = path
                    .as_ref()
                    .to_relative_path(current_path.as_ref())
                    .unwrap_or(path.as_ref());

                let path = path.to_rust_path(options)?;
                TypePath { qself: None, path }.into()
            }
        })
    }
}
impl I32Type {
    pub fn to_primitive_type(self, options: &CodeGeneratorOptions) -> Result<Type> {
        Ok(options.primitive_type(match self {
            I32Type::Fixed32 => "u32",
            I32Type::SFixed32 => "i32",
            I32Type::Float => "f32",
        })?)
    }
}
impl I64Type {
    pub fn to_primitive_type(self, options: &CodeGeneratorOptions) -> Result<Type> {
        Ok(options.primitive_type(match self {
            I64Type::Fixed64 => "u64",
            I64Type::SFixed64 => "i64",
            I64Type::Double => "f64",
        })?)
    }
}
