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
use ::puroro::Either;
use ::tests_pb::full_coverage3::{Msg, MsgTrait};

#[test]
fn test_get_i32_optional_field() {
    let none = Msg {
        i32_optional: None,
        ..Default::default()
    };
    let some_3 = Msg {
        i32_optional: Some(3),
        ..Default::default()
    };
    assert!(!Either::<&Msg, &Msg>::Left(&none).has_i32_optional());
    assert!(!Either::<&Msg, &Msg>::Right(&none).has_i32_optional());
    assert_eq!(3, Either::<&Msg, &Msg>::Left(&some_3).i32_optional());
    assert_eq!(3, Either::<&Msg, &Msg>::Right(&some_3).i32_optional());
}

#[test]
fn test_get_i32_unlabeled_field() {
    let val_0 = Msg {
        i32_unlabeled: 0,
        ..Default::default()
    };
    let val_3 = Msg {
        i32_unlabeled: 3,
        ..Default::default()
    };
    assert_eq!(0, Either::<&Msg, &Msg>::Left(&val_0).i32_unlabeled());
    assert_eq!(0, Either::<&Msg, &Msg>::Right(&val_0).i32_unlabeled());
    assert_eq!(3, Either::<&Msg, &Msg>::Left(&val_3).i32_unlabeled());
    assert_eq!(3, Either::<&Msg, &Msg>::Right(&val_3).i32_unlabeled());
}

#[test]
fn test_get_i32_repeated_field() {
    let empty = Msg {
        i32_repeated: [].iter().cloned().collect(),
        ..Default::default()
    };
    let val_1_2_5 = Msg {
        i32_repeated: [1, 2, 5].iter().cloned().collect(),
        ..Default::default()
    };
    assert_eq!(
        vec![] as Vec<i32>,
        Either::<&Msg, &Msg>::Left(&empty)
            .i32_repeated()
            .into_iter()
            .collect_vec(),
    );
    assert_eq!(
        vec![] as Vec<i32>,
        Either::<&Msg, &Msg>::Right(&empty)
            .i32_repeated()
            .into_iter()
            .collect_vec()
    );
    assert_eq!(
        vec![1, 2, 5],
        Either::<&Msg, &Msg>::Left(&val_1_2_5)
            .i32_repeated()
            .into_iter()
            .collect_vec()
    );
    assert_eq!(
        vec![1, 2, 5],
        Either::<&Msg, &Msg>::Right(&val_1_2_5)
            .i32_repeated()
            .into_iter()
            .collect_vec()
    );
}
