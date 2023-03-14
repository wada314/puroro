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

use ::puroro_inline::puroro_inline;

puroro_inline! {r#"
syntax = "proto2";
package proto2_defaults;

message Msg {
    optional int32 i32_default = 1;
    optional int32 i32_0 = 2 [default = 0];
    optional int32 i32_42 = 3 [default = 42];
    optional int32 i32_m42 = 4 [default = -42];
    optional int32 i32_2147483647 = 5 [default = 2147483647];
    optional int32 i32_m2147483648 = 6 [default = -2147483648];
    optional int32 i32_0123 = 7 [default = 0123];
    optional int32 i32_0x123 = 8 [default = 0x123];
    
    optional uint32 u32_default = 11;
    optional uint32 u32_0 = 12 [default = 0];
    optional uint32 u32_42 = 13 [default = 42];
    optional uint32 u32_4294967295 = 15 [default = 4294967295];
    optional uint32 u32_0123 = 17 [default = 0123];
    optional uint32 u32_0x123 = 18 [default = 0x123];

    optional int64 i64_default = 21;
    optional int64 i64_0 = 22 [default = 0];
    optional int64 i64_42 = 23 [default = 42];
    optional int64 i64_m42 = 24 [default = -42];
    optional int64 i64_9223372036854775807 = 25 [default = 9223372036854775807];
    optional int64 i64_m9223372036854775808 = 26 [default = -9223372036854775808];
    optional int64 i64_0123 = 27 [default = 0123];
    optional int64 i64_0x123 = 28 [default = 0x123];
    
    optional uint64 u64_default = 31;
    optional uint64 u64_0 = 32 [default = 0];
    optional uint64 u64_42 = 33 [default = 42];
    optional uint64 u64_18446744073709551615 = 35 [default = 18446744073709551615];
    optional uint64 u64_0123 = 37 [default = 0123];
    optional uint64 u64_0x123 = 38 [default = 0x123];

    optional float f32_default = 41;
    optional float f32_0 = 42 [default = 0];
    optional float f32_m0 = 43 [default = -0];
    optional float f32_0p = 44 [default = 0.];
    optional float f32_p0 = 45 [default = .0];
    optional float f32_0p0 = 46 [default = 0.0];
    optional float f32_42 = 47 [default = 42];
    optional float f32_m42 = 48 [default = -42];
    optional float f32_0p25 = 49 [default = 0.25];
    optional float f32_1p5e2 = 50 [default = 1.5e2];
    optional float f32_inf = 51 [default = inf];
    optional float f32_minf = 52 [default = -inf];
    optional float f32_nan = 53 [default = nan];
    optional float f32_mnan = 54 [default = -nan];
    
    optional bool bool_default = 61;
    optional bool bool_true = 62 [default = true];
    optional bool bool_false = 63 [default = false];

    optional string string_default = 71;
    optional string string_empty = 72 [default = ""];
    optional string string_abc = 73 [default = "abc"];
    optional string string_aiu = 74 [default = "あいう"];
    optional string string_backslash = 75 [default = "\\"];
    optional string string_tab = 76 [default = "	"];
    optional string string_crlf = 77 [default = "\r\n"];

    optional bytes bytes_default = 81;
    optional bytes bytes_empty = 82 [default = ""];
    optional bytes bytes_abc = 83 [default = "abc"];
    optional bytes bytes_aiu = 84 [default = "あいう"];
    optional bytes bytes_backslash = 85 [default = "\\"];
    optional bytes bytes_tab = 86 [default = "	"];
    optional bytes bytes_crlf = 87 [default = "\r\n"];

    optional MyEnum enum_default = 91;
    optional MyEnum enum_one = 92 [default = ONE];
    optional MyEnum enum_fourty_two = 93 [default = FOURTY_TWO];
}

enum MyEnum {
    ONE = 1;
    FOURTY_TWO = 42;
}
"#}

use proto2_defaults::{Msg, MyEnum};

#[test]
fn test_simple_int32() {
    let msg = Msg::default();

    // Check struct fields has specified default values
    assert_eq!(0, msg.i32_default());
    assert_eq!(0, msg.i32_0());
    assert_eq!(42, msg.i32_42());
    assert_eq!(-42, msg.i32_m42());
    assert_eq!(2147483647, msg.i32_2147483647());
    assert_eq!(-2147483648, msg.i32_m2147483648());
    assert_eq!(0o123, msg.i32_0123());
    assert_eq!(0x123, msg.i32_0x123());
    assert!(!msg.has_i32_default());
    assert!(!msg.has_i32_0());
    assert!(!msg.has_i32_42());
    assert!(!msg.has_i32_m42());
    assert!(!msg.has_i32_2147483647());
    assert!(!msg.has_i32_m2147483648());
    assert!(!msg.has_i32_0123());
    assert!(!msg.has_i32_0x123());
}

#[test]
fn test_simple_uint32() {
    let msg = Msg::default();

    assert_eq!(0, msg.u32_default());
    assert_eq!(0, msg.u32_0());
    assert_eq!(42, msg.u32_42());
    assert_eq!(4294967295, msg.u32_4294967295());
    assert_eq!(0o123, msg.u32_0123());
    assert_eq!(0x123, msg.u32_0x123());
    assert!(!msg.has_u32_default());
    assert!(!msg.has_u32_0());
    assert!(!msg.has_u32_42());
    assert!(!msg.has_u32_4294967295());
    assert!(!msg.has_u32_0123());
    assert!(!msg.has_u32_0x123());
}

#[test]
fn test_simple_int64() {
    let msg = Msg::default();

    assert_eq!(0, msg.i64_default());
    assert_eq!(0, msg.i64_0());
    assert_eq!(42, msg.i64_42());
    assert_eq!(-42, msg.i64_m42());
    assert_eq!(9223372036854775807, msg.i64_9223372036854775807());
    assert_eq!(-9223372036854775808, msg.i64_m9223372036854775808());
    assert_eq!(0o123, msg.i64_0123());
    assert_eq!(0x123, msg.i64_0x123());
    assert!(!msg.has_i64_default());
    assert!(!msg.has_i64_0());
    assert!(!msg.has_i64_42());
    assert!(!msg.has_i64_m42());
    assert!(!msg.has_i64_9223372036854775807());
    assert!(!msg.has_i64_m9223372036854775808());
    assert!(!msg.has_i64_0123());
    assert!(!msg.has_i64_0x123());
}

#[test]
fn test_simple_uint64() {
    let msg = Msg::default();

    assert_eq!(0, msg.u64_default());
    assert_eq!(0, msg.u64_0());
    assert_eq!(42, msg.u64_42());
    assert_eq!(18446744073709551615, msg.u64_18446744073709551615());
    assert_eq!(0o123, msg.u64_0123());
    assert_eq!(0x123, msg.u64_0x123());
    assert!(!msg.has_u64_default());
    assert!(!msg.has_u64_0());
    assert!(!msg.has_u64_42());
    assert!(!msg.has_u64_18446744073709551615());
    assert!(!msg.has_u64_0123());
    assert!(!msg.has_u64_0x123());
}

#[test]
fn test_simple_float() {
    let msg = Msg::default();

    assert_eq!(0.0, msg.f32_default());
    assert_eq!(0.0, msg.f32_0());
    assert_eq!(-0.0, msg.f32_m0());
    assert_eq!(0.0, msg.f32_0p());
    assert_eq!(0.0, msg.f32_p0());
    assert_eq!(0.0, msg.f32_0p0());
    assert_eq!(42.0, msg.f32_42());
    assert_eq!(-42.0, msg.f32_m42());
    assert_eq!(0.25, msg.f32_0p25());
    assert_eq!(150.0, msg.f32_1p5e2());
    assert_eq!(f32::INFINITY, msg.f32_inf());
    assert_eq!(f32::NEG_INFINITY, msg.f32_minf());
    assert!(msg.f32_nan().is_nan());
    assert!(msg.f32_mnan().is_nan());
    assert!(!msg.has_f32_default());
    assert!(!msg.has_f32_0());
    assert!(!msg.has_f32_m0());
    assert!(!msg.has_f32_0p());
    assert!(!msg.has_f32_p0());
    assert!(!msg.has_f32_0p0());
    assert!(!msg.has_f32_42());
    assert!(!msg.has_f32_m42());
    assert!(!msg.has_f32_0p25());
    assert!(!msg.has_f32_1p5e2());
    assert!(!msg.has_f32_inf());
    assert!(!msg.has_f32_minf());
}

#[test]
fn test_simple_bool() {
    let msg = Msg::default();

    assert_eq!(false, msg.bool_default());
    assert_eq!(false, msg.bool_false());
    assert_eq!(true, msg.bool_true());
    assert!(!msg.has_bool_default());
    assert!(!msg.has_bool_false());
    assert!(!msg.has_bool_true());
}

#[test]
fn test_simple_string() {
    let msg = Msg::default();

    assert_eq!("", msg.string_default());
    assert_eq!("", msg.string_empty());
    assert_eq!("abc", msg.string_abc());
    assert_eq!("あいう", msg.string_aiu());
    assert_eq!("\\", msg.string_backslash());
    assert_eq!("\t", msg.string_tab());
    assert_eq!("\r\n", msg.string_crlf());
    assert!(!msg.has_string_default());
    assert!(!msg.has_string_empty());
    assert!(!msg.has_string_abc());
    assert!(!msg.has_string_aiu());
    assert!(!msg.has_string_backslash());
    assert!(!msg.has_string_tab());
    assert!(!msg.has_string_crlf());
}

#[test]
fn test_simple_bytes() {
    let msg = Msg::default();

    assert_eq!(b"", msg.bytes_default());
    assert_eq!(b"", msg.bytes_empty());
    assert_eq!(b"abc", msg.bytes_abc());
    assert_eq!("あいう".as_bytes(), msg.bytes_aiu());
    assert_eq!(b"\\", msg.bytes_backslash());
    assert_eq!(b"\t", msg.bytes_tab());
    assert_eq!(b"\r\n", msg.bytes_crlf());
    assert!(!msg.has_bytes_default());
    assert!(!msg.has_bytes_empty());
    assert!(!msg.has_bytes_abc());
    assert!(!msg.has_bytes_aiu());
    assert!(!msg.has_bytes_backslash());
    assert!(!msg.has_bytes_tab());
    assert!(!msg.has_bytes_crlf());
}

#[test]
fn test_simple_enum() {
    let msg = Msg::default();

    assert_eq!(MyEnum::One, msg.enum_default());
    assert_eq!(MyEnum::One, msg.enum_one());
    assert_eq!(MyEnum::FourtyTwo, msg.enum_fourty_two());
    assert!(!msg.has_enum_default());
    assert!(!msg.has_enum_one());
    assert!(!msg.has_enum_fourty_two());
}
