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

use tests_pb::full_coverage3::MsgTrait;

use crate::tests_pb::full_coverage2::MsgTemplate as Msg2;

#[test]
fn test_basic() {
    let m = Msg2::<
        ::puroro::SimpleImpl,
        ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
    >::default();
    let i = m.string_optional();
    let i2 = i.clone();
}
