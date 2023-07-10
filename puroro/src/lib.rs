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

#![feature(allocator_api)]
#![feature(slice_as_chunks)]
#![feature(assert_matches)]

pub mod internal;
pub mod string;

use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserError {
    #[error("io error")]
    IoError(#[from] ::std::io::Error),
    #[error("Deserializing invalid variant (too long).")]
    InvalidVariant,
}
pub type DeserResult<T> = ::std::result::Result<T, DeserError>;

#[derive(Error, Debug)]
pub enum SerError {
    #[error("io error")]
    IoError(#[from] ::std::io::Error),
}
pub type SerResult<T> = ::std::result::Result<T, SerError>;

struct Foo<T1, T2> {
    item1: T1,
    item2: T2,
}
impl<T1: Default, T2: Default> Default for Foo<T1, T2> {
    fn default() -> Self {
        Self {
            item1: Default::default(),
            item2: Default::default(),
        }
    }
}
fn foo(foo: Foo<usize, i32>) {
    unimplemented!()
}
fn hoge() {
    let foovar = Foo {
        item1: 0,
        ..Default::default()
    };
    foo(foovar);
}
