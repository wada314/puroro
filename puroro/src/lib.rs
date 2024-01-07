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

#![feature(allocator_api)]
#![feature(slice_as_chunks)]
#![feature(assert_matches)]

pub mod internal;

use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("io error")]
    IoError(#[from] ::std::io::Error),
    #[error("Deserializing invalid variant (too long).")]
    TooLongEncodedVariant,
    #[error("The decoded variant value is not convertible to .proto specified int type")]
    VariantValueTooLarge,
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub trait Message {}
pub trait AsyncDeserMessage {
    type Message: Message;
    async fn wait_deser(self) -> Self::Message;
}

pub struct DescriptorProto {
    pub name: String,                      // 1
    pub fields: Vec<FieldDescriptorProto>, // 2
}

pub struct FieldDescriptorProto {
    pub name: String, // 1
    pub number: i32,  // 3
}
