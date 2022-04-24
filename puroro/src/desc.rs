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

pub struct MessageDescriptor {
    name: String,
    fields: Vec<FieldDescriptor>,
}

pub struct FieldDescriptor {
    name: String,
    number: i32,
    r#type: FieldType,
}

pub struct EnumDescriptor {
    name: String,
    values: Vec<EnumValueDescriptor>,
}

pub struct EnumValueDescriptor {
    name: String,
    number: i32,
}

pub enum FieldType {
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
    Enum(&'static EnumDescriptor),
    Float,
    Double,
    Bytes,
    String,
    Message(&'static MessageDescriptor),
}
