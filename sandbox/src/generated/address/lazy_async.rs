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

use ::bytes::Bytes;
use ::futures_io::AsyncRead;
use ::puroro::error::Error;
use ::puroro::lazy_async::{AsyncMessageParserStateRef, BytesReader};
use ::puroro::protobuf_core::{Field, FieldValue};
use ::std::cell::{Cell, RefCell};
use ::std::rc::{Rc, Weak};
use ::std::task::{Context, Poll};

/// Async/streaming lazy implementation of `Address`.
///
/// This implementation uses `puroro::lazy_async::AsyncMessageParserStateRef` and poll-based
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

            let field_filter = |n: u32| matches!(n, 1 | 2 | 3);

            let callback = move |field: Field<Bytes>| -> Result<(), Error> {
                if let Some(message_body) = message_body_weak.upgrade() {
                    message_body.update_field(field)?;
                }
                Ok(())
            };

            let parser_state =
                AsyncMessageParserStateRef::create(reader, limit, field_filter, callback);

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

    fn poll_ensure_all_fields_parsed(&self, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.parser_state
            .poll_parse_until_with_callback(cx, || false)
    }

    pub fn poll_street_bytes(&self, cx: &mut Context<'_>) -> Poll<Result<Bytes, Error>> {
        let _ = match self.poll_ensure_all_fields_parsed(cx) {
            Poll::Ready(r) => r?,
            Poll::Pending => return Poll::Pending,
        };
        let bytes = self.street.borrow().clone().unwrap_or_else(Bytes::new);
        Poll::Ready(Ok(bytes))
    }

    pub fn poll_city_bytes(&self, cx: &mut Context<'_>) -> Poll<Result<Bytes, Error>> {
        let _ = match self.poll_ensure_all_fields_parsed(cx) {
            Poll::Ready(r) => r?,
            Poll::Pending => return Poll::Pending,
        };
        let bytes = self.city.borrow().clone().unwrap_or_else(Bytes::new);
        Poll::Ready(Ok(bytes))
    }

    pub fn poll_zip_code(&self, cx: &mut Context<'_>) -> Poll<Result<i32, Error>> {
        let _ = match self.poll_ensure_all_fields_parsed(cx) {
            Poll::Ready(r) => r?,
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(Ok(self.zip_code.get()))
    }
}
