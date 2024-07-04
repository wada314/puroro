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
use crate::google::protobuf::field_descriptor_proto::Type as FieldType;
use crate::variant::{Int32, SInt32, Variant, VariantIntegerType as _};
use crate::{ErrorKind, Result};

pub trait OpenStructFieldType {
    fn get(&self) -> impl Any;
    fn deser_parse_variant(&mut self, _var: Variant, _field_type: FieldType) -> Result<()> {
        Err(ErrorKind::UnmatchingWireAndFieldType)?
    }
}

impl OpenStructFieldType for i32 {
    fn get(&self) -> impl Any {
        *self
    }
    fn deser_parse_variant(&mut self, var: Variant, field_type: FieldType) -> Result<()> {
        match field_type {
            FieldType::TypeInt32 => {
                *self = Int32::try_from_variant(var)?;
            }
            FieldType::TypeSInt32 => {
                *self = SInt32::try_from_variant(var)?;
            }
            _ => Err(ErrorKind::PuroroError(format!("bad field_type given")))?,
        }
        Ok(())
    }
}
