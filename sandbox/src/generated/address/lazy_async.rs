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

use super::traits::AddressAsync;
use ::bytes::Bytes;
use ::futures_io::AsyncRead;
use ::puroro::error::Error;
use ::puroro::lazy_async::{AsyncMessageParserStateRef, BytesReader};
use ::puroro::protobuf_core::{Field, FieldValue};
use ::std::boxed::Box as StdBox;
use ::std::cell::{Cell, OnceCell, RefCell};
use ::std::future::Future;
use ::std::pin::Pin;
use ::std::rc::{Rc, Weak};

/// Parse-intermediate state for Address: holds values as the parser updates them (interior mutability).
struct AddressParseIntermediate {
    street: RefCell<Option<Bytes>>,
    city: RefCell<Option<Bytes>>,
    zip_code: Cell<i32>,
}

/// Finalized, strictly immutable view of an Address (built once parsing is complete for the message).
struct AddressFinalized {
    street: String,
    city: String,
    #[allow(dead_code)] // filled for consistency; zip_code getter reads from intermediate
    zip_code: i32,
}

impl AddressFinalized {
    fn street(&self) -> &str {
        &self.street
    }
    fn city(&self) -> &str {
        &self.city
    }
}

/// Async/streaming lazy implementation of `Address`.
///
/// This implementation uses `puroro::lazy_async::AsyncMessageParserStateRef` and async
/// getters. It supports random-access getters by caching decoded values and/or raw bytes.
pub struct AddressLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    parser_state: AsyncMessageParserStateRef<R>,
    intermediate: AddressParseIntermediate,
    finalized: OnceCell<AddressFinalized>,
}

impl AddressLazyAsyncImpl<BytesReader> {
    /// Create an in-memory async-lazy Address from a length-delimited payload.
    pub fn from_bytes(bytes: Bytes) -> Rc<Self> {
        Self::new(BytesReader::new(bytes), None)
    }
}

impl<R> AddressLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    /// Create a new async-lazy Address from an async reader.
    ///
    /// If `limit` is `Some(n)`, the reader is treated as having exactly `n` bytes for this message.
    pub fn new(reader: R, limit: Option<usize>) -> Rc<Self> {
        Rc::new_cyclic(move |weak: &Weak<Self>| {
            let message_body_weak = weak.clone();

            let callback = move |field: Field<Bytes>| -> Result<(), Error> {
                if let Some(message_body) = message_body_weak.upgrade() {
                    message_body.update_field(field)?;
                }
                Ok(())
            };

            let parser_state = AsyncMessageParserStateRef::create(reader, limit, callback);

            Self {
                parser_state,
                intermediate: AddressParseIntermediate {
                    street: RefCell::new(None),
                    city: RefCell::new(None),
                    zip_code: Cell::new(0),
                },
                finalized: OnceCell::new(),
            }
        })
    }

    fn update_field(&self, field: Field<Bytes>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match (field_num, field.value) {
            (1, FieldValue::Len(bytes)) => {
                *self.intermediate.street.borrow_mut() = Some(bytes);
            }
            (2, FieldValue::Len(bytes)) => {
                *self.intermediate.city.borrow_mut() = Some(bytes);
            }
            (3, FieldValue::Varint(varint)) => {
                self.intermediate.zip_code.set(varint.try_to_int32()?);
            }
            _ => {}
        }
        Ok(())
    }

    /// Build finalized view from intermediate (decode bytes to UTF-8 strings). Called after parsing.
    fn build_finalized(intermediate: &AddressParseIntermediate) -> Result<AddressFinalized, Error> {
        let street = intermediate
            .street
            .borrow()
            .clone()
            .unwrap_or_else(Bytes::new);
        let city = intermediate.city.borrow().clone().unwrap_or_else(Bytes::new);
        let street_str = String::from_utf8(street.to_vec()).map_err(Error::from)?;
        let city_str = String::from_utf8(city.to_vec()).map_err(Error::from)?;
        Ok(AddressFinalized {
            street: street_str,
            city: city_str,
            zip_code: intermediate.zip_code.get(),
        })
    }

    /// Async getter for street bytes (internal; used when building finalized).
    pub async fn street_bytes(self: &Rc<Self>) -> Result<Bytes, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self
            .intermediate
            .street
            .borrow()
            .clone()
            .unwrap_or_else(Bytes::new))
    }

    /// Async getter for city bytes (internal; used when building finalized).
    pub async fn city_bytes(self: &Rc<Self>) -> Result<Bytes, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self
            .intermediate
            .city
            .borrow()
            .clone()
            .unwrap_or_else(Bytes::new))
    }

    /// Async getter for zip code.
    pub async fn zip_code(self: &Rc<Self>) -> Result<i32, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self.intermediate.zip_code.get())
    }
}

impl<R> AddressAsync for AddressLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    fn street(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<&str, Error>> + '_>> {
        let this = self;
        Box::pin(async move {
            this.parser_state
                .parse_until_with_callback(|| false)
                .await?;
            if this.finalized.get().is_none() {
                let f = AddressLazyAsyncImpl::<R>::build_finalized(&this.intermediate)?;
                let _ = this.finalized.set(f);
            }
            Ok(this.finalized.get().unwrap().street())
        })
    }

    fn city(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<&str, Error>> + '_>> {
        let this = self;
        Box::pin(async move {
            this.parser_state
                .parse_until_with_callback(|| false)
                .await?;
            if this.finalized.get().is_none() {
                let f = AddressLazyAsyncImpl::<R>::build_finalized(&this.intermediate)?;
                let _ = this.finalized.set(f);
            }
            Ok(this.finalized.get().unwrap().city())
        })
    }

    fn zip_code(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<i32, Error>> + '_>> {
        let this = self;
        Box::pin(async move { this.zip_code().await })
    }
}
