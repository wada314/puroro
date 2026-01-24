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

use super::{Address, AddressMut, AddressTry, Status};
use ::allocator_api2::boxed::Box;
use ::puroro::error::Error;
use ::puroro::repeated::{OptionRepeated, Repeated};
use ::std::rc::Rc;

/// Flexible view trait for Person message (not dyn-compatible), fallible variant.
///
/// This trait is intended for implementations where field access can fail (e.g. lazy parsing,
/// validation, IO-backed sources). Methods use the `try_` prefix to avoid collisions with the
/// infallible trait.
pub trait PersonTry {
    type Address<'a>: AddressTry + 'a
    where
        Self: 'a;
    type Scores<'a>: Repeated<'a, Item = i32> + 'a
    where
        Self: 'a;
    type AddressItem<'a>: AddressTry + 'a
    where
        Self: 'a;
    type Addresses<'a>: Repeated<'a, Item = Self::AddressItem<'a>> + 'a
    where
        Self: 'a;

    // Getters
    fn try_name(&self) -> Result<&str, Error>;
    fn try_age(&self) -> Result<i32, Error>;
    fn try_email(&self) -> Result<Option<&str>, Error>;
    fn try_score(&self) -> Result<Option<i32>, Error>;

    // Enum field getters (unknown values remain `Result<_, i32>`)
    fn try_status(&self) -> Result<Result<Status, i32>, Error>;
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error>;

    // Presence checks
    fn try_has_name(&self) -> Result<bool, Error>;

    // Message/repeated getters
    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error>;
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error>;
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error>;
}

// Blanket implementations (fallible)
impl<T: PersonTry> PersonTry for &T {
    type Address<'a> = T::Address<'a>
    where
        Self: 'a;
    type Scores<'a> = T::Scores<'a>
    where
        Self: 'a;
    type AddressItem<'a> = T::AddressItem<'a>
    where
        Self: 'a;
    type Addresses<'a> = T::Addresses<'a>
    where
        Self: 'a;

    fn try_name(&self) -> Result<&str, Error> {
        (*self).try_name()
    }
    fn try_age(&self) -> Result<i32, Error> {
        (*self).try_age()
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        (*self).try_email()
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        (*self).try_score()
    }
    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        (*self).try_status()
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        (*self).try_secondary_status()
    }
    fn try_has_name(&self) -> Result<bool, Error> {
        (*self).try_has_name()
    }
    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error> {
        (*self).try_address()
    }
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error> {
        (*self).try_scores()
    }
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error> {
        (*self).try_addresses()
    }
}

impl<T: PersonTry> PersonTry for &mut T {
    type Address<'a> = T::Address<'a>
    where
        Self: 'a;
    type Scores<'a> = T::Scores<'a>
    where
        Self: 'a;
    type AddressItem<'a> = T::AddressItem<'a>
    where
        Self: 'a;
    type Addresses<'a> = T::Addresses<'a>
    where
        Self: 'a;

    fn try_name(&self) -> Result<&str, Error> {
        (**self).try_name()
    }
    fn try_age(&self) -> Result<i32, Error> {
        (**self).try_age()
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        (**self).try_email()
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        (**self).try_score()
    }
    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        (**self).try_status()
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        (**self).try_secondary_status()
    }
    fn try_has_name(&self) -> Result<bool, Error> {
        (**self).try_has_name()
    }
    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error> {
        (**self).try_address()
    }
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error> {
        (**self).try_scores()
    }
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error> {
        (**self).try_addresses()
    }
}

impl<T: PersonTry> PersonTry for Box<T> {
    type Address<'a> = T::Address<'a>
    where
        Self: 'a;
    type Scores<'a> = T::Scores<'a>
    where
        Self: 'a;
    type AddressItem<'a> = T::AddressItem<'a>
    where
        Self: 'a;
    type Addresses<'a> = T::Addresses<'a>
    where
        Self: 'a;

    fn try_name(&self) -> Result<&str, Error> {
        (**self).try_name()
    }
    fn try_age(&self) -> Result<i32, Error> {
        (**self).try_age()
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        (**self).try_email()
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        (**self).try_score()
    }
    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        (**self).try_status()
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        (**self).try_secondary_status()
    }
    fn try_has_name(&self) -> Result<bool, Error> {
        (**self).try_has_name()
    }
    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error> {
        (**self).try_address()
    }
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error> {
        (**self).try_scores()
    }
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error> {
        (**self).try_addresses()
    }
}

impl<T: PersonTry> PersonTry for Rc<T> {
    type Address<'a> = T::Address<'a>
    where
        Self: 'a;
    type Scores<'a> = T::Scores<'a>
    where
        Self: 'a;
    type AddressItem<'a> = T::AddressItem<'a>
    where
        Self: 'a;
    type Addresses<'a> = T::Addresses<'a>
    where
        Self: 'a;

    fn try_name(&self) -> Result<&str, Error> {
        (**self).try_name()
    }
    fn try_age(&self) -> Result<i32, Error> {
        (**self).try_age()
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        (**self).try_email()
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        (**self).try_score()
    }
    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        (**self).try_status()
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        (**self).try_secondary_status()
    }
    fn try_has_name(&self) -> Result<bool, Error> {
        (**self).try_has_name()
    }
    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error> {
        (**self).try_address()
    }
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error> {
        (**self).try_scores()
    }
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error> {
        (**self).try_addresses()
    }
}

// Blanket implementation for Option (fallible)
impl<T: PersonTry> PersonTry for Option<T> {
    type Address<'a> = T::Address<'a>
    where
        Self: 'a;
    type Scores<'a> = OptionRepeated<T::Scores<'a>>
    where
        Self: 'a;
    type AddressItem<'a> = T::AddressItem<'a>
    where
        Self: 'a;
    type Addresses<'a> = OptionRepeated<T::Addresses<'a>>
    where
        Self: 'a;

    fn try_name(&self) -> Result<&str, Error> {
        match self.as_ref() {
            Some(v) => v.try_name(),
            None => Ok(""),
        }
    }
    fn try_age(&self) -> Result<i32, Error> {
        match self.as_ref() {
            Some(v) => v.try_age(),
            None => Ok(0),
        }
    }
    fn try_email(&self) -> Result<Option<&str>, Error> {
        match self.as_ref() {
            Some(v) => v.try_email(),
            None => Ok(None),
        }
    }
    fn try_score(&self) -> Result<Option<i32>, Error> {
        match self.as_ref() {
            Some(v) => v.try_score(),
            None => Ok(None),
        }
    }

    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        match self.as_ref() {
            Some(v) => v.try_status(),
            None => Ok(Status::from_wire(0)),
        }
    }
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        match self.as_ref() {
            Some(v) => v.try_secondary_status(),
            None => Ok(Ok(None)),
        }
    }

    fn try_has_name(&self) -> Result<bool, Error> {
        match self.as_ref() {
            Some(v) => v.try_has_name(),
            None => Ok(false),
        }
    }

    fn try_address(&self) -> Result<Option<Self::Address<'_>>, Error> {
        match self.as_ref() {
            Some(v) => v.try_address(),
            None => Ok(None),
        }
    }

    fn try_scores(&self) -> Result<Self::Scores<'_>, Error> {
        match self.as_ref() {
            Some(v) => Ok(OptionRepeated(Some(v.try_scores()?))),
            None => Ok(OptionRepeated(None)),
        }
    }

    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error> {
        match self.as_ref() {
            Some(v) => Ok(OptionRepeated(Some(v.try_addresses()?))),
            None => Ok(OptionRepeated(None)),
        }
    }
}

/// Flexible view trait for Person message (not dyn-compatible).
///
/// This is the non-object-safe “ergonomic extension” layer (uses `impl Trait` returns).
/// Since any infallible getter can be lifted into a fallible one by returning `Ok(...)`,
/// `Person` is modeled as a refinement of `PersonTry`.
///
/// Code generation note: This trait MUST NOT reference any implementation struct names (e.g., PersonImpl, AddressImpl).
/// Use only trait names and `impl Trait` syntax to maintain abstraction.
pub trait Person: PersonTry {
    type AddressView<'a>: Address + 'a
    where
        Self: 'a;
    type ScoresView<'a>: Repeated<'a, Item = i32> + 'a
    where
        Self: 'a;
    type AddressItemView<'a>: Address + 'a
    where
        Self: 'a;
    type AddressesView<'a>: Repeated<'a, Item = Self::AddressItemView<'a>> + 'a
    where
        Self: 'a;

    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> Option<&str>;
    fn score(&self) -> Option<i32>;

    // Enum field getters
    fn status(&self) -> Result<Status, i32>;
    fn secondary_status(&self) -> Result<Option<Status>, i32>;

    // Presence checks
    fn has_name(&self) -> bool;

    // Message/repeated getters
    fn address(&self) -> Self::AddressView<'_>;
    fn scores(&self) -> Self::ScoresView<'_>;
    fn addresses(&self) -> Self::AddressesView<'_>;
}

// Blanket implementations (infallible)
impl<T: Person> Person for &T {
    type AddressView<'a> = T::AddressView<'a>
    where
        Self: 'a;
    type ScoresView<'a> = T::ScoresView<'a>
    where
        Self: 'a;
    type AddressItemView<'a> = T::AddressItemView<'a>
    where
        Self: 'a;
    type AddressesView<'a> = T::AddressesView<'a>
    where
        Self: 'a;

    fn name(&self) -> &str {
        (*self).name()
    }
    fn age(&self) -> i32 {
        (*self).age()
    }
    fn email(&self) -> Option<&str> {
        (*self).email()
    }
    fn score(&self) -> Option<i32> {
        (*self).score()
    }
    fn status(&self) -> Result<Status, i32> {
        (*self).status()
    }
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        (*self).secondary_status()
    }
    fn has_name(&self) -> bool {
        (*self).has_name()
    }
    fn address(&self) -> Self::AddressView<'_> {
        (*self).address()
    }
    fn scores(&self) -> Self::ScoresView<'_> {
        (*self).scores()
    }
    fn addresses(&self) -> Self::AddressesView<'_> {
        (*self).addresses()
    }
}

impl<T: Person> Person for &mut T {
    type AddressView<'a> = T::AddressView<'a>
    where
        Self: 'a;
    type ScoresView<'a> = T::ScoresView<'a>
    where
        Self: 'a;
    type AddressItemView<'a> = T::AddressItemView<'a>
    where
        Self: 'a;
    type AddressesView<'a> = T::AddressesView<'a>
    where
        Self: 'a;

    fn name(&self) -> &str {
        (**self).name()
    }
    fn age(&self) -> i32 {
        (**self).age()
    }
    fn email(&self) -> Option<&str> {
        (**self).email()
    }
    fn score(&self) -> Option<i32> {
        (**self).score()
    }
    fn status(&self) -> Result<Status, i32> {
        (**self).status()
    }
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        (**self).secondary_status()
    }
    fn has_name(&self) -> bool {
        (**self).has_name()
    }
    fn address(&self) -> Self::AddressView<'_> {
        (**self).address()
    }
    fn scores(&self) -> Self::ScoresView<'_> {
        (**self).scores()
    }
    fn addresses(&self) -> Self::AddressesView<'_> {
        (**self).addresses()
    }
}

impl<T: Person> Person for Box<T> {
    type AddressView<'a> = T::AddressView<'a>
    where
        Self: 'a;
    type ScoresView<'a> = T::ScoresView<'a>
    where
        Self: 'a;
    type AddressItemView<'a> = T::AddressItemView<'a>
    where
        Self: 'a;
    type AddressesView<'a> = T::AddressesView<'a>
    where
        Self: 'a;

    fn name(&self) -> &str {
        (**self).name()
    }
    fn age(&self) -> i32 {
        (**self).age()
    }
    fn email(&self) -> Option<&str> {
        (**self).email()
    }
    fn score(&self) -> Option<i32> {
        (**self).score()
    }
    fn status(&self) -> Result<Status, i32> {
        (**self).status()
    }
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        (**self).secondary_status()
    }
    fn has_name(&self) -> bool {
        (**self).has_name()
    }
    fn address(&self) -> Self::AddressView<'_> {
        (**self).address()
    }
    fn scores(&self) -> Self::ScoresView<'_> {
        (**self).scores()
    }
    fn addresses(&self) -> Self::AddressesView<'_> {
        (**self).addresses()
    }
}

impl<T: Person> Person for Rc<T> {
    type AddressView<'a> = T::AddressView<'a>
    where
        Self: 'a;
    type ScoresView<'a> = T::ScoresView<'a>
    where
        Self: 'a;
    type AddressItemView<'a> = T::AddressItemView<'a>
    where
        Self: 'a;
    type AddressesView<'a> = T::AddressesView<'a>
    where
        Self: 'a;

    fn name(&self) -> &str {
        (**self).name()
    }
    fn age(&self) -> i32 {
        (**self).age()
    }
    fn email(&self) -> Option<&str> {
        (**self).email()
    }
    fn score(&self) -> Option<i32> {
        (**self).score()
    }
    fn status(&self) -> Result<Status, i32> {
        (**self).status()
    }
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        (**self).secondary_status()
    }
    fn has_name(&self) -> bool {
        (**self).has_name()
    }
    fn address(&self) -> Self::AddressView<'_> {
        (**self).address()
    }
    fn scores(&self) -> Self::ScoresView<'_> {
        (**self).scores()
    }
    fn addresses(&self) -> Self::AddressesView<'_> {
        (**self).addresses()
    }
}

// Blanket implementation for Option (infallible)
impl<T: Person> Person for Option<T> {
    type AddressView<'a> = Option<T::AddressView<'a>>
    where
        Self: 'a;
    type ScoresView<'a> = OptionRepeated<T::ScoresView<'a>>
    where
        Self: 'a;
    type AddressItemView<'a> = T::AddressItemView<'a>
    where
        Self: 'a;
    type AddressesView<'a> = OptionRepeated<T::AddressesView<'a>>
    where
        Self: 'a;

    fn name(&self) -> &str {
        self.as_ref().map(|v| v.name()).unwrap_or("")
    }
    fn age(&self) -> i32 {
        self.as_ref().map(|v| v.age()).unwrap_or(0)
    }
    fn email(&self) -> Option<&str> {
        self.as_ref().and_then(|v| v.email())
    }
    fn score(&self) -> Option<i32> {
        self.as_ref().and_then(|v| v.score())
    }
    fn status(&self) -> Result<Status, i32> {
        match self.as_ref() {
            Some(v) => v.status(),
            None => Status::from_wire(0),
        }
    }
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        match self.as_ref() {
            Some(v) => v.secondary_status(),
            None => Ok(None),
        }
    }
    fn has_name(&self) -> bool {
        self.as_ref().map(|v| v.has_name()).unwrap_or(false)
    }
    fn address(&self) -> Self::AddressView<'_> {
        self.as_ref().map(|v| v.address())
    }
    fn scores(&self) -> Self::ScoresView<'_> {
        OptionRepeated(self.as_ref().map(|v| v.scores()))
    }
    fn addresses(&self) -> Self::AddressesView<'_> {
        OptionRepeated(self.as_ref().map(|v| v.addresses()))
    }
}

/// Flexible view fully mutable trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonMut: Person {
    type AddressMut<'a>: AddressMut + 'a
    where
        Self: 'a;
    type AddressPush<'a>: AddressMut + 'a
    where
        Self: 'a;

    // Setters / clears / repeated mutators
    fn set_name(&mut self, v: &str);
    fn clear_name(&mut self);
    fn clear_scores(&mut self);
    fn clear_addresses(&mut self);
    fn push_score(&mut self, v: i32);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> Self::AddressMut<'_>;
    fn push_address(&mut self) -> Self::AddressPush<'_>;
}

// Blanket implementations (mutable)
impl<T: PersonMut> PersonMut for &mut T {
    type AddressMut<'a> = T::AddressMut<'a>
    where
        Self: 'a;
    type AddressPush<'a> = T::AddressPush<'a>
    where
        Self: 'a;

    fn set_name(&mut self, v: &str) {
        (**self).set_name(v)
    }
    fn clear_name(&mut self) {
        (**self).clear_name()
    }
    fn clear_scores(&mut self) {
        (**self).clear_scores()
    }
    fn clear_addresses(&mut self) {
        (**self).clear_addresses()
    }
    fn push_score(&mut self, v: i32) {
        (**self).push_score(v)
    }
    fn address_mut(&mut self) -> Self::AddressMut<'_> {
        (**self).address_mut()
    }
    fn push_address(&mut self) -> Self::AddressPush<'_> {
        (**self).push_address()
    }
}

impl<T: PersonMut> PersonMut for Box<T> {
    type AddressMut<'a> = T::AddressMut<'a>
    where
        Self: 'a;
    type AddressPush<'a> = T::AddressPush<'a>
    where
        Self: 'a;

    fn set_name(&mut self, v: &str) {
        (**self).set_name(v)
    }
    fn clear_name(&mut self) {
        (**self).clear_name()
    }
    fn clear_scores(&mut self) {
        (**self).clear_scores()
    }
    fn clear_addresses(&mut self) {
        (**self).clear_addresses()
    }
    fn push_score(&mut self, v: i32) {
        (**self).push_score(v)
    }
    fn address_mut(&mut self) -> Self::AddressMut<'_> {
        (**self).address_mut()
    }
    fn push_address(&mut self) -> Self::AddressPush<'_> {
        (**self).push_address()
    }
}

// Blanket implementation for Option (mutable)
impl<T: PersonMut> PersonMut for Option<T> {
    type AddressMut<'a> = Option<T::AddressMut<'a>>
    where
        Self: 'a;
    type AddressPush<'a> = Option<T::AddressPush<'a>>
    where
        Self: 'a;

    fn set_name(&mut self, v: &str) {
        if let Some(t) = self.as_mut() {
            t.set_name(v);
        }
    }
    fn clear_name(&mut self) {
        if let Some(t) = self.as_mut() {
            t.clear_name();
        }
    }
    fn clear_scores(&mut self) {
        if let Some(t) = self.as_mut() {
            t.clear_scores();
        }
    }
    fn clear_addresses(&mut self) {
        if let Some(t) = self.as_mut() {
            t.clear_addresses();
        }
    }
    fn push_score(&mut self, v: i32) {
        if let Some(t) = self.as_mut() {
            t.push_score(v);
        }
    }

    fn address_mut(&mut self) -> Self::AddressMut<'_> {
        self.as_mut().map(|t| t.address_mut())
    }

    fn push_address(&mut self) -> Self::AddressPush<'_> {
        self.as_mut().map(|t| t.push_address())
    }
}
