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

use super::super::address::AddressLazyAsyncImpl;
use ::allocator_extras::Global;
use ::bytes::{Bytes, BytesMut};
use ::futures_io::AsyncRead;
use ::once_list2::OnceListWithTailLen as OnceList;
use ::puroro::error::Error;
use ::puroro::lazy_async::{AsyncMessageParserStateRef, BytesReader};
use ::puroro::protobuf_core::{Field, FieldValue};
use ::puroro::repeated_lazy_async::LazyRepeatedAsync;
use ::std::cell::{Cell, OnceCell, RefCell};
use ::std::rc::{Rc, Weak};
use ::std::task::{Context, Poll};

/// Async/streaming lazy implementation of Person message that deserializes fields on-demand.
///
/// Fields handled:
/// - `age` (field 2)
/// - `scores` (field 10)
/// - `address` (field 6)
/// - `addresses` (field 9)
pub struct PersonLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    parser_state: AsyncMessageParserStateRef<R>,

    age: Cell<i32>,
    scores: OnceList<i32, Global>,

    // Scalar message field payload (concatenated across multiple occurrences).
    address_payload: RefCell<Option<BytesMut>>,
    address: OnceCell<Rc<AddressLazyAsyncImpl<BytesReader>>>,

    // Repeated message field (each occurrence is a separate embedded message).
    addresses: OnceList<Rc<AddressLazyAsyncImpl<BytesReader>>, Global>,
}

impl<R> PersonLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
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
                age: Cell::new(0),
                scores: OnceList::new_in(Global),
                address_payload: RefCell::new(None),
                address: OnceCell::new(),
                addresses: OnceList::new_in(Global),
            }
        })
    }

    fn update_field(&self, field: Field<Bytes>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match (field_num, field.value) {
            (2, FieldValue::Varint(varint)) => {
                self.age.set(varint.try_to_int32()?);
            }
            (10, FieldValue::Varint(varint)) => {
                self.scores.push(varint.try_to_int32()?);
            }
            (6, FieldValue::Len(bytes)) => {
                // Scalar message field: concatenate payloads across occurrences.
                let mut slot = self.address_payload.borrow_mut();
                match slot.as_mut() {
                    Some(buf) => buf.extend_from_slice(bytes.as_ref()),
                    None => {
                        let mut buf = BytesMut::with_capacity(bytes.len());
                        buf.extend_from_slice(bytes.as_ref());
                        *slot = Some(buf);
                    }
                }
            }
            (9, FieldValue::Len(bytes)) => {
                let child = AddressLazyAsyncImpl::from_bytes(bytes);
                self.addresses.push(child);
            }
            _ => {}
        }
        Ok(())
    }

    fn poll_ensure_all_fields_parsed(&self, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        todo!("poll_parse_until_with_callback was removed; switch to async API or implement")
    }

    /// Poll getter for age (parses the whole message to ensure the last value wins).
    pub fn poll_age(&self, cx: &mut Context<'_>) -> Poll<Result<i32, Error>> {
        let _ = match self.poll_ensure_all_fields_parsed(cx) {
            Poll::Ready(r) => r?,
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(Ok(self.age.get()))
    }

    /// Repeated scores adapter (poll-driven).
    pub fn scores(&self) -> LazyRepeatedAsync<'_, i32, Global, R> {
        LazyRepeatedAsync::new(self.parser_state.clone(), &self.scores)
    }

    /// Repeated addresses adapter (poll-driven).
    pub fn addresses(&self) -> LazyRepeatedAsync<'_, Rc<AddressLazyAsyncImpl<BytesReader>>, Global, R> {
        LazyRepeatedAsync::new(self.parser_state.clone(), &self.addresses)
    }

    /// Poll getter for the scalar `address` field.
    ///
    /// This parses the whole message before returning, so that all occurrences have been
    /// concatenated into the payload.
    pub fn poll_address(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<Rc<AddressLazyAsyncImpl<BytesReader>>>, Error>> {
        let _ = match self.poll_ensure_all_fields_parsed(cx) {
            Poll::Ready(r) => r?,
            Poll::Pending => return Poll::Pending,
        };

        if let Some(addr) = self.address.get() {
            return Poll::Ready(Ok(Some(addr.clone())));
        }

        let mut slot = self.address_payload.borrow_mut();
        let Some(buf) = slot.take() else {
            return Poll::Ready(Ok(None));
        };
        let bytes = buf.freeze();
        let child = AddressLazyAsyncImpl::from_bytes(bytes);
        let _ = self.address.set(child.clone());
        Poll::Ready(Ok(Some(child)))
    }
}

