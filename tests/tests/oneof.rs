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

use ::puroro_inline::puroro_inline;

puroro_inline! {r#"
syntax = "proto2";
package oneofs2;

message Msg {
    oneof group_one {
        int32 g1_int32 = 1;
        string g1_string = 2;
    }
    oneof group_two {
        float g2_f32 = 3;
        string g2_string = 4;
        Submsg g2_submsg = 5;
    }
    oneof group_three {
        int32 g3_int32 = 6;
    }
}

message Submsg {
    optional int32 i32_optional = 1;
}
"#}
use oneofs2::msg::{
    GroupOneCase as GroupOneCase2, GroupThreeCase as GroupThreeCase2, GroupTwoCase as GroupTwoCase2,
};
use oneofs2::{Msg as Msg2, Submsg as Submsg2};

#[test]
fn test_oneof2() {
    let mut msg = Msg2::default();

    // Check the default values are empty
    assert!(msg.group_one().is_none());
    assert!(msg.group_two().is_none());
    assert!(msg.group_three().is_none());
    assert!(!msg.has_g1_int32());
    assert!(!msg.has_g1_string());
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());
    assert!(!msg.has_g3_int32());

    // Set values and check it via getter methods
    *msg.g1_int32_mut() = 100;
    assert!(matches!(msg.group_one(), Some(GroupOneCase2::G1Int32(100))));
    assert_eq!(msg.g1_int32(), 100);
    assert!(!msg.has_g1_string());
    *msg.g1_string_mut() = "Test".into();
    assert!(matches!(
        msg.group_one(),
        Some(GroupOneCase2::G1String("Test"))
    ));
    assert_eq!(msg.g1_string(), "Test");
    assert!(!msg.has_g1_int32());
    msg.clear_group_one();
    assert!(msg.group_one().is_none());
    assert!(!msg.has_g1_int32());
    assert!(!msg.has_g1_string());

    *msg.g2_f32_mut() = 100.0;
    assert_eq!(msg.group_two(), Some(GroupTwoCase2::G2F32(100.0)));
    assert_eq!(msg.g2_f32(), 100.0);
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());
    *msg.g2_string_mut() = "Test".into();
    assert_eq!(msg.group_two(), Some(GroupTwoCase2::G2String("Test")));
    assert_eq!(msg.g2_string(), "Test");
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_submsg());
    *msg.g2_submsg_mut() = Submsg2::default();
    *msg.g2_submsg_mut().i32_optional_mut() = 100;
    assert!(matches!(msg.group_two(), Some(GroupTwoCase2::G2Submsg(_))));
    assert!(msg.g2_submsg().is_some());
    assert_eq!(msg.g2_submsg().unwrap().i32_optional(), 100);
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    msg.clear_group_two();
    assert!(msg.group_two().is_none());
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());

    *msg.g3_int32_mut() = 100;
    assert!(matches!(
        msg.group_three(),
        Some(GroupThreeCase2::G3Int32(100))
    ));
    assert_eq!(msg.g3_int32(), 100);
    msg.clear_group_three();
    assert!(msg.group_three().is_none());
    assert!(!msg.has_g3_int32());
}

puroro_inline! {r#"
syntax = "proto3";
package oneofs3;

message Msg {
    oneof group_one {
        int32 g1_int32 = 1;
        string g1_string = 2;
    }
    oneof group_two {
        float g2_f32 = 3;
        string g2_string = 4;
        Submsg g2_submsg = 5;
    }
    oneof group_three {
        int32 g3_int32 = 6;
    }
}

message Submsg {
    int32 i32_unlabeled = 1;
}
"#}

use oneofs3::msg::{
    GroupOneCase as GroupOneCase3, GroupThreeCase as GroupThreeCase3, GroupTwoCase as GroupTwoCase3,
};
use oneofs3::Msg as Msg3;

#[test]
fn test_oneof3() {
    let mut msg = Msg3::default();

    // Check the default values are empty
    assert!(msg.group_one().is_none());
    assert!(msg.group_two().is_none());
    assert!(msg.group_three().is_none());
    assert!(!msg.has_g1_int32());
    assert!(!msg.has_g1_string());
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());
    assert!(!msg.has_g3_int32());

    // Set values and check it via getter methods
    *msg.g1_int32_mut() = 100;
    assert!(matches!(msg.group_one(), Some(GroupOneCase3::G1Int32(100))));
    assert_eq!(msg.g1_int32(), 100);
    assert!(!msg.has_g1_string());
    *msg.g1_string_mut() = "Test".into();
    assert!(matches!(
        msg.group_one(),
        Some(GroupOneCase3::G1String("Test"))
    ));
    assert_eq!(msg.g1_string(), "Test");
    assert!(!msg.has_g1_int32());
    msg.clear_group_one();
    assert!(msg.group_one().is_none());
    assert!(!msg.has_g1_int32());
    assert!(!msg.has_g1_string());

    *msg.g2_f32_mut() = 100.0;
    assert_eq!(msg.group_two(), Some(GroupTwoCase3::G2F32(100.0)));
    assert_eq!(msg.g2_f32(), 100.0);
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());
    *msg.g2_string_mut() = "Test".into();
    assert_eq!(msg.group_two(), Some(GroupTwoCase3::G2String("Test")));
    assert_eq!(msg.g2_string(), "Test");
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_submsg());
    *msg.g2_submsg_mut().i32_unlabeled_mut() = 100;
    assert!(matches!(msg.group_two(), Some(GroupTwoCase3::G2Submsg(_))));
    assert!(msg.g2_submsg().is_some());
    assert_eq!(msg.g2_submsg().unwrap().i32_unlabeled(), 100);
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    msg.clear_group_two();
    assert!(msg.group_two().is_none());
    assert!(!msg.has_g2_f32());
    assert!(!msg.has_g2_string());
    assert!(!msg.has_g2_submsg());

    *msg.g3_int32_mut() = 100;
    assert!(matches!(
        msg.group_three(),
        Some(GroupThreeCase3::G3Int32(100))
    ));
    assert_eq!(msg.g3_int32(), 100);
    msg.clear_group_three();
    assert!(msg.group_three().is_none());
    assert!(!msg.has_g3_int32());
}
