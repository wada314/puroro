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

use ::puroro::internal::SerializableMessageToIoWrite;
use ::tests_pb::full_coverage2::MsgTrait as MsgTrait2;
use ::tests_pb::full_coverage3::Enum as Enum3;
use ::tests_pb::full_coverage3::MsgTrait as MsgTrait3;

#[test]
fn test_getters2() {
    assert!(<() as MsgTrait2>::i32_required(&()).is_none());
    assert!(<() as MsgTrait2>::i32_optional(&()).is_none());
    assert_eq!(0, <() as MsgTrait2>::i32_repeated(&()).into_iter().count());
    assert!(<() as MsgTrait2>::enum_required(&()).is_none());
    assert!(<() as MsgTrait2>::enum_optional(&()).is_none());
    assert_eq!(0, <() as MsgTrait2>::enum_repeated(&()).into_iter().count());
    assert!(<() as MsgTrait2>::string_required(&()).is_none());
    assert!(<() as MsgTrait2>::string_optional(&()).is_none());
    assert_eq!(
        0,
        <() as MsgTrait2>::string_repeated(&()).into_iter().count()
    );
    assert!(<() as MsgTrait2>::submsg_required(&()).is_none());
    assert!(<() as MsgTrait2>::submsg_optional(&()).is_none());
    assert_eq!(
        0,
        <() as MsgTrait2>::submsg_repeated(&()).into_iter().count()
    );
}

#[test]
fn test_getters3() {
    assert_eq!(0, <() as MsgTrait3>::i32_unlabeled(&()));
    assert!(<() as MsgTrait3>::i32_optional(&()).is_none());
    assert_eq!(0, <() as MsgTrait3>::i32_repeated(&()).into_iter().count());
    assert_eq!(Enum3::Zeroth, <() as MsgTrait3>::enum_unlabeled(&()));
    assert!(<() as MsgTrait3>::enum_optional(&()).is_none());
    assert_eq!(0, <() as MsgTrait3>::enum_repeated(&()).into_iter().count());
    assert_eq!("", <() as MsgTrait3>::string_unlabeled(&()));
    assert!(<() as MsgTrait3>::string_optional(&()).is_none());
    assert_eq!(
        0,
        <() as MsgTrait3>::string_repeated(&()).into_iter().count()
    );
    assert!(<() as MsgTrait3>::submsg_unlabeled(&()).is_none());
    assert!(<() as MsgTrait3>::submsg_optional(&()).is_none());
    assert_eq!(
        0,
        <() as MsgTrait3>::submsg_repeated(&()).into_iter().count()
    );
}

#[test]
fn test_ser() {
    let mut buffer = Vec::new();
    <() as SerializableMessageToIoWrite>::ser(&(), &mut buffer).unwrap();
    assert_eq!(0, buffer.len());
}
