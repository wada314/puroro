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

#![allow(incomplete_features)]
#![feature(arc_unwrap_or_clone)]
#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(is_some_and)]
#![feature(try_find)]
#![feature(trait_upcasting)]

mod codegen;
mod error;

#[cfg(debug_assertions)]
mod syn {
    pub(crate) use ::syn::{
        parse2, Expr, ExprMethodCall, Field, Ident, ImplItemMethod, Item, ItemImpl, Lifetime, Path,
        PathSegment, Type,
    };
    pub(crate) struct NamedField(::syn::Field);
    impl ::syn::parse::Parse for NamedField {
        fn parse(input: syn::parse::ParseStream) -> ::syn::Result<Self> {
            Ok(NamedField(::syn::Field::parse_named(input)?))
        }
    }
    impl ::std::convert::From<NamedField> for ::syn::Field {
        fn from(value: NamedField) -> Self {
            value.0
        }
    }
}
#[cfg(not(debug_assertions))]
mod syn {
    use ::proc_macro2::TokenStream;
    use ::syn::parse::Result;
    pub(crate) type Type = TokenStream;
    pub(crate) type Expr = TokenStream;
    pub(crate) type ExprMethodCall = TokenStream;
    pub(crate) type Field = TokenStream;
    pub(crate) type ImplItemMethod = TokenStream;
    pub(crate) type Item = TokenStream;
    pub(crate) type ItemImpl = TokenStream;
    pub(crate) type Ident = TokenStream;
    pub(crate) type Lifetime = TokenStream;
    pub(crate) type Path = TokenStream;
    pub(crate) type PathSegment = TokenStream;
    pub(crate) fn parse2(ts: TokenStream) -> Result<TokenStream> {
        Ok(ts)
    }
    pub(crate) type NamedField = TokenStream;
}

pub use crate::error::{ErrorKind, GeneratorError};
pub type Result<T> = ::std::result::Result<T, GeneratorError>;

pub use ::puroro_protobuf_compiled::google::protobuf::compiler::code_generator_response::File;
pub use ::puroro_protobuf_compiled::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
pub use ::puroro_protobuf_compiled::google::protobuf::{FileDescriptorProto, FileDescriptorSet};
pub use ::puroro_protobuf_compiled::puroro;

pub use crate::codegen::generate_output_file_protos;
pub use crate::codegen::generate_tokens_for_inline;
