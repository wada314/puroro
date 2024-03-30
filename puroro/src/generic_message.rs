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
use crate::message::MessageLite;
use crate::string::String;
use ::std::alloc::Allocator;

pub struct GenericMessage<A: Allocator> {
    fields: Vec<Field<A>>,
}

pub struct Field<A: Allocator> {
    number: i32,
    name: String<A>,
    records: Vec<WireTypeAndPayload<A>, A>,
}

enum WireTypeAndPayload<A: Allocator> {
    Varint(Variant),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    LengthDelimited(Vec<u8, A>),
    // StartGroup,
    // EndGroup,
}
