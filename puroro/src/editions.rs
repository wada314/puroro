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

pub trait Edition {
    fn enum_type(&self) -> EnumType;
    fn field_presence(&self) -> FieldPresence;
    fn json_format(&self) -> JsonFormat;
    fn message_encoding(&self) -> MessageEncoding;
    fn repeated_field_encoding(&self) -> RepeatedFieldEncoding;
    fn utf8_validation(&self) -> Utf8Validation;
}

pub enum EnumType {
    Closed,
    Open,
}

pub enum FieldPresence {
    LegacyRequired,
    Explicit,
    Implicit,
}

pub enum JsonFormat {
    Allow,
    LegacyBestEffort,
}

pub enum MessageEncoding {
    LengthPrefixed,
    Delimited,
}

pub enum RepeatedFieldEncoding {
    Packed,
    Expanded,
}

pub enum Utf8Validation {
    Verify,
    None,
}

pub struct Proto2Edition;
pub struct Proto3Edition;

impl Edition for Proto2Edition {
    fn enum_type(&self) -> EnumType {
        EnumType::Closed
    }
    fn field_presence(&self) -> FieldPresence {
        FieldPresence::Explicit
    }
    fn json_format(&self) -> JsonFormat {
        JsonFormat::LegacyBestEffort
    }
    fn message_encoding(&self) -> MessageEncoding {
        MessageEncoding::LengthPrefixed
    }
    fn repeated_field_encoding(&self) -> RepeatedFieldEncoding {
        RepeatedFieldEncoding::Expanded
    }
    fn utf8_validation(&self) -> Utf8Validation {
        Utf8Validation::None
    }
}

impl Edition for Proto3Edition {
    fn enum_type(&self) -> EnumType {
        EnumType::Open
    }
    fn field_presence(&self) -> FieldPresence {
        FieldPresence::Implicit
    }
    fn json_format(&self) -> JsonFormat {
        JsonFormat::Allow
    }
    fn message_encoding(&self) -> MessageEncoding {
        MessageEncoding::LengthPrefixed
    }
    fn repeated_field_encoding(&self) -> RepeatedFieldEncoding {
        RepeatedFieldEncoding::Packed
    }
    fn utf8_validation(&self) -> Utf8Validation {
        Utf8Validation::Verify
    }
}

pub const PROTO2_EDITION: Proto2Edition = Proto2Edition;
pub const PROTO3_EDITION: Proto3Edition = Proto3Edition;
