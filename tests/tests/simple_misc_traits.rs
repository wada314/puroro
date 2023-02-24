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

use ::tests::full_coverage2::Msg as Msg2;
use ::tests::full_coverage3::Msg as Msg3;

#[test]
pub fn test_partial_eq_2() {
    assert_eq!(Msg2::default(), Msg2::default());

    let mut msg1 = Msg2::default();
    let _ = *msg1.i32_optional_mut();
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

    let mut msg2 = Msg2::default();
    let _ = *msg2.i32_optional_mut();
    // i32_optional: None(10) vs. Some(0)
    assert_ne!(&msg1, &msg2);
    assert_ne!(&msg2, &msg1);

    *msg2.i32_optional_mut() = 10;
    // i32_optional: None(10) vs. Some(10)
    assert_ne!(&msg1, &msg2);
    assert_ne!(&msg2, &msg1);

    msg2.clear_i32_optional();
    // i32_optional: None(10) vs. None(10)
    assert_eq!(&msg1, &msg2);
    assert_eq!(&msg2, &msg1);
}

#[test]
pub fn test_partial_eq_3() {
    assert_eq!(Msg3::default(), Msg3::default());

    let mut msg1 = Msg3::default();
    let _ = *msg1.i32_unlabeled_mut();
    // i32_unlabeled: 0 vs. 0
    assert_eq!(&msg1, &Msg3::default());
    assert_eq!(&Msg3::default(), &msg1);

    *msg1.i32_unlabeled_mut() = 10;
    // i32_unlabeled: 10 vs. 0
    assert_ne!(&msg1, &Msg3::default());
    assert_ne!(&Msg3::default(), &msg1);

    msg1.clear_i32_unlabeled();
    // i32_unlabeled: 0 vs. 0
    assert_eq!(&msg1, &Msg3::default());
    assert_eq!(&Msg3::default(), &msg1);

    let mut msg2 = Msg3::default();
    let _ = *msg2.i32_unlabeled_mut();
    // i32_unlabeled: 0 vs. 0
    assert_eq!(&msg1, &msg2);
    assert_eq!(&msg2, &msg1);

    *msg2.i32_unlabeled_mut() = 10;
    // i32_unlabeled: 0 vs. 10
    assert_ne!(&msg1, &msg2);
    assert_ne!(&msg2, &msg1);

    msg2.clear_i32_unlabeled();
    // i32_unlabeled: 0 vs. 0
    assert_eq!(&msg1, &msg2);
    assert_eq!(&msg2, &msg1);
}

#[test]
fn test_clone_2() {
    let mut msg1 = Msg2::default();
    *msg1.i32_optional_mut() = 10;
    let cloned = msg1.clone();
    assert_eq!(&msg1, &cloned);

    // modify the original one
    *msg1.i32_optional_mut() = 20;
    assert_ne!(&msg1, &cloned);
    assert_eq!(10, cloned.i32_optional());
}

#[test]
fn test_clone_3() {
    let mut msg1 = Msg3::default();
    *msg1.i32_optional_mut() = 10;
    let cloned = msg1.clone();
    assert_eq!(&msg1, &cloned);

    // modify the original one
    *msg1.i32_optional_mut() = 20;
    assert_ne!(&msg1, &cloned);
    assert_eq!(10, cloned.i32_optional());
}
