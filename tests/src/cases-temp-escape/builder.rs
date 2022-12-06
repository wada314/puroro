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

#[test]
fn test_builder() {
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    let msg = MsgBuilder::new()
        .append_i32_unlabeled(10)
        .append_string_optional("Test")
        .build();
    assert_eq!(10, msg.i32_unlabeled());
    assert_eq!("Test", msg.string_optional());
    assert_eq!(0, msg.i64_unlabeled());
    assert!(::std::mem::size_of_val(&msg) < ::std::mem::size_of::<Msg>());
}

#[test]
fn test_builder_into_numericals() {
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    #[derive(Clone, Debug, PartialEq)]
    struct Ten();
    impl From<Ten> for i32 {
        fn from(_: Ten) -> Self {
            10
        }
    }
    let msg = MsgBuilder::new().append_i32_unlabeled(Ten()).build();
    assert_eq!(10, msg.i32_unlabeled());
    assert_eq!("", msg.string_optional());
    assert_eq!(0, msg.i64_unlabeled());
    assert_eq!(0, ::std::mem::size_of_val(&msg));
}

#[test]
fn test_builder_append_repeated() {
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    let msg = MsgBuilder::new()
        .append_i32_repeated(vec![1, 1])
        .append_i32_repeated(vec![] as Vec<i32>)
        .append_i32_repeated(vec![2, 3, 5])
        .build();
    assert_eq!(
        vec![1, 1, 2, 3, 5],
        msg.i32_repeated().into_iter().collect_vec()
    );
}

#[test]
fn test_oneof_field_builder() {
    use ::std::ops::Deref;
    use ::tests_pb::oneofs3::{Msg, MsgBuilder, MsgTrait};
    let msg1 = MsgBuilder::new().append_g1_int32(10).build();
    assert_eq!(10, msg1.g1_int32());
    assert_eq!(false, msg1.has_g1_string());

    let msg2 = MsgBuilder::new().append_g1_string("test").build();
    assert_eq!(false, msg2.has_g1_int32());
    assert_eq!("test", msg2.g1_string());

    let msg3 = MsgBuilder::new()
        .append_g1_string("test")
        .append_g1_int32(10)
        .build();
    assert_eq!(10, msg3.g1_int32());
    assert_eq!(false, msg3.has_g1_string());
}

#[test]
fn test_ld_field_builder() {
    use ::std::ops::Deref;
    use ::tests_pb::full_coverage3::msg::{SubmsgBuilder, SubmsgTrait};
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    let msg1 = MsgBuilder::new()
        .append_string_unlabeled("str")
        .append_string_optional("String".to_string())
        .build();
    assert_eq!("str", msg1.string_unlabeled());
    assert_eq!("String", msg1.string_optional());

    let msg2 = MsgBuilder::new()
        .append_bytes_unlabeled(b"slice" as &[u8])
        .append_bytes_optional(Vec::from(b"Vec" as &[u8]))
        .build();
    assert_eq!(b"slice", msg2.bytes_unlabeled());
    assert_eq!(b"Vec", msg2.bytes_optional());

    let msg3 = MsgBuilder::new()
        .append_submsg_unlabeled(SubmsgBuilder::new().append_i32_unlabeled(10).build())
        .append_submsg_unlabeled(SubmsgBuilder::new().append_i64_unlabeled(20).build())
        .build();
    assert!(msg3.submsg_unlabeled().is_some());
    assert_eq!(10, msg3.submsg_unlabeled().i32_unlabeled());
    assert_eq!(20, msg3.submsg_unlabeled().i64_unlabeled());
}
