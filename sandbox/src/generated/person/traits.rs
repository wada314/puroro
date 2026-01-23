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

use super::{Address, AddressMut, DynAddress, DynAddressMut, Status};
use ::puroro::error::Error;
use ::puroro::repeated::Repeated;
use ::puroro::view::ViewCow;

/// Dyn-compatible immutable trait for Person message, fallible variant.
///
/// All methods return `Result<.., Error>` to allow lazy parsing/validation to fail.
/// Enum fields keep the existing `Result<Enum, i32>` semantics for unknown values; this is wrapped
/// in an outer `Result` for operational failures.
pub trait DynPersonTry {
    // Getters
    fn try_name(&self) -> Result<&str, Error>;
    fn try_age(&self) -> Result<i32, Error>;
    fn try_email(&self) -> Result<Option<&str>, Error>;
    fn try_score(&self) -> Result<Option<i32>, Error>;

    // Enum field getters
    fn try_status(&self) -> Result<Result<Status, i32>, Error>;
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error>;

    // Message field getters
    fn try_address(&self) -> Result<Option<ViewCow<'_, dyn DynAddress>>, Error>;

    // Repeated field getters
    fn try_scores(&self) -> Result<ViewCow<'_, dyn Repeated<'_, Item = i32>>, Error>;
    fn try_addresses<'a: 'b, 'b>(
        &'a self,
    ) -> Result<ViewCow<'a, dyn Repeated<'a, Item = ViewCow<'b, dyn DynAddress>> + 'b>, Error>;

    // Presence checks
    fn try_has_name(&self) -> Result<bool, Error>;
}

/// Dyn-compatible immutable trait for Person message.
///
/// This is the object-safe (dyn-compatible) core API.
/// Since any infallible getter can be lifted into a fallible one by returning `Ok(...)`,
/// `DynPerson` is modeled as a refinement of `DynPersonTry`.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here;
/// use `ViewCow` or other dyn-compatible return types for message fields.
pub trait DynPerson: DynPersonTry {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> Option<&str>;
    fn score(&self) -> Option<i32>;

    // Enum field getters
    fn status(&self) -> Result<Status, i32>;
    fn secondary_status(&self) -> Result<Option<Status>, i32>;

    // Message field getters
    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>>; // Message fields always return Option, even for ImplicitOptional

    // Repeated field getters
    fn scores(&self) -> ViewCow<'_, dyn Repeated<'_, Item = i32>>;
    fn addresses<'a: 'b, 'b>(
        &'a self,
    ) -> ViewCow<'a, dyn Repeated<'a, Item = ViewCow<'b, dyn DynAddress>> + 'b>;

    // Presence checks (for optional semantics) - sample: has_name (others follow same pattern)
    fn has_name(&self) -> bool;
}

/// Flexible view trait for Person message (not dyn-compatible), fallible variant.
///
/// This trait is intended for implementations where field access can fail (e.g. lazy parsing,
/// validation, IO-backed sources). Methods use the `try_` prefix to avoid collisions with the
/// infallible trait.
pub trait PersonTry: DynPersonTry {
    // Methods that delegate to DynPersonTry (default implementations)
    #[inline]
    fn try_name(&self) -> Result<&str, Error> {
        DynPersonTry::try_name(self)
    }
    #[inline]
    fn try_age(&self) -> Result<i32, Error> {
        DynPersonTry::try_age(self)
    }
    #[inline]
    fn try_email(&self) -> Result<Option<&str>, Error> {
        DynPersonTry::try_email(self)
    }
    #[inline]
    fn try_score(&self) -> Result<Option<i32>, Error> {
        DynPersonTry::try_score(self)
    }
    #[inline]
    fn try_status(&self) -> Result<Result<Status, i32>, Error> {
        DynPersonTry::try_status(self)
    }
    #[inline]
    fn try_secondary_status(&self) -> Result<Result<Option<Status>, i32>, Error> {
        DynPersonTry::try_secondary_status(self)
    }

    // Presence checks (delegating to DynPersonTry)
    #[inline]
    fn try_has_name(&self) -> Result<bool, Error> {
        DynPersonTry::try_has_name(self)
    }

    // Methods with custom implementations (must be implemented)
    fn try_address(&self) -> Result<impl Address + use<'_, Self>, Error>;

    // Repeated field getters (must be implemented)
    fn try_scores(&self) -> Result<impl Repeated<'_, Item = i32> + use<'_, Self>, Error>;

    // The returned item type must implement `Address` and must not expose a concrete implementation type.
    fn try_addresses(
        &self,
    ) -> Result<impl Repeated<'_, Item = impl Address + '_> + use<'_, Self>, Error>;
}

/// Flexible view trait for Person message (not dyn-compatible).
///
/// This is the non-object-safe “ergonomic extension” layer (uses `impl Trait` returns).
/// Since any infallible getter can be lifted into a fallible one by returning `Ok(...)`,
/// `Person` is modeled as a refinement of `PersonTry`.
///
/// Code generation note: This trait MUST NOT reference any implementation struct names (e.g., PersonImpl, AddressImpl).
/// Use only trait names and `impl Trait` syntax to maintain abstraction.
pub trait Person: PersonTry + DynPerson {
    // Methods that delegate to DynPerson (default implementations)
    #[inline]
    fn name(&self) -> &str {
        DynPerson::name(self)
    }
    #[inline]
    fn age(&self) -> i32 {
        DynPerson::age(self)
    }
    #[inline]
    fn email(&self) -> Option<&str> {
        DynPerson::email(self)
    }
    #[inline]
    fn score(&self) -> Option<i32> {
        DynPerson::score(self)
    }
    #[inline]
    fn status(&self) -> Result<Status, i32> {
        DynPerson::status(self)
    }
    #[inline]
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        DynPerson::secondary_status(self)
    }

    // Presence checks (delegating to DynPerson)
    #[inline]
    fn has_name(&self) -> bool {
        DynPerson::has_name(self)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl Address`, not a concrete struct type
    fn address(&self) -> impl Address + use<'_, Self>;

    // Repeated field getters (must be implemented)
    // NOTE: Must return `impl Repeated<'_>`, not a dyn type
    fn scores(&self) -> impl Repeated<'_, Item = i32> + use<'_, Self>;

    // NOTE: Must return `impl Repeated<'_>`, not a dyn type
    // The returned item type must implement `Address` and must not expose a concrete implementation type.
    fn addresses(&self) -> impl Repeated<'_, Item = impl Address + '_> + use<'_, Self>;
}

/// Flexible view fully mutable trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonMut: Person + DynPersonMut {
    // Setters (delegating to DynPersonMut)
    #[inline]
    fn set_name(&mut self, v: &str) {
        DynPersonMut::set_name(self, v)
    }

    // Clear methods (delegating to DynPersonMut)
    #[inline]
    fn clear_name(&mut self) {
        DynPersonMut::clear_name(self)
    }
    #[inline]
    fn clear_scores(&mut self) {
        DynPersonMut::clear_scores(self)
    }
    #[inline]
    fn clear_addresses(&mut self) {
        DynPersonMut::clear_addresses(self)
    }

    // Repeated mutators (delegating to DynPersonMut)
    #[inline]
    fn push_score(&mut self, v: i32) {
        DynPersonMut::push_score(self, v)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl AddressMut`, not a concrete struct type
    fn address_mut(&mut self) -> impl AddressMut + use<'_, Self>;

    // Repeated mutators that return mutable references (requires full mutation ability)
    // NOTE: Must return `impl AddressMut`, not a concrete struct type
    fn push_address(&mut self) -> impl AddressMut + use<'_, Self>;
}

/// Dyn-compatible fully mutable trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynPersonMut: DynPerson {
    // Setters - sample: set_name (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str);

    // Clear methods - sample: clear_name (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self);
    fn clear_scores(&mut self);
    fn clear_addresses(&mut self);

    // Repeated mutators
    fn push_score(&mut self, v: i32);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> &mut dyn DynAddressMut;

    /// Appends a new default address and returns a mutable dyn view to build it.
    /// Note: This method requires full mutation ability, so it's only available in DynPersonMut.
    fn push_address(&mut self) -> &mut dyn DynAddressMut;
}
