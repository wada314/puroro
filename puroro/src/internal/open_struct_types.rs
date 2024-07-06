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

#![allow(refining_impl_trait)]

use super::Any;
use crate::internal::types::{field_types as ft, FieldType};
use crate::string::{StrExt as _, String as PString};
use crate::variant::variant_types as vt;
use crate::variant::{Variant, VariantIntegerType as _};
use crate::{ErrorKind, Result};
use ::std::alloc::Allocator;
use ::std::io::Read;

pub trait OpenStructFieldType<FT: FieldType> {
    fn get(&self) -> impl Any;
    fn deser_parse_variant(&mut self, _var: Variant) -> Result<()> {
        Err(ErrorKind::UnmatchingWireAndFieldType)?
    }
    fn deser_parse_len<R: Read>(&mut self, _read: &mut R) -> Result<usize> {
        Err(ErrorKind::UnmatchingWireAndFieldType)?
    }
}

impl OpenStructFieldType<ft::Int32> for i32 {
    #[inline]
    fn get(&self) -> i32 {
        *self
    }
    #[inline]
    fn deser_parse_variant(&mut self, var: Variant) -> Result<()> {
        *self = vt::Int32::try_from_variant(var)?;
        Ok(())
    }
}

impl<E> OpenStructFieldType<ft::Enum> for E
where
    E: Clone + TryFrom<i32>,
    i32: From<E>,
    crate::ErrorKind: From<<E as TryFrom<i32>>::Error>,
{
    #[inline]
    fn get(&self) -> E {
        self.clone()
    }
    #[inline]
    fn deser_parse_variant(&mut self, var: Variant) -> Result<()> {
        *self = vt::Int32::try_from_variant(var)?.try_into()?;
        Ok(())
    }
}

impl<A: Allocator> OpenStructFieldType<ft::String> for PString<A> {
    #[inline]
    fn get(&self) -> &str {
        self.as_ref()
    }
    #[inline]
    fn deser_parse_len<R: Read>(&mut self, read: &mut R) -> Result<usize> {
        // Inefficient but easy implementation
        let mut std_str = String::new();
        let len = read.read_to_string(&mut std_str)?;
        self.clear();
        self.push_str(&std_str);
        Ok(len)
    }
}

impl<M, A: Allocator> OpenStructFieldType<ft::Message> for Option<Box<M, A>> {
    #[inline]
    fn get(&self) -> Option<&M> {
        self.as_deref()
    }
}
