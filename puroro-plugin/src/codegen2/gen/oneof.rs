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
use super::super::{Field, FieldRule, FieldType, LengthDelimitedType, MessageExt, Oneof};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::{Ident, TokenStream};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;

pub trait OneofExt {
    fn gen_union(&self) -> Result<TokenStream>;
}

#[derive(Debug, Default)]
struct Cache {
    enum_ident: OnceCell<Rc<Ident>>,
}

impl<T: ?Sized + Oneof> OneofExt for T {
    fn gen_union(&self) -> Result<TokenStream> {
        let ident = gen_union_ident(self)?;
        Ok(quote! {
            union #ident {
                _none,
                Foo,
            }
        })
    }
}

fn gen_union_ident(this: &(impl ?Sized + Oneof)) -> Result<Rc<Ident>> {
    this.cache()
        .get::<Cache>()?
        .enum_ident
        .get_or_try_init(|| {
            Ok(Rc::new(format_ident!(
                "{}",
                this.name()?.to_camel_case().escape_rust_keywords()
            )))
        })
        .cloned()
}
