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

use crate::tests_pb::proto2_packed::Msg;
use ::puroro::Message;

#[test]
fn test_packed() {
    let mut msg = Msg::default();
    msg.packed_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_eq!(
        vec![
            (1 /* field num. */ << 3) | 2, /* field type */
            2,                             /* len */
            1,
            2
        ],
        buf
    );
}

#[test]
fn test_not_packed1() {
    let mut msg = Msg::default();
    msg.not_packed1_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_eq!(
        vec![
            (2 /* field num. */ << 3) | 0, /* field type */
            1,
            (2 /* field num. */ << 3) | 0, /* field type */
            2
        ],
        buf
    );
}

#[test]
fn test_not_packed2() {
    let mut msg = Msg::default();
    msg.not_packed2_mut().extend([1, 2].iter());
    let mut buf = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert_eq!(
        vec![
            (3 /* field num. */ << 3) | 0, /* field type */
            1,
            (3 /* field num. */ << 3) | 0, /* field type */
            2
        ],
        buf
    );
}
