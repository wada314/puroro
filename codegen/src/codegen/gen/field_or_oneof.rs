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
use super::{FieldExt, FieldOrOneof, OneofExt};
use crate::syn::{parse2, Field as SynField, Ident, ImplItemMethod, NamedField, Type};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait FieldOrOneofExt {
    fn gen_fields_struct_field_ident(&self) -> Result<Rc<Ident>>;
    fn gen_fields_struct_generic_param_ident(&self) -> Result<Rc<Ident>>;
    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>>;
    fn gen_fields_struct_field(&self) -> Result<SynField>;

    fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>>;
}

#[derive(Debug, Default)]
struct Cache {
    fields_struct_field_ident: OnceCell<Rc<Ident>>,
    fields_struct_generic_param_ident: OnceCell<Rc<Ident>>,
    fields_struct_field_type: OnceCell<Rc<Type>>,
}

impl<T: ?Sized + FieldOrOneof> FieldOrOneofExt for T {
    fn gen_fields_struct_field_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .fields_struct_field_ident
            .get_or_try_init(|| {
                Ok(Rc::new(format_ident!(
                    "{}",
                    self.name()?.to_lower_snake_case().escape_rust_keywords()
                )))
            })
            .cloned()
    }

    fn gen_fields_struct_generic_param_ident(&self) -> Result<Rc<Ident>> {
        self.cache()
            .get::<Cache>()?
            .fields_struct_generic_param_ident
            .get_or_try_init(|| Ok(Rc::new(format_ident!("T{}", self.name()?.to_camel_case()))))
            .cloned()
    }

    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>> {
        self.cache()
            .get::<Cache>()?
            .fields_struct_field_type
            .get_or_try_init(|| match self.either() {
                crate::codegen::data::FieldOrOneofCase::Field(f) => {
                    f.gen_fields_struct_field_type_impl()
                }
                crate::codegen::data::FieldOrOneofCase::Oneof(o) => {
                    o.gen_fields_struct_field_type_impl()
                }
            })
            .cloned()
    }

    fn gen_fields_struct_field(&self) -> Result<SynField> {
        let field_ident = self.gen_fields_struct_field_ident()?;
        let type_name = self.gen_fields_struct_generic_param_ident()?;
        Ok(parse2::<NamedField>(quote! {
            pub #field_ident: #type_name
        })?
        .into())
    }

    fn gen_message_struct_methods(&self) -> Result<Vec<ImplItemMethod>> {
        match self.either() {
            crate::codegen::data::FieldOrOneofCase::Field(f) => todo!(),
            crate::codegen::data::FieldOrOneofCase::Oneof(o) => todo!(),
        }
    }
}
