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

#![allow(unused)]

mod cases;

use ::puroro_inline::puroro_inline;
use ::tests_pb;

puroro_inline! { r#"
syntax = "proto3";
message SampleMessage {
    uint32 year = 1;
}
"# }

puroro_inline! { r#"
syntax = "proto3";
message SampleMessage2 {
    uint32 year = 1;
}
"# }

#[test]
fn test_inline_protobuf() {
    let mut sample = SampleMessage::default();
    *sample.year_mut() = 2022;
    assert_eq!(2022, sample.year());
}
