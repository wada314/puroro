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

use crate::tests_pb::proto2_defaults::{Msg, MsgTrait, MyEnum};
use ::puroro::internal::DeserializableMessageFromBytesIterator;

#[test]
fn test_simple_int32() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(None, msg.i32_default);
    assert_eq!(Some(0), msg.i32_0);
    assert_eq!(Some(42), msg.i32_42);
    assert_eq!(Some(-42), msg.i32_m42);
    assert_eq!(Some(2147483647), msg.i32_2147483647);
    assert_eq!(Some(-2147483648), msg.i32_m2147483648);
    assert_eq!(Some(0o123), msg.i32_0123);
    assert_eq!(Some(0x123), msg.i32_0x123);
    assert_eq!(None, msg.i32_default());
    assert_eq!(Some(0), msg.i32_0());
    assert_eq!(Some(42), msg.i32_42());
    assert_eq!(Some(-42), msg.i32_m42());
    assert_eq!(Some(2147483647), msg.i32_2147483647());
    assert_eq!(Some(-2147483648), msg.i32_m2147483648());
    assert_eq!(Some(0o123), msg.i32_0123());
    assert_eq!(Some(0x123), msg.i32_0x123());

    // When the fields are explicitly set to None, then it's None anyway.
    msg.i32_default = None;
    msg.i32_0 = None;
    msg.i32_42 = None;
    msg.i32_m42 = None;
    msg.i32_2147483647 = None;
    msg.i32_m2147483648 = None;
    msg.i32_0123 = None;
    msg.i32_0x123 = None;
    assert_eq!(None, msg.i32_default);
    assert_eq!(None, msg.i32_0);
    assert_eq!(None, msg.i32_42);
    assert_eq!(None, msg.i32_m42);
    assert_eq!(None, msg.i32_2147483647);
    assert_eq!(None, msg.i32_m2147483648);
    assert_eq!(None, msg.i32_0123);
    assert_eq!(None, msg.i32_0x123);

    // But the trait methods will return the default values.
    assert_eq!(None, msg.i32_default());
    assert_eq!(Some(0), msg.i32_0());
    assert_eq!(Some(42), msg.i32_42());
    assert_eq!(Some(-42), msg.i32_m42());
    assert_eq!(Some(2147483647), msg.i32_2147483647());
    assert_eq!(Some(-2147483648), msg.i32_m2147483648());
    assert_eq!(Some(0o123), msg.i32_0123());
    assert_eq!(Some(0x123), msg.i32_0x123());
}

#[test]
fn test_simple_uint32() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(None, msg.u32_default);
    assert_eq!(Some(0), msg.u32_0);
    assert_eq!(Some(42), msg.u32_42);
    assert_eq!(Some(4294967295), msg.u32_4294967295);
    assert_eq!(Some(0o123), msg.u32_0123);
    assert_eq!(Some(0x123), msg.u32_0x123);
    assert_eq!(None, msg.u32_default());
    assert_eq!(Some(0), msg.u32_0());
    assert_eq!(Some(42), msg.u32_42());
    assert_eq!(Some(4294967295), msg.u32_4294967295());
    assert_eq!(Some(0o123), msg.u32_0123());
    assert_eq!(Some(0x123), msg.u32_0x123());

    // When the fields are explicitly set to None, then it's None anyway.
    msg.u32_default = None;
    msg.u32_0 = None;
    msg.u32_42 = None;
    msg.u32_4294967295 = None;
    msg.u32_0123 = None;
    msg.u32_0x123 = None;
    assert_eq!(None, msg.u32_default);
    assert_eq!(None, msg.u32_0);
    assert_eq!(None, msg.u32_42);
    assert_eq!(None, msg.u32_4294967295);
    assert_eq!(None, msg.u32_0123);
    assert_eq!(None, msg.u32_0x123);

    // But the trait methods will return the default values.
    assert_eq!(None, msg.u32_default());
    assert_eq!(None, msg.u32_0());
    assert_eq!(None, msg.u32_42());
    assert_eq!(None, msg.u32_4294967295());
    assert_eq!(None, msg.u32_0123());
    assert_eq!(None, msg.u32_0x123());
}

#[test]
fn test_simple_int64() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(None, msg.i64_default);
    assert_eq!(Some(0), msg.i64_0);
    assert_eq!(Some(42), msg.i64_42);
    assert_eq!(Some(-42), msg.i64_m42);
    assert_eq!(Some(9223372036854775807), msg.i64_9223372036854775807);
    assert_eq!(Some(-9223372036854775808), msg.i64_m9223372036854775808);
    assert_eq!(Some(0o123), msg.i64_0123);
    assert_eq!(Some(0x123), msg.i64_0x123);
    assert_eq!(None, msg.i64_default());
    assert_eq!(Some(0), msg.i64_0());
    assert_eq!(Some(42), msg.i64_42());
    assert_eq!(Some(-42), msg.i64_m42());
    assert_eq!(Some(9223372036854775807), msg.i64_9223372036854775807());
    assert_eq!(Some(-9223372036854775808), msg.i64_m9223372036854775808());
    assert_eq!(Some(0o123), msg.i64_0123());
    assert_eq!(Some(0x123), msg.i64_0x123());

    // When the fields are explicitly set to None, then it's None anyway.
    msg.i64_default = None;
    msg.i64_0 = None;
    msg.i64_42 = None;
    msg.i64_m42 = None;
    msg.i64_9223372036854775807 = None;
    msg.i64_m9223372036854775808 = None;
    msg.i64_0123 = None;
    msg.i64_0x123 = None;
    assert_eq!(None, msg.i64_default);
    assert_eq!(None, msg.i64_0);
    assert_eq!(None, msg.i64_42);
    assert_eq!(None, msg.i64_m42);
    assert_eq!(None, msg.i64_9223372036854775807);
    assert_eq!(None, msg.i64_m9223372036854775808);
    assert_eq!(None, msg.i64_0123);
    assert_eq!(None, msg.i64_0x123);

    // But the trait methods will return the default values.
    assert_eq!(None, msg.i64_default());
    assert_eq!(None, msg.i64_0());
    assert_eq!(None, msg.i64_42());
    assert_eq!(None, msg.i64_m42());
    assert_eq!(None, msg.i64_9223372036854775807());
    assert_eq!(None, msg.i64_m9223372036854775808());
    assert_eq!(None, msg.i64_0123());
    assert_eq!(None, msg.i64_0x123());
}

#[test]
fn test_simple_uint64() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(None, msg.u64_default);
    assert_eq!(Some(0), msg.u64_0);
    assert_eq!(Some(42), msg.u64_42);
    assert_eq!(Some(18446744073709551615), msg.u64_18446744073709551615);
    assert_eq!(Some(0o123), msg.u64_0123);
    assert_eq!(Some(0x123), msg.u64_0x123);
    assert_eq!(None, msg.u64_default());
    assert_eq!(Some(0), msg.u64_0());
    assert_eq!(Some(42), msg.u64_42());
    assert_eq!(Some(18446744073709551615), msg.u64_18446744073709551615());
    assert_eq!(Some(0o123), msg.u64_0123());
    assert_eq!(Some(0x123), msg.u64_0x123());

    // When the fields are explicitly set to None, then it's None anyway.
    msg.u64_default = None;
    msg.u64_0 = None;
    msg.u64_42 = None;
    msg.u64_18446744073709551615 = None;
    msg.u64_0123 = None;
    msg.u64_0x123 = None;
    assert_eq!(None, msg.u64_default);
    assert_eq!(None, msg.u64_0);
    assert_eq!(None, msg.u64_42);
    assert_eq!(None, msg.u64_18446744073709551615);
    assert_eq!(None, msg.u64_0123);
    assert_eq!(None, msg.u64_0x123);

    // But the trait methods will return the default values.
    assert_eq!(None, msg.u64_default());
    assert_eq!(None, msg.u64_0());
    assert_eq!(None, msg.u64_42());
    assert_eq!(None, msg.u64_18446744073709551615());
    assert_eq!(None, msg.u64_0123());
    assert_eq!(None, msg.u64_0x123());
}

#[test]
fn test_simple_float() {
    let mut msg = Msg::default();

    // Check struct fields are initialized by the specified default values
    assert_eq!(None, msg.f32_default);
    assert_eq!(Some(0.0), msg.f32_0);
    assert_eq!(Some(-0.0), msg.f32_m0);
    assert_eq!(Some(0.0), msg.f32_0p);
    assert_eq!(Some(0.0), msg.f32_p0);
    assert_eq!(Some(0.0), msg.f32_0p0);
    assert_eq!(Some(42.0), msg.f32_42);
    assert_eq!(Some(-42.0), msg.f32_m42);
    assert_eq!(Some(0.25), msg.f32_0p25);
    assert_eq!(Some(150.0), msg.f32_1p5e2);
    assert_eq!(Some(f32::INFINITY), msg.f32_inf);
    assert_eq!(Some(f32::NEG_INFINITY), msg.f32_minf);
    assert!(msg.f32_nan.unwrap_or(0.0).is_nan());
    assert!(msg.f32_mnan.unwrap_or(0.0).is_nan());
    assert_eq!(None, msg.f32_default());
    assert_eq!(Some(0.0), msg.f32_0());
    assert_eq!(Some(-0.0), msg.f32_m0());
    assert_eq!(Some(0.0), msg.f32_0p());
    assert_eq!(Some(0.0), msg.f32_p0());
    assert_eq!(Some(0.0), msg.f32_0p0());
    assert_eq!(Some(42.0), msg.f32_42());
    assert_eq!(Some(-42.0), msg.f32_m42());
    assert_eq!(Some(0.25), msg.f32_0p25());
    assert_eq!(Some(150.0), msg.f32_1p5e2());
    assert_eq!(Some(f32::INFINITY), msg.f32_inf());
    assert_eq!(Some(f32::NEG_INFINITY), msg.f32_minf());
    assert!(msg.f32_nan().unwrap_or(0.0).is_nan());
    assert!(msg.f32_mnan().unwrap_or(0.0).is_nan());

    // When the fields are explicitly set to None, then it's None anyway.
    msg.f32_default = None;
    msg.f32_0 = None;
    msg.f32_m0 = None;
    msg.f32_0p = None;
    msg.f32_p0 = None;
    msg.f32_0p0 = None;
    msg.f32_42 = None;
    msg.f32_m42 = None;
    msg.f32_0p25 = None;
    msg.f32_1p5e2 = None;
    msg.f32_inf = None;
    msg.f32_minf = None;
    msg.f32_nan = None;
    msg.f32_mnan = None;

    // But the trait methods will return the default values.
}
