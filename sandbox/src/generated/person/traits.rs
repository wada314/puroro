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
use ::puroro::error::Error;
use ::puroro::repeated::Repeated;

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
    fn try_address(&self) -> Result<Self::Address<'_>, Error>;
    fn try_scores(&self) -> Result<Self::Scores<'_>, Error>;
    fn try_addresses(&self) -> Result<Self::Addresses<'_>, Error>;
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
    fn address(&self) -> impl Address + use<'_, Self>;
    fn scores(&self) -> impl Repeated<'_, Item = i32> + use<'_, Self>;
    fn addresses(&self) -> impl Repeated<'_, Item = impl Address + '_> + use<'_, Self>;
}

/// Flexible view fully mutable trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonMut: Person {
    // Setters / clears / repeated mutators
    fn set_name(&mut self, v: &str);
    fn clear_name(&mut self);
    fn clear_scores(&mut self);
    fn clear_addresses(&mut self);
    fn push_score(&mut self, v: i32);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> impl AddressMut + use<'_, Self>;
    fn push_address(&mut self) -> impl AddressMut + use<'_, Self>;
}
