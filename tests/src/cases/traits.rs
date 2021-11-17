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

use crate::tests_pb::full_coverage3::msg::{Submsg, SubmsgTrait};
use crate::tests_pb::full_coverage3::{Enum, Msg, MsgTrait};

#[test]
fn test_read_trait() {
    let mut msg: Msg = Msg::default();
    *msg.i32_unlabeled_mut() = 10;
    assert_eq!(10, msg.i32_unlabeled());
}
