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

use crate::tests_pb::ser_tests2::msg::Submsg as Submsg2;
use crate::tests_pb::ser_tests2::{Enum as Enum2, Msg as Msg2};
use crate::tests_pb::ser_tests3::msg::Submsg as Submsg3;
use crate::tests_pb::ser_tests3::{Enum as Enum3, Msg as Msg3};

#[test]
pub fn test_partial_eq_2() {
    assert_eq!(Msg2::default(), Msg2::default());

    let mut msg1 = Msg2::default();
    *msg1.i32_optional_mut();
    // i32_optional: Some(0) vs. None
    assert_ne!(&msg1, &Msg2::default());
    assert_ne!(&Msg2::default(), &msg1);

    *msg1.i32_optional_mut() = 10;
    // i32_optional: Some(10) vs. None
    assert_ne!(&msg1, &Msg2::default());
    assert_ne!(&Msg2::default(), &msg1);

    msg1.clear_i32_optional();
    // i32_optional: None(10) vs. None(0)
    assert_eq!(&msg1, &Msg2::default());
    assert_eq!(&Msg2::default(), &msg1);
}
