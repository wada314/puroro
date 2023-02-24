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

puroro_inline! {r#"
syntax = "proto2";
package deser_tests2;

message Msg {
    optional int32 i32_optional = 1;
    repeated int32 i32_repeated = 2;
    optional float float_optional = 3;
    repeated float float_repeated = 4;
    optional string string_optional = 5;
    repeated string string_repeated = 6;

    message Submsg {
        optional int32 i32_optional = 1;
    }
    optional Submsg submsg_optional = 7;
    repeated Submsg submsg_repeated = 8;

    optional Enum enum_optional = 9;
    repeated Enum enum_repeated = 10;

    optional int32 two_bytes_field_number = 16;

    optional int32 very_large_field_number = 536870911; // 2^29 - 1
}

enum Enum {
    ZEROTH = 0;
    FIRST = 1;
    TENTH = 10;
}
"#}

use deser_tests2::Msg;

#[test]
fn test_empty() {
    let msg = Msg::from_bytes_iter([].bytes()).unwrap();
    assert_eq!(0, msg.i32_optional());
    assert_eq!(&msg, &Msg::default());
}

#[test]
fn test_incomplete_input_middle_of_field_number() {
    let field_16: &[u8] = &[0x80, 0x01, 0x00];
    assert!(Msg::from_bytes_iter(field_16.bytes().take(1)).is_err());
    assert!(Msg::from_bytes_iter(field_16.bytes()).is_ok());
}

#[test]
fn test_incomplete_input_no_wire_body() {
    let field_16: &[u8] = &[0x80, 0x01, 0x00];
    assert!(Msg::from_bytes_iter(field_16.bytes().take(2)).is_err());
    assert!(Msg::from_bytes_iter(field_16.bytes()).is_ok());
}

#[test]
fn test_incomplete_input_string() {
    let field_5: &[u8] = &[0x2A, 0x03, 0x40, 0x41, 0x42];
    assert!(Msg::from_bytes_iter(field_5.bytes().take(4)).is_err());
    assert!(Msg::from_bytes_iter(field_5.bytes()).is_ok());
}

#[test]
fn test_incomplete_input_submsg() {
    let field_7: &[u8] = &[0x3A, 0x02, 0x08, 0x01];
    assert!(Msg::from_bytes_iter(field_7.bytes().take(3)).is_err());
    assert!(Msg::from_bytes_iter(field_7.bytes()).is_ok());
}

#[test]
fn test_message_length_is_too_long() {
    let field_7: &mut [u8] = &mut [0x3A, 0x02, 0x08, 0x01];
    assert!(Msg::from_bytes_iter(field_7.bytes()).is_ok());
    field_7[1] += 1; // extend the message length by 1 byte
    assert!(Msg::from_bytes_iter(field_7.bytes()).is_err());
}
