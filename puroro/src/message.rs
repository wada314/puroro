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

use crate::Result;
use ::std::io::{Result as IoResult, Write};

/// Owned protobuf message. Implemented by generated `Foo` structs.
///
/// Getter methods live on [`MessageView`]; this trait covers construction and
/// merging from a byte stream. See the crate-level documentation for usage.
pub trait Message: Sized {
    /// Immutable view type of this message, typically `FooView`.
    type ViewType: MessageView;
    /// Decode a new message from a fallible byte iterator (e.g. `std::io::Read::bytes()`).
    fn from_bytes_iter<I: Iterator<Item = IoResult<u8>>>(iter: I) -> Result<Self>;
    /// Merge another encoded message into `self`, overwriting / appending fields.
    fn merge_from_bytes_iter<I: Iterator<Item = IoResult<u8>>>(&mut self, iter: I) -> Result<()>;
}

/// Immutable view of a protobuf message. Implemented by generated `FooView` structs.
///
/// Owned messages `Deref` to this view, so getter methods and [`MessageView::to_bytes`]
/// can be called on either type.
pub trait MessageView {
    /// Owned message type corresponding to this view, typically `Foo`.
    type MessageType: Message;
    /// Encode this message into a [`std::io::Write`] sink.
    fn to_bytes<W: Write>(&self, out: &mut W) -> Result<()>;
}
