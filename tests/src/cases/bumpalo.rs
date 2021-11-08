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

use ::puroro::bumpalo::Bump;
use ::puroro::Message;
use ::std::io::Read;
use ::tests_pb::official_samples3::*;

fn is_test1_trait<T: Test1Trait>(t: &T) -> bool {
    true
}

#[test]
fn test_owned() {
    let mut t1 = Test1BumpaloOwned::new();
    t1.merge_from_bytes([0x08, 0x01].bytes());
    assert_eq!(1, t1.a());

    let mut t3 = Test3BumpaloOwned::new();
    t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
    assert_eq!(1, t3.c().a());

    assert!(is_test1_trait(&t1));
}

#[test]
fn test_ref() {
    let bump = Bump::new();
    let mut t1 = Test1Bumpalo::new_in(&bump);
    t1.merge_from_bytes([0x08, 0x01].bytes());
    assert_eq!(1, t1.a());

    let mut t3 = Test3Bumpalo::new_in(&bump);
    t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
    assert_eq!(1, t3.c().a());

    assert!(is_test1_trait(&t1));
}
