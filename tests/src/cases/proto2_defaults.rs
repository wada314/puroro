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

use crate::tests_pb::proto2_defaults::{Msg, MyEnum};
use ::puroro::internal::de::DeserMessageFromBytesIter;

#[test]
fn test_simple_int32() {
    let mut msg = Msg::default();

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

    // When the fields are explicitly set to None,
    msg.clear_i32_default();
    msg.clear_i32_0();
    msg.clear_i32_42();
    msg.clear_i32_m42();
    msg.clear_i32_2147483647();
    msg.clear_i32_m2147483648();
    msg.clear_i32_0123();
    msg.clear_i32_0x123();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None,
    msg.clear_u32_default();
    msg.clear_u32_0();
    msg.clear_u32_42();
    msg.clear_u32_4294967295();
    msg.clear_u32_0123();
    msg.clear_u32_0x123();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None,
    msg.clear_i64_default();
    msg.clear_i64_0();
    msg.clear_i64_42();
    msg.clear_i64_m42();
    msg.clear_i64_9223372036854775807();
    msg.clear_i64_m9223372036854775808();
    msg.clear_i64_0123();
    msg.clear_i64_0x123();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None,
    msg.clear_u64_default();
    msg.clear_u64_0();
    msg.clear_u64_42();
    msg.clear_u64_18446744073709551615();
    msg.clear_u64_0123();
    msg.clear_u64_0x123();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None, then it's None anyway.
    msg.clear_f32_default();
    msg.clear_f32_0();
    msg.clear_f32_m0();
    msg.clear_f32_0p();
    msg.clear_f32_p0();
    msg.clear_f32_0p0();
    msg.clear_f32_42();
    msg.clear_f32_m42();
    msg.clear_f32_0p25();
    msg.clear_f32_1p5e2();
    msg.clear_f32_inf();
    msg.clear_f32_minf();
    msg.clear_f32_nan();
    msg.clear_f32_mnan();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(false, msg.bool_default());
    assert_eq!(false, msg.bool_false());
    assert_eq!(true, msg.bool_true());
    assert!(!msg.has_bool_default());
    assert!(!msg.has_bool_false());
    assert!(!msg.has_bool_true());

    // When the fields are explicitly set to None,
    msg.clear_bool_default();
    msg.clear_bool_false();
    msg.clear_bool_true();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
    assert_eq!(false, msg.bool_default());
    assert_eq!(false, msg.bool_false());
    assert_eq!(true, msg.bool_true());
    assert!(!msg.has_bool_default());
    assert!(!msg.has_bool_false());
    assert!(!msg.has_bool_true());
}

#[test]
fn test_simple_string() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None,
    msg.clear_string_default();
    msg.clear_string_empty();
    msg.clear_string_abc();
    msg.clear_string_aiu();
    msg.clear_string_backslash();
    msg.clear_string_tab();
    msg.clear_string_crlf();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
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

    // When the fields are explicitly set to None,
    msg.clear_bytes_default();
    msg.clear_bytes_empty();
    msg.clear_bytes_abc();
    msg.clear_bytes_aiu();
    msg.clear_bytes_backslash();
    msg.clear_bytes_tab();
    msg.clear_bytes_crlf();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
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
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(MyEnum::One, msg.enum_default());
    assert_eq!(MyEnum::One, msg.enum_one());
    assert_eq!(MyEnum::FourtyTwo, msg.enum_fourty_two());
    assert!(!msg.has_enum_default());
    assert!(!msg.has_enum_one());
    assert!(!msg.has_enum_fourty_two());

    // When the fields are explicitly set to None,
    msg.clear_enum_default();
    msg.clear_enum_one();
    msg.clear_enum_fourty_two();

    // the trait methods still return the default values.
    // And has_xxx methods return false.
    assert_eq!(MyEnum::One, msg.enum_default());
    assert_eq!(MyEnum::One, msg.enum_one());
    assert_eq!(MyEnum::FourtyTwo, msg.enum_fourty_two());
    assert!(!msg.has_enum_default());
    assert!(!msg.has_enum_one());
    assert!(!msg.has_enum_fourty_two());
}

#[test]
fn test_empty_int32() {
    use crate::tests_pb::proto2_defaults::MsgTrait as _;

    let msg = ();

    // The trait values are "not set" but return the default values
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
