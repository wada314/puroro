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

use super::traits::{Address, AddressMut, AddressTry};
use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};
use ::puroro::{
    Message,
    error::Error,
    field_ops::{FieldOperations, FieldStorage, ImplicitOptional, StringFieldWrapper},
    shared::SharedFields,
};

/// Address message implementation
#[derive(Debug, Clone)]
pub struct AddressImpl<A: Allocator = Global> {
    street: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 1, 1, A>,
    city: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 2, 1, A>,
    zip_code: FieldStorage<i32, ImplicitOptional, 3, 1, A>,
    _shared: SharedFields<1, A>,
}

impl<A> AddressImpl<A>
where
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            street: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            city: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            zip_code: Default::default(),
            _shared: SharedFields::new_in(alloc),
        }
    }
}

impl AddressImpl<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A> Default for AddressImpl<A>
where
    A: Allocator + Clone + Default,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A> PartialEq for AddressImpl<A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.street == other.street
            && self.city == other.city
            && self.zip_code == other.zip_code
            && self._shared == other._shared
    }
}

impl<A> Eq for AddressImpl<A> where A: Allocator {}

impl<A: Allocator + Clone> AddressTry for AddressImpl<A> {
    fn try_street(&self) -> Result<&str, Error> {
        Ok(self.street())
    }
    fn try_city(&self) -> Result<&str, Error> {
        Ok(self.city())
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        Ok(self.zip_code())
    }
}

impl<A: Allocator + Clone> Address for AddressImpl<A> {
    fn street(&self) -> &str {
        self.street.get(&self._shared)
    }
    fn city(&self) -> &str {
        self.city.get(&self._shared)
    }
    fn zip_code(&self) -> i32 {
        self.zip_code.get(&self._shared)
    }
}

impl<A: Allocator + Clone> AddressMut for AddressImpl<A> {
    fn set_street(&mut self, v: &str) {
        self.street.set(&mut self._shared, v)
    }
    fn clear_street(&mut self) {
        self.street.clear(&mut self._shared)
    }
}

impl<A: Allocator + Clone> Message for AddressImpl<A> {
    fn parse_from_bytes_in<B>(_bytes: &[u8], _alloc: B) -> Result<Self, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes_in<B>(&self, _alloc: B) -> Result<AllocVec<u8, B>, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        todo!("Size computation not yet implemented")
    }
}
