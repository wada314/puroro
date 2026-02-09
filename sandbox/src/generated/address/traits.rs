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

use ::allocator_api2::boxed::Box;
use ::puroro::error::Error;
use ::std::boxed::Box as StdBox;
use ::std::future::Future;
use ::std::pin::Pin;
use ::std::rc::Rc;

/// Flexible view trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait Address: AddressTry {
    fn street(&self) -> &str;
    fn city(&self) -> &str;
    fn zip_code(&self) -> i32;
}

// Blanket implementation for references (infallible)
impl<T: Address> Address for &T {
    fn street(&self) -> &str {
        (*self).street()
    }
    fn city(&self) -> &str {
        (*self).city()
    }
    fn zip_code(&self) -> i32 {
        (*self).zip_code()
    }
}

impl<T: Address> Address for &mut T {
    fn street(&self) -> &str {
        (**self).street()
    }
    fn city(&self) -> &str {
        (**self).city()
    }
    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

// Blanket implementation for Option (infallible)
impl<T: Address> Address for Option<T> {
    fn street(&self) -> &str {
        self.as_ref().map(|v| v.street()).unwrap_or("")
    }

    fn city(&self) -> &str {
        self.as_ref().map(|v| v.city()).unwrap_or("")
    }

    fn zip_code(&self) -> i32 {
        self.as_ref().map(|v| v.zip_code()).unwrap_or(0)
    }
}

// Blanket implementation for Box
impl<T: Address> Address for Box<T> {
    fn street(&self) -> &str {
        (**self).street()
    }
    fn city(&self) -> &str {
        (**self).city()
    }
    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

/// Flexible view fully mutable trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressMut: Address {
    fn set_street(&mut self, v: &str);
    fn clear_street(&mut self);
}

/// Async view trait for Address message (streaming/lazy async implementations).
///
/// Implemented by types that read fields asynchronously (e.g. from an async stream).
/// Uses `self: &Rc<Self>` so that the future can hold a shared reference to the message.
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressAsync {
    /// Async getter for street (decoded as UTF-8 string).
    fn street(self: &Rc<Self>) -> Pin<StdBox<dyn Future<Output = Result<String, Error>> + '_>>;
    /// Async getter for city (decoded as UTF-8 string).
    fn city(self: &Rc<Self>) -> Pin<StdBox<dyn Future<Output = Result<String, Error>> + '_>>;
    /// Async getter for zip_code.
    fn zip_code(self: &Rc<Self>) -> Pin<StdBox<dyn Future<Output = Result<i32, Error>> + '_>>;
}

/// Flexible view trait for Address message (not dyn-compatible), fallible variant.
///
/// This trait is intended for implementations where field access can fail (e.g. lazy parsing,
/// validation, IO-backed sources). Methods use the `try_` prefix to avoid collisions with the
/// infallible trait.
pub trait AddressTry {
    fn try_street(&self) -> Result<&str, Error>;
    fn try_city(&self) -> Result<&str, Error>;
    fn try_zip_code(&self) -> Result<i32, Error>;
}

// Blanket implementation for references (fallible)
impl<T: AddressTry> AddressTry for &T {
    fn try_street(&self) -> Result<&str, Error> {
        (*self).try_street()
    }
    fn try_city(&self) -> Result<&str, Error> {
        (*self).try_city()
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        (*self).try_zip_code()
    }
}

impl<T: AddressTry> AddressTry for &mut T {
    fn try_street(&self) -> Result<&str, Error> {
        (**self).try_street()
    }
    fn try_city(&self) -> Result<&str, Error> {
        (**self).try_city()
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        (**self).try_zip_code()
    }
}

// Blanket implementation for Box (fallible)
impl<T: AddressTry> AddressTry for Box<T> {
    fn try_street(&self) -> Result<&str, Error> {
        (**self).try_street()
    }
    fn try_city(&self) -> Result<&str, Error> {
        (**self).try_city()
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        (**self).try_zip_code()
    }
}

// Blanket implementation for Rc (fallible)
impl<T: AddressTry> AddressTry for ::std::rc::Rc<T> {
    fn try_street(&self) -> Result<&str, Error> {
        (**self).try_street()
    }
    fn try_city(&self) -> Result<&str, Error> {
        (**self).try_city()
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        (**self).try_zip_code()
    }
}

// Blanket implementation for Option (fallible)
impl<T: AddressTry> AddressTry for Option<T> {
    fn try_street(&self) -> Result<&str, Error> {
        match self.as_ref() {
            Some(v) => v.try_street(),
            None => Ok(""),
        }
    }
    fn try_city(&self) -> Result<&str, Error> {
        match self.as_ref() {
            Some(v) => v.try_city(),
            None => Ok(""),
        }
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        match self.as_ref() {
            Some(v) => v.try_zip_code(),
            None => Ok(0),
        }
    }
}

// Blanket implementation for references (mutable)
impl<T: AddressMut> AddressMut for &mut T {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Box (mutable)
impl<T: AddressMut> AddressMut for Box<T> {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Option (mutable)
impl<T: AddressMut> AddressMut for Option<T> {
    fn set_street(&mut self, v: &str) {
        if let Some(t) = self {
            t.set_street(v);
        }
    }
    fn clear_street(&mut self) {
        if let Some(t) = self {
            t.clear_street();
        }
    }
}
