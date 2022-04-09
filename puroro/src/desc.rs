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

use crate::internal::Bitfield;
use crate::tags;
use crate::{ErrorKind, PuroroError, Result};

#[derive(Debug, PartialEq)]
pub struct MessageDescriptor {
    fields: &'static [FieldDescriptor],
}

impl<'a> MessageDescriptor {
    pub fn field_by_number(&self, number: i32) -> Result<&FieldDescriptor> {
        debug_assert!(self.fields.is_sorted_by_key(|f| f.number));
        Ok(self
            .fields
            .binary_search_by_key(&number, |f| f.number)
            .map(|index| &self.fields[index])
            .map_err(|_| ErrorKind::ReflectionError)?)
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldDescriptor {
    number: i32,
    default_value: FieldDefaultValue,
    field_type: FieldTypeEnum,
    field_label: FieldLabelEnum,
}

impl FieldDescriptor {
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn field_type(&self) -> FieldTypeEnum {
        self.field_type
    }
    pub fn field_label(&self) -> FieldLabelEnum {
        self.field_label
    }
    pub fn default_value_u32(&self) -> Result<u32> {
        match &self.default_value {
            FieldDefaultValue::U32(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    pub fn default_value_str(&self) -> Result<&'static str> {
        match &self.default_value {
            FieldDefaultValue::String(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
}

pub trait StaticMessageDescriptor {
    /// Only existing field implements `StaticFieldDescriptor`
    type Fields<const NUMBER: i32>;
    type OwnedBitfield: Bitfield;
}
pub trait StaticFieldDescriptor {
    const NUMBER: i32;
    const DEFAULT_VALUE: Option<FieldDefaultValue>;
    const OWNED_HASFIELD_BITFIELD_INDEX: Option<usize>;
    type FieldLabelTag: tags::FieldLabelTag;
    type FieldTypeTag: tags::FieldTypeTag;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldTypeEnum {
    Int32,
    String,
    M(&'static MessageDescriptor),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldLabelEnum {
    Optional,
    Required,
    Repeated,
    Unlabeled,
}

#[derive(Debug, PartialEq)]
pub enum FieldDefaultValue {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(&'static str),
    Bytes(&'static [u8]),
}
macro_rules! impl_try_from {
    ($ty:ty, $enum_value:ident) => {
        impl<'a> TryFrom<FieldDefaultValue> for $ty {
            type Error = PuroroError;
            fn try_from(value: FieldDefaultValue) -> ::std::result::Result<Self, Self::Error> {
                Ok(match value {
                    FieldDefaultValue::$enum_value(v) => v,
                    _ => Err(ErrorKind::ReflectionError)?,
                })
            }
        }
    };
}

impl_try_from!(u32, U32);
impl_try_from!(u64, U64);
impl_try_from!(i32, I32);
impl_try_from!(i64, I64);
impl_try_from!(f32, F32);
impl_try_from!(f64, F64);
impl_try_from!(bool, Bool);
impl_try_from!(&'a str, String);
impl_try_from!(&'a [u8], Bytes);
