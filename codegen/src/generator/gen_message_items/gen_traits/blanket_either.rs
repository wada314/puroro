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

use super::{BlanketImplsGenerator, Field2, FieldPresense};
use crate::generator::CodeGeneratorOptions;
use crate::Result;
use ::puroro::Either;
use ::quote::quote;
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{parse2, parse_str, Block, Expr, Ident, Item, Path, Signature};

pub struct GenBlanketEitherImpls;

impl BlanketImplsGenerator for GenBlanketEitherImpls {
    fn generate<'a>(
        &self,
        trait_name: &Ident,
        trait_path: &Path,
        options: Rc<CodeGeneratorOptions>,
        fields: impl Iterator<Item = &'a Field2>,
    ) -> Result<Vec<Item>> {
        todo!()
    }
}

impl GenBlanketEitherImpls {
    fn gen_blanket_either_try_get_method_body(
        &self,
        field: &Field2,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Block> {
        todo!()
    }

    fn gen_blanket_either_try_has_method_body(
        &self,
        field: &Field2,
        t1: &Ident,
        t2: &Ident,
        trait_path: &Path,
    ) -> crate::Result<Option<Block>> {
        todo!()
    }
}
