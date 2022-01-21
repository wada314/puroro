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

#[test]
fn test_basic1() {
    use crate::tests_pb::official_samples2::*;
    let m = Test2Simple2::default();
    let so = m.b();
}

#[test]
fn test_basic2() {
    use crate::tests_pb::full_coverage2::MsgSimple2 as Msg;
    let mut m = Msg::default();
    assert_eq!(0, m.i32_optional());
    *m.i32_optional_mut() = 10;
    assert_eq!(10, m.i32_optional());
    m.i32_repeated_mut().extend([10, 20]);
    assert_eq!(&[10, 20], m.i32_repeated());
}
