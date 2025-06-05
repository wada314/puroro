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

use crate::descriptor::{
    FieldDescriptorExt, FieldLabel, FieldType, I32Type, I64Type, LenType, VariantType, WireType,
};
use crate::proto_path::ProtoPath;
use ::quote::format_ident;
use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::cell::OnceCell;
use ::std::collections::HashSet;
use ::std::iter::once;
use ::syn::{parse_quote, parse_str, Expr, Ident, ItemUse, Path, Type, TypePath};

pub use compile::*;

#[derive(Clone)]
pub struct CodeGeneratorOptions {
    strict_type_path: bool,
    allow_import_common_types: bool,
    cache: Cache,
}

#[derive(Default, Clone)]
struct Cache {
    imports: OnceCell<Vec<ItemUse>>,
    clone_trait: OnceCell<Path>,
    ok_path: OnceCell<Path>,
}

#[derive(Default)]
pub struct CodeGeneratorOptionsBuilder {
    /// Should the generated code's type name be fully-qualified type name?
    /// e.g. should we just use `i32` or `::std::primitive::i32` ?
    ///
    /// Default to true. If false, there's a chance that the generated code
    /// will not compile.
    pub strict_type_path: bool,

    /// If true, in the generated code modules, we `use` the common types which
    /// are not included in the prelude: e.g. `::std::ops::Deref`.
    /// Instead, the generated code will be shorter and more readable.
    ///
    /// Default to false. If true, there's a chance that the generated code
    /// will not compile.
    pub allow_import_common_types: bool,
}
impl CodeGeneratorOptionsBuilder {
    pub fn build(self) -> CodeGeneratorOptions {
        CodeGeneratorOptions {
            strict_type_path: self.strict_type_path,
            allow_import_common_types: self.allow_import_common_types,
            cache: Cache::default(),
        }
    }
}

impl CodeGeneratorOptions {
    pub fn primitive_type(&self, ty: &str) -> Type {
        let ident: Ident = parse_str(ty).unwrap_or_else(|e| panic!("parse_str failed: {}", e));
        if self.strict_type_path {
            parse_quote! { ::std::primitive::#ident }
        } else {
            parse_quote! { #ident }
        }
    }
    pub fn imports(&self) -> &[ItemUse] {
        self.cache.imports.get_or_init(|| {
            if self.allow_import_common_types {
                vec![
                    parse_quote! { #[allow(unused)] use ::std::ops::Deref; },
                    parse_quote! { #[allow(unused)] use ::std::ops::DerefMut; },
                    parse_quote! { #[allow(unused)] use ::std::vec::Vec; },
                    parse_quote! { #[allow(unused)] use ::puroro::Result; },
                    parse_quote! { #[allow(unused)] use ::puroro::repeated::RepeatedView; },
                ]
            } else {
                vec![]
            }
        })
    }
    pub fn clone_trait(&self) -> &Path {
        self.cache.clone_trait.get_or_init(|| {
            if self.strict_type_path {
                parse_quote! { ::std::clone::Clone }
            } else {
                parse_quote! { Clone }
            }
        })
    }
    pub fn vec_type(&self, elem_type: &Type, alloc: Option<&Type>) -> Type {
        let generic_params = once(elem_type).chain(alloc.into_iter());
        if self.allow_import_common_types && !self.strict_type_path {
            parse_quote! { Vec<#(#generic_params),*> }
        } else {
            parse_quote! { ::std::vec::Vec<#(#generic_params),*> }
        }
    }
    pub fn option_type(&self, elem_type: &Type) -> Type {
        if self.strict_type_path {
            parse_quote! { ::std::option::Option<#elem_type> }
        } else {
            parse_quote! { Option<#elem_type> }
        }
    }
    pub fn puroro_result_type(&self, elem_type: &Type) -> Type {
        if self.strict_type_path {
            parse_quote! { ::std::result::Result<#elem_type, ::puroro::ErrorKind> }
        } else if self.allow_import_common_types {
            parse_quote! { Result<#elem_type> }
        } else {
            parse_quote! { Result<#elem_type, ::puroro::ErrorKind> }
        }
    }
    pub fn puroro_repeated_view_trait(&self, elem_type: &Type) -> Path {
        if self.strict_type_path {
            parse_quote! { ::puroro::repeated::RepeatedView<Item=#elem_type> }
        } else {
            parse_quote! { RepeatedView<Item=#elem_type> }
        }
    }
    pub fn ok_value(&self, value: &Expr) -> Expr {
        let path = self.ok_path();
        parse_quote! { #path(#value) }
    }
    pub fn ok_path(&self) -> &Path {
        self.cache.ok_path.get_or_init(|| {
            if self.strict_type_path {
                parse_quote! { ::std::result::Result::Ok }
            } else {
                parse_quote! { Ok }
            }
        })
    }
    pub fn iter_trait(&self, elem_type: &Type) -> Path {
        if self.strict_type_path {
            parse_quote! { ::std::iter::Iterator<Item=#elem_type> }
        } else {
            parse_quote! { Iterator<Item=#elem_type> }
        }
    }
    pub fn deref_mut_trait(&self, target: &Type) -> Path {
        if !self.strict_type_path && self.allow_import_common_types {
            parse_quote! { DerefMut<Target=#target> }
        } else {
            parse_quote! { ::std::ops::DerefMut<Target=#target> }
        }
    }
    pub fn path_in_self_module(&self, path: &Path) -> Path {
        if self.strict_type_path {
            parse_quote! { self::#path }
        } else {
            parse_quote! { #path }
        }
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
    ) -> ::std::result::Result<Type, LenType<M>> {
        let wire_type = self.into_wire_type();
        match wire_type {
            WireType::Variant(v) => Ok(v.to_primitive_type(current_path, options)),
            WireType::I32(i) => Ok(i.to_primitive_type(options)),
            WireType::I64(i) => Ok(i.to_primitive_type(options)),
            WireType::Len(l) => Err(l),
            _ => panic!("Group field is not supported"),
        }
    }
}
impl<E: AsRef<ProtoPath>> VariantType<E> {
    pub fn to_primitive_type(
        self,
        current_path: impl AsRef<ProtoPath>,
        options: &CodeGeneratorOptions,
    ) -> Type {
        match self {
            VariantType::Int32 => options.primitive_type("i32"),
            VariantType::Int64 => options.primitive_type("i64"),
            VariantType::UInt32 => options.primitive_type("u32"),
            VariantType::UInt64 => options.primitive_type("u64"),
            VariantType::SInt32 => options.primitive_type("i32"),
            VariantType::SInt64 => options.primitive_type("i64"),
            VariantType::Bool => options.primitive_type("bool"),
            VariantType::Enum(path) => {
                let path = path
                    .as_ref()
                    .to_relative_path(current_path.as_ref())
                    .unwrap_or(path.as_ref());

                let path = path
                    .to_rust_path(options)
                    .unwrap_or_else(|e| panic!("to_rust_path failed: {}", e));
                TypePath { qself: None, path }.into()
            }
        }
    }
}
impl I32Type {
    pub fn to_primitive_type(self, options: &CodeGeneratorOptions) -> Type {
        options.primitive_type(match self {
            I32Type::Fixed32 => "u32",
            I32Type::SFixed32 => "i32",
            I32Type::Float => "f32",
        })
    }
}
impl I64Type {
    pub fn to_primitive_type(self, options: &CodeGeneratorOptions) -> Type {
        options.primitive_type(match self {
            I64Type::Fixed64 => "u64",
            I64Type::SFixed64 => "i64",
            I64Type::Double => "f64",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPresense {
    Implicit,
    Explicit,
    Repeated,
}

impl FieldPresense {
    fn from_field_desc(field: &FieldDescriptorExt) -> Self {
        if field.has_presence() {
            FieldPresense::Explicit
        } else if field.label() == FieldLabel::Repeated {
            FieldPresense::Repeated
        } else {
            FieldPresense::Implicit
        }
    }
}
