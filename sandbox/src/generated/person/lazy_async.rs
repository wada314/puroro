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
use super::traits::PersonAsync;
use ::allocator_extras::Global;
use ::bytes::Bytes;
use ::futures_io::AsyncRead;
use ::once_list2::OnceListWithTailLen as OnceList;
use ::puroro::error::Error;
use ::puroro::lazy_async::{AsyncMessageParserStateRef, BytesReader, SegmentsReader};
use ::puroro::protobuf_core::{Field, FieldValue};
use ::puroro::repeated_lazy_async::LazyRepeatedAsync;
use ::std::boxed::Box as StdBox;
use ::std::cell::{Cell, OnceCell, RefCell};
use ::std::future::Future;
use ::std::pin::Pin;
use ::std::rc::{Rc, Weak};

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

    // Scalar message field (field 6). Segments are concatenated; reader is shared with Address.
    address_segments: RefCell<Option<SegmentsReader<R>>>,
    address: OnceCell<Rc<AddressLazyAsyncImpl<SegmentsReader<R>>>>,

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
                address_segments: RefCell::new(None),
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
                // Scalar message field: concatenate chunks; reader treats segments as one input.
                let mut slot = self.address_segments.borrow_mut();
                match slot.as_mut() {
                    Some(reader) => reader.append(bytes),
                    None => *slot = Some(SegmentsReader::new(bytes, Some(self.parser_state.clone()))),
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

    /// Async getter for age (parses the whole message to ensure the last value wins).
    ///
    /// Takes `&Rc<Self>` because the future needs to read from the message.
    pub async fn age(self: &Rc<Self>) -> Result<i32, Error> {
        self.parser_state.parse_until_with_callback(|| false).await?;
        Ok(self.age.get())
    }

    /// Repeated scores adapter (async).
    pub fn scores(&self) -> LazyRepeatedAsync<'_, i32, OnceList<i32, Global>, R> {
        LazyRepeatedAsync::new(self.parser_state.clone(), &self.scores)
    }

    /// Repeated addresses adapter (async).
    pub fn addresses(&self) -> LazyRepeatedAsync<'_, Rc<AddressLazyAsyncImpl<BytesReader>>, OnceList<Rc<AddressLazyAsyncImpl<BytesReader>>, Global>, R> {
        LazyRepeatedAsync::new(self.parser_state.clone(), &self.addresses)
    }

    /// Async getter for the scalar `address` field.
    ///
    /// Parses until at least the first occurrence of field 6 is read; then returns an
    /// Address that reads from the concatenated segment buffer. Later field 6 chunks
    /// are appended to that buffer and the Address sees them as one stream.
    ///
    /// Takes `&Rc<Self>` because the future needs to read from shared fields.
    pub async fn address(
        self: &Rc<Self>,
    ) -> Result<Option<Rc<AddressLazyAsyncImpl<SegmentsReader<R>>>>, Error> {
        self.parser_state
            .parse_until_with_callback(|| self.address_segments.borrow().as_ref().is_some())
            .await?;
        if let Some(addr) = self.address.get() {
            return Ok(Some(addr.clone()));
        }
        let reader = match self.address_segments.borrow().as_ref() {
            Some(seg) => seg.clone(),
            None => return Ok(None),
        };
        let child = AddressLazyAsyncImpl::new(reader, None);
        let _ = self.address.set(child.clone());
        Ok(Some(child))
    }
}

impl<R> PersonAsync for PersonLazyAsyncImpl<R>
where
    R: AsyncRead + Unpin + 'static,
{
    type AddressAsyncItem = AddressLazyAsyncImpl<SegmentsReader<R>>;

    fn age(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<i32, Error>> + '_>> {
        let this = self.clone();
        Box::pin(async move { this.age().await })
    }

    fn address(
        self: &Rc<Self>,
    ) -> Pin<StdBox<dyn Future<Output = Result<Option<Rc<Self::AddressAsyncItem>>, Error>> + '_>> {
        let this = self.clone();
        Box::pin(async move { this.address().await })
    }
}

