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

use super::Any;
use crate::internal::types::{field_types as ft, FieldType};
use crate::variant::variant_types as vt;
use crate::variant::{Variant, VariantIntegerType as _};
use crate::{ErrorKind, Result};

pub trait OpenStructFieldType<FT: FieldType> {
    fn get(&self) -> impl Any;
    fn deser_parse_variant(&mut self, _var: Variant) -> Result<()> {
        Err(ErrorKind::UnmatchingWireAndFieldType)?
    }
}

impl OpenStructFieldType<ft::Int32> for i32 {
    fn get(&self) -> impl Any {
        *self
    }
    fn deser_parse_variant(&mut self, var: Variant) -> Result<()> {
        *self = vt::Int32::try_from_variant(var)?;
        Ok(())
    }
}
