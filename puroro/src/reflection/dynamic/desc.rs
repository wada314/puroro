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

pub struct FileDescriptor<'a> {
    pub messages: &'a [&'a MessageDescriptor<'a>],
}

pub struct MessageDescriptor<'a> {
    pub parent: &'a FileDescriptor<'a>,
    pub name: &'a str,
    pub fields: &'a [&'a FieldDescriptor<'a>],
}

pub struct FieldDescriptor<'a> {
    pub parent: &'a MessageDescriptor<'a>,
    pub name: &'a str,
    pub number: i32,
    pub r#type: FieldType<'a>,
    pub label: FieldLabel,
}

pub struct EnumDescriptor<'a> {
    pub name: &'a str,
    pub values: &'a [EnumValueDescriptor<'a>],
}

pub struct EnumValueDescriptor<'a> {
    pub name: &'a str,
    pub number: i32,
}

pub enum FieldLabel {
    Required,
    Optional,
    Repeated,
}

pub enum FieldType<'a> {
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    Enum(&'a EnumDescriptor<'a>),
    Float,
    Double,
    Bytes,
    String,
    Message(&'a MessageDescriptor<'a>),
}
