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

use ::futures::io::{AsyncRead, AsyncWrite};
use ::std::borrow::Cow;

pub trait MessageLite {}
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

pub trait Field {}

pub trait FieldDescriptor {
    fn name(&self) -> &str;
    fn number(&self) -> i32;
    fn label(&self) -> field_descriptor::Label;
}
pub mod field_descriptor {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Type {
        DOUBLE,
        FLOAT,
        INT64,
        UINT64,
        INT32,
        FIXED64,
        FIXED32,
        BOOL,
        STRING,
        GROUP,
        MESSAGE,
        BYTES,
        UINT32,
        ENUM,
        SFIXED32,
        SFIXED64,
        SINT32,
        SINT64,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Label {
        OPTIONAL,
        REQUIRED,
        REPEATED,
    }
}

pub trait EnumDescriptor {
    fn name(&self) -> &str;
    fn value(&self) -> i32;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDescriptorImpl<'a> {
    name: Cow<'a, str>,
    number: i32,
    label: field_descriptor::Label,
}
impl FieldDescriptor for FieldDescriptorImpl<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn number(&self) -> i32 {
        self.number
    }
    fn label(&self) -> field_descriptor::Label {
        self.label
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumDescriptorImpl<'a> {
    name: Cow<'a, str>,
    value: i32,
}
