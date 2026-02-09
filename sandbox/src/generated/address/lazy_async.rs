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
use ::std::cell::{Cell, RefCell};
use ::std::future::Future;
use ::std::pin::Pin;
use ::std::rc::{Rc, Weak};

/// Async/streaming lazy implementation of `Address`.
///
/// This implementation uses `puroro::lazy_async::AsyncMessageParserStateRef` and async
/// getters. It supports random-access getters by caching decoded values and/or raw bytes.
pub struct AddressLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    parser_state: AsyncMessageParserStateRef<R>,
    street: RefCell<Option<Bytes>>,
    city: RefCell<Option<Bytes>>,
    zip_code: Cell<i32>,
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
                street: RefCell::new(None),
                city: RefCell::new(None),
                zip_code: Cell::new(0),
            }
        })
    }

    fn update_field(&self, field: Field<Bytes>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match (field_num, field.value) {
            (1, FieldValue::Len(bytes)) => {
                *self.street.borrow_mut() = Some(bytes);
            }
            (2, FieldValue::Len(bytes)) => {
                *self.city.borrow_mut() = Some(bytes);
            }
            (3, FieldValue::Varint(varint)) => {
                self.zip_code.set(varint.try_to_int32()?);
            }
            _ => {}
        }
        Ok(())
    }

    /// Async getter for street bytes.
    ///
    /// Takes `&Rc<Self>` because the future needs to read from the message.
    pub async fn street_bytes(self: &Rc<Self>) -> Result<Bytes, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self.street.borrow().clone().unwrap_or_else(Bytes::new))
    }

    /// Async getter for city bytes.
    ///
    /// Takes `&Rc<Self>` because the future needs to read from the message.
    pub async fn city_bytes(self: &Rc<Self>) -> Result<Bytes, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self.city.borrow().clone().unwrap_or_else(Bytes::new))
    }

    /// Async getter for zip code.
    ///
    /// Takes `&Rc<Self>` because the future needs to read from the message.
    pub async fn zip_code(self: &Rc<Self>) -> Result<i32, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self.zip_code.get())
    }
}

impl<R> AddressAsync for AddressLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    fn street(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<String, Error>> + '_>> {
        let this = self.clone();
        Box::pin(async move {
            let bytes = this.street_bytes().await?;
            String::from_utf8(bytes.to_vec()).map_err(Error::from)
        })
    }

    fn city(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<String, Error>> + '_>> {
        let this = self.clone();
        Box::pin(async move {
            let bytes = this.city_bytes().await?;
            String::from_utf8(bytes.to_vec()).map_err(Error::from)
        })
    }

    fn zip_code(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<i32, Error>> + '_>> {
        let this = self.clone();
        Box::pin(async move { this.zip_code().await })
    }
}
