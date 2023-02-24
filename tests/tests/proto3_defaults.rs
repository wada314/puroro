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

/// About the proto3's default values, check the following official document:
/// https://github.com/protocolbuffers/protobuf/blob/master/docs/field_presence.md
use ::puroro::Message;
use ::puroro_inline::puroro_inline;

const INPUT_FIELD1_I32_ZERO: &[u8] = &[(1 << 3) | 0, 0x00];
const INPUT_FIELD1_I32_ONE: &[u8] = &[(1 << 3) | 0, 0x01];
const INPUT_FIELD1_I32_PACKED_ZERO: &[u8] = &[(1 << 3) | 2, 0x01, 0x00];
const INPUT_FIELD2_I32_ZERO: &[u8] = &[(2 << 3) | 0, 0x00];
const INPUT_FIELD2_I32_PACKED_ZERO: &[u8] = &[(2 << 3) | 2, 0x01, 0x00];
const INPUT_FIELD2_I32_ONE: &[u8] = &[(2 << 3) | 0, 0x01];
const INPUT_FIELD3_I32_ZERO: &[u8] = &[(3 << 3) | 0, 0x00];
const INPUT_FIELD3_I32_ONE: &[u8] = &[(3 << 3) | 0, 0x01];
const INPUT_FIELD3_I32_PACKED_ZERO_TO_THREE: &[u8] = &[(3 << 3) | 2, 0x04, 0x00, 0x01, 0x02, 0x03];
const INPUT_FIELD5_STRING_EMPTY: &[u8] = &[(5 << 3) | 0x02, 0x00];
const INPUT_FIELD5_STRING_TEST2: &[u8] = &[
    (5 << 3) | 0x02,
    0x05,
    't' as u8,
    'e' as u8,
    's' as u8,
    't' as u8,
    '2' as u8,
];
const INPUT_FIELDS6_MSG_FIELD1_I32_ZERO: &[u8] = &[(6 << 3) | 0x02, 0x02, (1 << 3) | 0, 0x00];
const INPUT_FIELDS6_MSG_FIELD1_I32_ONE: &[u8] = &[(6 << 3) | 0x02, 0x02, (1 << 3) | 0, 0x01];

puroro_inline! {r#"
syntax = "proto3";
package proto3_defaults;

message Msg {
    int32 i32_unlabeled = 1;
    optional int32 i32_optional = 2;
    repeated int32 i32_repeated = 3;
    float f32_unlabeled = 4;
    string string_unlabeled = 5;
    Submsg submsg_unlabeled = 6;
}

message Submsg {
    int32 i32_unlabeled = 1;
}
"#}
use proto3_defaults::Msg;

#[test]
fn test_i32_unlabeled() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    assert_eq!(0, msg.i32_unlabeled());
    assert!(!msg.has_i32_unlabeled());

    *msg.i32_unlabeled_mut() = 10;
    assert_eq!(10, msg.i32_unlabeled());
    assert!(msg.has_i32_unlabeled());

    // merge_from_bytes_iter 0 into the field, but it is a default value so it should be ignored
    msg.merge_from_bytes_iter(INPUT_FIELD1_I32_ZERO.bytes())
        .unwrap();
    assert_eq!(10, msg.i32_unlabeled());
    assert!(msg.has_i32_unlabeled());
    msg.merge_from_bytes_iter(INPUT_FIELD1_I32_PACKED_ZERO.bytes())
        .unwrap();
    assert_eq!(10, msg.i32_unlabeled());
    assert!(msg.has_i32_unlabeled());

    // merge_from_bytes_iter 1 into the field. Should overwrite the value.
    msg.merge_from_bytes_iter(INPUT_FIELD1_I32_ONE.bytes())
        .unwrap();
    assert_eq!(1, msg.i32_unlabeled());
    assert!(msg.has_i32_unlabeled());
}

#[test]
fn test_i32_optional() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    assert_eq!(0, msg.i32_optional());
    assert!(!msg.has_i32_optional());

    *msg.i32_optional_mut() = 10;
    assert_eq!(10, msg.i32_optional());
    assert!(msg.has_i32_optional());

    // Deserializing (merging into) the field by 0 value.
    // Doing the same thing with i32_unlabeled, but for optional field
    // the field value is overwritten even if the input value is 0.
    msg.merge_from_bytes_iter(INPUT_FIELD2_I32_ZERO.bytes())
        .unwrap();
    assert_eq!(Some(0), msg.i32_optional_opt());
    *msg.i32_optional_mut() = 10;
    assert_eq!(10, msg.i32_optional());
    assert!(msg.has_i32_optional());
    msg.merge_from_bytes_iter(INPUT_FIELD2_I32_PACKED_ZERO.bytes())
        .unwrap();
    assert_eq!(0, msg.i32_optional());
    assert!(msg.has_i32_optional());

    msg.merge_from_bytes_iter(INPUT_FIELD2_I32_ONE.bytes())
        .unwrap();
    assert_eq!(1, msg.i32_optional());
    assert!(msg.has_i32_optional());
}

#[test]
fn test_i32_repeated() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();

    *msg.i32_repeated_mut() = vec![10, 20];
    assert_eq!(
        vec![10, 20],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );

    msg.merge_from_bytes_iter(INPUT_FIELD3_I32_ZERO.bytes())
        .unwrap();
    assert_eq!(
        vec![10, 20, 0],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );

    msg.merge_from_bytes_iter(INPUT_FIELD3_I32_ONE.bytes())
        .unwrap();
    assert_eq!(
        vec![10, 20, 0, 1],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );

    msg.merge_from_bytes_iter(INPUT_FIELD3_I32_PACKED_ZERO_TO_THREE.bytes())
        .unwrap();
    assert_eq!(
        vec![10, 20, 0, 1, 0, 1, 2, 3],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );
}

#[test]
fn test_string_unlabeled() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    assert_eq!("", msg.string_unlabeled());
    assert!(!msg.has_string_unlabeled());

    *msg.string_unlabeled_mut() = "test1".into();
    assert_eq!("test1", msg.string_unlabeled());
    assert!(msg.has_string_unlabeled());

    msg.merge_from_bytes_iter(INPUT_FIELD5_STRING_EMPTY.bytes())
        .unwrap();
    assert_eq!("test1", msg.string_unlabeled());
    assert!(msg.has_string_unlabeled());

    msg.merge_from_bytes_iter(INPUT_FIELD5_STRING_TEST2.bytes())
        .unwrap();
    assert_eq!("test2", msg.string_unlabeled());
    assert!(msg.has_string_unlabeled());
}

#[test]
fn test_message_unlabeled() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    assert_eq!(None, msg.submsg_unlabeled());
    assert!(!msg.has_submsg_unlabeled());

    // calling the mut getter automatically initializes the message field
    msg.submsg_unlabeled_mut();
    assert_eq!(Some(0), msg.submsg_unlabeled().map(|m| m.i32_unlabeled()));
    assert!(msg.has_submsg_unlabeled());
    *msg.submsg_unlabeled_mut().i32_unlabeled_mut() = 10;
    assert_eq!(Some(10), msg.submsg_unlabeled().map(|m| m.i32_unlabeled()));
    assert!(msg.has_submsg_unlabeled());

    msg.merge_from_bytes_iter(INPUT_FIELDS6_MSG_FIELD1_I32_ZERO.bytes())
        .unwrap();
    assert_eq!(Some(10), msg.submsg_unlabeled().map(|m| m.i32_unlabeled()));
    assert!(msg.has_submsg_unlabeled());

    msg.merge_from_bytes_iter(INPUT_FIELDS6_MSG_FIELD1_I32_ONE.bytes())
        .unwrap();
    assert_eq!(Some(1), msg.submsg_unlabeled().map(|m| m.i32_unlabeled()));
    assert!(msg.has_submsg_unlabeled());
}
