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
use super::{Enum, FieldOrOneof, PackageOrMessageExt, Syntax, PURORO_LIB};
use crate::syn;
use crate::syn::{parse2, Field as SynField, Ident, Item, ItemEnum, NamedField, Path, Type};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;
use ::syn::ItemImpl;

pub trait FieldOrOneofExt {
    fn gen_message_struct_field_ident(&self) -> Result<Rc<Ident>>;
    fn gen_fields_struct_generic_param_ident(&self) -> Result<Rc<Ident>>;
    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>>;
    fn gen_fields_struct_field(&self) -> Result<SynField>;
}

impl<T: FieldOrOneof> FieldOrOneofExt for T {
    fn gen_message_struct_field_ident(&self) -> Result<Rc<Ident>> {
        todo!()
    }

    fn gen_fields_struct_generic_param_ident(&self) -> Result<Rc<Ident>> {
        todo!()
    }

    fn gen_fields_struct_field_type(&self) -> Result<Rc<Type>> {
        todo!()
    }

    fn gen_fields_struct_field(&self) -> Result<SynField> {
        let field_ident = self.gen_message_struct_field_ident()?;
        let type_name = self.gen_fields_struct_generic_param_ident()?;
        Ok(parse2::<NamedField>(quote! {
            pub #field_ident: #type_name
        })?
        .into())
    }
}
