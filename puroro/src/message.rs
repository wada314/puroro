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
use ::futures::io::{AsyncRead, AsyncWrite};
use ::std::io::Read;

/// Protobuf message, which can be serialized, deserialized, and field accessible.
pub trait MessageLite {
    fn merge_from_read<R: Read>(&mut self, read: R) -> Result<()>;
}

/// [`MessageLite`] + descriptors and reflections.
pub trait Message: MessageLite {}
pub trait ReadableMessage: MessageLite {}
pub trait AppendableMessage: MessageLite {}
pub trait MutableMessage: AppendableMessage {}
pub trait AsyncReadableMessage: ReadableMessage {}
pub trait AsyncDeserializableMessage: ReadableMessage {
    fn async_deser(&mut self, read: impl AsyncRead) -> impl '_ + AsyncReadableMessage;
}
pub trait AsyncSerializableMessage {
    fn async_ser(&self, write: impl AsyncWrite) -> impl '_ + AppendableMessage;
}
