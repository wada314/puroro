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
use ::std::borrow::Cow;
use ::std::default::Default;
use ::std::io::Read;
/// The sample cases documented in the Protobuf official encoding document:
/// https://developers.google.com/protocol-buffers/docs/encoding

const TEST1_INPUT: &[u8] = &[0x08, 0x96, 0x01];
const TEST2_INPUT: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
const TEST3_INPUT: &[u8] = &[0x1a, 0x03, 0x08, 0x96, 0x01];
const TEST4_INPUT: &[u8] = &[0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];

const TEST1_EXPECTED: i32 = 150;
const TEST2_EXPECTED: &str = "testing";
const TEST3_EXPECTED: i32 = 150;
const TEST4_EXPECTED: &[i32] = &[3, 270, 86942];

puroro_inline! {r#"
syntax = "proto2";
package p2;
message Test1 {
  optional int32 a = 1;
}

message Test2 {
  optional string b = 2;
}

message Test3 {
  optional Test1 c = 3;
}

message Test4 {
  repeated int32 d = 4 [packed=true];
}
"#}

#[test]
fn proto2_test1() {
    let t1 = p2::Test1::from_bytes_iter(TEST1_INPUT.bytes()).unwrap();
    assert_eq!(TEST1_EXPECTED, t1.a());
    let mut encoded = Vec::new();
    t1.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST1_INPUT, &encoded);
}

#[test]
fn proto2_test2() {
    let mut t2 = p2::Test2::from_bytes_iter(TEST2_INPUT.bytes()).unwrap();
    assert_eq!(TEST2_EXPECTED.to_string(), t2.b());
    let mut encoded = Vec::new();
    t2.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST2_INPUT, &encoded);
}

#[test]
fn proto2_test3() {
    let mut t3 = p2::Test3::from_bytes_iter(TEST3_INPUT.bytes()).unwrap();
    assert!(t3.c().is_some());
    assert_eq!(TEST3_EXPECTED, t3.c().unwrap().a());
    let mut encoded = Vec::new();
    t3.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST3_INPUT, &encoded);
}

#[test]
fn proto2_test4() {
    let mut t4 = p2::Test4::from_bytes_iter(TEST4_INPUT.bytes()).unwrap();
    assert_eq!(TEST4_EXPECTED, t4.d());
    let mut encoded = Vec::new();
    t4.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST4_INPUT, &encoded);
}

puroro_inline! {r#"
syntax = "proto3";
package p3;
message Test1 {
  int32 a = 1;
}

message Test2 {
  string b = 2;
}

message Test3 {
  Test1 c = 3;
}

message Test4 {
  repeated int32 d = 4 [packed=true];
}
"#}

#[test]
fn proto3_test1() {
    let t1 = p3::Test1::from_bytes_iter(TEST1_INPUT.bytes()).unwrap();
    assert_eq!(TEST1_EXPECTED, t1.a());
    let mut encoded = Vec::new();
    t1.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST1_INPUT, &encoded);
}

#[test]
fn proto3_test2() {
    let mut t2 = p3::Test2::from_bytes_iter(TEST2_INPUT.bytes()).unwrap();
    assert_eq!(TEST2_EXPECTED, t2.b());
    let mut encoded = Vec::new();
    t2.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST2_INPUT, &encoded);
}

#[test]
fn proto3_test3() {
    let mut t3 = p3::Test3::from_bytes_iter(TEST3_INPUT.bytes()).unwrap();
    assert!(t3.c().is_some());
    assert_eq!(TEST3_EXPECTED, t3.c().unwrap().a());
    let mut encoded = Vec::new();
    t3.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST3_INPUT, &encoded);
}

#[test]
fn proto3_test4() {
    let mut t4 = p3::Test4::from_bytes_iter(TEST4_INPUT.bytes()).unwrap();
    assert_eq!(TEST4_EXPECTED, t4.d());
    let mut encoded = Vec::new();
    t4.to_bytes(&mut encoded).unwrap();
    assert_eq!(TEST4_INPUT, &encoded);
}
