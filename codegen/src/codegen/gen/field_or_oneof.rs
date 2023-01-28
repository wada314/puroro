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
use super::{FieldExt, FieldOrOneof, FieldOrOneofCase};
use crate::syn::{
    parse2, Arm, Expr, Field as SynField, FieldValue, Ident, ImplItemMethod, NamedField, Stmt, Type,
};
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

    fn gen_message_struct_impl_message_deser_arms(
        &self,
        field_data_expr: &Expr,
    ) -> Result<Vec<Arm>>;
    fn gen_message_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt>;
    fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue>;
    fn gen_message_struct_impl_debug_method_call(&self, receiver: &mut Expr) -> Result<()>;
    fn gen_message_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr>;
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
                FieldOrOneofCase::Field(f) => FieldExt::gen_fields_struct_field_type(f),
                FieldOrOneofCase::Oneof(o) => o.gen_fields_struct_field_type(),
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
            FieldOrOneofCase::Field(f) => FieldExt::gen_message_struct_methods(f),
            FieldOrOneofCase::Oneof(o) => o.gen_message_struct_methods(),
        }
    }

    fn gen_message_struct_impl_message_deser_arms(
        &self,
        field_data_expr: &Expr,
    ) -> Result<Vec<Arm>> {
        match self.either() {
            FieldOrOneofCase::Field(f) => {
                let arm = FieldExt::gen_message_struct_impl_message_deser_arm(f, field_data_expr)?;
                Ok(vec![arm])
            }
            FieldOrOneofCase::Oneof(o) => {
                o.gen_message_struct_impl_message_deser_arms(field_data_expr)
            }
        }
    }

    fn gen_message_struct_impl_message_ser_stmt(&self, out_expr: &Expr) -> Result<Stmt> {
        match self.either() {
            FieldOrOneofCase::Field(f) => {
                FieldExt::gen_message_struct_impl_message_ser_stmt(f, out_expr)
            }
            FieldOrOneofCase::Oneof(o) => o.gen_message_struct_impl_message_ser_stmt(out_expr),
        }
    }

    fn gen_message_struct_impl_clone_field_value(&self) -> Result<FieldValue> {
        match self.either() {
            FieldOrOneofCase::Field(f) => FieldExt::gen_message_struct_impl_clone_field_value(f),
            FieldOrOneofCase::Oneof(o) => o.gen_message_struct_impl_clone_field_value(),
        }
    }

    fn gen_message_struct_impl_debug_method_call(&self, receiver: &mut Expr) -> Result<()> {
        match self.either() {
            FieldOrOneofCase::Field(f) => {
                FieldExt::gen_message_struct_impl_debug_method_call(f, receiver)
            }
            FieldOrOneofCase::Oneof(o) => o.gen_message_struct_impl_debug_method_call(receiver),
        }
    }

    fn gen_message_struct_impl_partial_eq_cmp(&self, rhs_expr: &Expr) -> Result<Expr> {
        match self.either() {
            FieldOrOneofCase::Field(f) => {
                FieldExt::gen_message_struct_impl_partial_eq_cmp(f, rhs_expr)
            }
            FieldOrOneofCase::Oneof(o) => o.gen_message_struct_impl_partial_eq_cmp(rhs_expr),
        }
    }
}
