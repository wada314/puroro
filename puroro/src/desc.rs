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

const FD: FileDescriptor<'static> = FileDescriptor { messages: &[MD] };
const MD: MessageDescriptor<'static> = MessageDescriptor {
    file_descriptor: &FD,
    name: todo!(),
    fields: todo!(),
};

pub struct FileDescriptor<'a> {
    messages: &'a [MessageDescriptor<'a>],
}

pub struct MessageDescriptor<'a> {
    file_descriptor: &'a FileDescriptor<'a>,
    name: &'a str,
    fields: &'a [FieldDescriptor<'a>],
}

pub struct FieldDescriptor<'a> {
    name: &'a str,
    number: i32,
    r#type: FieldType<'a>,
    label: FieldLabel,
}

pub struct EnumDescriptor<'a> {
    name: &'a str,
    values: &'a [EnumValueDescriptor<'a>],
}

pub struct EnumValueDescriptor<'a> {
    name: &'a str,
    number: i32,
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
