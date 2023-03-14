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

#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

use ::puroro_inline::puroro_inline2;
use ::std::convert::{TryFrom, TryInto};

puroro_inline2! {
    syntax = "proto2";
    package enum2;

    enum Enum {
        VALUE_SEVEN = 7;
        VALUE_ZERO = 0;
        VALUE_ONE = 1;
        VALUE_FOURTY_TWO = 42;
    }
}

use enum2::Enum as Enum2;

#[test]
fn test_enum2_default() {
    assert_eq!(Enum2::ValueSeven, Enum2::default());
}

#[test]
fn test_enum2_convert() {
    assert_eq!(Some(Enum2::ValueSeven), 7i32.try_into().ok());
    assert_eq!(Some(Enum2::ValueZero), 0i32.try_into().ok());
    assert_eq!(Some(Enum2::ValueFourtyTwo), 42i32.try_into().ok());
    assert!(Enum2::try_from(12345).is_err());

    assert_eq!(7, Enum2::ValueSeven.into());
    assert_eq!(0, Enum2::ValueZero.into());
    assert_eq!(42, Enum2::ValueFourtyTwo.into());
}

puroro_inline2! {
    syntax = "proto3";
    package enum3;

    enum Enum {
        VALUE_ZERO = 0;
        VALUE_SEVEN = 7;
        VALUE_ONE = 1;
        VALUE_FOURTY_TWO = 42;
    }
}

use enum3::Enum as Enum3;

#[test]
fn test_enum3_default() {
    assert_eq!(Enum3::ValueZero, Enum3::default());
}

#[test]
fn test_enum3_convert() {
    assert_eq!(Enum3::ValueSeven, 7.into());
    assert_eq!(Enum3::ValueZero, 0.into());
    assert_eq!(Enum3::ValueFourtyTwo, 42.into());
    assert_eq!(Enum3::_None(12345), 12345.into());

    assert_eq!(7, Enum3::ValueSeven.into());
    assert_eq!(0, Enum3::ValueZero.into());
    assert_eq!(42, Enum3::ValueFourtyTwo.into());
}
