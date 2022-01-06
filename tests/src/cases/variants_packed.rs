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

use crate::tests_pb::proto2_packed::Msg as Msg2;
use crate::tests_pb::proto3_packed::Msg as Msg3;
use ::puroro::Message;

fn assert_is_packed_output(field_num: u8, buf: &[u8]) {
    assert_eq!(
        &[
            (field_num << 3) | 2, /* field type */
            2,                    /* len */
            1,
            2
        ],
        buf
    );
}

fn assert_is_not_packed_output(field_num: u8, buf: &[u8]) {
    assert_eq!(
        &[
            (field_num << 3) | 0, /* field type */
            1,
            (field_num << 3) | 0, /* field type */
            2
        ],
        buf
    );
}

#[test]
fn test_explicitly_packed2() {
    let mut msg = Msg2::default();
    msg.explicitly_packed_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_packed_output(1, &buf);
}
#[test]
fn test_explicitly_packed3() {
    let mut msg = Msg3::default();
    msg.explicitly_packed_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_packed_output(1, &buf);
}

#[test]
fn test_explicitly_not_packed2() {
    let mut msg = Msg2::default();
    msg.explicitly_not_packed_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_not_packed_output(2, &buf);
}

#[test]
fn test_explicitly_not_packed3() {
    let mut msg = Msg3::default();
    msg.explicitly_not_packed_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_not_packed_output(2, &buf);
}

#[test]
fn test_not_annotated2() {
    let mut msg = Msg2::default();
    msg.not_annotated_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_not_packed_output(3, &buf);
}

#[test]
fn test_not_annotated3() {
    let mut msg = Msg3::default();
    msg.not_annotated_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_is_packed_output(3, &buf);
}
