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

#![allow(unused)]
use ::once_cell::unsync::OnceCell;

pub struct Oneof<'a> {
    name: &'a str,
    fields: OnceCell<Box<[OneofField<'a>]>>,
}
impl Oneof<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
}

pub struct OneofField<'a> {
    parent: &'a Oneof<'a>,
    name: &'a str,
}
impl OneofField<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
}

enum FieldType {
    Int32,
    UInt32,
    SInt32,
    Int64,
    UInt64,
    SInt64,
    Enum2(),
    Enum3(),
    Fixed32,
    SFixed32,
    Fixed64,
    SFixed64,
    Float,
    Double,
    Bool,
    String,
    Bytes,
    Message(),
}
