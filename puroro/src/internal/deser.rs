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

use crate::internal::variant::Variant;
use crate::{ErrorKind, Result};

pub struct Record<T> {
    number: i32,
    payload: Payload<T>,
}
pub enum Payload<T> {
    Variant(Variant),
    I32([u8; 4]),
    I64([u8; 8]),
    Len(T),
}

pub trait DeseringMessage {
    fn finish(&mut self);
    fn deser_record(&mut self, record: Record<&[u8]>) -> Option<&mut dyn DeseringMessage>;
}
pub trait AsyncDeseringMessage {
    fn finish(&mut self);
}
