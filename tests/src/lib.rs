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
use ::tests_pb;

use ::puroro_inline::inline;

inline! { r#"
syntax = "proto3";
message FooBar {
    optional int32 yeah = 1;
}
"# }

inline! { r#"
syntax = "proto3";
message FooBar2 {
    optional int32 yeah = 1;
}
"# }

#[test]
fn test_inline() {
    let gen = FooBar::default();
}
