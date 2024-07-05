// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.num (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.num
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub trait FieldType {}

pub mod field_types {
    use super::FieldType;
    pub struct Int32;
    pub struct SInt32;
    pub struct UInt32;
    pub struct Int64;
    pub struct SInt64;
    pub struct UInt64;
    pub struct Bool;
    pub struct Enum;
    pub struct Float;
    pub struct Fixed32;
    pub struct SFixed32;
    pub struct Double;
    pub struct Fixed64;
    pub struct SFixed64;
    pub struct String;
    pub struct Bytes;
    pub struct Message;

    impl FieldType for Int32 {}
    impl FieldType for SInt32 {}
    impl FieldType for UInt32 {}
    impl FieldType for Int64 {}
    impl FieldType for SInt64 {}
    impl FieldType for UInt64 {}
    impl FieldType for Bool {}
    impl FieldType for Enum {}
    impl FieldType for Float {}
    impl FieldType for Fixed32 {}
    impl FieldType for SFixed32 {}
    impl FieldType for Double {}
    impl FieldType for Fixed64 {}
    impl FieldType for SFixed64 {}
    impl FieldType for String {}
    impl FieldType for Bytes {}
    impl FieldType for Message {}
}
