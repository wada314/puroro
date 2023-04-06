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

use ::puroro::RepeatedFieldView;
use ::tests::full_coverage2::Msg as Msg2;
use ::tests::full_coverage3::Msg as Msg3;

#[test]
fn simple2_get_set_int32() {
    let mut msg = Msg2::default();
    assert_eq!(0, msg.i32_optional());
    assert_eq!(0, msg.i32_repeated().len());
    assert!(!msg.has_i32_optional());

    msg.i32_optional_mut();
    assert_eq!(0, msg.i32_optional());
    assert!(msg.has_i32_optional());

    *msg.i32_optional_mut() = 10;
    msg.i32_repeated_mut().extend([30, 40].iter());
    assert_eq!(10, msg.i32_optional());
    assert_eq!(
        vec![30, 40],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );
    assert!(msg.has_i32_optional());

    msg.clear_i32_optional();
    assert_eq!(0, msg.i32_optional());
    assert!(!msg.has_i32_optional());
}

#[test]
fn simple3_get_set_int32() {
    let mut msg = Msg3::default();
    assert_eq!(0, msg.i32_optional());
    assert_eq!(0, msg.i32_unlabeled());
    assert_eq!(0, msg.i32_repeated().len());
    assert!(!msg.has_i32_optional());
    assert!(!msg.has_i32_unlabeled());

    msg.i32_optional_mut();
    assert_eq!(0, msg.i32_optional());
    assert!(msg.has_i32_optional());
    msg.i32_unlabeled_mut();
    assert_eq!(0, msg.i32_unlabeled());
    assert!(!msg.has_i32_unlabeled());

    *msg.i32_optional_mut() = 10;
    *msg.i32_unlabeled_mut() = 20;
    msg.i32_repeated_mut().extend([30, 40].iter());
    assert_eq!(10, msg.i32_optional());
    assert_eq!(20, msg.i32_unlabeled());
    assert_eq!(
        vec![30, 40],
        msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
    );
    assert!(msg.has_i32_optional());
    assert!(msg.has_i32_unlabeled());

    msg.clear_i32_optional();
    msg.clear_i32_unlabeled();
    assert_eq!(0, msg.i32_optional());
    assert_eq!(0, msg.i32_unlabeled());
    assert!(!msg.has_i32_optional());
    assert!(!msg.has_i32_unlabeled());
}

#[test]
fn simple2_get_set_string() {
    let mut msg = Msg3::default();
    assert_eq!("", msg.string_optional());
    assert_eq!(0, msg.string_repeated().len());
    assert!(!msg.has_string_optional());

    msg.string_optional_mut();
    assert_eq!("", msg.string_optional());
    assert!(msg.has_string_optional());

    *msg.string_optional_mut() = "test1".into();
    assert_eq!("test1", msg.string_optional());
    assert!(msg.has_string_optional());

    msg.clear_string_optional();
    assert_eq!("", msg.string_optional());
    assert!(!msg.has_string_optional());
}

#[test]
fn simple3_get_set_string() {
    let mut msg = Msg3::default();
    assert_eq!("", msg.string_optional());
    assert_eq!("", msg.string_unlabeled());
    assert_eq!(0, msg.string_repeated().len());
    assert!(!msg.has_string_optional());
    assert!(!msg.has_string_unlabeled());

    msg.string_optional_mut();
    assert_eq!("", msg.string_optional());
    assert!(msg.has_string_optional());
    msg.string_unlabeled_mut();
    assert_eq!("", msg.string_unlabeled());
    assert!(!msg.has_string_unlabeled());

    *msg.string_optional_mut() = "test1".into();
    assert_eq!("test1", msg.string_optional());
    assert!(msg.has_string_optional());
    *msg.string_unlabeled_mut() = "test2".into();
    assert_eq!("test2", msg.string_unlabeled());
    assert!(msg.has_string_unlabeled());

    msg.clear_string_optional();
    assert_eq!("", msg.string_optional());
    assert!(!msg.has_string_optional());
    msg.clear_string_unlabeled();
    assert_eq!("", msg.string_unlabeled());
    assert!(!msg.has_string_unlabeled());
}

#[test]
fn simple2_get_set_message() {
    let mut msg = Msg3::default();
    assert!(msg.submsg_optional().is_none());
    assert!(msg.submsg_optional_opt().is_none());
    assert!(!msg.has_submsg_optional());
    assert_eq!(0, msg.submsg_repeated().len());

    msg.submsg_optional_mut();
    assert!(msg.submsg_optional().is_some());
    assert!(msg.submsg_optional_opt().is_some());
    assert!(msg.has_submsg_optional());
    assert_eq!(0, msg.submsg_optional().unwrap().i32_optional());
    assert!(!msg.submsg_optional().unwrap().has_i32_optional());

    *msg.submsg_optional_mut().i32_optional_mut() = 10;
    assert!(msg.submsg_optional().is_some());
    assert_eq!(10, msg.submsg_optional().unwrap().i32_optional());

    msg.clear_submsg_optional();
    assert!(msg.submsg_optional().is_none());
    assert!(msg.submsg_optional_opt().is_none());
    assert!(!msg.has_submsg_optional());
}

#[test]
fn simple3_get_set_message() {
    let mut msg = Msg3::default();
    assert!(msg.submsg_optional().is_none());
    assert!(msg.submsg_optional_opt().is_none());
    assert!(!msg.has_submsg_optional());
    assert!(msg.submsg_unlabeled().is_none());
    assert!(msg.submsg_unlabeled_opt().is_none());
    assert!(!msg.has_submsg_unlabeled());
    assert_eq!(0, msg.submsg_repeated().len());

    msg.submsg_optional_mut();
    assert!(msg.submsg_optional().is_some());
    assert!(msg.submsg_optional_opt().is_some());
    assert!(msg.has_submsg_optional());
    assert_eq!(0, msg.submsg_optional().unwrap().i32_optional());
    assert!(!msg.submsg_optional().unwrap().has_i32_optional());
    msg.submsg_unlabeled_mut();
    assert!(msg.submsg_unlabeled().is_some());
    assert!(msg.submsg_unlabeled_opt().is_some());
    assert!(msg.has_submsg_unlabeled());
    assert_eq!(0, msg.submsg_unlabeled().unwrap().i32_optional());
    assert!(!msg.submsg_unlabeled().unwrap().has_i32_optional());

    *msg.submsg_optional_mut().i32_optional_mut() = 10;
    assert!(msg.submsg_optional().is_some());
    assert_eq!(10, msg.submsg_optional().unwrap().i32_optional());
    *msg.submsg_unlabeled_mut().i32_optional_mut() = 10;
    assert!(msg.submsg_unlabeled().is_some());
    assert_eq!(10, msg.submsg_unlabeled().unwrap().i32_optional());

    msg.clear_submsg_optional();
    assert!(msg.submsg_optional().is_none());
    assert!(msg.submsg_optional_opt().is_none());
    assert!(!msg.has_submsg_optional());
    msg.clear_submsg_unlabeled();
    assert!(msg.submsg_unlabeled().is_none());
    assert!(msg.submsg_unlabeled_opt().is_none());
    assert!(!msg.has_submsg_unlabeled());
}
