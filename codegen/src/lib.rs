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
#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![feature(is_some_and)]
#![feature(result_flattening)]
#![feature(try_find)]
#![feature(trait_upcasting)]
#![feature(allocator_api)]

mod codegen;
mod error;

mod syn {
    pub(crate) use ::syn::{
        parse2, parse_str, Arm, Attribute, Expr, ExprMethodCall, Field, FieldValue, File, Ident,
        ImplItemMethod, Item, ItemEnum, ItemImpl, Lifetime, Path, PathSegment, Stmt, Type,
    };
    pub(crate) struct NamedField(::syn::Field);
    impl ::syn::parse::Parse for NamedField {
        fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
            Ok(NamedField(::syn::Field::parse_named(input)?))
        }
    }
    impl ::std::convert::From<NamedField> for ::syn::Field {
        fn from(value: NamedField) -> Self {
            value.0
        }
    }
    pub(crate) struct OuterAttributes(Vec<Attribute>);
    impl ::syn::parse::Parse for OuterAttributes {
        fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
            Ok(OuterAttributes(::syn::Attribute::parse_outer(&input)?))
        }
    }
    impl ::std::convert::From<OuterAttributes> for Vec<Attribute> {
        fn from(value: OuterAttributes) -> Self {
            value.0
        }
    }
}

pub use crate::error::{FatalErrorKind, GeneratorError};
pub type Result<T> = ::std::result::Result<T, GeneratorError>;

pub use ::puroro::protobuf::google::protobuf::compiler::code_generator_response::File;
// SOMEHOW these `pub use`s are causing cargo doc to stack overflow!
#[doc(hidden)]
pub use ::puroro::protobuf::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
#[doc(hidden)]
pub use ::puroro::protobuf::google::protobuf::{FileDescriptorProto, FileDescriptorSet};
pub use ::puroro::protobuf::puroro;

pub use crate::codegen::generate_output_file_protos;
pub use crate::codegen::generate_tokens_for_inline;
pub use crate::codegen::CodegenOptions;

//////////////////////////////////////////////////////

use std::alloc::{Allocator, Global};
use std::ops::Deref;

pub struct PersonImpl<P: ?Sized> {
    name: String,
    partner: Option<Box<PersonImpl<>>>,
    params: P,
}

pub trait UnknownFields {}
impl UnknownFields for () {}
pub trait Params {
    fn allocator(&self) -> &dyn Allocator;
    fn unknown_fields(&self) -> &dyn UnknownFields;
}
pub struct ParamsImpl<A, U> {
    allocator: A,
    unknown_fields: U,
}
impl<A: Allocator, U: UnknownFields> Params for ParamsImpl<A, U> {
    fn allocator(&self) -> &dyn Allocator {
        &self.allocator
    }
    fn unknown_fields(&self) -> &dyn UnknownFields {
        &self.unknown_fields
    }
}

#[repr(transparent)]
pub struct Person<A, U>(PersonImpl<ParamsImpl<A, U>>);
#[repr(transparent)]
pub struct PersonView(PersonImpl<dyn Params>);

impl<A: Allocator, U: UnknownFields> Deref for Person<A, U> {
    type Target = PersonView;
    fn deref(&self) -> &Self::Target {
        let inner: &PersonImpl<dyn Params> = &self.0;
        unsafe { &*(inner as *const PersonImpl<dyn Params> as *const PersonView) }
    }
}

fn hoge(p: Person<Global, ()>) {
    let rp = &p;
    let dp = rp.deref();
}
