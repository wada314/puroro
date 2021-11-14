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
use ::puroro::{BumpBox, BumpRc, BumpRef, Message};
use ::std::io::Read;
use ::std::rc::Rc;
use ::tests_pb::official_samples3::*;
use puroro::BumpaloMessage;

fn is_test1_trait<T: Test1Trait>(t: &T) -> bool {
    true
}

#[test]
fn test_ref() {
    let bump = Bump::new();
    let mut t1 = Test1Bumpalo::<BumpRef>::new_in(&bump);
    t1.merge_from_bytes([0x08, 0x01].bytes());
    assert_eq!(1, t1.a());

    let mut t3 = Test3Bumpalo::<BumpRef>::new_in(&bump);
    t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
    assert_eq!(1, t3.c().a());

    assert!(is_test1_trait(&t1));
}

#[test]
fn test_rc() {
    let bump_rc = Rc::new(Bump::new());
    let mut t1 = Test1Bumpalo::<BumpRc>::new_in(bump_rc.clone());
    t1.merge_from_bytes([0x08, 0x01].bytes());
    assert_eq!(1, t1.a());

    let mut t3 = Test3Bumpalo::<BumpRc>::new_in(bump_rc.clone());
    t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
    assert_eq!(1, t3.c().a());

    // [local var, t1._bump, t3._bump, t3.c._bump]
    assert_eq!(4, Rc::strong_count(&bump_rc));
    assert!(is_test1_trait(&t1));
}

#[test]
fn test_box() {
    let bump_box = Box::new(Bump::new());
    let mut t1 = Test1Bumpalo::<BumpBox>::new_in(bump_box);
    t1.merge_from_bytes([0x08, 0x01].bytes());
    assert_eq!(1, t1.a());

    let bump_box = Box::new(Bump::new());
    let mut t3 = Test3Bumpalo::<BumpBox>::new_in(bump_box);
    t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
    assert_eq!(1, t3.c().a());
    let t1: &Test1Bumpalo<BumpRef> = t3.c().unwrap();

    assert!(is_test1_trait(&t1));
}

/// ```compile_fail
/// let bump_boxed = Box::new(Bump::new());
/// let t1 = {
///     let mut t3 = Test3Bumpalo::<BumpBox>::new_in(bump_boxed);
///     t3.merge_from_bytes([0x1a, 0x02, 0x08, 0x01].bytes());
///     assert_eq!(1, t3.c().a());
///     t3.c()
/// }; // <- t1 cannot leave here because the owner t3 gets dropped!
/// assert!(is_test1_trait(&t1));
/// ```
#[cfg(doctest)]
fn doctest_box_fails() {}
