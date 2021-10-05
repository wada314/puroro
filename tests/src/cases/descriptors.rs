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

use ::puroro::{Either, Message};
use ::std::ptr;
use ::tests_pb::full_coverage3::Msg;

#[test]
fn test_get_msg_descriptor() {
    // Descriptor name
    assert_eq!("Msg", Msg::descriptor().name());
    assert_eq!("Msg", <() as Message<Msg>>::descriptor().name());
    assert_eq!("Msg", <((), Msg)>::descriptor().name());
    assert_eq!("Msg", <(Msg, ())>::descriptor().name());
    assert_eq!("Msg", Either::<Msg, ()>::descriptor().name());
    assert_eq!("Msg", Either::<(), Msg>::descriptor().name());
    // Descriptor ptr eq
    assert!(ptr::eq(
        Msg::descriptor(),
        <() as Message<Msg>>::descriptor()
    ));
    assert!(ptr::eq(Msg::descriptor(), <((), Msg)>::descriptor()));
    assert!(ptr::eq(Msg::descriptor(), <(Msg, ())>::descriptor()));
    assert!(ptr::eq(
        Msg::descriptor(),
        <Either::<(), Msg>>::descriptor()
    ));
    assert!(ptr::eq(
        Msg::descriptor(),
        <Either::<Msg, ()>>::descriptor()
    ));
}

#[test]
fn test_get_field_descriptor() {
    let fields = Msg::descriptor().fields();
    assert_eq!(
        1,
        fields
            .iter()
            .filter(|f| f.name() == "i32_unlabeled" && f.number() == 1)
            .count()
    );
}
