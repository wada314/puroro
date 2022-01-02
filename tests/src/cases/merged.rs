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

use ::itertools::Itertools;
use ::std::ops::Deref;
use ::tests_pb::full_coverage3::msg::{Submsg, SubmsgBuilder, SubmsgTrait as _};
use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait as _};

#[test]
fn test_get_i32_optional_field() {
    let none = Msg::default();
    let some_3 = MsgBuilder::new().append_i32_optional(3).build();
    let some_7 = MsgBuilder::new().append_i32_optional(7).build();
    assert!(!(&none, &none).has_i32_optional());
    assert_eq!(3, (&none, &some_3).i32_optional());
    assert_eq!(3, (&some_3, &none).i32_optional());
    assert_eq!(7, (&some_3, &some_7).i32_optional());
}

#[test]
fn test_get_i32_unlabeled_field() {
    let msg_0 = Msg::default();
    let msg_3 = MsgBuilder::new().append_i32_unlabeled(3).build();
    let msg_7 = MsgBuilder::new().append_i32_unlabeled(7).build();
    assert!(!(&msg_0, &msg_0).has_i32_unlabeled());
    assert_eq!(3, (&msg_0, &msg_3).i32_unlabeled());
    assert_eq!(3, (&msg_3, &msg_0).i32_unlabeled());
    assert_eq!(7, (&msg_3, &msg_7).i32_unlabeled());
}

#[test]
fn test_get_i32_repeated_field() {
    let empty = Msg::default();
    let msg_1 = MsgBuilder::new().append_i32_repeated(vec![1]).build();
    let msg_3_5 = MsgBuilder::new().append_i32_repeated(vec![3, 5]).build();
    assert_eq!(
        vec![] as Vec<i32>,
        (&empty, &empty).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1],
        (&empty, &msg_1).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1],
        (&msg_1, &empty).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1, 3, 5],
        (&msg_1, &msg_3_5).i32_repeated().into_iter().collect_vec()
    );
}

#[test]
fn test_get_msg_optional_field() {
    let submsg_3 = SubmsgBuilder::new().append_i32_unlabeled(3).build();
    let submsg_7 = SubmsgBuilder::new().append_i32_unlabeled(7).build();
    let none = Msg::default();
    let msg_3 = MsgBuilder::new()
        .append_submsg_optional(submsg_3.clone())
        .build();
    let msg_7 = MsgBuilder::new()
        .append_submsg_optional(submsg_7.clone())
        .build();
    assert!(!(&none, &none).has_submsg_optional());
    assert_eq!(3, (&msg_3, &none).submsg_optional().i32_unlabeled());
    assert_eq!(3, (&none, &msg_3).submsg_optional().i32_unlabeled());
    assert_eq!(7, (&msg_3, &msg_7).submsg_optional().i32_unlabeled());
}

#[test]
fn test_get_msg_repeated_field() {
    let submsg_3 = SubmsgBuilder::new().append_i32_unlabeled(3).build();
    let submsg_7 = SubmsgBuilder::new().append_i32_unlabeled(7).build();
    let empty = Msg::default();
    let msg_3 = MsgBuilder::new()
        .append_submsg_repeated(vec![submsg_3.clone()])
        .build();
    let msg_7_7 = MsgBuilder::new()
        .append_submsg_repeated(vec![submsg_7.clone(), submsg_7.clone()])
        .build();
    assert_eq!(0, (&empty, &empty).submsg_repeated().into_iter().count());
    assert_eq!(
        vec![3],
        (&empty, &msg_3)
            .submsg_repeated()
            .into_iter()
            .map(|submsg| submsg.i32_unlabeled())
            .collect_vec()
    );
    assert_eq!(
        vec![3],
        (&msg_3, &empty)
            .submsg_repeated()
            .into_iter()
            .map(|submsg| submsg.i32_unlabeled())
            .collect_vec()
    );
    assert_eq!(
        vec![3, 7, 7],
        (&msg_3, &msg_7_7)
            .submsg_repeated()
            .into_iter()
            .map(|submsg| submsg.i32_unlabeled())
            .collect_vec()
    );
}

#[test]
fn test_get_oneof_field() {
    use ::tests_pb::oneofs3::msg::GroupTwo;
    use ::tests_pb::oneofs3::{Msg, MsgTrait as _, Submsg, SubmsgTrait as _};
    let mut none = Msg::default();
    let mut msg_3 = Msg::default();
    *msg_3.g2_f32_mut() = 3.0;
    let mut msg_7 = Msg::default();
    *msg_7.g2_f32_mut() = 7.0;
    let mut msg_test = Msg::default();
    *msg_test.g2_string_mut() = "Test".to_string();
    let mut msg_test2 = Msg::default();
    *msg_test2.g2_string_mut() = "Test2".to_string();
    let mut msg_submsg_0 = Msg::default();
    *msg_submsg_0.g2_submsg_mut().i32_unlabeled_mut() = 0;
    let mut msg_submsg_3 = Msg::default();
    *msg_submsg_3.g2_submsg_mut().i32_unlabeled_mut() = 3;
    // None x None
    assert_eq!(None, (&none, &none).group_two());

    // None x Some
    assert_eq!(
        Some(3.0),
        (&msg_3, &none).group_two().and_then(|o| o.g2_f32())
    );
    assert_eq!(
        Some(3.0),
        (&none, &msg_3).group_two().and_then(|o| o.g2_f32())
    );
    assert_eq!(
        Some("Test"),
        (&msg_test, &none).group_two().and_then(|o| o.g2_string())
    );
    assert_eq!(
        Some("Test"),
        (&none, &msg_test).group_two().and_then(|o| o.g2_string())
    );
    assert_eq!(
        Some(3),
        (&msg_submsg_3, &none)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );
    assert_eq!(
        Some(3),
        (&none, &msg_submsg_3)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );

    // Some x Some, same types
    assert_eq!(
        Some(7.0),
        (&msg_3, &msg_7).group_two().and_then(|o| o.g2_f32())
    );
    assert_eq!(
        Some("Test2"),
        (&msg_test, &msg_test2)
            .group_two()
            .and_then(|o| o.g2_string())
    );
    assert_eq!(
        Some(3),
        (&msg_submsg_0, &msg_submsg_3)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );
    assert_eq!(
        Some(3),
        (&msg_submsg_3, &msg_submsg_0)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );

    // Some x Some, different types
    assert_eq!(
        Some("Test"),
        (&msg_3, &msg_test).group_two().and_then(|o| o.g2_string())
    );
    assert_eq!(
        Some(0),
        (&msg_3, &msg_submsg_0)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );
    assert_eq!(
        Some(3.0),
        (&msg_test, &msg_3).group_two().and_then(|o| o.g2_f32())
    );
    assert_eq!(
        Some(0),
        (&msg_test, &msg_submsg_0)
            .group_two()
            .and_then(|o| o.g2_submsg().map(|submsg| submsg.i32_unlabeled()))
    );
    assert_eq!(
        Some(7.0),
        (&msg_submsg_3, &msg_7).group_two().and_then(|o| o.g2_f32())
    );
    assert_eq!(
        Some("Test"),
        (&msg_submsg_3, &msg_test)
            .group_two()
            .and_then(|o| o.g2_string())
    );
}
