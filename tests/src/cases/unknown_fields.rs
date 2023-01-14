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

use ::puroro::Message;
use ::puroro_inline::puroro_inline;
use ::std::io::Read;

const UNKNOWN_FIELD_NUMBER: u8 = 15;

puroro_inline! {r#"
syntax = "proto3";
package unknown_fields;

message Msg {
    // Reserve for unknown field number test
    reserved 15;

    int32 i32_unlabeled = 1;
    repeated int32 i32_repeated = 2;
    float float_unlabeled = 3;
    repeated float float_repeated = 4;
    string string_unlabeled = 5;
    repeated string string_repeated = 6;

    message Submsg {
        int32 i32_unlabeled = 1;
    }
    Submsg submsg_unlabeled = 7;
    repeated Submsg submsg_repeated = 8;

    Enum enum_unlabeled = 9;
    repeated Enum enum_repeated = 10;
}

enum Enum {
    ZEROTH = 0;
    FIRST = 1;
    TENTH = 10;
}
"#}

use unknown_fields::{msg::Submsg, Enum, Msg};

#[test]
fn test_unknown_field_number_is_ignored() {
    // field #15 has variant value 1.
    let buffer = [(UNKNOWN_FIELD_NUMBER << 3) | 0, 0x01];
    let msg = Msg::from_bytes_iter(buffer.bytes()).unwrap();
    assert!(!msg.has_i32_unlabeled());
}
