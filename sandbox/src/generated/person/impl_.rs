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

use super::{
    Address, AddressImpl, AddressMut, DynAddress, DynAddressMut, DynPerson, DynPersonMut,
    DynPersonTry, Person, PersonMut, PersonTry, Status,
};
use ::allocator_api2::boxed::Box;
use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};
use ::puroro::{
    Message,
    error::Error,
    field_ops::{
        ExplicitOptional, FieldOperations, FieldStorage, ImplicitOptional, MessageFieldWrapper,
        SingularMessage, StringFieldWrapper,
    },
    repeated::{RefVec, RefVecMap, Repeated, repeated_from_slice},
    shared::SharedFields,
    view::ViewCow,
};

/// Standard implementation of Person message.
///
/// Uses trait-based field operations for type-safe code generation.
/// Fields are ordered by size (descending) to minimize padding.
/// Memory layout optimized for performance.
#[derive(Debug, Clone)]
pub struct PersonImpl<A: Allocator = Global> {
    // Shared fields: presence tracking, etc.
    // For 3 explicit optional fields (email, score, secondary_status): ⌈3/8⌉ = 1 byte (stack-allocated)
    // Message fields use heap allocation with Option<Box<M>> for presence tracking
    _shared: SharedFields<1, A>,

    // Exclusive fields ordered by size (descending)
    // String: 24 bytes (3 words on 64-bit)
    // Direct field types with explicit parameters for clarity
    // Format: FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN>
    name: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 1, 1, A>, // Field 1, implicit presence, 1 byte shared
    email: FieldStorage<StringFieldWrapper<A>, ExplicitOptional<0>, 3, 1, A>, // Field 3, explicit presence, bit 0, 1 byte shared

    // Message fields: use heap allocation with Option<Box<M>> for presence tracking
    // No presence bits needed - Option<Box<M>> handles presence directly
    address: FieldStorage<MessageFieldWrapper<AddressImpl<A>, A>, SingularMessage, 6, 1, A>, // Field 6, heap-allocated presence

    // Enum fields: stored as i32
    status: FieldStorage<i32, ImplicitOptional, 4, 1, A>, // Field 4, implicit presence, 1 byte shared
    secondary_status: FieldStorage<i32, ExplicitOptional<2>, 8, 1, A>, // Field 8, explicit presence, bit 2, 1 byte shared

    // Scalar fields: 4 bytes
    age: FieldStorage<i32, ImplicitOptional, 2, 1, A>, // Field 2, implicit presence, 1 byte shared
    score: FieldStorage<i32, ExplicitOptional<1>, 5, 1, A>, // Field 5, explicit presence, bit 1, 1 byte shared

    // Repeated fields
    scores: FieldStorage<AllocVec<i32, A>, puroro::field_ops::Repeated, 10, 1, A>,
    addresses: FieldStorage<AllocVec<AddressImpl<A>, A>, puroro::field_ops::Repeated, 9, 1, A>,
}

impl<A> PersonImpl<A>
where
    A: Allocator + Clone,
{
    /// Creates a new Person with default values using the provided allocator.
    pub fn new_in(alloc: A) -> Self {
        let shared = SharedFields::new_in(alloc.clone());
        Self {
            name: FieldStorage::default_in(alloc.clone()),
            email: FieldStorage::default_in(alloc.clone()),
            address: FieldStorage::default_in(alloc.clone()),
            status: Default::default(),
            secondary_status: Default::default(),
            _shared: shared,
            age: Default::default(),
            score: Default::default(),
            scores: FieldStorage::new_in(alloc.clone(), AllocVec::new_in),
            addresses: FieldStorage::new_in(alloc.clone(), AllocVec::new_in),
        }
    }
}

impl PersonImpl<Global> {
    /// Creates a new Person with default values using the global allocator.
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A> Default for PersonImpl<A>
where
    A: Allocator + Clone + Default,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A> PartialEq for PersonImpl<A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.email == other.email
            && self.address == other.address
            && self.status == other.status
            && self.secondary_status == other.secondary_status
            && self._shared == other._shared
            && self.age == other.age
            && self.score == other.score
    }
}

impl<A> Eq for PersonImpl<A> where A: Allocator {}

impl<A: Allocator + Clone> Person for PersonImpl<A> {
    fn address(&self) -> impl Address + use<'_, A> {
        self.address.get(&self._shared)
    }

    fn scores(&self) -> impl Repeated<'_, Item = i32> + use<'_, A> {
        RefVec::new(&self.scores.data)
    }

    fn addresses(&self) -> impl Repeated<'_, Item = impl Address + '_> + use<'_, A> {
        RefVecMap::new(&self.addresses.data, |addr: &AddressImpl<A>| addr)
    }
    // All other methods use default implementations from the trait definition
}

impl<A: Allocator + Clone> PersonMut for PersonImpl<A> {
    fn address_mut(&mut self) -> impl AddressMut + use<'_, A> {
        self.address.data.as_mut()
    }

    fn push_address(&mut self) -> impl AddressMut + use<'_, A> {
        let alloc = self._shared.allocator().clone();
        self.addresses.data.push(AddressImpl::new_in(alloc));
        // Safe to unwrap: just pushed one
        let last_index = self.addresses.data.len() - 1;
        &mut self.addresses.data[last_index]
    }
}

impl<A: Allocator + Clone> DynPersonTry for PersonImpl<A> {
    fn try_name(&self) -> Result<&str, Error> {
        Ok(DynPerson::name(self))
    }
    fn try_age(&self) -> Result<i32, Error> {
        Ok(DynPerson::age(self))
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        Ok(DynPerson::email(self))
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        Ok(DynPerson::score(self))
    }

    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        Ok(DynPerson::status(self))
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        Ok(DynPerson::secondary_status(self))
    }

    fn try_address(&self) -> Result<Option<ViewCow<'_, dyn DynAddress>>, Error> {
        Ok(DynPerson::address(self))
    }

    fn try_scores(&self) -> Result<ViewCow<'_, dyn Repeated<'_, Item = i32>>, Error> {
        Ok(DynPerson::scores(self))
    }

    fn try_addresses<'a: 'b, 'b>(
        &'a self,
    ) -> Result<ViewCow<'a, dyn Repeated<'a, Item = ViewCow<'b, dyn DynAddress>> + 'b>, Error> {
        Ok(DynPerson::addresses(self))
    }

    fn try_has_name(&self) -> Result<bool, Error> {
        Ok(DynPerson::has_name(self))
    }
}

impl<A: Allocator + Clone> PersonTry for PersonImpl<A> {
    fn try_address(&self) -> Result<impl Address + use<'_, A>, Error> {
        Ok(Person::address(self))
    }

    fn try_scores(&self) -> Result<impl Repeated<'_, Item = i32> + use<'_, A>, Error> {
        Ok(Person::scores(self))
    }

    fn try_addresses(
        &self,
    ) -> Result<impl Repeated<'_, Item = impl Address + '_> + use<'_, A>, Error> {
        Ok(Person::addresses(self))
    }
}

impl<A: Allocator + Clone> DynPerson for PersonImpl<A> {
    fn name(&self) -> &str {
        self.name.get(&self._shared)
    }
    fn age(&self) -> i32 {
        self.age.get(&self._shared)
    }
    fn email(&self) -> Option<&str> {
        self.email.get(&self._shared)
    }
    fn score(&self) -> Option<i32> {
        self.score.get(&self._shared)
    }
    fn status(&self) -> Result<Status, i32> {
        Status::from_wire(self.status.get(&self._shared))
    }

    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        if self.secondary_status.is_present(&self._shared) {
            match Status::from_wire(self.secondary_status.get(&self._shared).unwrap_or(0)) {
                Ok(status) => Ok(Some(status)),
                Err(unknown) => Err(unknown),
            }
        } else {
            Ok(None)
        }
    }

    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>> {
        self.address
            .get(&self._shared)
            .map(|address| ViewCow::Borrowed(address as &dyn DynAddress))
    }

    fn scores(&self) -> ViewCow<'_, dyn Repeated<'_, Item = i32>> {
        let rep = repeated_from_slice(self.scores.data.as_slice());
        ViewCow::Owned(rep)
    }

    fn addresses<'a: 'b, 'b>(
        &'a self,
    ) -> ViewCow<'a, dyn Repeated<'a, Item = ViewCow<'b, dyn DynAddress>> + 'b> {
        let adapter = RefVecMap::new(&self.addresses.data, |addr: &AddressImpl<A>| {
            ViewCow::Borrowed(addr as &dyn DynAddress)
        });
        let boxed = Box::new_in(adapter, Global);
        let boxed_dyn: Box<dyn Repeated<'a, Item = ViewCow<'b, dyn DynAddress>> + 'b> =
            ::allocator_api2::unsize_box!(boxed);
        ViewCow::Owned(boxed_dyn)
    }

    // has_* methods - sample implementation (others follow same pattern: field.is_present(&self._shared))
    fn has_name(&self) -> bool {
        self.name.is_present(&self._shared)
    }
}

impl<A: Allocator + Clone> DynPersonMut for PersonImpl<A> {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v)
    }

    fn push_score(&mut self, v: i32) {
        self.scores.data.push(v)
    }
    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self) {
        self.name.clear(&mut self._shared)
    }

    fn clear_scores(&mut self) {
        self.scores.data.clear()
    }

    fn clear_addresses(&mut self) {
        self.addresses.data.clear()
    }

    fn address_mut(&mut self) -> &mut dyn DynAddressMut {
        let alloc = self._shared.allocator().clone();
        self.address
            .data
            .get_or_insert_with(|| AddressImpl::new_in(alloc)) as &mut dyn DynAddressMut
    }

    fn push_address(&mut self) -> &mut dyn DynAddressMut {
        let alloc = self._shared.allocator().clone();
        self.addresses.data.push(AddressImpl::new_in(alloc));
        // Safe to unwrap: just pushed one
        let last_index = self.addresses.data.len() - 1;
        &mut self.addresses.data[last_index] as &mut dyn DynAddressMut
    }
}

impl<A: Allocator + Clone> Message for PersonImpl<A> {
    fn parse_from_bytes_in<B>(_bytes: &[u8], _alloc: B) -> Result<Self, Error>
    where
        B: Allocator + Clone,
    {
        // TODO: Implement actual parsing
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes_in<B>(&self, _alloc: B) -> Result<AllocVec<u8, B>, Error>
    where
        B: Allocator + Clone,
    {
        // TODO: Implement actual serialization
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        // TODO: Implement actual size computation
        todo!("Size computation not yet implemented")
    }
}
