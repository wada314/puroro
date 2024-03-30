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

pub struct Edition {
    pub enum_type: EnumType,
    pub field_presence: FieldPresence,
    pub json_format: JsonFormat,
    pub message_encoding: MessageEncoding,
    pub repeated_field_encoding: RepeatedFieldEncoding,
    pub utf8_validation: Utf8Validation,
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

pub const PROTO2_EDITION: Edition = Edition {
    enum_type: EnumType::Closed,
    field_presence: FieldPresence::Explicit,
    json_format: JsonFormat::LegacyBestEffort,
    message_encoding: MessageEncoding::LengthPrefixed,
    repeated_field_encoding: RepeatedFieldEncoding::Expanded,
    utf8_validation: Utf8Validation::None,
};

pub const PROTO3_EDITION: Edition = Edition {
    enum_type: EnumType::Open,
    field_presence: FieldPresence::Implicit,
    json_format: JsonFormat::Allow,
    message_encoding: MessageEncoding::LengthPrefixed,
    repeated_field_encoding: RepeatedFieldEncoding::Packed,
    utf8_validation: Utf8Validation::Verify,
};
